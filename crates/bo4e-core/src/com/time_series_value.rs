//! Time series value (Zeitreihenwert) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::{MeasuredValueStatus, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A single value in a time series.
///
/// German: Zeitreihenwert
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TimeSeriesValue;
/// use bo4e_core::enums::Unit;
/// use chrono::Utc;
///
/// let value = TimeSeriesValue {
///     timestamp: Some(Utc::now()),
///     value: Some(1234.56),
///     unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zeitreihenwert"))]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesValue {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of the value (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitpunkt"))]
    pub timestamp: Option<DateTime<Utc>>,

    /// Value (Wert)
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

    /// Sequence number in the time series (Sequenznummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sequenznummer"))]
    pub sequence_number: Option<i64>,
}

impl Bo4eObject for TimeSeriesValue {
    fn type_name_german() -> &'static str {
        "Zeitreihenwert"
    }

    fn type_name_english() -> &'static str {
        "TimeSeriesValue"
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
    fn test_time_series_value() {
        let value = TimeSeriesValue {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap()),
            value: Some(1234.56),
            unit: Some(Unit::KilowattHour),
            sequence_number: Some(1),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("1234.56"));
    }

    #[test]
    fn test_roundtrip() {
        let value = TimeSeriesValue {
            timestamp: Some(Utc::now()),
            value: Some(999.99),
            unit: Some(Unit::CubicMeter),
            status: Some(MeasuredValueStatus::Read),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        let parsed: TimeSeriesValue = serde_json::from_str(&json).unwrap();
        assert_eq!(value.value, parsed.value);
        assert_eq!(value.status, parsed.status);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TimeSeriesValue::type_name_german(), "Zeitreihenwert");
        assert_eq!(TimeSeriesValue::type_name_english(), "TimeSeriesValue");
    }
}
