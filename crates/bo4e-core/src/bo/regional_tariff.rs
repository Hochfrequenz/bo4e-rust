//! Regional tariff (Regionaltarif) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{RegionalPriceTier, RegionalSurcharge, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A regional tariff definition.
///
/// German: Regionaltarif
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::RegionalTariff;
/// use bo4e_core::enums::Division;
///
/// let tariff = RegionalTariff {
///     tariff_code: Some("RT-2024-001".to_string()),
///     name: Some("Regional Standard Tariff".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionalTariff {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Tariff code (Tarifcode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_code: Option<String>,

    /// Tariff name (Tarifname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Tariff provider (Tarifanbieter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Box<super::BusinessPartner>>,

    /// Region this tariff applies to (Region)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Box<super::Region>>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Start date (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,

    /// End date (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    /// Regional price tiers (Regionale Preisstufen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_tiers: Vec<RegionalPriceTier>,

    /// Regional surcharges (Regionale Aufschlaege)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surcharges: Vec<RegionalSurcharge>,
}

impl Bo4eObject for RegionalTariff {
    fn type_name_german() -> &'static str {
        "Regionaltarif"
    }

    fn type_name_english() -> &'static str {
        "RegionalTariff"
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
    fn test_regional_tariff_creation() {
        let tariff = RegionalTariff {
            tariff_code: Some("RT-2024-001".to_string()),
            name: Some("Regional Standard Tariff".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(tariff.tariff_code, Some("RT-2024-001".to_string()));
        assert_eq!(tariff.division, Some(Division::Electricity));
    }

    #[test]
    fn test_tariff_with_region() {
        use crate::bo::Region;
        use crate::enums::RegionType;

        let region = Box::new(Region {
            region_code: Some("DE-BY".to_string()),
            name: Some("Bavaria".to_string()),
            region_type: Some(RegionType::SupplyArea),
            ..Default::default()
        });

        let tariff = RegionalTariff {
            tariff_code: Some("RT-BY-001".to_string()),
            name: Some("Bavaria Regional Tariff".to_string()),
            region: Some(region),
            ..Default::default()
        };

        assert!(tariff.region.is_some());
    }

    #[test]
    fn test_serialize() {
        let tariff = RegionalTariff {
            meta: Bo4eMeta::with_type("Regionaltarif"),
            tariff_code: Some("RT-001".to_string()),
            name: Some("Test Tariff".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff).unwrap();
        assert!(json.contains(r#""tariffCode":"RT-001""#));
        assert!(json.contains(r#""name":"Test Tariff""#));
    }

    #[test]
    fn test_roundtrip() {
        let tariff = RegionalTariff {
            meta: Bo4eMeta::with_type("Regionaltarif"),
            tariff_code: Some("RT-001".to_string()),
            name: Some("Test Tariff".to_string()),
            description: Some("A test regional tariff".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff).unwrap();
        let parsed: RegionalTariff = serde_json::from_str(&json).unwrap();
        assert_eq!(tariff, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(RegionalTariff::type_name_german(), "Regionaltarif");
        assert_eq!(RegionalTariff::type_name_english(), "RegionalTariff");
    }
}
