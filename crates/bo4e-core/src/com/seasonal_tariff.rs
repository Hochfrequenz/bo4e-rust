//! Seasonal tariff (Saisontarif) component.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// A seasonal tariff period with date range and tariff name.
///
/// German: Saisontarif
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::SeasonalTariff;
/// use chrono::NaiveDate;
///
/// let tariff = SeasonalTariff {
///     season_name: Some("Winter".to_string()),
///     start_date: Some(NaiveDate::from_ymd_opt(2024, 11, 1).unwrap()),
///     end_date: Some(NaiveDate::from_ymd_opt(2025, 3, 31).unwrap()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Saisontarif"))]
#[serde(rename_all = "camelCase")]
pub struct SeasonalTariff {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name of the season/tariff period (Saisonbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "saisonbezeichnung"))]
    pub season_name: Option<String>,

    /// Start date of the season (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "startdatum"))]
    pub start_date: Option<NaiveDate>,

    /// End date of the season (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "enddatum"))]
    pub end_date: Option<NaiveDate>,

    /// Tariff identifier (Tarifkennung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifkennung"))]
    pub tariff_id: Option<String>,

    /// Price factor for the season (Preisfaktor)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisfaktor"))]
    pub price_factor: Option<f64>,
}

impl Bo4eObject for SeasonalTariff {
    fn type_name_german() -> &'static str {
        "Saisontarif"
    }

    fn type_name_english() -> &'static str {
        "SeasonalTariff"
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
    fn test_winter_tariff() {
        let tariff = SeasonalTariff {
            season_name: Some("Winter".to_string()),
            start_date: Some(NaiveDate::from_ymd_opt(2024, 11, 1).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2025, 3, 31).unwrap()),
            price_factor: Some(1.2),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff).unwrap();
        assert!(json.contains("Winter"));
        assert!(json.contains("1.2"));
    }

    #[test]
    fn test_roundtrip() {
        let tariff = SeasonalTariff {
            season_name: Some("Summer".to_string()),
            tariff_id: Some("SUMMER_2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff).unwrap();
        let parsed: SeasonalTariff = serde_json::from_str(&json).unwrap();
        assert_eq!(tariff, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(SeasonalTariff::type_name_german(), "Saisontarif");
        assert_eq!(SeasonalTariff::type_name_english(), "SeasonalTariff");
    }
}
