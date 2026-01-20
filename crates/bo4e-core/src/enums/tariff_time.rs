//! Tariff time (Tarifzeit) enumeration.

use serde::{Deserialize, Serialize};

/// Tariff time period.
///
/// Used to identify different tariff times, for example for pricing or consumption measurement.
///
/// German: Tarifzeit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Tarifzeit"))]
#[non_exhaustive]
pub enum TariffTime {
    /// Standard tariff time for single-tariff configurations
    #[serde(rename = "TZ_STANDARD")]
    Standard,

    /// High tariff time for multi-tariff configurations (HT - Hochtarif)
    #[serde(rename = "TZ_HT")]
    HighTariff,

    /// Low tariff time for multi-tariff configurations (NT - Niedrigtarif)
    #[serde(rename = "TZ_NT")]
    LowTariff,
}

impl TariffTime {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Standard => "Tarifzeit Standard",
            Self::HighTariff => "Tarifzeit HT (Hochtarif)",
            Self::LowTariff => "Tarifzeit NT (Niedrigtarif)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TariffTime::Standard).unwrap(),
            r#""TZ_STANDARD""#
        );
        assert_eq!(
            serde_json::to_string(&TariffTime::HighTariff).unwrap(),
            r#""TZ_HT""#
        );
        assert_eq!(
            serde_json::to_string(&TariffTime::LowTariff).unwrap(),
            r#""TZ_NT""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for time in [
            TariffTime::Standard,
            TariffTime::HighTariff,
            TariffTime::LowTariff,
        ] {
            let json = serde_json::to_string(&time).unwrap();
            let parsed: TariffTime = serde_json::from_str(&json).unwrap();
            assert_eq!(time, parsed);
        }
    }
}
