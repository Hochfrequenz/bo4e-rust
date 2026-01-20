//! # bo4e
//!
//! Rust implementation of BO4E (Business Objects for Energy) - a standard for
//! data exchange in the German energy industry.
//!
//! This crate re-exports types from `bo4e-core` and serialization from `bo4e-serde`
//! for convenient single-crate usage.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use bo4e::prelude::*;
//!
//! // Create a meter
//! let meter = Meter::default();
//!
//! // Serialize to German JSON (BO4E standard)
//! let json = to_json_german(&meter).unwrap();
//!
//! // Deserialize
//! let mut bytes = json.into_bytes();
//! let parsed: Meter = from_json(&mut bytes).unwrap();
//! ```
//!
//! ## Crate Structure
//!
//! - `bo4e::bo` - Business Objects (Meter, MarketLocation, Contract, etc.)
//! - `bo4e::com` - Components (Address, Price, MeterReading, etc.)
//! - `bo4e::enums` - Enumerations (MeterType, Division, EnergyDirection, etc.)
//!
//! ## Serialization
//!
//! BO4E uses German field names in JSON by default. Use `to_json_english` for
//! English field names.

// Re-export core types
pub use bo4e_core::bo;
pub use bo4e_core::com;
pub use bo4e_core::enums;
pub use bo4e_core::traits;
pub use bo4e_core::{Bo4eMeta, Bo4eObject};

// Re-export serialization
pub use bo4e_serde::{from_json, to_json_english, to_json_german};
pub use bo4e_serde::{Error, JsonLanguage, SerializeConfig};

/// Prelude for convenient imports.
#[allow(unused_imports)]
pub mod prelude {
    pub use crate::bo::*;
    pub use crate::com::*;
    pub use crate::enums::*;
    pub use crate::{from_json, to_json_english, to_json_german};
    pub use crate::{Bo4eMeta, Bo4eObject};
}
