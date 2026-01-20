//! Price sheet (Preisblatt) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{PricePosition, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A generic price sheet containing price positions.
///
/// German: Preisblatt
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::PriceSheet;
/// use bo4e_core::enums::Division;
///
/// let price_sheet = PriceSheet {
///     designation: Some("Standardpreisblatt 2024".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceSheet {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the price sheet (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designation: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Price number/identifier (Preisnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_number: Option<String>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Valid from date (Gueltig ab)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    /// Valid until date (Gueltig bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,

    /// Price positions (Preispositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<PricePosition>,

    /// Publisher of the price sheet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for PriceSheet {
    fn type_name_german() -> &'static str {
        "Preisblatt"
    }

    fn type_name_english() -> &'static str {
        "PriceSheet"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_sheet_creation() {
        let price_sheet = PriceSheet {
            designation: Some("Standardpreisblatt 2024".to_string()),
            division: Some(Division::Electricity),
            price_number: Some("PB-2024-001".to_string()),
            ..Default::default()
        };

        assert_eq!(
            price_sheet.designation,
            Some("Standardpreisblatt 2024".to_string())
        );
        assert_eq!(price_sheet.division, Some(Division::Electricity));
    }

    #[test]
    fn test_serialize() {
        let price_sheet = PriceSheet {
            meta: Bo4eMeta::with_type("Preisblatt"),
            designation: Some("Test Price Sheet".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Test Price Sheet""#));
        assert!(json.contains(r#""_typ":"Preisblatt""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = PriceSheet {
            meta: Bo4eMeta::with_type("Preisblatt"),
            designation: Some("Network Fee Sheet".to_string()),
            description: Some("Standard network fees".to_string()),
            division: Some(Division::Electricity),
            price_number: Some("NF-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: PriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(PriceSheet::type_name_german(), "Preisblatt");
        assert_eq!(PriceSheet::type_name_english(), "PriceSheet");
    }
}
