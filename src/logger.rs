//! Core logger implementation.
//!
//! Contains the `ProLogger` struct which implements the `log::Log` trait
//! and dispatches records to configured sinks through a formatter.

use crate::filter::Filter;
use crate::formatter::Format;
use crate::sink::Sink;
use log::{Log, Metadata, Record, SetLoggerError};

/// A configured logger instance ready to process log records.
///
/// `ProLogger` implements the `log::Log` trait and can be installed
/// as the global logger using the the `init` method.
///
/// Use [`ProLoggerBuilder`](crate::builder::ProLoggerBuilder) to construct instances.
pub struct ProLogger {
    /// The filter engine for level-based filtering.
    pub(crate) filter: Filter,
    /// The formatter for serializing log records.
    pub(crate) formatter: Box<dyn Format>,
    /// The output sinks where formatted records are written.
    pub(crate) sinks: Vec<Box<dyn Sink>>,
}

impl ProLogger {
    /// Creates a new `ProLoggerBuilder` for configuring a logger.
    pub fn builder() -> crate::builder::ProLoggerBuilder {
        crate::builder::ProLoggerBuilder::new()
    }

    /// Installs this logger as the global `log` facade logger.
    ///
    /// This consumes the logger and sets it as the global logger.
    /// This function can only be called once; subsequent calls will
    /// return an error.
    ///
    /// # Errors
    ///
    /// Returns `SetLoggerError` if a global logger has already been set.
    pub fn init(self) -> Result<(), SetLoggerError> {
        let max_level = self.filter.global_level();
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(max_level))
    }
}

// ProLogger is Send + Sync because all its fields are Send + Sync
// (Format and Sink traits require Send + Sync)
unsafe impl Send for ProLogger {}
unsafe impl Sync for ProLogger {}

impl Log for ProLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.is_enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        for sink in &self.sinks {
            sink.write(record, self.formatter.as_ref());
        }
    }

    fn flush(&self) {
        for sink in &self.sinks {
            sink.flush();
        }
    }
}

impl std::fmt::Debug for ProLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProLogger")
            .field("filter", &self.filter)
            .field("sinks_count", &self.sinks.len())
            .finish()
    }
}
