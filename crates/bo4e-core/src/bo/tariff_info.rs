//! Tariff info (Tarifinfo) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{EnergyMix, PriceGuarantee, TariffRestriction, TimePeriod};
use crate::enums::{CustomerType, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Tariff information/overview.
///
/// Provides summary information about a tariff without the full details.
///
/// German: Tarifinfo
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::TariffInfo;
/// use bo4e_core::enums::{CustomerType, Division};
///
/// let tariff_info = TariffInfo {
///     tariff_name: Some("Oekostrom Basis".to_string()),
///     division: Some(Division::Electricity),
///     customer_type: Some(CustomerType::Household),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Tarifinfo"))]
#[serde(rename_all = "camelCase")]
pub struct TariffInfo {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Tariff name (Tarifname)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifname"))]
    pub tariff_name: Option<String>,

    /// Tariff description (Tarifbeschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifbeschreibung"))]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Target customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kundentyp"))]
    pub customer_type: Option<CustomerType>,

    /// Website URL for tariff information (Website)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "website"))]
    pub website: Option<String>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitszeitraum"))]
    pub validity_period: Option<TimePeriod>,

    /// Start date of tariff availability (Angebotsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "angebotsdatum"))]
    pub available_from: Option<DateTime<Utc>>,

    /// End date of tariff availability (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "enddatum"))]
    pub available_until: Option<DateTime<Utc>>,

    /// Energy mix composition (Energiemix)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energiemix"))]
    pub energy_mix: Option<EnergyMix>,

    /// Price guarantee (Preisgarantie)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisgarantie"))]
    pub price_guarantee: Option<PriceGuarantee>,

    /// Tariff restrictions (Tarifeinschraenkungen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifeinschraenkungen"))]
    pub restrictions: Vec<TariffRestriction>,

    /// Provider/supplier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "anbieter"))]
    pub supplier: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for TariffInfo {
    fn type_name_german() -> &'static str {
        "Tarifinfo"
    }

    fn type_name_english() -> &'static str {
        "TariffInfo"
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
    fn test_tariff_info_creation() {
        let tariff_info = TariffInfo {
            tariff_name: Some("Oekostrom Basis".to_string()),
            division: Some(Division::Electricity),
            customer_type: Some(CustomerType::Household),
            website: Some("https://example.com/tariff".to_string()),
            ..Default::default()
        };

        assert_eq!(tariff_info.tariff_name, Some("Oekostrom Basis".to_string()));
        assert_eq!(tariff_info.customer_type, Some(CustomerType::Household));
    }

    #[test]
    fn test_serialize() {
        let tariff_info = TariffInfo {
            meta: Bo4eMeta::with_type("Tarifinfo"),
            tariff_name: Some("Green Energy".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff_info).unwrap();
        assert!(json.contains(r#""tariffName":"Green Energy""#));
        assert!(json.contains(r#""_typ":"Tarifinfo""#));
    }

    #[test]
    fn test_roundtrip() {
        let tariff_info = TariffInfo {
            meta: Bo4eMeta::with_type("Tarifinfo"),
            tariff_name: Some("Test Tariff".to_string()),
            description: Some("A test tariff info".to_string()),
            division: Some(Division::Gas),
            customer_type: Some(CustomerType::Commercial),
            website: Some("https://test.com".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff_info).unwrap();
        let parsed: TariffInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(tariff_info, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TariffInfo::type_name_german(), "Tarifinfo");
        assert_eq!(TariffInfo::type_name_english(), "TariffInfo");
    }
}
