//! Console sink — writes log output to stdout or stderr.
//!
//! By default, `Error` and `Warn` levels are written to stderr,
//! while all other levels go to stdout.

use super::Sink;
use crate::formatter::Format;
use log::Record;
use std::io::Write;

/// Determines which output stream to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConsoleTarget {
    /// Write all output to stdout.
    Stdout,
    /// Write all output to stderr.
    Stderr,
    /// Write errors/warnings to stderr, everything else to stdout.
    #[default]
    Mixed,
}

/// A sink that writes formatted log output to the console.
#[derive(Debug)]
pub struct ConsoleSink {
    target: ConsoleTarget,
}

impl ConsoleSink {
    /// Creates a new console sink with the given target configuration.
    pub fn new(target: ConsoleTarget) -> Self {
        Self { target }
    }

    /// Creates a console sink that writes everything to stdout.
    pub fn stdout() -> Self {
        Self::new(ConsoleTarget::Stdout)
    }

    /// Creates a console sink that writes everything to stderr.
    pub fn stderr() -> Self {
        Self::new(ConsoleTarget::Stderr)
    }

    /// Creates a console sink that splits output between stdout and stderr.
    pub fn mixed() -> Self {
        Self::new(ConsoleTarget::Mixed)
    }
}

impl Default for ConsoleSink {
    fn default() -> Self {
        Self::mixed()
    }
}

impl Sink for ConsoleSink {
    fn write(&self, record: &Record, formatter: &dyn Format) {
        let formatted = formatter.format(record);

        let use_stderr = match self.target {
            ConsoleTarget::Stdout => false,
            ConsoleTarget::Stderr => true,
            ConsoleTarget::Mixed => matches!(record.level(), log::Level::Error | log::Level::Warn),
        };

        if use_stderr {
            let _ = std::io::stderr().write_all(formatted.as_bytes());
        } else {
            let _ = std::io::stdout().write_all(formatted.as_bytes());
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}
