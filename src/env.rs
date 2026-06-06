//! Environment variable configuration for prologger.
//!
//! Parses `RUST_LOG`-style environment variables into filter configuration.
//!
//! # Supported Formats
//!
//! ```text
//! RUST_LOG=debug                        # Global level
//! RUST_LOG=my_app=debug                 # Module-specific
//! RUST_LOG=warn,my_app=debug            # Global + module-specific
//! RUST_LOG=warn,my_app::db=trace,hyper=error  # Multiple overrides
//! ```
//!
//! # Example
//!
//! ```rust
//! use prologger::env::EnvConfig;
//!
//! // Parse from a string (same format as RUST_LOG)
//! let config = EnvConfig::parse("warn,my_app=debug").unwrap();
//! assert_eq!(config.global_level(), Some(log::LevelFilter::Warn));
//! ```

use log::LevelFilter;

/// Parsed environment variable configuration.
///
/// Contains a global log level and per-module level overrides
/// extracted from an environment variable string.
#[derive(Debug, Clone)]
pub struct EnvConfig {
    global_level: Option<LevelFilter>,
    module_levels: Vec<(String, LevelFilter)>,
}

impl EnvConfig {
    /// The default environment variable name.
    pub const DEFAULT_ENV_VAR: &'static str = "RUST_LOG";

    /// Reads and parses the `RUST_LOG` environment variable.
    ///
    /// Returns `None` if the variable is not set.
    /// Returns `Some(Err(...))` if the variable is set but contains invalid syntax.
    pub fn from_default_env() -> Option<Result<Self, EnvParseError>> {
        Self::from_env(Self::DEFAULT_ENV_VAR)
    }

    /// Reads and parses a custom environment variable.
    ///
    /// Returns `None` if the variable is not set.
    /// Returns `Some(Err(...))` if the variable is set but contains invalid syntax.
    pub fn from_env(var_name: &str) -> Option<Result<Self, EnvParseError>> {
        match std::env::var(var_name) {
            Ok(val) => Some(Self::parse(&val)),
            Err(std::env::VarError::NotPresent) => None,
            Err(std::env::VarError::NotUnicode(_)) => {
                Some(Err(EnvParseError::InvalidUnicode(var_name.to_string())))
            }
        }
    }

    /// Parses a `RUST_LOG`-style filter string.
    ///
    /// # Format
    ///
    /// The string is a comma-separated list of directives:
    /// - `level` — sets the global level (e.g., `debug`, `warn`)
    /// - `module=level` — sets a module-specific level (e.g., `hyper=warn`)
    ///
    /// Levels are case-insensitive: `DEBUG`, `Debug`, `debug` all work.
    ///
    /// # Examples
    ///
    /// ```
    /// use prologger::env::EnvConfig;
    /// use log::LevelFilter;
    ///
    /// let config = EnvConfig::parse("warn,my_app=debug,hyper=error").unwrap();
    /// assert_eq!(config.global_level(), Some(LevelFilter::Warn));
    /// assert_eq!(config.module_levels().len(), 2);
    /// ```
    pub fn parse(input: &str) -> Result<Self, EnvParseError> {
        let mut global_level = None;
        let mut module_levels = Vec::new();

        let input = input.trim();
        if input.is_empty() {
            return Ok(Self {
                global_level: None,
                module_levels: Vec::new(),
            });
        }

        for directive in input.split(',') {
            let directive = directive.trim();
            if directive.is_empty() {
                continue;
            }

            if let Some((module, level_str)) = directive.split_once('=') {
                let module = module.trim();
                let level_str = level_str.trim();

                if module.is_empty() {
                    return Err(EnvParseError::EmptyModule(directive.to_string()));
                }

                let level = parse_level(level_str)
                    .ok_or_else(|| EnvParseError::InvalidLevel(level_str.to_string()))?;

                module_levels.push((module.to_string(), level));
            } else {
                // Bare level — sets the global level
                let level = parse_level(directive)
                    .ok_or_else(|| EnvParseError::InvalidLevel(directive.to_string()))?;

                if global_level.is_some() {
                    return Err(EnvParseError::DuplicateGlobalLevel(directive.to_string()));
                }
                global_level = Some(level);
            }
        }

        Ok(Self {
            global_level,
            module_levels,
        })
    }

    /// Returns the global log level, if one was specified.
    pub fn global_level(&self) -> Option<LevelFilter> {
        self.global_level
    }

    /// Returns the module-specific level overrides.
    pub fn module_levels(&self) -> &[(String, LevelFilter)] {
        &self.module_levels
    }
}

/// Errors that can occur when parsing an environment variable.
#[derive(Debug, Clone)]
pub enum EnvParseError {
    /// The environment variable contained non-Unicode data.
    InvalidUnicode(String),
    /// An unrecognized log level string was found.
    InvalidLevel(String),
    /// A module=level directive had an empty module name.
    EmptyModule(String),
    /// Multiple bare level directives were found (e.g., `debug,warn`).
    DuplicateGlobalLevel(String),
}

impl std::fmt::Display for EnvParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUnicode(var) => {
                write!(f, "environment variable '{}' contains invalid Unicode", var)
            }
            Self::InvalidLevel(level) => {
                write!(
                    f,
                    "invalid log level '{}' (expected: trace, debug, info, warn, error, off)",
                    level
                )
            }
            Self::EmptyModule(directive) => {
                write!(f, "empty module name in directive '{}'", directive)
            }
            Self::DuplicateGlobalLevel(level) => {
                write!(
                    f,
                    "duplicate global level '{}' (only one bare level is allowed)",
                    level
                )
            }
        }
    }
}

impl std::error::Error for EnvParseError {}

/// Parse a level string (case-insensitive).
fn parse_level(s: &str) -> Option<LevelFilter> {
    match s.to_lowercase().as_str() {
        "trace" => Some(LevelFilter::Trace),
        "debug" => Some(LevelFilter::Debug),
        "info" => Some(LevelFilter::Info),
        "warn" | "warning" => Some(LevelFilter::Warn),
        "error" => Some(LevelFilter::Error),
        "off" | "none" => Some(LevelFilter::Off),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global_level() {
        let config = EnvConfig::parse("debug").unwrap();
        assert_eq!(config.global_level(), Some(LevelFilter::Debug));
        assert!(config.module_levels().is_empty());
    }

    #[test]
    fn test_parse_case_insensitive() {
        assert_eq!(
            EnvConfig::parse("DEBUG").unwrap().global_level(),
            Some(LevelFilter::Debug)
        );
        assert_eq!(
            EnvConfig::parse("Debug").unwrap().global_level(),
            Some(LevelFilter::Debug)
        );
        assert_eq!(
            EnvConfig::parse("WARNING").unwrap().global_level(),
            Some(LevelFilter::Warn)
        );
    }

    #[test]
    fn test_parse_module_level() {
        let config = EnvConfig::parse("hyper=warn").unwrap();
        assert_eq!(config.global_level(), None);
        assert_eq!(config.module_levels().len(), 1);
        assert_eq!(
            config.module_levels()[0],
            ("hyper".to_string(), LevelFilter::Warn)
        );
    }

    #[test]
    fn test_parse_mixed() {
        let config = EnvConfig::parse("warn,my_app=debug,hyper=error").unwrap();
        assert_eq!(config.global_level(), Some(LevelFilter::Warn));
        assert_eq!(config.module_levels().len(), 2);
        assert_eq!(
            config.module_levels()[0],
            ("my_app".to_string(), LevelFilter::Debug)
        );
        assert_eq!(
            config.module_levels()[1],
            ("hyper".to_string(), LevelFilter::Error)
        );
    }

    #[test]
    fn test_parse_nested_modules() {
        let config = EnvConfig::parse("my_app::db=trace,my_app::api=warn").unwrap();
        assert_eq!(config.module_levels().len(), 2);
        assert_eq!(
            config.module_levels()[0],
            ("my_app::db".to_string(), LevelFilter::Trace)
        );
    }

    #[test]
    fn test_parse_empty() {
        let config = EnvConfig::parse("").unwrap();
        assert_eq!(config.global_level(), None);
        assert!(config.module_levels().is_empty());
    }

    #[test]
    fn test_parse_whitespace() {
        let config = EnvConfig::parse("  warn , my_app = debug  ").unwrap();
        assert_eq!(config.global_level(), Some(LevelFilter::Warn));
        assert_eq!(config.module_levels().len(), 1);
    }

    #[test]
    fn test_parse_off() {
        let config = EnvConfig::parse("off").unwrap();
        assert_eq!(config.global_level(), Some(LevelFilter::Off));
    }

    #[test]
    fn test_parse_invalid_level() {
        assert!(EnvConfig::parse("banana").is_err());
    }

    #[test]
    fn test_parse_duplicate_global() {
        assert!(EnvConfig::parse("debug,warn").is_err());
    }

    #[test]
    fn test_parse_empty_module() {
        assert!(EnvConfig::parse("=debug").is_err());
    }
}
