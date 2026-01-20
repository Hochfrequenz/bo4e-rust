//! Meter business object.

use crate::traits::{Bo4eMeta, Bo4eObject};
use serde::{Deserialize, Serialize};

/// A meter (Zähler) for measuring energy consumption or production.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,
    /// The meter number (Zählernummer)
    pub meter_number: Option<String>,
}

impl Bo4eObject for Meter {
    fn type_name_german() -> &'static str {
        "Zaehler"
    }

    fn type_name_english() -> &'static str {
        "Meter"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}
