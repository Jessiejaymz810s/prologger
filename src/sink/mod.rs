//! Log output sinks.
//!
//! Sinks are destinations where formatted log messages are written.
//! Multiple sinks can be active simultaneously to send logs to
//! different destinations.

pub mod console;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "async")]
pub mod async_sink;
#[cfg(feature = "syslog")]
pub mod syslog;

use crate::formatter::Format;
use log::Record;

/// Trait for log output destinations.
///
/// A sink receives formatted log records and writes them to their
/// underlying destination (console, file, network, etc.).
pub trait Sink: Send + Sync {
    /// Writes a formatted log record to this sink.
    fn write(&self, record: &Record, formatter: &dyn Format);

    /// Flushes any buffered output.
    fn flush(&self);
}
