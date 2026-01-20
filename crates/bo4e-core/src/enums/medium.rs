//! Physical medium (Medium) enumeration.

use serde::{Deserialize, Serialize};

/// Physical medium type.
///
/// Specifies a physical medium.
///
/// German: Medium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Medium {
    /// Electricity (Strom)
    #[serde(rename = "STROM")]
    Electricity,

    /// Gas
    #[serde(rename = "GAS")]
    Gas,

    /// Water (Wasser)
    #[serde(rename = "WASSER")]
    Water,

    /// Steam (Dampf)
    #[serde(rename = "DAMPF")]
    Steam,
}

impl Medium {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Electricity => "Strom",
            Self::Gas => "Gas",
            Self::Water => "Wasser",
            Self::Steam => "Dampf",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&Medium::Electricity).unwrap(),
            r#""STROM""#
        );
        assert_eq!(serde_json::to_string(&Medium::Gas).unwrap(), r#""GAS""#);
    }

    #[test]
    fn test_roundtrip() {
        for medium in [
            Medium::Electricity,
            Medium::Gas,
            Medium::Water,
            Medium::Steam,
        ] {
            let json = serde_json::to_string(&medium).unwrap();
            let parsed: Medium = serde_json::from_str(&json).unwrap();
            assert_eq!(medium, parsed);
        }
    }
}
