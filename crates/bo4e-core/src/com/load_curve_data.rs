//! Load curve data (Lastkurvendaten) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Load curve data containing power measurements over time.
///
/// German: Lastkurvendaten
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::LoadCurveData;
/// use bo4e_core::enums::Unit;
/// use chrono::Utc;
///
/// let data = LoadCurveData {
///     timestamp: Some(Utc::now()),
///     power_value: Some(125.5),
///     power_unit: Some(Unit::Kilowatt),
///     interval_minutes: Some(15),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Lastkurvendaten"))]
#[serde(rename_all = "camelCase")]
pub struct LoadCurveData {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of the measurement (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitpunkt"))]
    pub timestamp: Option<DateTime<Utc>>,

    /// Power value (Leistungswert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "leistungswert"))]
    pub power_value: Option<f64>,

    /// Unit of power measurement (Leistungseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "leistungseinheit"))]
    pub power_unit: Option<Unit>,

    /// Energy value for the interval (Energiewert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energiewert"))]
    pub energy_value: Option<f64>,

    /// Unit of energy measurement (Energieeinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energieeinheit"))]
    pub energy_unit: Option<Unit>,

    /// Interval duration in minutes (Intervalllaenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "intervalllaenge"))]
    pub interval_minutes: Option<i32>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,

    /// Measurement location ID (Messlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messlokationsId"))]
    pub measurement_location_id: Option<String>,
}

impl Bo4eObject for LoadCurveData {
    fn type_name_german() -> &'static str {
        "Lastkurvendaten"
    }

    fn type_name_english() -> &'static str {
        "LoadCurveData"
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
    use chrono::TimeZone;

    #[test]
    fn test_load_curve_data() {
        let data = LoadCurveData {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 15, 0).unwrap()),
            power_value: Some(125.5),
            power_unit: Some(Unit::Kilowatt),
            interval_minutes: Some(15),
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("125.5"));
        assert!(json.contains("15"));
    }

    #[test]
    fn test_with_energy() {
        let data = LoadCurveData {
            timestamp: Some(Utc::now()),
            power_value: Some(100.0),
            power_unit: Some(Unit::Kilowatt),
            energy_value: Some(25.0),
            energy_unit: Some(Unit::KilowattHour),
            interval_minutes: Some(15),
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("25"));
    }

    #[test]
    fn test_roundtrip() {
        let data = LoadCurveData {
            timestamp: Some(Utc::now()),
            power_value: Some(999.99),
            power_unit: Some(Unit::Kilowatt),
            obis_code: Some("1-0:1.4.0".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        let parsed: LoadCurveData = serde_json::from_str(&json).unwrap();
        assert_eq!(data.power_value, parsed.power_value);
        assert_eq!(data.obis_code, parsed.obis_code);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(LoadCurveData::type_name_german(), "Lastkurvendaten");
        assert_eq!(LoadCurveData::type_name_english(), "LoadCurveData");
    }
}
