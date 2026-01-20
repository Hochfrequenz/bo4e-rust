//! # bo4e-serde
//!
//! High-performance serialization for BO4E types using SIMD-accelerated JSON parsing.
//!
//! ## Features
//!
//! - SIMD-accelerated JSON parsing via `simd-json`
//! - Runtime configuration for German or English field names
//! - Compatible with standard `serde` traits
//!
//! ## Example
//!
//! ```rust,ignore
//! use bo4e_serde::{to_json_german, from_json};
//! use bo4e_core::bo::Meter;
//!
//! let meter = Meter::default();
//! let json = to_json_german(&meter).unwrap();
//! ```

mod config;

pub use config::{JsonLanguage, SerializeConfig};

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

/// Serialize a BO4E object to JSON with German field names (default).
pub fn to_json_german<T: Bo4eObject + Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Serialize a BO4E object to JSON with English field names.
pub fn to_json_english<T: Bo4eObject + Serialize>(value: &T) -> Result<String, Error> {
    // TODO: Implement English field name mapping
    serde_json::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Deserialize a BO4E object from JSON.
///
/// Note: simd-json requires a mutable slice for in-place parsing.
pub fn from_json<T: Bo4eObject + DeserializeOwned>(json: &mut [u8]) -> Result<T, Error> {
    simd_json::from_slice(json).map_err(|e| Error::Deserialize(e.to_string()))
}
