//! File logging example with rotation.
//!
//! Run with: `cargo run --example file_logging --features file`

use log::{debug, error, info, warn};
use prologger::{ProLoggerBuilder, RotationConfig};

fn main() {
    // Set up logging to both console and a rotating file
    ProLoggerBuilder::new()
        .with_level(log::LevelFilter::Debug)
        .with_console_default()
        .with_rotating_file(
            "logs/app.log",
            RotationConfig::megabytes(5, 3), // 5MB per file, keep 3 backups
        )
        .init()
        .expect("Failed to initialize logger");

    info!("Application started — logging to console and logs/app.log");
    debug!("Debug info will appear in both console and file");
    warn!("This warning is captured everywhere");
    error!("Errors are logged with high visibility");

    // Simulate some activity
    for i in 0..10 {
        info!("Processing batch {}/10", i + 1);
    }

    info!("Done! Check the logs/ directory for output files.");
}
