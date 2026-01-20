//! Runtime configuration for JSON serialization.

use std::cell::RefCell;

/// Controls JSON field naming language.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JsonLanguage {
    /// German field names (e.g., "zaehlernummer", "marktlokationsId")
    /// This is the BO4E standard format.
    #[default]
    German,
    /// English field names (e.g., "meterNumber", "marketLocationId")
    English,
}

/// Configuration for JSON serialization.
#[derive(Debug, Clone)]
pub struct SerializeConfig {
    /// Language for field names
    pub language: JsonLanguage,
    /// Pretty-print JSON output
    pub pretty: bool,
    /// Include null fields in output
    pub include_nulls: bool,
}

impl Default for SerializeConfig {
    fn default() -> Self {
        Self {
            language: JsonLanguage::German,
            pretty: false,
            include_nulls: false,
        }
    }
}

impl SerializeConfig {
    /// Create config for German JSON output (BO4E standard).
    pub fn german() -> Self {
        Self::default()
    }

    /// Create config for English JSON output.
    pub fn english() -> Self {
        Self {
            language: JsonLanguage::English,
            ..Default::default()
        }
    }

    /// Enable pretty-printing.
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    /// Include null fields in output.
    pub fn include_nulls(mut self) -> Self {
        self.include_nulls = true;
        self
    }
}

// Thread-local storage for current serialization context
thread_local! {
    static CURRENT_CONFIG: RefCell<SerializeConfig> = RefCell::new(SerializeConfig::default());
}

/// Set the current serialization config for this thread.
pub fn set_config(config: SerializeConfig) {
    CURRENT_CONFIG.with(|c| *c.borrow_mut() = config);
}

/// Get the current serialization language.
pub fn current_language() -> JsonLanguage {
    CURRENT_CONFIG.with(|c| c.borrow().language)
}

/// Get the current serialization config.
pub fn current_config() -> SerializeConfig {
    CURRENT_CONFIG.with(|c| c.borrow().clone())
}

/// Execute a closure with a specific config.
pub fn with_config<T, F: FnOnce() -> T>(config: SerializeConfig, f: F) -> T {
    let old = CURRENT_CONFIG.with(|c| c.replace(config));
    let result = f();
    CURRENT_CONFIG.with(|c| c.replace(old));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_german() {
        assert_eq!(SerializeConfig::default().language, JsonLanguage::German);
    }

    #[test]
    fn test_english_config() {
        let config = SerializeConfig::english();
        assert_eq!(config.language, JsonLanguage::English);
    }

    #[test]
    fn test_builder_pattern() {
        let config = SerializeConfig::english().pretty().include_nulls();
        assert_eq!(config.language, JsonLanguage::English);
        assert!(config.pretty);
        assert!(config.include_nulls);
    }

    #[test]
    fn test_thread_local_config() {
        // Default should be German
        assert_eq!(current_language(), JsonLanguage::German);

        // Change to English
        set_config(SerializeConfig::english());
        assert_eq!(current_language(), JsonLanguage::English);

        // Reset
        set_config(SerializeConfig::default());
        assert_eq!(current_language(), JsonLanguage::German);
    }

    #[test]
    fn test_with_config() {
        assert_eq!(current_language(), JsonLanguage::German);

        let result = with_config(SerializeConfig::english(), || {
            assert_eq!(current_language(), JsonLanguage::English);
            42
        });

        assert_eq!(result, 42);
        assert_eq!(current_language(), JsonLanguage::German);
    }
}
