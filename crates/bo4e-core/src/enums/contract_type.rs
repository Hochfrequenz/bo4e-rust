//! Contract type (Vertragsart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of contract in the energy sector.
///
/// German: Vertragsart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContractType {
    /// Energy supply contract (Energieliefervertrag)
    #[serde(rename = "ENERGIELIEFERVERTRAG")]
    EnergySupplyContract,

    /// Network usage contract (Netznutzungsvertrag)
    #[serde(rename = "NETZNUTZUNGSVERTRAG")]
    NetworkUsageContract,

    /// Balancing contract (Bilanzierungsvertrag)
    #[serde(rename = "BILANZIERUNGSVERTRAG")]
    BalancingContract,

    /// Metering point operation contract (Messstellenbetriebsvertrag)
    #[serde(rename = "MESSSTELLENBETRIEBSVERTRAG")]
    MeteringPointOperationContract,

    /// Bundle contract (Buendelvertrag)
    #[serde(rename = "BUENDELVERTRAG")]
    BundleContract,
}

impl ContractType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::EnergySupplyContract => "Energieliefervertrag",
            Self::NetworkUsageContract => "Netznutzungsvertrag",
            Self::BalancingContract => "Bilanzierungsvertrag",
            Self::MeteringPointOperationContract => "Messstellenbetriebsvertrag",
            Self::BundleContract => "Buendelvertrag",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ContractType::EnergySupplyContract).unwrap(),
            r#""ENERGIELIEFERVERTRAG""#
        );
        assert_eq!(
            serde_json::to_string(&ContractType::NetworkUsageContract).unwrap(),
            r#""NETZNUTZUNGSVERTRAG""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<ContractType>(r#""ENERGIELIEFERVERTRAG""#).unwrap(),
            ContractType::EnergySupplyContract
        );
        assert_eq!(
            serde_json::from_str::<ContractType>(r#""MESSSTELLENBETRIEBSVERTRAG""#).unwrap(),
            ContractType::MeteringPointOperationContract
        );
    }

    #[test]
    fn test_roundtrip() {
        for contract_type in [
            ContractType::EnergySupplyContract,
            ContractType::NetworkUsageContract,
            ContractType::BalancingContract,
            ContractType::MeteringPointOperationContract,
            ContractType::BundleContract,
        ] {
            let json = serde_json::to_string(&contract_type).unwrap();
            let parsed: ContractType = serde_json::from_str(&json).unwrap();
            assert_eq!(contract_type, parsed);
        }
    }
}
