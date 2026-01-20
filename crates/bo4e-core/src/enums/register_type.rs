//! Register type/count (Registeranzahl) enumeration.

use serde::{Deserialize, Serialize};

/// Register type/tariff register count.
///
/// The register count is used to characterize meters and resulting tariffs.
///
/// German: Registeranzahl (also known as Registerart)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RegisterType {
    /// Single tariff (Eintarif)
    #[serde(rename = "EINTARIF")]
    SingleTariff,

    /// Dual tariff (Zweitarif)
    #[serde(rename = "ZWEITARIF")]
    DualTariff,

    /// Multi-tariff (Mehrtarif)
    #[serde(rename = "MEHRTARIF")]
    MultiTariff,
}

impl RegisterType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::SingleTariff => "Eintarif",
            Self::DualTariff => "Zweitarif",
            Self::MultiTariff => "Mehrtarif",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&RegisterType::SingleTariff).unwrap(),
            r#""EINTARIF""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for reg in [
            RegisterType::SingleTariff,
            RegisterType::DualTariff,
            RegisterType::MultiTariff,
        ] {
            let json = serde_json::to_string(&reg).unwrap();
            let parsed: RegisterType = serde_json::from_str(&json).unwrap();
            assert_eq!(reg, parsed);
        }
    }
}
