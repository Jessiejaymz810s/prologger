//! Asynchronous sink wrapper.
//!
//! Wraps any `Sink` to perform disk/console I/O on a background thread.
//! This prevents slow I/O from blocking the main application thread.

use super::Sink;
use crate::formatter::Format;
use log::{Level, Record};
use std::sync::mpsc::{self, SyncSender};
use std::thread;

/// An owned representation of a log record for passing across threads.
struct OwnedRecord {
    level: Level,
    target: String,
    file: Option<String>,
    line: Option<u32>,
    module_path: Option<String>,
    formatted: String,
}

impl OwnedRecord {
    fn from_record(record: &Record, formatter: &dyn Format) -> Self {
        Self {
            level: record.level(),
            target: record.target().to_string(),
            file: record.file().map(|s| s.to_string()),
            line: record.line(),
            module_path: record.module_path().map(|s| s.to_string()),
            formatted: formatter.format(record),
        }
    }
}

/// A dummy formatter that just returns the pre-formatted string.
struct Preformatted<'a>(&'a str);

impl<'a> Format for Preformatted<'a> {
    fn format(&self, _record: &Record) -> String {
        self.0.to_string()
    }
}

enum AsyncMessage {
    Log(OwnedRecord),
    Flush(mpsc::Sender<()>),
    Shutdown,
}

/// A sink that offloads formatting and I/O to a background thread.
pub struct AsyncSink {
    sender: SyncSender<AsyncMessage>,
}

impl AsyncSink {
    /// Wraps a list of sinks, spawning a background worker thread.
    ///
    /// The channel has a bounded capacity. If the channel fills up
    /// (e.g., during a massive burst of logs where disk I/O cannot keep up),
    /// new log records will be dropped to prevent blocking the main thread.
    pub fn new(inners: Vec<Box<dyn Sink>>, capacity: usize) -> Self {
        let (sender, receiver) = mpsc::sync_channel::<AsyncMessage>(capacity);

        thread::Builder::new()
            .name("prologger-async-worker".to_string())
            .spawn(move || {
                for msg in receiver {
                    match msg {
                        AsyncMessage::Log(owned) => {
                            let mut builder = log::Record::builder();
                            builder
                                .level(owned.level)
                                .target(&owned.target)
                                .file(owned.file.as_deref())
                                .line(owned.line)
                                .module_path(owned.module_path.as_deref());

                            let record = builder.args(format_args!("")).build();
                            let formatter = Preformatted(&owned.formatted);
                            for inner in &inners {
                                inner.write(&record, &formatter);
                            }
                        }
                        AsyncMessage::Flush(reply) => {
                            for inner in &inners {
                                inner.flush();
                            }
                            let _ = reply.send(());
                        }
                        AsyncMessage::Shutdown => {
                            for inner in &inners {
                                inner.flush();
                            }
                            break;
                        }
                    }
                }
            })
            .expect("Failed to spawn prologger async worker thread");

        Self { sender }
    }
}

impl Sink for AsyncSink {
    fn write(&self, record: &Record, formatter: &dyn Format) {
        let owned = OwnedRecord::from_record(record, formatter);
        // We use try_send to avoid blocking the main thread if the queue is full.
        // Logs will be dropped if the worker thread is overwhelmed.
        let _ = self.sender.try_send(AsyncMessage::Log(owned));
    }

    fn flush(&self) {
        let (tx, rx) = mpsc::channel();
        if self.sender.send(AsyncMessage::Flush(tx)).is_ok() {
            let _ = rx.recv();
        }
    }
}

impl Drop for AsyncSink {
    fn drop(&mut self) {
        let _ = self.sender.send(AsyncMessage::Shutdown);
    }
}
