//! Tariff feature (Tarifmerkmal) enumeration.

use serde::{Deserialize, Serialize};

/// Tariff feature/product characteristic.
///
/// Product features in the context of tariff definition.
///
/// German: Tarifmerkmal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TariffFeature {
    /// Standard product (Standardprodukt)
    #[serde(rename = "STANDARD")]
    Standard,

    /// Prepayment product (Vorkassenprodukt)
    #[serde(rename = "VORKASSE")]
    Prepayment,

    /// Package price product (Paketpreisprodukt)
    #[serde(rename = "PAKET")]
    Package,

    /// Combined product (Kombiprodukt)
    #[serde(rename = "KOMBI")]
    Combined,

    /// Fixed price product (Festpreisprodukt)
    #[serde(rename = "FESTPREIS")]
    FixedPrice,

    /// Construction power product (Baustromprodukt)
    #[serde(rename = "BAUSTROM")]
    ConstructionPower,

    /// Building lighting product (Hauslichtprodukt)
    #[serde(rename = "HAUSLICHT")]
    BuildingLighting,

    /// Heating power product (Heizstromprodukt)
    #[serde(rename = "HEIZSTROM")]
    HeatingPower,

    /// Online product (Onlineprodukt)
    #[serde(rename = "ONLINE")]
    Online,
}

impl TariffFeature {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Standard => "Standardprodukt",
            Self::Prepayment => "Vorkassenprodukt",
            Self::Package => "Paketpreisprodukt",
            Self::Combined => "Kombiprodukt",
            Self::FixedPrice => "Festpreisprodukt",
            Self::ConstructionPower => "Baustromprodukt",
            Self::BuildingLighting => "Hauslichtprodukt",
            Self::HeatingPower => "Heizstromprodukt",
            Self::Online => "Onlineprodukt",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TariffFeature::Standard).unwrap(),
            r#""STANDARD""#
        );
        assert_eq!(
            serde_json::to_string(&TariffFeature::FixedPrice).unwrap(),
            r#""FESTPREIS""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for feature in [
            TariffFeature::Standard,
            TariffFeature::Prepayment,
            TariffFeature::Package,
            TariffFeature::Combined,
            TariffFeature::FixedPrice,
            TariffFeature::ConstructionPower,
            TariffFeature::BuildingLighting,
            TariffFeature::HeatingPower,
            TariffFeature::Online,
        ] {
            let json = serde_json::to_string(&feature).unwrap();
            let parsed: TariffFeature = serde_json::from_str(&json).unwrap();
            assert_eq!(feature, parsed);
        }
    }
}
