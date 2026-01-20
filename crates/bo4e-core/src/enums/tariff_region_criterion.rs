//! Tariff region criterion (Tarifregionskriterium) enumeration.

use serde::{Deserialize, Serialize};

/// Tariff region criterion.
///
/// Criteria to define regional areas for tariffs.
///
/// German: Tarifregionskriterium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Tarifregionskriterium"))]
#[non_exhaustive]
pub enum TariffRegionCriterion {
    /// Network number (Netznummer)
    #[serde(rename = "NETZ_NUMMER")]
    NetworkNumber,

    /// Postal code (Postleitzahl)
    #[serde(rename = "POSTLEITZAHL")]
    PostalCode,

    /// City/Town (Ort)
    #[serde(rename = "ORT")]
    City,

    /// Basic supplier number (Grundversorgernummer)
    #[serde(rename = "GRUNDVERSORGER_NUMMER")]
    BasicSupplierNumber,

    /// Reference to a Region business object (URL)
    #[serde(rename = "REGION")]
    Region,
}

impl TariffRegionCriterion {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::NetworkNumber => "Netznummer",
            Self::PostalCode => "Postleitzahl",
            Self::City => "Ort",
            Self::BasicSupplierNumber => "Grundversorgernummer",
            Self::Region => "Region",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TariffRegionCriterion::PostalCode).unwrap(),
            r#""POSTLEITZAHL""#
        );
        assert_eq!(
            serde_json::to_string(&TariffRegionCriterion::City).unwrap(),
            r#""ORT""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for criterion in [
            TariffRegionCriterion::NetworkNumber,
            TariffRegionCriterion::PostalCode,
            TariffRegionCriterion::City,
            TariffRegionCriterion::BasicSupplierNumber,
            TariffRegionCriterion::Region,
        ] {
            let json = serde_json::to_string(&criterion).unwrap();
            let parsed: TariffRegionCriterion = serde_json::from_str(&json).unwrap();
            assert_eq!(criterion, parsed);
        }
    }
}
