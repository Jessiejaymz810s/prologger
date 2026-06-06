//! Builder pattern for constructing a configured `ProLogger`.
//!
//! Provides a fluent API for configuring all aspects of the logger
//! before building and initializing it.

use crate::filter::Filter;
use crate::formatter::{self, Format, FormatterType};
use crate::logger::ProLogger;
use crate::sink::console::{ConsoleSink, ConsoleTarget};
use crate::sink::Sink;
use log::{LevelFilter, SetLoggerError};

#[cfg(feature = "color")]
use crate::color::ColorMode;

#[cfg(feature = "file")]
use crate::rotation::RotationConfig;
#[cfg(feature = "file")]
use crate::sink::file::FileSink;

/// Builder for constructing a [`ProLogger`] instance.
///
/// # Example
///
/// ```rust,no_run
/// use prologger::ProLoggerBuilder;
/// use log::LevelFilter;
///
/// ProLoggerBuilder::new()
///     .with_level(LevelFilter::Debug)
///     .with_console_default()
///     .build()
///     .init()
///     .unwrap();
/// ```
pub struct ProLoggerBuilder {
    level: LevelFilter,
    module_filters: Vec<(String, LevelFilter)>,
    sinks: Vec<Box<dyn Sink>>,
    formatter_type: FormatterType,
    custom_formatter: Option<Box<dyn Format>>,

    #[cfg(feature = "color")]
    color_mode: ColorMode,
    #[cfg(not(feature = "color"))]
    _use_color: bool,
}

impl ProLoggerBuilder {
    /// Creates a new builder with default settings.
    ///
    /// Defaults:
    /// - Level: `Info`
    /// - Formatter: `Pretty`
    /// - Color: `Auto` (when `color` feature is enabled)
    /// - No sinks configured (you must add at least one)
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Info,
            module_filters: Vec::new(),
            sinks: Vec::new(),
            formatter_type: FormatterType::Pretty,
            custom_formatter: None,

            #[cfg(feature = "color")]
            color_mode: ColorMode::Auto,
            #[cfg(not(feature = "color"))]
            _use_color: false,
        }
    }

    /// Sets the global maximum log level.
    pub fn with_level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    /// Adds a module-specific level filter.
    ///
    /// Module paths are matched by prefix, so `"hyper"` will match
    /// `"hyper::client"`, `"hyper::server"`, etc.
    pub fn with_module_filter(mut self, module: &str, level: LevelFilter) -> Self {
        self.module_filters.push((module.to_string(), level));
        self
    }

    /// Sets the formatter type.
    pub fn with_formatter(mut self, formatter_type: FormatterType) -> Self {
        self.formatter_type = formatter_type;
        self.custom_formatter = None;
        self
    }

    /// Sets a custom formatter implementation.
    pub fn with_custom_formatter(mut self, formatter: Box<dyn Format>) -> Self {
        self.custom_formatter = Some(formatter);
        self
    }

    /// Sets the color mode (requires `color` feature).
    #[cfg(feature = "color")]
    pub fn with_color(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    /// Adds a console sink with the default mixed output (errors to stderr).
    pub fn with_console_default(self) -> Self {
        self.with_console(ConsoleTarget::Mixed)
    }

    /// Adds a console sink with the specified target.
    pub fn with_console(mut self, target: ConsoleTarget) -> Self {
        self.sinks.push(Box::new(ConsoleSink::new(target)));
        self
    }

    /// Adds a file sink without rotation.
    #[cfg(feature = "file")]
    pub fn with_file(mut self, path: &str) -> Self {
        match FileSink::new(path) {
            Ok(sink) => self.sinks.push(Box::new(sink)),
            Err(e) => eprintln!("[prologger] failed to open log file '{}': {}", path, e),
        }
        self
    }

    /// Adds a file sink with rotation enabled.
    #[cfg(feature = "file")]
    pub fn with_rotating_file(mut self, path: &str, config: RotationConfig) -> Self {
        match FileSink::with_rotation(path, config) {
            Ok(sink) => self.sinks.push(Box::new(sink)),
            Err(e) => eprintln!("[prologger] failed to open log file '{}': {}", path, e),
        }
        self
    }

    /// Adds a custom sink implementation.
    pub fn with_sink(mut self, sink: Box<dyn Sink>) -> Self {
        self.sinks.push(sink);
        self
    }

    /// Builds the configured `ProLogger`.
    ///
    /// If no sinks have been configured, a default console sink is added
    /// automatically.
    pub fn build(mut self) -> ProLogger {
        // If no sinks configured, add a default console sink
        if self.sinks.is_empty() {
            self.sinks.push(Box::new(ConsoleSink::default()));
        }

        // Determine color usage
        #[cfg(feature = "color")]
        let use_color = self.color_mode.should_color();
        #[cfg(not(feature = "color"))]
        let use_color = false;

        // Create the formatter
        let formatter = self
            .custom_formatter
            .unwrap_or_else(|| formatter::create_formatter(self.formatter_type, use_color));

        // Build the filter
        let mut filter = Filter::new(self.level);
        for (module, level) in self.module_filters {
            filter = filter.with_module(&module, level);
        }

        ProLogger {
            filter,
            formatter,
            sinks: self.sinks,
        }
    }

    /// Convenience method: builds and initializes the logger in one step.
    ///
    /// Equivalent to calling `.build().init()`.
    pub fn init(self) -> Result<(), SetLoggerError> {
        self.build().init()
    }
}

impl Default for ProLoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
