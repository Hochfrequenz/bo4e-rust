//! Network usage price sheet (PreisblattNetznutzung) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{NetworkCharge, PricePosition, TimePeriod};
use crate::enums::{CustomerType, Division, VoltageLevel};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price sheet for network usage charges.
///
/// German: PreisblattNetznutzung
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::NetworkUsagePriceSheet;
/// use bo4e_core::enums::{Division, VoltageLevel};
///
/// let price_sheet = NetworkUsagePriceSheet {
///     designation: Some("Netzentgelte 2024".to_string()),
///     division: Some(Division::Electricity),
///     voltage_level: Some(VoltageLevel::LowVoltage),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "PreisblattNetznutzung"))]
#[serde(rename_all = "camelCase")]
pub struct NetworkUsagePriceSheet {
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

    /// Voltage level this applies to (Spannungsebene)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "spannungsebene"))]
    pub voltage_level: Option<VoltageLevel>,

    /// Customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kundentyp"))]
    pub customer_type: Option<CustomerType>,

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

    /// Network charges (Netzentgelte)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzentgelte"))]
    pub network_charges: Vec<NetworkCharge>,

    /// Price positions (Preispositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preispositionen"))]
    pub positions: Vec<PricePosition>,

    /// Network operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzbetreiber"))]
    pub operator: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for NetworkUsagePriceSheet {
    fn type_name_german() -> &'static str {
        "PreisblattNetznutzung"
    }

    fn type_name_english() -> &'static str {
        "NetworkUsagePriceSheet"
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
    fn test_network_usage_price_sheet_creation() {
        let price_sheet = NetworkUsagePriceSheet {
            designation: Some("Netzentgelte 2024".to_string()),
            division: Some(Division::Electricity),
            voltage_level: Some(VoltageLevel::LowVoltage),
            customer_type: Some(CustomerType::Household),
            price_sheet_number: Some("NE-2024-001".to_string()),
            ..Default::default()
        };

        assert_eq!(price_sheet.voltage_level, Some(VoltageLevel::LowVoltage));
    }

    #[test]
    fn test_serialize() {
        let price_sheet = NetworkUsagePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattNetznutzung"),
            designation: Some("Network Fees".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Network Fees""#));
        assert!(json.contains(r#""_typ":"PreisblattNetznutzung""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = NetworkUsagePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattNetznutzung"),
            designation: Some("High Voltage Network".to_string()),
            description: Some("Network fees for high voltage".to_string()),
            division: Some(Division::Electricity),
            voltage_level: Some(VoltageLevel::HighVoltage),
            price_sheet_number: Some("HV-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: NetworkUsagePriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            NetworkUsagePriceSheet::type_name_german(),
            "PreisblattNetznutzung"
        );
        assert_eq!(
            NetworkUsagePriceSheet::type_name_english(),
            "NetworkUsagePriceSheet"
        );
    }
}
