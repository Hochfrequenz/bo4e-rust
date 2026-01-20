//! Voltage level (Spannungsebene) enumeration.

use serde::{Deserialize, Serialize};

/// Voltage level for electrical networks.
///
/// Specific voltage levels used in electricity distribution.
///
/// German: Spannungsebene
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VoltageLevel {
    /// Extra high voltage (Höchstspannung) - typically 220kV or 380kV
    #[serde(rename = "HOECHSTSPANNUNG")]
    ExtraHighVoltage,

    /// High voltage (Hochspannung) - typically 60kV to 110kV
    #[serde(rename = "HOCHSPANNUNG")]
    HighVoltage,

    /// Medium voltage (Mittelspannung) - typically 1kV to 36kV
    #[serde(rename = "MITTELSPANNUNG")]
    MediumVoltage,

    /// Low voltage (Niederspannung) - typically 230V/400V
    #[serde(rename = "NIEDERSPANNUNG")]
    LowVoltage,
}

impl VoltageLevel {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ExtraHighVoltage => "Höchstspannung",
            Self::HighVoltage => "Hochspannung",
            Self::MediumVoltage => "Mittelspannung",
            Self::LowVoltage => "Niederspannung",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&VoltageLevel::HighVoltage).unwrap(),
            r#""HOCHSPANNUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for level in [
            VoltageLevel::ExtraHighVoltage,
            VoltageLevel::HighVoltage,
            VoltageLevel::MediumVoltage,
            VoltageLevel::LowVoltage,
        ] {
            let json = serde_json::to_string(&level).unwrap();
            let parsed: VoltageLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(level, parsed);
        }
    }
}
