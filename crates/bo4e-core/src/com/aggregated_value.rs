//! Aggregated value (Aggregiertwert) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An aggregated value computed from multiple source values.
///
/// German: Aggregiertwert
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::AggregatedValue;
/// use bo4e_core::enums::Unit;
/// use chrono::Utc;
///
/// let value = AggregatedValue {
///     timestamp: Some(Utc::now()),
///     value: Some(15000.0),
///     unit: Some(Unit::KilowattHour),
///     aggregation_method: Some("SUM".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedValue {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp for the aggregated value (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// The aggregated value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Unit of the value (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Aggregation method (Aggregationsmethode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_method: Option<String>,

    /// Period start for aggregation (Periodenbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<DateTime<Utc>>,

    /// Period end for aggregation (Periodenende)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<DateTime<Utc>>,

    /// Number of source values aggregated (Anzahl Quellwerte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_count: Option<i32>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,
}

impl Bo4eObject for AggregatedValue {
    fn type_name_german() -> &'static str {
        "Aggregiertwert"
    }

    fn type_name_english() -> &'static str {
        "AggregatedValue"
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
    fn test_aggregated_value() {
        let value = AggregatedValue {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 31, 23, 59, 59).unwrap()),
            value: Some(15000.0),
            unit: Some(Unit::KilowattHour),
            aggregation_method: Some("SUM".to_string()),
            source_count: Some(31),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("15000"));
        assert!(json.contains("SUM"));
    }

    #[test]
    fn test_average_aggregation() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 31, 23, 59, 59).unwrap();

        let value = AggregatedValue {
            value: Some(125.5),
            aggregation_method: Some("AVERAGE".to_string()),
            period_start: Some(start),
            period_end: Some(end),
            source_count: Some(2976),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("AVERAGE"));
        assert!(json.contains("2976"));
    }

    #[test]
    fn test_roundtrip() {
        let value = AggregatedValue {
            timestamp: Some(Utc::now()),
            value: Some(999.99),
            aggregation_method: Some("MAX".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        let parsed: AggregatedValue = serde_json::from_str(&json).unwrap();
        assert_eq!(value.value, parsed.value);
        assert_eq!(value.aggregation_method, parsed.aggregation_method);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(AggregatedValue::type_name_german(), "Aggregiertwert");
        assert_eq!(AggregatedValue::type_name_english(), "AggregatedValue");
    }
}
