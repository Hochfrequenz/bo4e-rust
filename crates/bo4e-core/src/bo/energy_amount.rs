//! Energy amount (Energiemenge) business object.
//!
//! Represents a quantity of energy with associated measurements.

use serde::{Deserialize, Serialize};

use crate::com::{MeasuredValue, TimePeriod};
use crate::enums::{Division, EnergyDirection, MeasurementType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An amount of energy with time series data.
///
/// German: Energiemenge
///
/// Energy amounts represent measured or calculated energy quantities
/// over time, typically associated with a market or metering location.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::EnergyAmount;
/// use bo4e_core::enums::{Division, EnergyDirection};
///
/// let energy = EnergyAmount {
///     energy_amount_id: Some("EA001".to_string()),
///     division: Some(Division::Electricity),
///     energy_direction: Some(EnergyDirection::FeedOut),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyAmount {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Energy amount ID (Energiemenge-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Energy direction (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_direction: Option<EnergyDirection>,

    /// Measurement type (Messart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_type: Option<MeasurementType>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Time series data (Messwerte)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub measured_values: Vec<MeasuredValue>,

    /// Associated market location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location_id: Option<String>,

    /// Associated metering location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_location_id: Option<String>,

    /// OBIS code for the measurement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,

    /// Total energy value (Gesamtenergie)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_energy: Option<f64>,
}

impl Bo4eObject for EnergyAmount {
    fn type_name_german() -> &'static str {
        "Energiemenge"
    }

    fn type_name_english() -> &'static str {
        "EnergyAmount"
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
    fn test_energy_amount_creation() {
        let energy = EnergyAmount {
            energy_amount_id: Some("EA001".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::FeedOut),
            ..Default::default()
        };

        assert_eq!(energy.energy_amount_id, Some("EA001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let energy = EnergyAmount {
            meta: Bo4eMeta::with_type("Energiemenge"),
            energy_amount_id: Some("EA001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&energy).unwrap();
        assert!(json.contains(r#""_typ":"Energiemenge""#));
    }

    #[test]
    fn test_roundtrip() {
        let energy = EnergyAmount {
            meta: Bo4eMeta::with_type("Energiemenge"),
            energy_amount_id: Some("EA001".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::FeedOut),
            total_energy: Some(1234.56),
            obis_code: Some("1-0:1.8.0".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&energy).unwrap();
        let parsed: EnergyAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(energy, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(EnergyAmount::type_name_german(), "Energiemenge");
        assert_eq!(EnergyAmount::type_name_english(), "EnergyAmount");
    }
}
