//! Tariff calculation method (Tarifkalkulationsmethode) enumeration.

use serde::{Deserialize, Serialize};

/// Tariff calculation method.
///
/// List of different calculation methods for a price sheet in tariff context.
///
/// German: Tarifkalkulationsmethode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TariffCalculationMethod {
    /// No calculation, just multiply quantity by price
    #[serde(rename = "KEINE")]
    None,

    /// Tier model - total quantity assigned to one tier, price applies to entire quantity
    #[serde(rename = "STAFFELN")]
    Tiers,

    /// Zone model - total quantity distributed across zones with respective prices
    #[serde(rename = "ZONEN")]
    Zones,

    /// Best billing within tiers
    #[serde(rename = "BESTABRECHNUNG_STAFFEL")]
    BestBillingTier,

    /// Package price (price for a quantity package)
    #[serde(rename = "PAKETPREIS")]
    PackagePrice,
}

impl TariffCalculationMethod {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::None => "Keine",
            Self::Tiers => "Staffeln",
            Self::Zones => "Zonen",
            Self::BestBillingTier => "Bestabrechnung Staffel",
            Self::PackagePrice => "Paketpreis",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TariffCalculationMethod::None).unwrap(),
            r#""KEINE""#
        );
        assert_eq!(
            serde_json::to_string(&TariffCalculationMethod::Tiers).unwrap(),
            r#""STAFFELN""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for method in [
            TariffCalculationMethod::None,
            TariffCalculationMethod::Tiers,
            TariffCalculationMethod::Zones,
            TariffCalculationMethod::BestBillingTier,
            TariffCalculationMethod::PackagePrice,
        ] {
            let json = serde_json::to_string(&method).unwrap();
            let parsed: TariffCalculationMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(method, parsed);
        }
    }
}
