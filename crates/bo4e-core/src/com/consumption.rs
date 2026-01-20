//! Consumption (Verbrauch) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::{MeasuredValueStatus, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Consumption data for a specific period.
///
/// German: Verbrauch
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Consumption;
/// use bo4e_core::enums::Unit;
///
/// let consumption = Consumption {
///     value: Some(3660.0),
///     unit: Some(Unit::KilowattHour),
///     obis_code: Some("1-1:1.8.0".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumption {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Consumption value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Start date of consumption period inclusive (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,

    /// End date of consumption period exclusive (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    /// OBIS code identifying the measured value (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,

    /// Status of the measured value (Messwertstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measured_value_status: Option<MeasuredValueStatus>,
}

impl Bo4eObject for Consumption {
    fn type_name_german() -> &'static str {
        "Verbrauch"
    }

    fn type_name_english() -> &'static str {
        "Consumption"
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
    fn test_electricity_consumption() {
        let consumption = Consumption {
            value: Some(3660.0),
            unit: Some(Unit::KilowattHour),
            obis_code: Some("1-1:1.8.0".to_string()),
            measured_value_status: Some(MeasuredValueStatus::Read),
            ..Default::default()
        };

        assert_eq!(consumption.value, Some(3660.0));
        assert_eq!(consumption.unit, Some(Unit::KilowattHour));
    }

    #[test]
    fn test_gas_consumption() {
        let consumption = Consumption {
            value: Some(1250.0),
            unit: Some(Unit::CubicMeter),
            obis_code: Some("7-0:3.0.0".to_string()),
            ..Default::default()
        };

        assert_eq!(consumption.unit, Some(Unit::CubicMeter));
    }

    #[test]
    fn test_default() {
        let consumption = Consumption::default();
        assert!(consumption.value.is_none());
        assert!(consumption.unit.is_none());
        assert!(consumption.obis_code.is_none());
    }

    #[test]
    fn test_serialize() {
        let consumption = Consumption {
            value: Some(1500.0),
            unit: Some(Unit::KilowattHour),
            obis_code: Some("1-1:1.8.1".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&consumption).unwrap();
        assert!(json.contains(r#""value":1500"#));
        assert!(json.contains(r#""obisCode":"1-1:1.8.1""#));
    }

    #[test]
    fn test_roundtrip() {
        let consumption = Consumption {
            value: Some(4500.5),
            unit: Some(Unit::KilowattHour),
            start_date: Some(
                DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .into(),
            ),
            end_date: Some(
                DateTime::parse_from_rfc3339("2024-12-31T23:59:59Z")
                    .unwrap()
                    .into(),
            ),
            obis_code: Some("1-1:1.8.0".to_string()),
            measured_value_status: Some(MeasuredValueStatus::Read),
            ..Default::default()
        };

        let json = serde_json::to_string(&consumption).unwrap();
        let parsed: Consumption = serde_json::from_str(&json).unwrap();
        assert_eq!(consumption, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Consumption::type_name_german(), "Verbrauch");
        assert_eq!(Consumption::type_name_english(), "Consumption");
    }
}
