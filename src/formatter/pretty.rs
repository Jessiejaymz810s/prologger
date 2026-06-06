//! Pretty formatter — human-readable, colorized log output.
//!
//! Produces output like:
//! ```text
//! 2026-06-05 20:15:00.123 INFO  [my_app::api] Request processed successfully
//! 2026-06-05 20:15:00.456 WARN  [my_app::db]  Connection pool running low
//! 2026-06-05 20:15:00.789 ERROR [my_app::api] Failed to process request
//! ```

use super::Format;
use log::Record;
use std::time::SystemTime;

/// A human-readable formatter with optional ANSI color output.
#[derive(Debug)]
pub struct PrettyFormatter {
    use_color: bool,
}

impl PrettyFormatter {
    /// Creates a new pretty formatter.
    ///
    /// If `use_color` is true, log levels and metadata are colorized
    /// using ANSI escape codes.
    pub fn new(use_color: bool) -> Self {
        Self { use_color }
    }

    /// Formats the current timestamp as an ISO 8601 string.
    fn timestamp(&self) -> String {
        let now = SystemTime::now();
        let duration = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        // Convert to date/time components
        let days = secs / 86400;
        let time_secs = secs % 86400;
        let hours = time_secs / 3600;
        let minutes = (time_secs % 3600) / 60;
        let seconds = time_secs % 60;

        // Simple date calculation from days since epoch
        let (year, month, day) = days_to_date(days);

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            year, month, day, hours, minutes, seconds, millis
        )
    }

    /// Formats the level string, padded to 5 characters.
    fn format_level(&self, level: log::Level) -> String {
        let level_str = format!("{:<5}", level);

        if self.use_color {
            #[cfg(feature = "color")]
            {
                use crate::color::{colorize, level_color};
                return colorize(&level_str, level_color(level), true);
            }
        }

        level_str
    }
}

impl Format for PrettyFormatter {
    fn format(&self, record: &Record) -> String {
        let timestamp = self.timestamp();
        let level = self.format_level(record.level());
        let target = record.target();
        let message = record.args();

        if self.use_color {
            #[cfg(feature = "color")]
            {
                use crate::color::{AnsiColor, colorize};
                let ts = colorize(&timestamp, AnsiColor::GRAY, false);
                let tgt = colorize(&format!("[{}]", target), AnsiColor::CYAN, false);
                return format!("{} {} {} {}\n", ts, level, tgt, message);
            }
        }

        format!("{} {} [{}] {}\n", timestamp, level, target, message)
    }
}

/// Converts days since Unix epoch to (year, month, day).
fn days_to_date(days_since_epoch: u64) -> (u64, u64, u64) {
    // Algorithm from Howard Hinnant's `chrono`-compatible date calculation
    let z = days_since_epoch + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_to_date_epoch() {
        let (y, m, d) = days_to_date(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_days_to_date_known_date() {
        // 2024-01-01 = 19723 days since epoch
        let (y, m, d) = days_to_date(19723);
        assert_eq!((y, m, d), (2024, 1, 1));
    }

    #[test]
    fn test_pretty_format_no_color() {
        let formatter = PrettyFormatter::new(false);
        let record = log::Record::builder()
            .args(format_args!("hello world"))
            .level(log::Level::Info)
            .target("test_mod")
            .build();

        let output = formatter.format(&record);
        assert!(output.contains("INFO"));
        assert!(output.contains("[test_mod]"));
        assert!(output.contains("hello world"));
        assert!(output.ends_with('\n'));
    }
}
