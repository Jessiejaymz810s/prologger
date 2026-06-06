//! ANSI color support for terminal output.
//!
#![allow(dead_code)]
//! Provides level-based coloring with automatic terminal detection.

/// ANSI escape codes for log level coloring.
#[derive(Debug, Clone, Copy)]
pub(crate) struct AnsiColor {
    pub code: &'static str,
}

impl AnsiColor {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";

    // Level colors
    pub const RED: AnsiColor = AnsiColor { code: "\x1b[31m" };
    pub const YELLOW: AnsiColor = AnsiColor { code: "\x1b[33m" };
    pub const GREEN: AnsiColor = AnsiColor { code: "\x1b[32m" };
    pub const BLUE: AnsiColor = AnsiColor { code: "\x1b[34m" };
    pub const MAGENTA: AnsiColor = AnsiColor { code: "\x1b[35m" };
    pub const CYAN: AnsiColor = AnsiColor { code: "\x1b[36m" };
    pub const WHITE: AnsiColor = AnsiColor { code: "\x1b[37m" };
    pub const BRIGHT_RED: AnsiColor = AnsiColor { code: "\x1b[91m" };
    pub const BRIGHT_YELLOW: AnsiColor = AnsiColor { code: "\x1b[93m" };
    pub const BRIGHT_GREEN: AnsiColor = AnsiColor { code: "\x1b[92m" };
    pub const BRIGHT_CYAN: AnsiColor = AnsiColor { code: "\x1b[96m" };
    pub const GRAY: AnsiColor = AnsiColor { code: "\x1b[90m" };
}

/// Returns the ANSI color for a given log level.
pub(crate) fn level_color(level: log::Level) -> AnsiColor {
    match level {
        log::Level::Error => AnsiColor::BRIGHT_RED,
        log::Level::Warn => AnsiColor::BRIGHT_YELLOW,
        log::Level::Info => AnsiColor::BRIGHT_GREEN,
        log::Level::Debug => AnsiColor::BRIGHT_CYAN,
        log::Level::Trace => AnsiColor::GRAY,
    }
}

/// Colorize a string with the given ANSI color.
pub(crate) fn colorize(text: &str, color: AnsiColor, bold: bool) -> String {
    if bold {
        format!(
            "{}{}{}{}",
            AnsiColor::BOLD,
            color.code,
            text,
            AnsiColor::RESET
        )
    } else {
        format!("{}{}{}", color.code, text, AnsiColor::RESET)
    }
}

/// Determines whether the given file descriptor likely supports color output.
///
/// Checks the `NO_COLOR` environment variable and attempts to detect
/// whether stdout/stderr is a terminal.
pub(crate) fn supports_color() -> bool {
    // Respect the NO_COLOR convention (https://no-color.org/)
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Check TERM environment variable
    match std::env::var("TERM") {
        Ok(term) => term != "dumb",
        Err(_) => false,
    }
}

/// Color mode configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    /// Automatically detect terminal color support.
    #[default]
    Auto,
    /// Always use colors.
    Always,
    /// Never use colors.
    Never,
}

impl ColorMode {
    /// Returns whether colors should be used given this mode.
    pub(crate) fn should_color(self) -> bool {
        match self {
            ColorMode::Auto => supports_color(),
            ColorMode::Always => true,
            ColorMode::Never => false,
        }
    }
}
