//! Energy division (Sparte) enumeration.

use serde::{Deserialize, Serialize};

/// Energy division/sector.
///
/// Indicates which energy sector a business object belongs to.
///
/// German: Sparte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Sparte"))]
#[non_exhaustive]
pub enum Division {
    /// Electricity (Strom)
    #[serde(rename = "STROM")]
    Electricity,

    /// Natural gas (Gas)
    #[serde(rename = "GAS")]
    Gas,

    /// District heating (Fernwaerme)
    #[serde(rename = "FERNWAERME")]
    DistrictHeating,

    /// Local/near heating (Nahwaerme)
    #[serde(rename = "NAHWAERME")]
    LocalHeating,

    /// Water (Wasser)
    #[serde(rename = "WASSER")]
    Water,

    /// Wastewater (Abwasser)
    #[serde(rename = "ABWASSER")]
    Wastewater,

    /// Cross-divisional electricity and gas (Strom und Gas)
    #[serde(rename = "STROM_UND_GAS")]
    ElectricityAndGas,
}

impl Division {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Electricity => "Strom",
            Self::Gas => "Gas",
            Self::DistrictHeating => "Fernwaerme",
            Self::LocalHeating => "Nahwaerme",
            Self::Water => "Wasser",
            Self::Wastewater => "Abwasser",
            Self::ElectricityAndGas => "Strom und Gas",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&Division::Electricity).unwrap(),
            r#""STROM""#
        );
        assert_eq!(serde_json::to_string(&Division::Gas).unwrap(), r#""GAS""#);
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Division>(r#""STROM""#).unwrap(),
            Division::Electricity
        );
        assert_eq!(
            serde_json::from_str::<Division>(r#""FERNWAERME""#).unwrap(),
            Division::DistrictHeating
        );
    }

    #[test]
    fn test_roundtrip() {
        for division in [
            Division::Electricity,
            Division::Gas,
            Division::DistrictHeating,
            Division::LocalHeating,
            Division::Water,
            Division::Wastewater,
            Division::ElectricityAndGas,
        ] {
            let json = serde_json::to_string(&division).unwrap();
            let parsed: Division = serde_json::from_str(&json).unwrap();
            assert_eq!(division, parsed);
        }
    }
}
