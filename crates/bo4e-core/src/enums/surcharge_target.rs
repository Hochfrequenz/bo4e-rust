//! Surcharge target (AufAbschlagsziel) enumeration.

use serde::{Deserialize, Serialize};

/// Target price for surcharge or discount.
///
/// The price to which a surcharge or discount applies.
///
/// German: AufAbschlagsziel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "AufAbschlagsziel"))]
#[non_exhaustive]
pub enum SurchargeTarget {
    /// Working price single tariff (Arbeitspreis Eintarif)
    #[serde(rename = "ARBEITSPREIS_EINTARIF")]
    WorkingPriceSingleTariff,

    /// Working price high tariff (Arbeitspreis HT)
    #[serde(rename = "ARBEITSPREIS_HT")]
    WorkingPriceHT,

    /// Working price low tariff (Arbeitspreis NT)
    #[serde(rename = "ARBEITSPREIS_NT")]
    WorkingPriceNT,

    /// Working price HT and NT combined (Arbeitspreis HT und NT)
    #[serde(rename = "ARBEITSPREIS_HT_NT")]
    WorkingPriceHTNT,

    /// Base price (Grundpreis)
    #[serde(rename = "GRUNDPREIS")]
    BasePrice,

    /// Total price (Gesamtpreis)
    #[serde(rename = "GESAMTPREIS")]
    TotalPrice,
}

impl SurchargeTarget {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::WorkingPriceSingleTariff => "Auf-/Abschlag auf den Arbeitspreis Eintarif",
            Self::WorkingPriceHT => "Auf-/Abschlag auf den Arbeitspreis HT",
            Self::WorkingPriceNT => "Auf-/Abschlag auf den Arbeitspreis NT",
            Self::WorkingPriceHTNT => "Auf-/Abschlag auf den Arbeitspreis HT und NT",
            Self::BasePrice => "Auf-/Abschlag auf den Grundpreis",
            Self::TotalPrice => "Auf-/Abschlag auf den Gesamtpreis",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&SurchargeTarget::BasePrice).unwrap(),
            r#""GRUNDPREIS""#
        );
        assert_eq!(
            serde_json::to_string(&SurchargeTarget::TotalPrice).unwrap(),
            r#""GESAMTPREIS""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for target in [
            SurchargeTarget::WorkingPriceSingleTariff,
            SurchargeTarget::WorkingPriceHT,
            SurchargeTarget::WorkingPriceNT,
            SurchargeTarget::WorkingPriceHTNT,
            SurchargeTarget::BasePrice,
            SurchargeTarget::TotalPrice,
        ] {
            let json = serde_json::to_string(&target).unwrap();
            let parsed: SurchargeTarget = serde_json::from_str(&json).unwrap();
            assert_eq!(target, parsed);
        }
    }
}
