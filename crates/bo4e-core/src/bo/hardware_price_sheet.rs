//! Hardware price sheet (PreisblattHardware) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{Hardware, Price, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price sheet for hardware (meters, communication devices, etc.).
///
/// German: PreisblattHardware
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::HardwarePriceSheet;
/// use bo4e_core::enums::Division;
///
/// let price_sheet = HardwarePriceSheet {
///     designation: Some("Hardware Preisblatt 2024".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwarePriceSheet {
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

    /// Hardware items with pricing (Hardware)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hardware_items: Vec<Hardware>,

    /// Installation price (Installationspreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_price: Option<Price>,

    /// Rental price per unit (Mietpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rental_price: Option<Price>,

    /// Purchase price (Kaufpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<Price>,

    /// Hardware provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for HardwarePriceSheet {
    fn type_name_german() -> &'static str {
        "PreisblattHardware"
    }

    fn type_name_english() -> &'static str {
        "HardwarePriceSheet"
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
    use crate::com::Price;

    #[test]
    fn test_hardware_price_sheet_creation() {
        let price_sheet = HardwarePriceSheet {
            designation: Some("Hardware Preisblatt 2024".to_string()),
            division: Some(Division::Electricity),
            price_sheet_number: Some("HW-2024-001".to_string()),
            installation_price: Some(Price::eur_per_month(5.0)),
            ..Default::default()
        };

        assert_eq!(
            price_sheet.designation,
            Some("Hardware Preisblatt 2024".to_string())
        );
    }

    #[test]
    fn test_serialize() {
        let price_sheet = HardwarePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattHardware"),
            designation: Some("Meter Hardware".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Meter Hardware""#));
        assert!(json.contains(r#""_typ":"PreisblattHardware""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = HardwarePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattHardware"),
            designation: Some("Smart Meter Prices".to_string()),
            description: Some("Prices for smart meter hardware".to_string()),
            division: Some(Division::Electricity),
            price_sheet_number: Some("SM-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: HardwarePriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(HardwarePriceSheet::type_name_german(), "PreisblattHardware");
        assert_eq!(HardwarePriceSheet::type_name_english(), "HardwarePriceSheet");
    }
}
