//! # BO4E - Business Objects for Energy
//!
//! Rust implementation of the BO4E standard for data exchange in the
//! German energy industry.
//!
//! ## Quick Start
//!
//! ```rust
//! use bo4e::prelude::*;
//!
//! // Create a meter
//! let meter = Meter {
//!     meter_number: Some("1EMH0012345678".to_string()),
//!     division: Some(Division::Electricity),
//!     meter_type: Some(MeterType::ModernMeasuringDevice),
//!     ..Default::default()
//! };
//!
//! // Serialize to German JSON (BO4E standard)
//! let json = to_json_german(&meter).unwrap();
//! println!("{}", json);
//!
//! // Deserialize
//! let mut bytes = json.into_bytes();
//! let parsed: Meter = from_json(&mut bytes).unwrap();
//! assert_eq!(parsed.meter_number, Some("1EMH0012345678".to_string()));
//! ```
//!
//! ## Crate Structure
//!
//! - [`bo`] - Business Objects (Meter, MarketLocation, Contract, etc.)
//! - [`com`] - Components (Address, Price, MeterReading, etc.)
//! - [`enums`] - Enumerations (Division, MeterType, ContractStatus, etc.)
//!
//! ## Serialization
//!
//! BO4E uses German field names in JSON by default to maintain compatibility
//! with the standard. Use [`to_json_english`] for English field names.
//!
//! ```rust
//! use bo4e::prelude::*;
//!
//! let meter = Meter::default();
//!
//! // German (standard)
//! let german = to_json_german(&meter).unwrap();
//!
//! // English
//! let english = to_json_english(&meter).unwrap();
//! ```
//!
//! ## Performance
//!
//! This crate uses [simd-json](https://github.com/simd-lite/simd-json) for
//! SIMD-accelerated JSON parsing. For best performance, use [`from_json`]
//! with a mutable byte slice.
//!
//! ## Related Projects
//!
//! - [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python) - Python implementation
//! - [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) - JSON schemas
//! - [bo4e.de](https://www.bo4e.de/) - Official website

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
