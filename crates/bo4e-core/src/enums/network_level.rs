//! Network level (Netzebene) enumeration.

use serde::{Deserialize, Serialize};

/// Network level within electricity and gas energy types.
///
/// Lists possible network levels for electricity (voltage levels) and gas (pressure levels).
///
/// German: Netzebene
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Netzebene"))]
#[non_exhaustive]
pub enum NetworkLevel {
    // Electricity levels
    /// Low voltage (Niederspannung) - Electricity
    #[serde(rename = "NSP")]
    LowVoltage,

    /// Medium voltage (Mittelspannung) - Electricity
    #[serde(rename = "MSP")]
    MediumVoltage,

    /// High voltage (Hochspannung) - Electricity
    #[serde(rename = "HSP")]
    HighVoltage,

    /// Extra high voltage (Höchstspannung) - Electricity
    #[serde(rename = "HSS")]
    ExtraHighVoltage,

    /// Medium to low voltage transformation (MS/NS Umspannung) - Electricity
    #[serde(rename = "MSP_NSP_UMSP")]
    MediumLowVoltageTransformation,

    /// High to medium voltage transformation (HS/MS Umspannung) - Electricity
    #[serde(rename = "HSP_MSP_UMSP")]
    HighMediumVoltageTransformation,

    /// Extra high to high voltage transformation (HöS/HS Umspannung) - Electricity
    #[serde(rename = "HSS_HSP_UMSP")]
    ExtraHighHighVoltageTransformation,

    // Gas levels
    /// High pressure (Hochdruck) - Gas
    #[serde(rename = "HD")]
    HighPressure,

    /// Medium pressure (Mitteldruck) - Gas
    #[serde(rename = "MD")]
    MediumPressure,

    /// Low pressure (Niederdruck) - Gas
    #[serde(rename = "ND")]
    LowPressure,
}

impl NetworkLevel {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::LowVoltage => "Niederspannung",
            Self::MediumVoltage => "Mittelspannung",
            Self::HighVoltage => "Hochspannung",
            Self::ExtraHighVoltage => "Höchstspannung",
            Self::MediumLowVoltageTransformation => "MS/NS Umspannung",
            Self::HighMediumVoltageTransformation => "HS/MS Umspannung",
            Self::ExtraHighHighVoltageTransformation => "HöS/HS Umspannung",
            Self::HighPressure => "Hochdruck",
            Self::MediumPressure => "Mitteldruck",
            Self::LowPressure => "Niederdruck",
        }
    }

    /// Returns true if this is an electricity network level.
    pub fn is_electricity(&self) -> bool {
        matches!(
            self,
            Self::LowVoltage
                | Self::MediumVoltage
                | Self::HighVoltage
                | Self::ExtraHighVoltage
                | Self::MediumLowVoltageTransformation
                | Self::HighMediumVoltageTransformation
                | Self::ExtraHighHighVoltageTransformation
        )
    }

    /// Returns true if this is a gas network level.
    pub fn is_gas(&self) -> bool {
        matches!(
            self,
            Self::HighPressure | Self::MediumPressure | Self::LowPressure
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&NetworkLevel::LowVoltage).unwrap(),
            r#""NSP""#
        );
        assert_eq!(
            serde_json::to_string(&NetworkLevel::HighPressure).unwrap(),
            r#""HD""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for level in [
            NetworkLevel::LowVoltage,
            NetworkLevel::MediumVoltage,
            NetworkLevel::HighVoltage,
            NetworkLevel::ExtraHighVoltage,
            NetworkLevel::MediumLowVoltageTransformation,
            NetworkLevel::HighMediumVoltageTransformation,
            NetworkLevel::ExtraHighHighVoltageTransformation,
            NetworkLevel::HighPressure,
            NetworkLevel::MediumPressure,
            NetworkLevel::LowPressure,
        ] {
            let json = serde_json::to_string(&level).unwrap();
            let parsed: NetworkLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(level, parsed);
        }
    }

    #[test]
    fn test_is_electricity() {
        assert!(NetworkLevel::LowVoltage.is_electricity());
        assert!(NetworkLevel::HighVoltage.is_electricity());
        assert!(!NetworkLevel::HighPressure.is_electricity());
    }

    #[test]
    fn test_is_gas() {
        assert!(NetworkLevel::HighPressure.is_gas());
        assert!(NetworkLevel::LowPressure.is_gas());
        assert!(!NetworkLevel::LowVoltage.is_gas());
    }
}
