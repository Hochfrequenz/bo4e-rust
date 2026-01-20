//! Location type (Lokationstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of location.
///
/// Indicates whether it is a market location, metering location, or other location type.
///
/// German: Lokationstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LocationType {
    /// Market location (Marktlokation)
    #[serde(rename = "MALO")]
    MarketLocation,

    /// Metering location (Messlokation)
    #[serde(rename = "MELO")]
    MeteringLocation,

    /// Network location (Netzlokation)
    #[serde(rename = "NELO")]
    NetworkLocation,

    /// Controllable resource (Steuerbare Ressource)
    #[serde(rename = "SR")]
    ControllableResource,

    /// Technical resource (Technische Ressource)
    #[serde(rename = "TR")]
    TechnicalResource,
}

impl LocationType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::MarketLocation => "Marktlokation",
            Self::MeteringLocation => "Messlokation",
            Self::NetworkLocation => "Netzlokation",
            Self::ControllableResource => "Steuerbare Ressource",
            Self::TechnicalResource => "Technische Ressource",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&LocationType::MarketLocation).unwrap(),
            r#""MALO""#
        );
        assert_eq!(
            serde_json::to_string(&LocationType::MeteringLocation).unwrap(),
            r#""MELO""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for loc in [
            LocationType::MarketLocation,
            LocationType::MeteringLocation,
            LocationType::NetworkLocation,
            LocationType::ControllableResource,
            LocationType::TechnicalResource,
        ] {
            let json = serde_json::to_string(&loc).unwrap();
            let parsed: LocationType = serde_json::from_str(&json).unwrap();
            assert_eq!(loc, parsed);
        }
    }
}
