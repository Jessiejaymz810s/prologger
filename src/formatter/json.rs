//! JSON formatter — machine-readable structured log output.
//!
//! Produces output like:
//! ```json
//! {"timestamp":"2026-06-05T20:15:00.123Z","level":"INFO","target":"my_app::api","message":"Request processed"}
//! ```

use super::Format;
use log::Record;
use serde::Serialize;
use std::time::SystemTime;

/// A formatter that outputs log records as JSON objects (one per line).
///
/// Each line is a valid JSON object, making the output compatible with
/// log aggregation tools like Elasticsearch, Loki, and CloudWatch.
#[derive(Debug)]
pub struct JsonFormatter;

/// Internal struct for JSON serialization of a log record.
#[derive(Serialize)]
struct JsonRecord<'a> {
    timestamp: String,
    level: &'a str,
    target: &'a str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    module_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
}

impl JsonFormatter {
    /// Generates an ISO 8601 timestamp string.
    fn timestamp() -> String {
        let now = SystemTime::now();
        let duration = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        let days = secs / 86400;
        let time_secs = secs % 86400;
        let hours = time_secs / 3600;
        let minutes = (time_secs % 3600) / 60;
        let seconds = time_secs % 60;

        let (year, month, day) = days_to_date(days);

        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            year, month, day, hours, minutes, seconds, millis
        )
    }
}

impl Format for JsonFormatter {
    fn format(&self, record: &Record) -> String {
        let json_record = JsonRecord {
            timestamp: Self::timestamp(),
            level: record.level().as_str(),
            target: record.target(),
            message: format!("{}", record.args()),
            module_path: record.module_path(),
            file: record.file(),
            line: record.line(),
        };

        match serde_json::to_string(&json_record) {
            Ok(json) => format!("{}\n", json),
            Err(e) => format!(
                "{{\"error\":\"failed to serialize log record\",\"detail\":\"{}\"}}\n",
                e
            ),
        }
    }
}

/// Converts days since Unix epoch to (year, month, day).
fn days_to_date(days_since_epoch: u64) -> (u64, u64, u64) {
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
    fn test_json_format_is_valid_json() {
        let formatter = JsonFormatter;
        let record = log::Record::builder()
            .args(format_args!("test message"))
            .level(log::Level::Info)
            .target("test_mod")
            .build();

        let output = formatter.format(&record);
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();

        assert_eq!(parsed["level"], "INFO");
        assert_eq!(parsed["target"], "test_mod");
        assert_eq!(parsed["message"], "test message");
        assert!(parsed["timestamp"].is_string());
    }

    #[test]
    fn test_json_format_special_characters() {
        let formatter = JsonFormatter;
        let record = log::Record::builder()
            .args(format_args!("message with \"quotes\" and \\backslash"))
            .level(log::Level::Error)
            .target("test")
            .build();

        let output = formatter.format(&record);
        // Should be valid JSON even with special characters
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(output.trim());
        assert!(parsed.is_ok());
    }
}
