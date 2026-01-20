//! Price model (Preismodell) enumeration.

use serde::{Deserialize, Serialize};

/// Price model for energy delivery tenders.
///
/// Designation of price models in tenders for energy delivery.
///
/// German: Preismodell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PriceModel {
    /// Fixed price (Festpreis)
    #[serde(rename = "FESTPREIS")]
    FixedPrice,

    /// Tranche-based pricing (Tranche)
    #[serde(rename = "TRANCHE")]
    Tranche,
}

impl PriceModel {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::FixedPrice => "Festpreis",
            Self::Tranche => "Tranche",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&PriceModel::FixedPrice).unwrap(),
            r#""FESTPREIS""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for model in [PriceModel::FixedPrice, PriceModel::Tranche] {
            let json = serde_json::to_string(&model).unwrap();
            let parsed: PriceModel = serde_json::from_str(&json).unwrap();
            assert_eq!(model, parsed);
        }
    }
}
