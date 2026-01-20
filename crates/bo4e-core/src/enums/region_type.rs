//! Region type (Gebiettyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of geographical region or area.
///
/// List of possible region types in the German energy market.
///
/// German: Gebiettyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Regiontyp"))]
#[non_exhaustive]
pub enum RegionType {
    /// Control area / regulation zone (Regelzone)
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

    /// Area network / closed distribution system (Arealnetz)
    #[serde(rename = "AREALNETZ")]
    AreaNetwork,

    /// Basic supply area (Grundversorgungsgebiet)
    #[serde(rename = "GRUNDVERSORGUNGSGEBIET")]
    BasicSupplyArea,

    /// Supply area (Versorgungsgebiet)
    #[serde(rename = "VERSORGUNGSGEBIET")]
    SupplyArea,
}

impl RegionType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ControlArea => "Regelzone",
            Self::MarketArea => "Marktgebiet",
            Self::BalancingArea => "Bilanzierungsgebiet",
            Self::DistributionNetwork => "Verteilnetz",
            Self::TransmissionNetwork => "Transportnetz",
            Self::RegionalNetwork => "Regionalnetz",
            Self::AreaNetwork => "Arealnetz",
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
            serde_json::to_string(&RegionType::ControlArea).unwrap(),
            r#""REGELZONE""#
        );
        assert_eq!(
            serde_json::to_string(&RegionType::MarketArea).unwrap(),
            r#""MARKTGEBIET""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<RegionType>(r#""VERTEILNETZ""#).unwrap(),
            RegionType::DistributionNetwork
        );
    }

    #[test]
    fn test_roundtrip() {
        for region_type in [
            RegionType::ControlArea,
            RegionType::MarketArea,
            RegionType::BalancingArea,
            RegionType::DistributionNetwork,
            RegionType::TransmissionNetwork,
            RegionType::RegionalNetwork,
            RegionType::AreaNetwork,
            RegionType::BasicSupplyArea,
            RegionType::SupplyArea,
        ] {
            let json = serde_json::to_string(&region_type).unwrap();
            let parsed: RegionType = serde_json::from_str(&json).unwrap();
            assert_eq!(region_type, parsed);
        }
    }
}
