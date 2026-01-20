//! Tariff calculation parameter (Tarifberechnungsparameter) component.

use serde::{Deserialize, Serialize};

use crate::enums::{TariffCalculationMethod, TariffTime};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Parameters for tariff calculation.
///
/// German: Tarifberechnungsparameter
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TariffCalculationParameter;
/// use bo4e_core::enums::TariffCalculationMethod;
///
/// let param = TariffCalculationParameter {
///     calculation_method: Some(TariffCalculationMethod::Tiers),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "json-schema",
    schemars(rename = "Tarifberechnungsparameter")
)]
#[serde(rename_all = "camelCase")]
pub struct TariffCalculationParameter {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Method for tariff calculation (Berechnungsmethode)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "berechnungsmethode"))]
    pub calculation_method: Option<TariffCalculationMethod>,

    /// Tariff time (day/night, etc.) (Tarifzeit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifzeit"))]
    pub tariff_time: Option<TariffTime>,

    /// Whether this applies to peak demand (Ist Leistungsabhängig)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "istLeistungsabhaengig"))]
    pub is_demand_based: Option<bool>,

    /// Minimum annual consumption for this tariff (Mindestjahresverbrauch)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "mindestjahresverbrauch"))]
    pub min_annual_consumption: Option<f64>,

    /// Maximum annual consumption for this tariff (Höchstjahresverbrauch)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "hoechstjahresverbrauch"))]
    pub max_annual_consumption: Option<f64>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,
}

impl Bo4eObject for TariffCalculationParameter {
    fn type_name_german() -> &'static str {
        "Tarifberechnungsparameter"
    }

    fn type_name_english() -> &'static str {
        "TariffCalculationParameter"
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
    fn test_tiered_tariff() {
        let param = TariffCalculationParameter {
            calculation_method: Some(TariffCalculationMethod::Tiers),
            min_annual_consumption: Some(0.0),
            max_annual_consumption: Some(10000.0),
            ..Default::default()
        };

        assert_eq!(
            param.calculation_method,
            Some(TariffCalculationMethod::Tiers)
        );
    }

    #[test]
    fn test_time_based_tariff() {
        let param = TariffCalculationParameter {
            calculation_method: Some(TariffCalculationMethod::None),
            tariff_time: Some(TariffTime::HighTariff),
            ..Default::default()
        };

        assert_eq!(param.tariff_time, Some(TariffTime::HighTariff));
    }

    #[test]
    fn test_default() {
        let param = TariffCalculationParameter::default();
        assert!(param.calculation_method.is_none());
        assert!(param.min_annual_consumption.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let param = TariffCalculationParameter {
            calculation_method: Some(TariffCalculationMethod::None),
            tariff_time: Some(TariffTime::LowTariff),
            is_demand_based: Some(false),
            description: Some("Night rate tariff".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&param).unwrap();
        let parsed: TariffCalculationParameter = serde_json::from_str(&json).unwrap();
        assert_eq!(param, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            TariffCalculationParameter::type_name_german(),
            "Tarifberechnungsparameter"
        );
        assert_eq!(
            TariffCalculationParameter::type_name_english(),
            "TariffCalculationParameter"
        );
    }
}
