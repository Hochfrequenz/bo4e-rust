//! Unit prefix (Mengeneinheitenpraefix) enumeration.

use serde::{Deserialize, Serialize};

/// SI unit prefix.
///
/// Metric prefixes used with units of measurement.
///
/// German: Mengeneinheitenpraefix
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UnitPrefix {
    /// Exa (10^18)
    #[serde(rename = "EXA")]
    Exa,

    /// Peta (10^15)
    #[serde(rename = "PETA")]
    Peta,

    /// Tera (10^12)
    #[serde(rename = "TERA")]
    Tera,

    /// Giga (10^9)
    #[serde(rename = "GIGA")]
    Giga,

    /// Mega (10^6)
    #[serde(rename = "MEGA")]
    Mega,

    /// Kilo (10^3)
    #[serde(rename = "KILO")]
    Kilo,

    /// Hecto (10^2)
    #[serde(rename = "HEKTO")]
    Hecto,

    /// Deca (10^1)
    #[serde(rename = "DEKA")]
    Deca,

    /// No prefix (10^0)
    #[serde(rename = "OHNE")]
    None,

    /// Deci (10^-1)
    #[serde(rename = "DEZI")]
    Deci,

    /// Centi (10^-2)
    #[serde(rename = "ZENTI")]
    Centi,

    /// Milli (10^-3)
    #[serde(rename = "MILLI")]
    Milli,

    /// Micro (10^-6)
    #[serde(rename = "MIKRO")]
    Micro,

    /// Nano (10^-9)
    #[serde(rename = "NANO")]
    Nano,

    /// Pico (10^-12)
    #[serde(rename = "PIKO")]
    Pico,

    /// Femto (10^-15)
    #[serde(rename = "FEMTO")]
    Femto,

    /// Atto (10^-18)
    #[serde(rename = "ATTO")]
    Atto,
}

impl UnitPrefix {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Exa => "Exa",
            Self::Peta => "Peta",
            Self::Tera => "Tera",
            Self::Giga => "Giga",
            Self::Mega => "Mega",
            Self::Kilo => "Kilo",
            Self::Hecto => "Hekto",
            Self::Deca => "Deka",
            Self::None => "Ohne",
            Self::Deci => "Dezi",
            Self::Centi => "Zenti",
            Self::Milli => "Milli",
            Self::Micro => "Mikro",
            Self::Nano => "Nano",
            Self::Pico => "Piko",
            Self::Femto => "Femto",
            Self::Atto => "Atto",
        }
    }

    /// Returns the power of 10 for this prefix.
    pub fn exponent(&self) -> i32 {
        match self {
            Self::Exa => 18,
            Self::Peta => 15,
            Self::Tera => 12,
            Self::Giga => 9,
            Self::Mega => 6,
            Self::Kilo => 3,
            Self::Hecto => 2,
            Self::Deca => 1,
            Self::None => 0,
            Self::Deci => -1,
            Self::Centi => -2,
            Self::Milli => -3,
            Self::Micro => -6,
            Self::Nano => -9,
            Self::Pico => -12,
            Self::Femto => -15,
            Self::Atto => -18,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&UnitPrefix::Kilo).unwrap(),
            r#""KILO""#
        );
        assert_eq!(
            serde_json::to_string(&UnitPrefix::Mega).unwrap(),
            r#""MEGA""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for prefix in [
            UnitPrefix::Exa,
            UnitPrefix::Kilo,
            UnitPrefix::None,
            UnitPrefix::Milli,
            UnitPrefix::Nano,
        ] {
            let json = serde_json::to_string(&prefix).unwrap();
            let parsed: UnitPrefix = serde_json::from_str(&json).unwrap();
            assert_eq!(prefix, parsed);
        }
    }

    #[test]
    fn test_exponent() {
        assert_eq!(UnitPrefix::Kilo.exponent(), 3);
        assert_eq!(UnitPrefix::Mega.exponent(), 6);
        assert_eq!(UnitPrefix::Milli.exponent(), -3);
        assert_eq!(UnitPrefix::None.exponent(), 0);
    }
}
