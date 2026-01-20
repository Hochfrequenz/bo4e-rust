//! Concession fee price sheet (PreisblattKonzessionsabgabe) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{ConcessionFee, TimePeriod};
use crate::enums::{ConcessionFeeCustomerGroup, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price sheet for concession fees.
///
/// Concession fees are paid to municipalities for the use of public
/// ways for energy infrastructure.
///
/// German: PreisblattKonzessionsabgabe
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::ConcessionFeePriceSheet;
/// use bo4e_core::enums::Division;
///
/// let price_sheet = ConcessionFeePriceSheet {
///     designation: Some("Konzessionsabgaben 2024".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "json-schema",
    schemars(rename = "PreisblattKonzessionsabgabe")
)]
#[serde(rename_all = "camelCase")]
pub struct ConcessionFeePriceSheet {
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

    /// Customer group for concession fees (Kundengruppe)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kundengruppe"))]
    pub customer_group: Option<ConcessionFeeCustomerGroup>,

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

    /// Concession fees (Konzessionsabgaben)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "konzessionsabgaben"))]
    pub concession_fees: Vec<ConcessionFee>,

    /// Municipality/area name (Gemeindebezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gemeindebezeichnung"))]
    pub municipality: Option<String>,

    /// Network operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzbetreiber"))]
    pub operator: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for ConcessionFeePriceSheet {
    fn type_name_german() -> &'static str {
        "PreisblattKonzessionsabgabe"
    }

    fn type_name_english() -> &'static str {
        "ConcessionFeePriceSheet"
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
    fn test_concession_fee_price_sheet_creation() {
        let price_sheet = ConcessionFeePriceSheet {
            designation: Some("Konzessionsabgaben 2024".to_string()),
            division: Some(Division::Electricity),
            customer_group: Some(ConcessionFeeCustomerGroup::ElectricityTariff25000),
            municipality: Some("Musterstadt".to_string()),
            price_sheet_number: Some("KA-2024-001".to_string()),
            ..Default::default()
        };

        assert_eq!(
            price_sheet.customer_group,
            Some(ConcessionFeeCustomerGroup::ElectricityTariff25000)
        );
    }

    #[test]
    fn test_serialize() {
        let price_sheet = ConcessionFeePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattKonzessionsabgabe"),
            designation: Some("Concession Fees".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Concession Fees""#));
        assert!(json.contains(r#""_typ":"PreisblattKonzessionsabgabe""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = ConcessionFeePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattKonzessionsabgabe"),
            designation: Some("Municipal Fees".to_string()),
            description: Some("Concession fees for municipality".to_string()),
            division: Some(Division::Electricity),
            municipality: Some("Teststadt".to_string()),
            price_sheet_number: Some("MF-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: ConcessionFeePriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            ConcessionFeePriceSheet::type_name_german(),
            "PreisblattKonzessionsabgabe"
        );
        assert_eq!(
            ConcessionFeePriceSheet::type_name_english(),
            "ConcessionFeePriceSheet"
        );
    }
}
