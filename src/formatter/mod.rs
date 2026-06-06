//! Log record formatters.
//!
//! Formatters control how log records are serialized into strings
//! before being written to a sink.

pub mod compact;
#[cfg(feature = "json")]
pub mod json;
pub mod pretty;

use log::Record;

/// Trait for formatting log records into strings.
///
/// Implementors of this trait control the output format of log messages.
pub trait Format: Send + Sync {
    /// Formats a log record into a string.
    ///
    /// The returned string should include a trailing newline if appropriate.
    fn format(&self, record: &Record) -> String;
}

/// Available built-in formatter types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatterType {
    /// Human-readable format with timestamps and optional colors.
    Pretty,
    /// Minimal single-character level prefix format.
    Compact,
    /// Machine-readable JSON format (requires `json` feature).
    #[cfg(feature = "json")]
    Json,
}

/// Creates a boxed formatter from a formatter type.
pub fn create_formatter(formatter_type: FormatterType, use_color: bool) -> Box<dyn Format> {
    match formatter_type {
        FormatterType::Pretty => Box::new(pretty::PrettyFormatter::new(use_color)),
        FormatterType::Compact => Box::new(compact::CompactFormatter),
        #[cfg(feature = "json")]
        FormatterType::Json => Box::new(json::JsonFormatter),
    }
}
