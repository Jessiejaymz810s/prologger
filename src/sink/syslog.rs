//! Syslog sink — writes log output to the local syslog daemon.

use super::Sink;
use crate::formatter::Format;
use log::Record;
use std::sync::Mutex;
use syslog::{Facility, Formatter3164};

/// A sink that writes formatted log output to the local syslog daemon.
pub struct SyslogSink {
    logger: Mutex<syslog::Logger<syslog::LoggerBackend, Formatter3164>>,
}

impl SyslogSink {
    /// Creates a new syslog sink connected to the local syslog daemon.
    pub fn new(process_name: String) -> Result<Self, String> {
        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: process_name,
            pid: std::process::id(),
        };

        let logger = syslog::unix(formatter).map_err(|e| e.to_string())?;

        Ok(Self {
            logger: Mutex::new(logger),
        })
    }
}

impl Sink for SyslogSink {
    fn write(&self, record: &Record, formatter: &dyn Format) {
        let formatted = formatter.format(record);
        if let Ok(mut logger) = self.logger.lock() {
            match record.level() {
                log::Level::Error => {
                    let _ = logger.err(formatted);
                }
                log::Level::Warn => {
                    let _ = logger.warning(formatted);
                }
                log::Level::Info => {
                    let _ = logger.info(formatted);
                }
                log::Level::Debug => {
                    let _ = logger.debug(formatted);
                }
                log::Level::Trace => {
                    let _ = logger.debug(formatted);
                } // syslog has no trace
            };
        }
    }

    fn flush(&self) {}
}
