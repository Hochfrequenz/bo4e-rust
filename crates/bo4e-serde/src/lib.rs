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
pub mod serialize;
pub mod simd;

pub use config::{
    current_config, current_language, set_config, with_config, JsonLanguage, SerializeConfig,
};
pub use serialize::{to_string, to_string_pretty, to_vec};
pub use simd::{from_slice, from_str, from_vec};

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
pub fn to_json_with_config<T: Serialize>(value: &T, config: &SerializeConfig) -> Result<String, Error> {
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
        // With serde rename_all camelCase, field is "meterNumber"
        assert!(json.contains("meterNumber"));
    }

    #[test]
    fn test_english_serialization() {
        let meter = Meter {
            meter_number: Some("TEST456".to_string()),
            ..Default::default()
        };

        let json = to_json_english(&meter).unwrap();
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

    #[test]
    fn test_from_json_str() {
        let json = r#"{"meterNumber":"STRTEST"}"#;
        let meter: Meter = from_json_str(json).unwrap();
        assert_eq!(meter.meter_number, Some("STRTEST".to_string()));
    }

    #[test]
    fn test_with_config_pretty() {
        let meter = Meter {
            meter_number: Some("PRETTY".to_string()),
            ..Default::default()
        };

        let config = SerializeConfig::german().pretty();
        let json = to_json_with_config(&meter, &config).unwrap();

        // Pretty-printed JSON should contain newlines
        assert!(json.contains('\n'));
    }
}
