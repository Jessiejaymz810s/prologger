//! X/Grok API Sink.
//!
//! Sends log records to the xAI Grok API. 
//! **Note:** It is highly recommended to wrap this sink in an `AsyncSink`
//! (if the `async` feature is enabled) to prevent blocking the application thread
//! during HTTP requests.

use super::Sink;
use crate::formatter::Format;
use log::Record;
use reqwest::blocking::Client;
use serde_json::json;

/// A sink that writes log records to the xAI Grok API.
pub struct XGrokSink {
    api_key: String,
    client: Client,
}

impl XGrokSink {
    /// Creates a new `XGrokSink` with the given xAI API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }
}

impl Sink for XGrokSink {
    fn write(&self, record: &Record, formatter: &dyn Format) {
        let formatted = formatter.format(record);
        
        let payload = json!({
            "model": "grok-2",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a log analyzer. Please ingest this log silently."
                },
                {
                    "role": "user",
                    "content": formatted
                }
            ]
        });

        // Send the HTTP request blocking
        // Note: For high-throughput logging, this sink should be wrapped in `AsyncSink`.
        let _ = self.client.post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send();
    }

    fn flush(&self) {
        // Blocking client requests are sent immediately, no internal buffer to flush.
    }
}
