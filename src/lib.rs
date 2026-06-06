//! # Prologger
//!
//! A production-grade, ergonomic Rust logging library with colored output,
//! file rotation, and structured formatting.
//!
//! Prologger implements the [`log`] crate facade, so you can use the standard
//! `log::info!()`, `log::warn!()`, etc. macros throughout your codebase.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use log::{info, warn, error, debug};
//!
//! fn main() {
//!     // Initialize with sensible defaults (colored console output at Info level)
//!     prologger::init();
//!
//!     info!("Application started");
//!     debug!("This won't show at Info level");
//!     warn!("Low disk space");
//!     error!("Connection failed");
//! }
//! ```
//!
//! ## Builder API
//!
//! For fine-grained control, use the builder:
//!
//! ```rust,no_run
//! use log::LevelFilter;
//! use prologger::ProLoggerBuilder;
//!
//! ProLoggerBuilder::new()
//!     .with_level(LevelFilter::Debug)
//!     .with_console_default()
//!     .with_module_filter("hyper", LevelFilter::Warn)
//!     .init()
//!     .unwrap();
//! ```
//!
//! ## Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `color` | ✅ | ANSI colored terminal output |
//! | `file`  | ✅ | File logging with size-based rotation |
//! | `json`  | ❌ | JSON structured output formatter |
//! | `full`  | ❌ | Enables all features |

// ─── Modules ──────────────────────────────────────────────────────────────

pub mod builder;
#[cfg(feature = "color")]
pub mod color;
pub mod env;
pub mod filter;
pub mod formatter;
pub mod logger;
#[cfg(feature = "file")]
pub mod rotation;
pub mod sink;

// ─── Re-exports ───────────────────────────────────────────────────────────

pub use builder::ProLoggerBuilder;
#[cfg(feature = "color")]
pub use color::ColorMode;
pub use env::{EnvConfig, EnvParseError};
pub use formatter::FormatterType;
pub use logger::ProLogger;
#[cfg(feature = "file")]
pub use rotation::RotationConfig;

// Re-export log crate essentials for convenience
pub use log::{self, Level, LevelFilter};

// ─── Convenience Functions ────────────────────────────────────────────────

/// Initializes prologger with sensible defaults.
///
/// This sets up:
/// - Console output with auto-detected color support
/// - `Info` level filtering
/// - Pretty formatter
///
/// For more control, use [`ProLoggerBuilder`] instead.
///
/// # Panics
///
/// Panics if a global logger has already been set.
///
/// # Example
///
/// ```rust,no_run
/// prologger::init();
/// log::info!("Ready to go!");
/// ```
pub fn init() {
    ProLoggerBuilder::new()
        .with_console_default()
        .init()
        .expect("prologger: failed to initialize logger (was a logger already set?)")
}

/// Initializes prologger with the given maximum log level.
///
/// # Panics
///
/// Panics if a global logger has already been set.
///
/// # Example
///
/// ```rust,no_run
/// use log::LevelFilter;
///
/// prologger::init_with_level(LevelFilter::Debug);
/// log::debug!("Verbose logging enabled");
/// ```
pub fn init_with_level(level: LevelFilter) {
    ProLoggerBuilder::new()
        .with_level(level)
        .with_console_default()
        .init()
        .expect("prologger: failed to initialize logger (was a logger already set?)")
}

/// Tries to initialize prologger with sensible defaults.
///
/// Returns `Ok(())` on success, or `Err` if a logger was already set.
/// Use this instead of [`init`] when you want to handle the error gracefully.
pub fn try_init() -> Result<(), log::SetLoggerError> {
    ProLoggerBuilder::new().with_console_default().init()
}

/// Tries to initialize prologger with the given maximum log level.
pub fn try_init_with_level(level: LevelFilter) -> Result<(), log::SetLoggerError> {
    ProLoggerBuilder::new()
        .with_level(level)
        .with_console_default()
        .init()
}

/// Initializes prologger using the `RUST_LOG` environment variable.
///
/// This reads the `RUST_LOG` env var for level configuration:
/// - `RUST_LOG=debug` — sets global level to debug
/// - `RUST_LOG=warn,my_app=debug` — warn globally, debug for my_app
///
/// If `RUST_LOG` is not set, defaults to `Info` level.
///
/// # Panics
///
/// Panics if `RUST_LOG` contains invalid syntax or if a logger is already set.
///
/// # Example
///
/// ```rust,no_run
/// // Set RUST_LOG=debug before running
/// prologger::init_from_env();
/// log::debug!("This shows when RUST_LOG=debug");
/// ```
pub fn init_from_env() {
    ProLoggerBuilder::new()
        .with_env()
        .with_console_default()
        .init()
        .expect("prologger: failed to initialize logger (was a logger already set?)")
}

/// Tries to initialize prologger using the `RUST_LOG` environment variable.
///
/// Non-panicking version of [`init_from_env`].
pub fn try_init_from_env() -> Result<(), log::SetLoggerError> {
    ProLoggerBuilder::new()
        .with_env()
        .with_console_default()
        .init()
}
