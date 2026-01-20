//! Measurement type (Messart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of measurement.
///
/// Indicates how a value was measured.
///
/// German: Messart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Messart"))]
#[non_exhaustive]
pub enum MeasurementType {
    /// Current/actual value (Aktueller Wert)
    #[serde(rename = "AKTUELLERWERT")]
    CurrentValue,

    /// Mean/average value (Mittelwert)
    #[serde(rename = "MITTELWERT")]
    MeanValue,

    /// Maximum value (Maximalwert)
    #[serde(rename = "MAXIMALWERT")]
    MaximumValue,
}

impl MeasurementType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::CurrentValue => "Aktueller Wert",
            Self::MeanValue => "Mittelwert",
            Self::MaximumValue => "Maximalwert",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeasurementType::CurrentValue).unwrap(),
            r#""AKTUELLERWERT""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for mtype in [
            MeasurementType::CurrentValue,
            MeasurementType::MeanValue,
            MeasurementType::MaximumValue,
        ] {
            let json = serde_json::to_string(&mtype).unwrap();
            let parsed: MeasurementType = serde_json::from_str(&json).unwrap();
            assert_eq!(mtype, parsed);
        }
    }
}
