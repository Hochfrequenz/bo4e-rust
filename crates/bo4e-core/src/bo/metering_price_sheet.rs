//! Metering price sheet (PreisblattMessung) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{Price, PricePosition, TimePeriod};
use crate::enums::{Division, MeterType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price sheet for metering services.
///
/// German: PreisblattMessung
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::MeteringPriceSheet;
/// use bo4e_core::enums::Division;
///
/// let price_sheet = MeteringPriceSheet {
///     designation: Some("Messpreisblatt 2024".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "PreisblattMessung"))]
#[serde(rename_all = "camelCase")]
pub struct MeteringPriceSheet {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the price sheet (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub designation: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Type of meter this applies to (Zaehlerart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehlerart"))]
    pub meter_type: Option<MeterType>,

    /// Price sheet number/identifier (Preisblattnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisblattnummer"))]
    pub price_sheet_number: Option<String>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitszeitraum"))]
    pub validity_period: Option<TimePeriod>,

    /// Valid from date (Gueltig ab)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigAb"))]
    pub valid_from: Option<DateTime<Utc>>,

    /// Valid until date (Gueltig bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigBis"))]
    pub valid_until: Option<DateTime<Utc>>,

    /// Metering point operation price (Messstellenbetrieb)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messstellenbetrieb"))]
    pub metering_point_operation_price: Option<Price>,

    /// Meter reading price (Ablesepreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ablesepreis"))]
    pub reading_price: Option<Price>,

    /// Price positions (Preispositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preispositionen"))]
    pub positions: Vec<PricePosition>,

    /// Metering operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messstellenbetreiber"))]
    pub operator: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for MeteringPriceSheet {
    fn type_name_german() -> &'static str {
        "PreisblattMessung"
    }

    fn type_name_english() -> &'static str {
        "MeteringPriceSheet"
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
    fn test_metering_price_sheet_creation() {
        let price_sheet = MeteringPriceSheet {
            designation: Some("Messpreisblatt 2024".to_string()),
            division: Some(Division::Electricity),
            meter_type: Some(MeterType::ElectronicMeter),
            price_sheet_number: Some("MP-2024-001".to_string()),
            ..Default::default()
        };

        assert_eq!(price_sheet.meter_type, Some(MeterType::ElectronicMeter));
    }

    #[test]
    fn test_serialize() {
        let price_sheet = MeteringPriceSheet {
            meta: Bo4eMeta::with_type("PreisblattMessung"),
            designation: Some("Metering Prices".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Metering Prices""#));
        assert!(json.contains(r#""_typ":"PreisblattMessung""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = MeteringPriceSheet {
            meta: Bo4eMeta::with_type("PreisblattMessung"),
            designation: Some("Gas Metering".to_string()),
            description: Some("Gas metering service prices".to_string()),
            division: Some(Division::Gas),
            price_sheet_number: Some("GM-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: MeteringPriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MeteringPriceSheet::type_name_german(), "PreisblattMessung");
        assert_eq!(
            MeteringPriceSheet::type_name_english(),
            "MeteringPriceSheet"
        );
    }
}
