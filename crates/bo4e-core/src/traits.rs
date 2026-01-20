//! Core traits for BO4E types.

use serde::{Deserialize, Serialize};

/// Metadata common to all BO4E objects.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Bo4eMeta {
    /// BO4E schema version
    pub version: Option<String>,
    /// External system ID
    pub id: Option<String>,
}

/// Trait implemented by all BO4E types.
pub trait Bo4eObject {
    /// Returns the type name as used in the `_typ` field.
    fn type_name() -> &'static str;
}
