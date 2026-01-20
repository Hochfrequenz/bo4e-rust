//! Runtime configuration for JSON serialization.

/// Controls JSON field naming language.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JsonLanguage {
    /// German field names (e.g., "zaehlernummer", "marktlokationsId")
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
}

impl Default for SerializeConfig {
    fn default() -> Self {
        Self {
            language: JsonLanguage::German,
            pretty: false,
        }
    }
}

impl SerializeConfig {
    /// Create config for German JSON output.
    pub fn german() -> Self {
        Self::default()
    }

    /// Create config for English JSON output.
    pub fn english() -> Self {
        Self {
            language: JsonLanguage::English,
            pretty: false,
        }
    }

    /// Enable pretty-printing.
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }
}
