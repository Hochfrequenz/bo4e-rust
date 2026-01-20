//! Load profile (Lastgang) business object.
//!
//! Represents a load profile - time series of power consumption or generation.

use serde::{Deserialize, Serialize};

use crate::com::{LoadProfileValue, TimePeriod};
use crate::enums::{Division, EnergyDirection, MeasurementType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A load profile containing time series of power data.
///
/// German: Lastgang
///
/// Load profiles represent power measurements over time,
/// typically in 15-minute or hourly intervals.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::LoadProfile;
/// use bo4e_core::enums::{Division, EnergyDirection};
///
/// let profile = LoadProfile {
///     load_profile_id: Some("LP001".to_string()),
///     division: Some(Division::Electricity),
///     energy_direction: Some(EnergyDirection::FeedOut),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadProfile {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Load profile ID (Lastgang-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_profile_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Energy direction (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_direction: Option<EnergyDirection>,

    /// Measurement type (Messart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_type: Option<MeasurementType>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Load profile values (Lastgangwerte)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<LoadProfileValue>,

    /// Associated market location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location_id: Option<String>,

    /// Associated metering location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_location_id: Option<String>,

    /// OBIS code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,

    /// Interval duration in minutes (Intervalllaenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_minutes: Option<i32>,

    /// Standard load profile type (Standardlastprofil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_profile_type: Option<String>,
}

impl Bo4eObject for LoadProfile {
    fn type_name_german() -> &'static str {
        "Lastgang"
    }

    fn type_name_english() -> &'static str {
        "LoadProfile"
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
    fn test_load_profile_creation() {
        let profile = LoadProfile {
            load_profile_id: Some("LP001".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::FeedOut),
            ..Default::default()
        };

        assert_eq!(profile.load_profile_id, Some("LP001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let profile = LoadProfile {
            meta: Bo4eMeta::with_type("Lastgang"),
            load_profile_id: Some("LP001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains(r#""_typ":"Lastgang""#));
    }

    #[test]
    fn test_roundtrip() {
        let profile = LoadProfile {
            meta: Bo4eMeta::with_type("Lastgang"),
            load_profile_id: Some("LP001".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::FeedOut),
            unit: Some(Unit::Kilowatt),
            interval_minutes: Some(15),
            ..Default::default()
        };

        let json = serde_json::to_string(&profile).unwrap();
        let parsed: LoadProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(LoadProfile::type_name_german(), "Lastgang");
        assert_eq!(LoadProfile::type_name_english(), "LoadProfile");
    }
}
