//! Time series (Zeitreihe) business object.
//!
//! Represents a generic time series of data values.

use serde::{Deserialize, Serialize};

use crate::com::{TimePeriod, TimeSeriesValue};
use crate::enums::{Division, MeasurementType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A time series of data values.
///
/// German: Zeitreihe
///
/// Time series contain sequences of timestamped data values
/// for various purposes (forecasts, historical data, etc.).
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::TimeSeries;
/// use bo4e_core::enums::Division;
///
/// let series = TimeSeries {
///     time_series_id: Some("TS001".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zeitreihe"))]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Time series ID (Zeitreihe-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitreiheId"))]
    pub time_series_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Measurement type (Messart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messart"))]
    pub measurement_type: Option<MeasurementType>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitszeitraum"))]
    pub validity_period: Option<TimePeriod>,

    /// Time series values (Zeitreihenwerte)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitreihenwerte"))]
    pub values: Vec<TimeSeriesValue>,

    /// Associated market location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktlokationsId"))]
    pub market_location_id: Option<String>,

    /// Associated metering location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messlokationsId"))]
    pub metering_location_id: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// OBIS code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,

    /// Version of the time series
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "version"))]
    pub series_version: Option<String>,

    /// Resolution/interval in minutes (Aufloesung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aufloesung"))]
    pub resolution_minutes: Option<i32>,
}

impl Bo4eObject for TimeSeries {
    fn type_name_german() -> &'static str {
        "Zeitreihe"
    }

    fn type_name_english() -> &'static str {
        "TimeSeries"
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
    fn test_time_series_creation() {
        let series = TimeSeries {
            time_series_id: Some("TS001".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(series.time_series_id, Some("TS001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let series = TimeSeries {
            meta: Bo4eMeta::with_type("Zeitreihe"),
            time_series_id: Some("TS001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&series).unwrap();
        assert!(json.contains(r#""_typ":"Zeitreihe""#));
    }

    #[test]
    fn test_roundtrip() {
        let series = TimeSeries {
            meta: Bo4eMeta::with_type("Zeitreihe"),
            time_series_id: Some("TS001".to_string()),
            division: Some(Division::Electricity),
            unit: Some(Unit::KilowattHour),
            resolution_minutes: Some(15),
            description: Some("Test time series".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&series).unwrap();
        let parsed: TimeSeries = serde_json::from_str(&json).unwrap();
        assert_eq!(series, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TimeSeries::type_name_german(), "Zeitreihe");
        assert_eq!(TimeSeries::type_name_english(), "TimeSeries");
    }
}
