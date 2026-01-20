//! Meter reading (Zaehlwerksstand) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::{MeasuredValueStatus, ReadingType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A meter reading at a specific point in time.
///
/// German: Zaehlwerksstand
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::MeterReading;
/// use bo4e_core::enums::{ReadingType, Unit};
/// use chrono::Utc;
///
/// let reading = MeterReading {
///     timestamp: Some(Utc::now()),
///     value: Some(12345.67),
///     unit: Some(Unit::KilowattHour),
///     reading_type: Some(ReadingType::RemoteReading),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehlwerksstand"))]
#[serde(rename_all = "camelCase")]
pub struct MeterReading {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of the reading (Ablesezeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ablesezeitpunkt"))]
    pub timestamp: Option<DateTime<Utc>>,

    /// Meter reading value (Zaehlwerksstand)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehlwerksstand"))]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Type of reading (Ableseart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ableseart"))]
    pub reading_type: Option<ReadingType>,

    /// Status/quality of the reading (Status)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "status"))]
    pub status: Option<MeasuredValueStatus>,

    /// OBIS code for the register (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,

    /// Register ID (Zaehlwerkskennung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehlwerkskennung"))]
    pub register_id: Option<String>,
}

impl Bo4eObject for MeterReading {
    fn type_name_german() -> &'static str {
        "Zaehlwerksstand"
    }

    fn type_name_english() -> &'static str {
        "MeterReading"
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
    fn test_meter_reading() {
        let reading = MeterReading {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
            value: Some(12345.67),
            unit: Some(Unit::KilowattHour),
            reading_type: Some(ReadingType::RemoteReading),
            obis_code: Some("1-0:1.8.0".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&reading).unwrap();
        assert!(json.contains("12345.67"));
        assert!(json.contains("1-0:1.8.0"));
    }

    #[test]
    fn test_roundtrip() {
        let reading = MeterReading {
            timestamp: Some(Utc::now()),
            value: Some(999.99),
            unit: Some(Unit::CubicMeter),
            ..Default::default()
        };

        let json = serde_json::to_string(&reading).unwrap();
        let parsed: MeterReading = serde_json::from_str(&json).unwrap();
        assert_eq!(reading.value, parsed.value);
        assert_eq!(reading.unit, parsed.unit);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MeterReading::type_name_german(), "Zaehlwerksstand");
        assert_eq!(MeterReading::type_name_english(), "MeterReading");
    }
}
