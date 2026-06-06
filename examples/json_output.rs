//! JSON output example for production log aggregation.
//!
//! Run with: `cargo run --example json_output --features json`

use log::{error, info, warn};
use prologger::{FormatterType, ProLoggerBuilder};

fn main() {
    // Set up JSON-formatted logging for production
    ProLoggerBuilder::new()
        .with_level(log::LevelFilter::Debug)
        .with_formatter(FormatterType::Json)
        .with_console_default()
        .init()
        .expect("Failed to initialize logger");

    info!("Service started on port 8080");
    info!("Connected to database at localhost:5432");
    warn!("Request rate approaching limit: 950/1000 req/s");
    error!("Payment processing failed: timeout after 30s");

    // Each line is a valid JSON object, ready for tools like:
    // - Elasticsearch / Kibana
    // - Grafana Loki
    // - AWS CloudWatch
    // - Google Cloud Logging
}
