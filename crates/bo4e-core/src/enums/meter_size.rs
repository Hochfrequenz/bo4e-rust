//! Meter size (Zaehlergroesse) enumeration.

use serde::{Deserialize, Serialize};

/// Size of a gas meter.
///
/// Lists possible sizes of gas meters.
///
/// German: Zaehlergroesse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehlergroesse"))]
#[non_exhaustive]
pub enum MeterSize {
    /// Gas meter size G2.5
    #[serde(rename = "G2KOMMA5")]
    G2_5,

    /// Gas meter size G4
    #[serde(rename = "G4")]
    G4,

    /// Gas meter size G6
    #[serde(rename = "G6")]
    G6,

    /// Gas meter size G10
    #[serde(rename = "G10")]
    G10,

    /// Gas meter size G16
    #[serde(rename = "G16")]
    G16,

    /// Gas meter size G25
    #[serde(rename = "G25")]
    G25,

    /// Gas meter size G40
    #[serde(rename = "G40")]
    G40,

    /// Gas meter size G65
    #[serde(rename = "G65")]
    G65,

    /// Gas meter size G100
    #[serde(rename = "G100")]
    G100,

    /// Gas meter size G160
    #[serde(rename = "G160")]
    G160,

    /// Gas meter size G250
    #[serde(rename = "G250")]
    G250,

    /// Gas meter size G400
    #[serde(rename = "G400")]
    G400,

    /// Gas meter size G650
    #[serde(rename = "G650")]
    G650,

    /// Gas meter size G1000
    #[serde(rename = "G1000")]
    G1000,

    /// Gas meter size G1600
    #[serde(rename = "G1600")]
    G1600,

    /// Gas meter size G2500
    #[serde(rename = "G2500")]
    G2500,

    /// Gas meter size G4000
    #[serde(rename = "G4000")]
    G4000,

    /// Gas meter size G6500
    #[serde(rename = "G6500")]
    G6500,

    /// Gas meter size G10000
    #[serde(rename = "G10000")]
    G10000,

    /// Gas meter size G12500
    #[serde(rename = "G12500")]
    G12500,

    /// Gas meter size G16000
    #[serde(rename = "G16000")]
    G16000,
}

impl MeterSize {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::G2_5 => "G2,5",
            Self::G4 => "G4",
            Self::G6 => "G6",
            Self::G10 => "G10",
            Self::G16 => "G16",
            Self::G25 => "G25",
            Self::G40 => "G40",
            Self::G65 => "G65",
            Self::G100 => "G100",
            Self::G160 => "G160",
            Self::G250 => "G250",
            Self::G400 => "G400",
            Self::G650 => "G650",
            Self::G1000 => "G1000",
            Self::G1600 => "G1600",
            Self::G2500 => "G2500",
            Self::G4000 => "G4000",
            Self::G6500 => "G6500",
            Self::G10000 => "G10000",
            Self::G12500 => "G12500",
            Self::G16000 => "G16000",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&MeterSize::G4).unwrap(), r#""G4""#);
        assert_eq!(
            serde_json::to_string(&MeterSize::G2_5).unwrap(),
            r#""G2KOMMA5""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for size in [
            MeterSize::G2_5,
            MeterSize::G4,
            MeterSize::G6,
            MeterSize::G10,
            MeterSize::G16,
            MeterSize::G25,
            MeterSize::G40,
            MeterSize::G65,
            MeterSize::G100,
        ] {
            let json = serde_json::to_string(&size).unwrap();
            let parsed: MeterSize = serde_json::from_str(&json).unwrap();
            assert_eq!(size, parsed);
        }
    }
}
