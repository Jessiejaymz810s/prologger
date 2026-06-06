//! Environment variable configuration example for prologger.
//!
//! Demonstrates using `RUST_LOG` to control log levels at runtime.
//!
//! Try these:
//!   RUST_LOG=trace cargo run --example env_config
//!   RUST_LOG=warn cargo run --example env_config
//!   RUST_LOG=error,env_config=debug cargo run --example env_config
//!   cargo run --example env_config  (defaults to Info)

use log::{debug, error, info, trace, warn};

fn main() {
    // Initialize from RUST_LOG env var (defaults to Info if not set)
    prologger::init_from_env();

    trace!("TRACE: Very detailed diagnostic info");
    debug!("DEBUG: Useful for development troubleshooting");
    info!("INFO: Normal operational messages");
    warn!("WARN: Something unexpected but recoverable");
    error!("ERROR: Something went wrong!");

    info!("Try setting RUST_LOG=trace to see all levels");
    info!("Or RUST_LOG=error to see only errors");
}
