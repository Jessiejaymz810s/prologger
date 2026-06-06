//! Compact formatter — minimal log output.
//!
//! Produces output like:
//! ```text
//! I: Request processed
//! W: Connection pool low
//! E: Request failed
//! ```

use super::Format;
use log::Record;

/// A minimal formatter using single-character level prefixes.
///
/// Ideal for space-constrained environments or when log volume is high.
#[derive(Debug)]
pub struct CompactFormatter;

impl CompactFormatter {
    /// Returns the single-character prefix for a log level.
    fn level_char(level: log::Level) -> char {
        match level {
            log::Level::Error => 'E',
            log::Level::Warn => 'W',
            log::Level::Info => 'I',
            log::Level::Debug => 'D',
            log::Level::Trace => 'T',
        }
    }
}

impl Format for CompactFormatter {
    fn format(&self, record: &Record) -> String {
        format!("{}: {}\n", Self::level_char(record.level()), record.args())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_format() {
        let formatter = CompactFormatter;
        let record = log::Record::builder()
            .args(format_args!("something happened"))
            .level(log::Level::Warn)
            .target("test")
            .build();

        let output = formatter.format(&record);
        assert_eq!(output, "W: something happened\n");
    }

    #[test]
    fn test_all_levels() {
        let formatter = CompactFormatter;
        let levels = [
            (log::Level::Error, 'E'),
            (log::Level::Warn, 'W'),
            (log::Level::Info, 'I'),
            (log::Level::Debug, 'D'),
            (log::Level::Trace, 'T'),
        ];

        for (level, expected_char) in levels {
            let record = log::Record::builder()
                .args(format_args!("msg"))
                .level(level)
                .target("test")
                .build();

            let output = formatter.format(&record);
            assert!(output.starts_with(expected_char));
        }
    }
}
