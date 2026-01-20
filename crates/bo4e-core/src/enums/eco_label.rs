//! Eco label (Oekolabel) enumeration.

use serde::{Deserialize, Serialize};

/// Eco label for green electricity and gas.
///
/// Enumeration of labels for eco-electricity from various issuers.
///
/// German: Oekolabel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Oekolabel"))]
#[non_exhaustive]
pub enum EcoLabel {
    /// Energreen label
    #[serde(rename = "ENERGREEN")]
    Energreen,

    /// Gasgreen + Gruener Strom combined label
    #[serde(rename = "GASGREEN_GRUENER_STROM")]
    GasgreenGruenerStrom,

    /// Gasgreen label
    #[serde(rename = "GASGREEN")]
    Gasgreen,

    /// Gruener Strom Gold label
    #[serde(rename = "GRUENER_STROM_GOLD")]
    GruenerStromGold,

    /// Gruener Strom Silver label
    #[serde(rename = "GRUENER_STROM_SILBER")]
    GruenerStromSilber,

    /// Gruener Strom label
    #[serde(rename = "GRUENER_STROM")]
    GruenerStrom,

    /// Gruenes Gas label
    #[serde(rename = "GRUENES_GAS")]
    GruenesGas,

    /// Naturwatt Strom label
    #[serde(rename = "NATURWATT_STROM")]
    NaturwattStrom,

    /// ok-power label
    #[serde(rename = "OK_POWER")]
    OkPower,

    /// RenewablePLUS label
    #[serde(rename = "RENEWABLE_PLUS")]
    RenewablePlus,

    /// Watergreen label
    #[serde(rename = "WATERGREEN")]
    Watergreen,

    /// Watergreen Plus label
    #[serde(rename = "WATERGREEN_PLUS")]
    WatergreenPlus,
}

impl EcoLabel {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Energreen => "Energreen",
            Self::GasgreenGruenerStrom => "Gasgreen Grüner Strom",
            Self::Gasgreen => "Gasgreen",
            Self::GruenerStromGold => "Grüner Strom Gold",
            Self::GruenerStromSilber => "Grüner Strom Silber",
            Self::GruenerStrom => "Grüner Strom",
            Self::GruenesGas => "Grünes Gas",
            Self::NaturwattStrom => "Naturwatt Strom",
            Self::OkPower => "ok-power",
            Self::RenewablePlus => "RenewablePLUS",
            Self::Watergreen => "Watergreen",
            Self::WatergreenPlus => "Watergreen Plus",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&EcoLabel::GruenerStromGold).unwrap(),
            r#""GRUENER_STROM_GOLD""#
        );
        assert_eq!(
            serde_json::to_string(&EcoLabel::OkPower).unwrap(),
            r#""OK_POWER""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for label in [
            EcoLabel::Energreen,
            EcoLabel::GasgreenGruenerStrom,
            EcoLabel::Gasgreen,
            EcoLabel::GruenerStromGold,
            EcoLabel::GruenerStromSilber,
            EcoLabel::GruenerStrom,
            EcoLabel::GruenesGas,
            EcoLabel::NaturwattStrom,
            EcoLabel::OkPower,
            EcoLabel::RenewablePlus,
            EcoLabel::Watergreen,
            EcoLabel::WatergreenPlus,
        ] {
            let json = serde_json::to_string(&label).unwrap();
            let parsed: EcoLabel = serde_json::from_str(&json).unwrap();
            assert_eq!(label, parsed);
        }
    }
}
