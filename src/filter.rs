//! Log filtering engine with global and per-module level overrides.
//!
//! Allows fine-grained control over which log messages are emitted
//! based on their level and originating module path.

use log::LevelFilter;
use std::collections::HashMap;

/// A filter that determines whether a log record should be processed.
///
/// Supports a global maximum level and per-module level overrides.
///
/// # Example
///
/// ```
/// use log::LevelFilter;
/// use prologger::filter::Filter;
///
/// let filter = Filter::new(LevelFilter::Info)
///     .with_module("hyper", LevelFilter::Warn)
///     .with_module("my_app::db", LevelFilter::Debug);
/// ```
#[derive(Debug, Clone)]
pub struct Filter {
    /// The global maximum log level.
    global_level: LevelFilter,
    /// Per-module level overrides. Module paths are matched by prefix.
    module_levels: HashMap<String, LevelFilter>,
}

impl Filter {
    /// Creates a new filter with the given global level.
    pub fn new(level: LevelFilter) -> Self {
        Self {
            global_level: level,
            module_levels: HashMap::new(),
        }
    }

    /// Adds a module-specific level override.
    ///
    /// The module path is matched as a prefix, so `"my_app"` will match
    /// `"my_app::db"`, `"my_app::api"`, etc.
    pub fn with_module(mut self, module: &str, level: LevelFilter) -> Self {
        self.module_levels.insert(module.to_string(), level);
        self
    }

    /// Returns the global level filter.
    pub fn global_level(&self) -> LevelFilter {
        self.global_level
    }

    /// Determines whether a log record with the given metadata should be logged.
    pub fn is_enabled(&self, metadata: &log::Metadata) -> bool {
        let level = metadata.level();
        let target = metadata.target();

        // Check module-specific overrides (longest prefix match wins)
        if let Some(module_level) = self.find_module_level(target) {
            return level <= module_level;
        }

        // Fall back to global level
        level <= self.global_level
    }

    /// Finds the most specific module level for the given target.
    ///
    /// Uses longest-prefix matching so "my_app::db" matches before "my_app".
    fn find_module_level(&self, target: &str) -> Option<LevelFilter> {
        let mut best_match: Option<(&str, LevelFilter)> = None;

        for (module, &level) in &self.module_levels {
            if target == module.as_str() || target.starts_with(&format!("{}::", module)) {
                match best_match {
                    Some((current, _)) if module.len() > current.len() => {
                        best_match = Some((module.as_str(), level));
                    }
                    None => {
                        best_match = Some((module.as_str(), level));
                    }
                    _ => {}
                }
            }
        }

        best_match.map(|(_, level)| level)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new(LevelFilter::Info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;

    fn metadata(level: Level, target: &str) -> log::Metadata<'_> {
        log::Metadata::builder().level(level).target(target).build()
    }

    #[test]
    fn test_global_level() {
        let filter = Filter::new(LevelFilter::Warn);
        assert!(filter.is_enabled(&metadata(Level::Error, "any")));
        assert!(filter.is_enabled(&metadata(Level::Warn, "any")));
        assert!(!filter.is_enabled(&metadata(Level::Info, "any")));
        assert!(!filter.is_enabled(&metadata(Level::Debug, "any")));
    }

    #[test]
    fn test_module_override() {
        let filter = Filter::new(LevelFilter::Info).with_module("noisy_lib", LevelFilter::Warn);

        // Normal module uses global level
        assert!(filter.is_enabled(&metadata(Level::Info, "my_app")));

        // Overridden module is more restrictive
        assert!(filter.is_enabled(&metadata(Level::Warn, "noisy_lib")));
        assert!(!filter.is_enabled(&metadata(Level::Info, "noisy_lib")));
        assert!(!filter.is_enabled(&metadata(Level::Info, "noisy_lib::submodule")));
    }

    #[test]
    fn test_longest_prefix_match() {
        let filter = Filter::new(LevelFilter::Info)
            .with_module("my_app", LevelFilter::Warn)
            .with_module("my_app::db", LevelFilter::Debug);

        // "my_app" gets Warn
        assert!(!filter.is_enabled(&metadata(Level::Info, "my_app")));

        // "my_app::db" gets Debug (more specific)
        assert!(filter.is_enabled(&metadata(Level::Debug, "my_app::db")));

        // "my_app::api" gets Warn (falls back to "my_app")
        assert!(!filter.is_enabled(&metadata(Level::Info, "my_app::api")));
    }
}
