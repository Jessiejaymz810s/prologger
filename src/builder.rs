//! Builder pattern for constructing a configured `ProLogger`.
//!
//! Provides a fluent API for configuring all aspects of the logger
//! before building and initializing it.

use crate::env::EnvConfig;
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

    #[cfg(feature = "async")]
    async_capacity: Option<usize>,
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

            #[cfg(feature = "async")]
            async_capacity: None,
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

    /// Configures the logger from the `RUST_LOG` environment variable.
    ///
    /// If `RUST_LOG` is set, its value overrides the global level and adds
    /// any module-specific filters. If not set, the builder is unchanged.
    ///
    /// # Format
    ///
    /// ```text
    /// RUST_LOG=debug                          # Global level
    /// RUST_LOG=warn,my_app=debug,hyper=error  # Global + module overrides
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the environment variable contains invalid syntax.
    /// Use [`with_env_var_opt`](Self::with_env_var_opt) for non-panicking behavior.
    pub fn with_env(self) -> Self {
        self.with_env_var(EnvConfig::DEFAULT_ENV_VAR)
    }

    /// Configures the logger from a custom environment variable.
    ///
    /// Works the same as [`with_env`](Self::with_env) but reads from the
    /// specified variable name instead of `RUST_LOG`.
    ///
    /// # Panics
    ///
    /// Panics if the environment variable contains invalid syntax.
    pub fn with_env_var(self, var_name: &str) -> Self {
        match EnvConfig::from_env(var_name) {
            Some(Ok(config)) => self.apply_env_config(config),
            Some(Err(e)) => panic!("prologger: failed to parse {}: {}", var_name, e),
            None => self, // Env var not set — no-op
        }
    }

    /// Configures the logger from the `RUST_LOG` environment variable,
    /// returning an error instead of panicking on invalid syntax.
    ///
    /// Returns `Ok(self)` if the variable is not set or was parsed successfully.
    /// Returns `Err(...)` if the variable contains invalid syntax.
    pub fn with_env_opt(self) -> Result<Self, crate::env::EnvParseError> {
        self.with_env_var_opt(EnvConfig::DEFAULT_ENV_VAR)
    }

    /// Configures the logger from a custom environment variable,
    /// returning an error instead of panicking on invalid syntax.
    pub fn with_env_var_opt(self, var_name: &str) -> Result<Self, crate::env::EnvParseError> {
        match EnvConfig::from_env(var_name) {
            Some(Ok(config)) => Ok(self.apply_env_config(config)),
            Some(Err(e)) => Err(e),
            None => Ok(self),
        }
    }

    /// Applies a parsed env config to this builder.
    fn apply_env_config(mut self, config: EnvConfig) -> Self {
        if let Some(level) = config.global_level() {
            self.level = level;
        }
        for (module, level) in config.module_levels() {
            self.module_filters.push((module.clone(), *level));
        }
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

    /// Adds a syslog sink.
    #[cfg(feature = "syslog")]
    pub fn with_syslog(mut self, process_name: &str) -> Self {
        match crate::sink::syslog::SyslogSink::new(process_name.to_string()) {
            Ok(sink) => self.sinks.push(Box::new(sink)),
            Err(e) => eprintln!("[prologger] failed to init syslog: {}", e),
        }
        self
    }

    /// Adds an xAI Grok sink.
    #[cfg(feature = "x_grok")]
    pub fn with_x_grok(mut self, api_key: impl Into<String>) -> Self {
        self.sinks.push(Box::new(crate::sink::x_grok::XGrokSink::new(api_key)));
        self
    }

    /// Makes the logger asynchronous, processing logs on a background thread.
    ///
    /// The `capacity` parameter determines how many log messages can be queued
    /// before dropping new messages. A good default is `10_000`.
    #[cfg(feature = "async")]
    pub fn with_async(mut self, capacity: usize) -> Self {
        self.async_capacity = Some(capacity);
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

        #[cfg(feature = "async")]
        let sinks: Vec<Box<dyn Sink>> = if let Some(capacity) = self.async_capacity {
            vec![Box::new(crate::sink::async_sink::AsyncSink::new(self.sinks, capacity))]
        } else {
            self.sinks
        };
        #[cfg(not(feature = "async"))]
        let sinks = self.sinks;

        ProLogger {
            filter,
            formatter,
            sinks,
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
