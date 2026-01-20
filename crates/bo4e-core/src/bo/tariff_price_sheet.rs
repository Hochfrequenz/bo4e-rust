//! Tariff price sheet (Tarifpreisblatt) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{TariffPricePosition, TimePeriod};
use crate::enums::{CustomerType, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A tariff price sheet containing tariff-specific pricing.
///
/// German: Tarifpreisblatt
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::TariffPriceSheet;
/// use bo4e_core::enums::{CustomerType, Division};
///
/// let price_sheet = TariffPriceSheet {
///     designation: Some("Haushaltstarif Preisblatt 2024".to_string()),
///     division: Some(Division::Electricity),
///     customer_type: Some(CustomerType::Household),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffPriceSheet {
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

    /// Target customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,

    /// Price sheet number/identifier (Preisblattnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_sheet_number: Option<String>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Valid from date (Gueltig ab)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    /// Valid until date (Gueltig bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,

    /// Tariff price positions (Tarifpreispositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<TariffPricePosition>,

    /// Publisher/provider of the price sheet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for TariffPriceSheet {
    fn type_name_german() -> &'static str {
        "Tarifpreisblatt"
    }

    fn type_name_english() -> &'static str {
        "TariffPriceSheet"
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
    fn test_tariff_price_sheet_creation() {
        let price_sheet = TariffPriceSheet {
            designation: Some("Haushaltstarif Preisblatt 2024".to_string()),
            division: Some(Division::Electricity),
            customer_type: Some(CustomerType::Household),
            price_sheet_number: Some("TPB-2024-001".to_string()),
            ..Default::default()
        };

        assert_eq!(price_sheet.customer_type, Some(CustomerType::Household));
    }

    #[test]
    fn test_serialize() {
        let price_sheet = TariffPriceSheet {
            meta: Bo4eMeta::with_type("Tarifpreisblatt"),
            designation: Some("Test Tariff Sheet".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Test Tariff Sheet""#));
        assert!(json.contains(r#""_typ":"Tarifpreisblatt""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = TariffPriceSheet {
            meta: Bo4eMeta::with_type("Tarifpreisblatt"),
            designation: Some("Gas Tariff Sheet".to_string()),
            division: Some(Division::Gas),
            customer_type: Some(CustomerType::Commercial),
            price_sheet_number: Some("GAS-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: TariffPriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TariffPriceSheet::type_name_german(), "Tarifpreisblatt");
        assert_eq!(TariffPriceSheet::type_name_english(), "TariffPriceSheet");
    }
}
