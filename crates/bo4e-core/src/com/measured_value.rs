//! Measured value (Messwert) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::{MeasuredValueStatus, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A measured value at a specific timestamp.
///
/// German: Messwert
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::MeasuredValue;
/// use bo4e_core::enums::Unit;
/// use chrono::Utc;
///
/// let value = MeasuredValue {
///     timestamp: Some(Utc::now()),
///     value: Some(12345.67),
///     unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasuredValue {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of measurement (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Measured value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Status/quality of the value (Status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MeasuredValueStatus>,

    /// OBIS code identifying the measurement (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,
}

impl Bo4eObject for MeasuredValue {
    fn type_name_german() -> &'static str {
        "Messwert"
    }

    fn type_name_english() -> &'static str {
        "MeasuredValue"
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
    fn test_measured_value() {
        let value = MeasuredValue {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap()),
            value: Some(12345.67),
            unit: Some(Unit::KilowattHour),
            obis_code: Some("1-0:1.8.0".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("12345.67"));
        assert!(json.contains("1-0:1.8.0"));
    }

    #[test]
    fn test_roundtrip() {
        let value = MeasuredValue {
            timestamp: Some(Utc::now()),
            value: Some(999.99),
            unit: Some(Unit::CubicMeter),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        let parsed: MeasuredValue = serde_json::from_str(&json).unwrap();
        assert_eq!(value.value, parsed.value);
        assert_eq!(value.unit, parsed.unit);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MeasuredValue::type_name_german(), "Messwert");
        assert_eq!(MeasuredValue::type_name_english(), "MeasuredValue");
    }
}
