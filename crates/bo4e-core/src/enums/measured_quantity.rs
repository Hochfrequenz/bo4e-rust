//! Measured quantity (Messgroesse) enumeration.

use serde::{Deserialize, Serialize};

/// Measured physical quantity.
///
/// Indicates the physical quantity that was measured.
///
/// German: Messgroesse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MeasuredQuantity {
    /// Electric current (Strom)
    #[serde(rename = "STROM")]
    Current,

    /// Voltage (Spannung)
    #[serde(rename = "SPANNUNG")]
    Voltage,

    /// Active power (Wirkleistung)
    #[serde(rename = "WIRKLEISTUNG")]
    ActivePower,

    /// Reactive power (Blindleistung)
    #[serde(rename = "BLINDLEISTUNG")]
    ReactivePower,

    /// Pressure (Druck)
    #[serde(rename = "DRUCK")]
    Pressure,

    /// Load profile (Lastgang)
    #[serde(rename = "LASTGANG")]
    LoadProfile,

    /// Standard load profile (Lastprofil)
    #[serde(rename = "LASTPROFIL")]
    StandardLoadProfile,

    /// Temperature (Temperatur)
    #[serde(rename = "TEMPERATUR")]
    Temperature,

    /// State number (Zustandszahl)
    #[serde(rename = "ZZAHL")]
    StateNumber,

    /// Calorific value (Brennwert)
    #[serde(rename = "BRENNWERT")]
    CalorificValue,

    /// Degree days (Gradtagszahlen)
    #[serde(rename = "GRADTZAGSZAHLEN")]
    DegreeDays,

    /// Volume flow (Volumenstrom)
    #[serde(rename = "VOLUMENSTROM")]
    VolumeFlow,

    /// Prices (Preise)
    #[serde(rename = "PREISE")]
    Prices,
}

impl MeasuredQuantity {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Current => "Strom",
            Self::Voltage => "Spannung",
            Self::ActivePower => "Wirkleistung",
            Self::ReactivePower => "Blindleistung",
            Self::Pressure => "Druck",
            Self::LoadProfile => "Lastgang",
            Self::StandardLoadProfile => "Lastprofil",
            Self::Temperature => "Temperatur",
            Self::StateNumber => "Zustandszahl",
            Self::CalorificValue => "Brennwert",
            Self::DegreeDays => "Gradtagszahlen",
            Self::VolumeFlow => "Volumenstrom",
            Self::Prices => "Preise",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeasuredQuantity::Voltage).unwrap(),
            r#""SPANNUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for quantity in [
            MeasuredQuantity::Current,
            MeasuredQuantity::Voltage,
            MeasuredQuantity::ActivePower,
            MeasuredQuantity::ReactivePower,
            MeasuredQuantity::Pressure,
            MeasuredQuantity::LoadProfile,
            MeasuredQuantity::StandardLoadProfile,
            MeasuredQuantity::Temperature,
            MeasuredQuantity::StateNumber,
            MeasuredQuantity::CalorificValue,
            MeasuredQuantity::DegreeDays,
            MeasuredQuantity::VolumeFlow,
            MeasuredQuantity::Prices,
        ] {
            let json = serde_json::to_string(&quantity).unwrap();
            let parsed: MeasuredQuantity = serde_json::from_str(&json).unwrap();
            assert_eq!(quantity, parsed);
        }
    }
}
