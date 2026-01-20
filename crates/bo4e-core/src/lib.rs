//! # bo4e-core
//!
//! Core types for BO4E (Business Objects for Energy) - a standard for
//! data exchange in the German energy industry.
//!
//! This crate provides:
//! - Business Objects (BOs): Top-level entities like `Meter`, `MarketLocation`
//! - Components (COMs): Composite types like `Address`, `Price`
//! - Enumerations: Type-safe enums for all BO4E enum values
//!
//! ## Example
//!
//! ```rust
//! use bo4e_core::bo::Meter;
//!
//! let meter = Meter::default();
//! ```

pub mod bo;
pub mod com;
pub mod enums;
pub mod traits;

pub use traits::{Bo4eMeta, Bo4eObject};
