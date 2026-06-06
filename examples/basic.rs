//! Basic usage example for prologger.
//!
//! Run with: `cargo run --example basic`

use log::{debug, error, info, trace, warn};

fn main() {
    // Initialize with debug level to show more output
    prologger::init_with_level(log::LevelFilter::Trace);

    info!("Application starting up");
    debug!("Loading configuration from default path");
    trace!("Config file parsed in 0.3ms");
    warn!("No API key configured, using defaults");
    error!("Failed to connect to database");

    info!("Processing {} items", 42);
    debug!("Item processing complete, {} succeeded, {} failed", 40, 2);

    info!("Application shutting down gracefully");
}
