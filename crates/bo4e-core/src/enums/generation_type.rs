//! Energy generation type (Erzeugungsart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of energy generation.
///
/// Lists the types of energy generation.
///
/// German: Erzeugungsart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Erzeugungsart"))]
#[non_exhaustive]
pub enum GenerationType {
    /// Fossil fuels
    #[serde(rename = "FOSSIL")]
    Fossil,

    /// Combined heat and power (Kraft-Waerme-Kopplung)
    #[serde(rename = "KWK")]
    CombinedHeatPower,

    /// Wind power
    #[serde(rename = "WIND")]
    Wind,

    /// Solar energy
    #[serde(rename = "SOLAR")]
    Solar,

    /// Nuclear power
    #[serde(rename = "KERNKRAFT")]
    Nuclear,

    /// Hydropower
    #[serde(rename = "WASSER")]
    Hydro,

    /// Geothermal
    #[serde(rename = "GEOTHERMIE")]
    Geothermal,

    /// Biomass
    #[serde(rename = "BIOMASSE")]
    Biomass,

    /// Coal
    #[serde(rename = "KOHLE")]
    Coal,

    /// Natural gas
    #[serde(rename = "GAS")]
    Gas,

    /// Other
    #[serde(rename = "SONSTIGE")]
    Other,

    /// Other per EEG (Renewable Energy Sources Act)
    #[serde(rename = "SONSTIGE_EEG")]
    OtherEeg,

    /// Biogas
    #[serde(rename = "BIOGAS")]
    Biogas,

    /// Climate-neutral gas
    #[serde(rename = "KLIMANEUTRALES_GAS")]
    ClimateNeutralGas,
}

impl GenerationType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Fossil => "Fossile Brennstoffe",
            Self::CombinedHeatPower => "Kraft-Waerme-Kopplung",
            Self::Wind => "Windkraft",
            Self::Solar => "Solarenergie",
            Self::Nuclear => "Kernkraft",
            Self::Hydro => "Wasserkraft",
            Self::Geothermal => "Geothermie",
            Self::Biomass => "Biomasse",
            Self::Coal => "Kohle",
            Self::Gas => "Erdgas",
            Self::Other => "Sonstige",
            Self::OtherEeg => "Sonstige nach EEG",
            Self::Biogas => "Biogas",
            Self::ClimateNeutralGas => "Klimaneutrales Erdgas",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&GenerationType::Wind).unwrap(),
            r#""WIND""#
        );
        assert_eq!(
            serde_json::to_string(&GenerationType::Solar).unwrap(),
            r#""SOLAR""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for gen_type in [
            GenerationType::Fossil,
            GenerationType::CombinedHeatPower,
            GenerationType::Wind,
            GenerationType::Solar,
            GenerationType::Nuclear,
            GenerationType::Hydro,
            GenerationType::Geothermal,
            GenerationType::Biomass,
            GenerationType::Coal,
            GenerationType::Gas,
            GenerationType::Other,
            GenerationType::OtherEeg,
            GenerationType::Biogas,
            GenerationType::ClimateNeutralGas,
        ] {
            let json = serde_json::to_string(&gen_type).unwrap();
            let parsed: GenerationType = serde_json::from_str(&json).unwrap();
            assert_eq!(gen_type, parsed);
        }
    }
}
