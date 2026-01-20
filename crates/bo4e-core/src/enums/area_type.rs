//! Area type (Gebiettyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of geographical/administrative area in the energy sector.
///
/// German: Gebiettyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AreaType {
    /// Control area (Regelzone)
    #[serde(rename = "REGELZONE")]
    ControlArea,

    /// Market area (Marktgebiet)
    #[serde(rename = "MARKTGEBIET")]
    MarketArea,

    /// Balancing area (Bilanzierungsgebiet)
    #[serde(rename = "BILANZIERUNGSGEBIET")]
    BalancingArea,

    /// Distribution network (Verteilnetz)
    #[serde(rename = "VERTEILNETZ")]
    DistributionNetwork,

    /// Transmission network (Transportnetz)
    #[serde(rename = "TRANSPORTNETZ")]
    TransmissionNetwork,

    /// Regional network (Regionalnetz)
    #[serde(rename = "REGIONALNETZ")]
    RegionalNetwork,

    /// Areal network/local grid (Arealnetz)
    #[serde(rename = "AREALNETZ")]
    ArealNetwork,

    /// Basic supply area (Grundversorgungsgebiet)
    #[serde(rename = "GRUNDVERSORGUNGSGEBIET")]
    BasicSupplyArea,

    /// Supply area (Versorgungsgebiet)
    #[serde(rename = "VERSORGUNGSGEBIET")]
    SupplyArea,
}

impl AreaType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ControlArea => "Regelzone",
            Self::MarketArea => "Marktgebiet",
            Self::BalancingArea => "Bilanzierungsgebiet",
            Self::DistributionNetwork => "Verteilnetz",
            Self::TransmissionNetwork => "Transportnetz",
            Self::RegionalNetwork => "Regionalnetz",
            Self::ArealNetwork => "Arealnetz",
            Self::BasicSupplyArea => "Grundversorgungsgebiet",
            Self::SupplyArea => "Versorgungsgebiet",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&AreaType::ControlArea).unwrap(),
            r#""REGELZONE""#
        );
        assert_eq!(
            serde_json::to_string(&AreaType::MarketArea).unwrap(),
            r#""MARKTGEBIET""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<AreaType>(r#""REGELZONE""#).unwrap(),
            AreaType::ControlArea
        );
        assert_eq!(
            serde_json::from_str::<AreaType>(r#""VERTEILNETZ""#).unwrap(),
            AreaType::DistributionNetwork
        );
    }

    #[test]
    fn test_roundtrip() {
        for area_type in [
            AreaType::ControlArea,
            AreaType::MarketArea,
            AreaType::BalancingArea,
            AreaType::DistributionNetwork,
            AreaType::TransmissionNetwork,
            AreaType::RegionalNetwork,
            AreaType::ArealNetwork,
            AreaType::BasicSupplyArea,
            AreaType::SupplyArea,
        ] {
            let json = serde_json::to_string(&area_type).unwrap();
            let parsed: AreaType = serde_json::from_str(&json).unwrap();
            assert_eq!(area_type, parsed);
        }
    }
}
