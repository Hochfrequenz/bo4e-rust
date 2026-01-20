//! Price guarantee type (Preisgarantietyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of price guarantee.
///
/// Enumeration of options for granting price guarantees.
///
/// German: Preisgarantietyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Preisgarantietyp"))]
#[non_exhaustive]
pub enum PriceGuaranteeType {
    /// All price components gross (Alle Preisbestandteile brutto)
    #[serde(rename = "ALLE_PREISBESTANDTEILE_BRUTTO")]
    AllComponentsGross,

    /// All price components net (Alle Preisbestandteile netto)
    #[serde(rename = "ALLE_PREISBESTANDTEILE_NETTO")]
    AllComponentsNet,

    /// Price components without taxes/fees (Preisbestandteile ohne Abgaben)
    #[serde(rename = "PREISBESTANDTEILE_OHNE_ABGABEN")]
    ComponentsWithoutFees,

    /// Energy price only (Nur Energiepreis)
    #[serde(rename = "NUR_ENERGIEPREIS")]
    EnergyPriceOnly,
}

impl PriceGuaranteeType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::AllComponentsGross => "Alle Preisbestandteile brutto",
            Self::AllComponentsNet => "Alle Preisbestandteile netto",
            Self::ComponentsWithoutFees => "Preisbestandteile ohne Abgaben",
            Self::EnergyPriceOnly => "Nur Energiepreis",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&PriceGuaranteeType::AllComponentsGross).unwrap(),
            r#""ALLE_PREISBESTANDTEILE_BRUTTO""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for guarantee_type in [
            PriceGuaranteeType::AllComponentsGross,
            PriceGuaranteeType::AllComponentsNet,
            PriceGuaranteeType::ComponentsWithoutFees,
            PriceGuaranteeType::EnergyPriceOnly,
        ] {
            let json = serde_json::to_string(&guarantee_type).unwrap();
            let parsed: PriceGuaranteeType = serde_json::from_str(&json).unwrap();
            assert_eq!(guarantee_type, parsed);
        }
    }
}
