//! Tariff type (Tariftyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of tariff.
///
/// Differentiates between basic/backup supply tariffs and other offered tariffs.
///
/// German: Tariftyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Tariftyp"))]
#[non_exhaustive]
pub enum TariffType {
    /// Basic and backup supply (Grund- und Ersatzversorgung)
    #[serde(rename = "GRUND_ERSATZVERSORGUNG")]
    BasicAndBackupSupply,

    /// Basic supply (Grundversorgung)
    #[serde(rename = "GRUNDVERSORGUNG")]
    BasicSupply,

    /// Backup supply (Ersatzversorgung)
    #[serde(rename = "ERSATZVERSORGUNG")]
    BackupSupply,

    /// Special tariff (Sondertarif)
    #[serde(rename = "SONDERTARIF")]
    SpecialTariff,
}

impl TariffType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::BasicAndBackupSupply => "Grund- und Ersatzversorgung",
            Self::BasicSupply => "Grundversorgung",
            Self::BackupSupply => "Ersatzversorgung",
            Self::SpecialTariff => "Sondertarif",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TariffType::BasicSupply).unwrap(),
            r#""GRUNDVERSORGUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for tariff in [
            TariffType::BasicAndBackupSupply,
            TariffType::BasicSupply,
            TariffType::BackupSupply,
            TariffType::SpecialTariff,
        ] {
            let json = serde_json::to_string(&tariff).unwrap();
            let parsed: TariffType = serde_json::from_str(&json).unwrap();
            assert_eq!(tariff, parsed);
        }
    }
}
