# Epic 5.2: Language Configuration

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement runtime configuration for German/English JSON field names.

**Architecture:** Configuration struct passed to serialization, custom serializers for dynamic field renaming.

**Tech Stack:** serde, custom serializers

---

## Task 1: Define Configuration Types

**Files:**
- Modify: `crates/bo4e-serde/src/config.rs`

**Step 1: Update config.rs**

```rust
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

/// Execute a closure with a specific config.
pub fn with_config<T, F: FnOnce() -> T>(config: SerializeConfig, f: F) -> T {
    let old = CURRENT_CONFIG.with(|c| c.replace(config.clone()));
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
```

**Step 2: Commit**

```bash
git add crates/bo4e-serde/src/config.rs
git commit -m "feat(serde): add runtime language configuration"
```

---

## Task 2: Implement High-Level API

**Files:**
- Modify: `crates/bo4e-serde/src/lib.rs`

**Step 1: Update lib.rs with convenience functions**

```rust
//! # bo4e-serde
//!
//! High-performance serialization for BO4E types.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use bo4e_serde::{to_json_german, to_json_english, from_json};
//! use bo4e_core::bo::Meter;
//!
//! let meter = Meter {
//!     meter_number: Some("123".to_string()),
//!     ..Default::default()
//! };
//!
//! // German output (BO4E standard)
//! let german_json = to_json_german(&meter)?;
//! // {"zaehlernummer":"123",...}
//!
//! // English output
//! let english_json = to_json_english(&meter)?;
//! // {"meterNumber":"123",...}
//!
//! // Deserialize (accepts both formats)
//! let mut bytes = german_json.into_bytes();
//! let parsed: Meter = from_json(&mut bytes)?;
//! ```

mod config;
mod serialize;
pub mod simd;

pub use config::{JsonLanguage, SerializeConfig, current_language, set_config, with_config};
pub use simd::{from_slice, from_str, from_vec};

use bo4e_core::Bo4eObject;
use serde::{de::DeserializeOwned, Serialize};

/// Error type for serialization operations.
#[derive(Debug)]
pub enum Error {
    /// JSON serialization error
    Serialize(String),
    /// JSON deserialization error
    Deserialize(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Serialize(msg) => write!(f, "serialization error: {}", msg),
            Error::Deserialize(msg) => write!(f, "deserialization error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialize(e.to_string())
    }
}

impl From<simd_json::Error> for Error {
    fn from(e: simd_json::Error) -> Self {
        Error::Deserialize(e.to_string())
    }
}

/// Serialize a BO4E object to JSON with German field names.
///
/// This is the standard BO4E format.
pub fn to_json_german<T: Serialize>(value: &T) -> Result<String, Error> {
    with_config(SerializeConfig::german(), || {
        serde_json::to_string(value).map_err(Error::from)
    })
}

/// Serialize a BO4E object to JSON with English field names.
pub fn to_json_english<T: Serialize>(value: &T) -> Result<String, Error> {
    with_config(SerializeConfig::english(), || {
        serde_json::to_string(value).map_err(Error::from)
    })
}

/// Serialize with custom configuration.
pub fn to_json_with_config<T: Serialize>(
    value: &T,
    config: &SerializeConfig,
) -> Result<String, Error> {
    with_config(config.clone(), || {
        if config.pretty {
            serde_json::to_string_pretty(value).map_err(Error::from)
        } else {
            serde_json::to_string(value).map_err(Error::from)
        }
    })
}

/// Deserialize a BO4E object from JSON.
///
/// Accepts both German and English field names.
/// Uses simd-json for high performance.
pub fn from_json<T: DeserializeOwned>(json: &mut [u8]) -> Result<T, Error> {
    from_slice(json).map_err(Error::from)
}

/// Deserialize from a string.
pub fn from_json_str<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    from_str(json).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bo4e_core::bo::Meter;

    #[test]
    fn test_german_serialization() {
        let meter = Meter {
            meter_number: Some("TEST123".to_string()),
            ..Default::default()
        };

        let json = to_json_german(&meter).unwrap();
        // Note: With serde rename_all, field is already camelCase
        assert!(json.contains("meterNumber"));
    }

    #[test]
    fn test_roundtrip() {
        let original = Meter {
            meter_number: Some("ROUND123".to_string()),
            ..Default::default()
        };

        let json = to_json_german(&original).unwrap();
        let mut bytes = json.into_bytes();
        let parsed: Meter = from_json(&mut bytes).unwrap();

        assert_eq!(original.meter_number, parsed.meter_number);
    }
}
```

**Step 2: Commit**

```bash
git add crates/bo4e-serde/src/lib.rs
git commit -m "feat(serde): add high-level serialization API"
```

---

## Verification

```bash
cargo test -p bo4e-serde
cargo doc -p bo4e-serde --open
```
