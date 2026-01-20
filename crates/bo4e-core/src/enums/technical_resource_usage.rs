//! Technical resource usage (TechnischeRessourceNutzung) enumeration.

use serde::{Deserialize, Serialize};

/// Usage type of a technical resource.
///
/// Describes how a technical resource is used in the energy system.
///
/// German: TechnischeRessourceNutzung
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TechnicalResourceUsage {
    /// Electricity consumption type (Stromverbrauchsart)
    #[serde(rename = "STROMVERBRAUCHSART")]
    ElectricityConsumptionType,

    /// Electricity generation type (Stromerzeugungsart)
    #[serde(rename = "STROMERZEUGUNGSART")]
    ElectricityGenerationType,

    /// Storage (Speicher)
    #[serde(rename = "SPEICHER")]
    Storage,
}

impl TechnicalResourceUsage {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ElectricityConsumptionType => "Stromverbrauchsart",
            Self::ElectricityGenerationType => "Stromerzeugungsart",
            Self::Storage => "Speicher",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TechnicalResourceUsage::Storage).unwrap(),
            r#""SPEICHER""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for usage in [
            TechnicalResourceUsage::ElectricityConsumptionType,
            TechnicalResourceUsage::ElectricityGenerationType,
            TechnicalResourceUsage::Storage,
        ] {
            let json = serde_json::to_string(&usage).unwrap();
            let parsed: TechnicalResourceUsage = serde_json::from_str(&json).unwrap();
            assert_eq!(usage, parsed);
        }
    }
}
