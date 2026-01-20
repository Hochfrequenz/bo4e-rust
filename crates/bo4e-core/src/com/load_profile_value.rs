//! Load profile value (Lastgangwert) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::{MeasuredValueStatus, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A single value in a load profile (time series of power measurements).
///
/// German: Lastgangwert
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::LoadProfileValue;
/// use bo4e_core::enums::Unit;
/// use chrono::Utc;
///
/// let value = LoadProfileValue {
///     timestamp: Some(Utc::now()),
///     value: Some(125.5),
///     unit: Some(Unit::Kilowatt),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Lastprofilwert"))]
#[serde(rename_all = "camelCase")]
pub struct LoadProfileValue {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of the measurement (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitpunkt"))]
    pub timestamp: Option<DateTime<Utc>>,

    /// Power/load value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Status/quality of the value (Status)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "status"))]
    pub status: Option<MeasuredValueStatus>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,

    /// Interval duration in minutes (Intervalllaenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "intervalllaenge"))]
    pub interval_minutes: Option<i32>,
}

impl Bo4eObject for LoadProfileValue {
    fn type_name_german() -> &'static str {
        "Lastgangwert"
    }

    fn type_name_english() -> &'static str {
        "LoadProfileValue"
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
    fn test_load_profile_value() {
        let value = LoadProfileValue {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 15, 0).unwrap()),
            value: Some(125.5),
            unit: Some(Unit::Kilowatt),
            interval_minutes: Some(15),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("125.5"));
    }

    #[test]
    fn test_roundtrip() {
        let value = LoadProfileValue {
            timestamp: Some(Utc::now()),
            value: Some(99.9),
            unit: Some(Unit::Kilowatt),
            status: Some(MeasuredValueStatus::Read),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        let parsed: LoadProfileValue = serde_json::from_str(&json).unwrap();
        assert_eq!(value.value, parsed.value);
        assert_eq!(value.status, parsed.status);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(LoadProfileValue::type_name_german(), "Lastgangwert");
        assert_eq!(LoadProfileValue::type_name_english(), "LoadProfileValue");
    }
}
