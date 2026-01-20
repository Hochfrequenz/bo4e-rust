//! Energy direction (Energierichtung) enumeration.

use serde::{Deserialize, Serialize};

/// Direction of energy flow.
///
/// Specifies the energy direction of a market and/or metering location.
///
/// German: Energierichtung
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Energierichtung"))]
#[non_exhaustive]
pub enum EnergyDirection {
    /// Energy feed-out/withdrawal (Ausspeisung)
    #[serde(rename = "AUSSP")]
    FeedOut,

    /// Energy feed-in/injection (Einspeisung)
    #[serde(rename = "EINSP")]
    FeedIn,
}

impl EnergyDirection {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::FeedOut => "Ausspeisung",
            Self::FeedIn => "Einspeisung",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&EnergyDirection::FeedOut).unwrap(),
            r#""AUSSP""#
        );
        assert_eq!(
            serde_json::to_string(&EnergyDirection::FeedIn).unwrap(),
            r#""EINSP""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<EnergyDirection>(r#""AUSSP""#).unwrap(),
            EnergyDirection::FeedOut
        );
        assert_eq!(
            serde_json::from_str::<EnergyDirection>(r#""EINSP""#).unwrap(),
            EnergyDirection::FeedIn
        );
    }

    #[test]
    fn test_roundtrip() {
        for dir in [EnergyDirection::FeedOut, EnergyDirection::FeedIn] {
            let json = serde_json::to_string(&dir).unwrap();
            let parsed: EnergyDirection = serde_json::from_str(&json).unwrap();
            assert_eq!(dir, parsed);
        }
    }
}
