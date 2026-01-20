//! Concession fee customer group (KundengruppeKA) enumeration.

use serde::{Deserialize, Serialize};

/// Customer group classification for concession fee calculation.
///
/// An enumeration for classifying the level of concession fees.
///
/// German: KundengruppeKA (Kundengruppe Konzessionsabgabe)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConcessionFeeCustomerGroup {
    // Electricity tariff groups
    /// Electricity off-peak/low load (Strom Schwachlast)
    #[serde(rename = "S_SCHWACHLAST")]
    ElectricityOffPeak,

    /// Electricity tariff up to 25,000 kWh
    #[serde(rename = "S_TARIF_25000")]
    ElectricityTariff25000,

    /// Electricity tariff up to 100,000 kWh
    #[serde(rename = "S_TARIF_100000")]
    ElectricityTariff100000,

    /// Electricity tariff up to 500,000 kWh
    #[serde(rename = "S_TARIF_500000")]
    ElectricityTariff500000,

    /// Electricity tariff above 500,000 kWh
    #[serde(rename = "S_TARIF_G_500000")]
    ElectricityTariffAbove500000,

    /// Electricity special contract customer (Strom Sonderkunde)
    #[serde(rename = "S_SONDERKUNDE")]
    ElectricitySpecialCustomer,

    // Gas cooking/hot water tariff groups
    /// Gas cooking/hot water up to 25,000 kWh
    #[serde(rename = "G_KOWA_25000")]
    GasCookingHotWater25000,

    /// Gas cooking/hot water up to 100,000 kWh
    #[serde(rename = "G_KOWA_100000")]
    GasCookingHotWater100000,

    /// Gas cooking/hot water up to 500,000 kWh
    #[serde(rename = "G_KOWA_500000")]
    GasCookingHotWater500000,

    /// Gas cooking/hot water above 500,000 kWh
    #[serde(rename = "G_KOWA_G_500000")]
    GasCookingHotWaterAbove500000,

    // Gas tariff groups
    /// Gas tariff up to 25,000 kWh
    #[serde(rename = "G_TARIF_25000")]
    GasTariff25000,

    /// Gas tariff up to 100,000 kWh
    #[serde(rename = "G_TARIF_100000")]
    GasTariff100000,

    /// Gas tariff up to 500,000 kWh
    #[serde(rename = "G_TARIF_500000")]
    GasTariff500000,

    /// Gas tariff above 500,000 kWh
    #[serde(rename = "G_TARIF_G_500000")]
    GasTariffAbove500000,

    /// Gas special contract customer (Gas Sonderkunde)
    #[serde(rename = "G_SONDERKUNDE")]
    GasSpecialCustomer,

    // Special categories for both electricity and gas
    /// Special KAS - applies to both electricity and gas
    #[serde(rename = "SONDER_KAS")]
    SpecialKAS,

    /// Special SAS - applies to both electricity and gas
    #[serde(rename = "SONDER_SAS")]
    SpecialSAS,

    /// Special TAS - applies to both electricity and gas
    #[serde(rename = "SONDER_TAS")]
    SpecialTAS,

    /// Special TKS - applies to gas
    #[serde(rename = "SONDER_TKS")]
    SpecialTKS,

    /// Special TSS - applies to electricity
    #[serde(rename = "SONDER_TSS")]
    SpecialTSS,
}

impl ConcessionFeeCustomerGroup {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ElectricityOffPeak => "Strom Schwachlast",
            Self::ElectricityTariff25000 => "Strom Tarif bis 25.000 kWh",
            Self::ElectricityTariff100000 => "Strom Tarif bis 100.000 kWh",
            Self::ElectricityTariff500000 => "Strom Tarif bis 500.000 kWh",
            Self::ElectricityTariffAbove500000 => "Strom Tarif über 500.000 kWh",
            Self::ElectricitySpecialCustomer => "Strom Sonderkunde",
            Self::GasCookingHotWater25000 => "Gas Kochen/Warmwasser bis 25.000 kWh",
            Self::GasCookingHotWater100000 => "Gas Kochen/Warmwasser bis 100.000 kWh",
            Self::GasCookingHotWater500000 => "Gas Kochen/Warmwasser bis 500.000 kWh",
            Self::GasCookingHotWaterAbove500000 => "Gas Kochen/Warmwasser über 500.000 kWh",
            Self::GasTariff25000 => "Gas Tarif bis 25.000 kWh",
            Self::GasTariff100000 => "Gas Tarif bis 100.000 kWh",
            Self::GasTariff500000 => "Gas Tarif bis 500.000 kWh",
            Self::GasTariffAbove500000 => "Gas Tarif über 500.000 kWh",
            Self::GasSpecialCustomer => "Gas Sonderkunde",
            Self::SpecialKAS => "Sonder KAS",
            Self::SpecialSAS => "Sonder SAS",
            Self::SpecialTAS => "Sonder TAS",
            Self::SpecialTKS => "Sonder TKS (Gas)",
            Self::SpecialTSS => "Sonder TSS (Strom)",
        }
    }

    /// Returns true if this group applies to electricity.
    pub fn is_electricity(&self) -> bool {
        matches!(
            self,
            Self::ElectricityOffPeak
                | Self::ElectricityTariff25000
                | Self::ElectricityTariff100000
                | Self::ElectricityTariff500000
                | Self::ElectricityTariffAbove500000
                | Self::ElectricitySpecialCustomer
                | Self::SpecialKAS
                | Self::SpecialSAS
                | Self::SpecialTAS
                | Self::SpecialTSS
        )
    }

    /// Returns true if this group applies to gas.
    pub fn is_gas(&self) -> bool {
        matches!(
            self,
            Self::GasCookingHotWater25000
                | Self::GasCookingHotWater100000
                | Self::GasCookingHotWater500000
                | Self::GasCookingHotWaterAbove500000
                | Self::GasTariff25000
                | Self::GasTariff100000
                | Self::GasTariff500000
                | Self::GasTariffAbove500000
                | Self::GasSpecialCustomer
                | Self::SpecialKAS
                | Self::SpecialSAS
                | Self::SpecialTAS
                | Self::SpecialTKS
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ConcessionFeeCustomerGroup::ElectricityOffPeak).unwrap(),
            r#""S_SCHWACHLAST""#
        );
        assert_eq!(
            serde_json::to_string(&ConcessionFeeCustomerGroup::GasTariff25000).unwrap(),
            r#""G_TARIF_25000""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for group in [
            ConcessionFeeCustomerGroup::ElectricityOffPeak,
            ConcessionFeeCustomerGroup::ElectricityTariff25000,
            ConcessionFeeCustomerGroup::ElectricityTariff100000,
            ConcessionFeeCustomerGroup::ElectricityTariff500000,
            ConcessionFeeCustomerGroup::ElectricityTariffAbove500000,
            ConcessionFeeCustomerGroup::ElectricitySpecialCustomer,
            ConcessionFeeCustomerGroup::GasCookingHotWater25000,
            ConcessionFeeCustomerGroup::GasCookingHotWater100000,
            ConcessionFeeCustomerGroup::GasCookingHotWater500000,
            ConcessionFeeCustomerGroup::GasCookingHotWaterAbove500000,
            ConcessionFeeCustomerGroup::GasTariff25000,
            ConcessionFeeCustomerGroup::GasTariff100000,
            ConcessionFeeCustomerGroup::GasTariff500000,
            ConcessionFeeCustomerGroup::GasTariffAbove500000,
            ConcessionFeeCustomerGroup::GasSpecialCustomer,
            ConcessionFeeCustomerGroup::SpecialKAS,
            ConcessionFeeCustomerGroup::SpecialSAS,
            ConcessionFeeCustomerGroup::SpecialTAS,
            ConcessionFeeCustomerGroup::SpecialTKS,
            ConcessionFeeCustomerGroup::SpecialTSS,
        ] {
            let json = serde_json::to_string(&group).unwrap();
            let parsed: ConcessionFeeCustomerGroup = serde_json::from_str(&json).unwrap();
            assert_eq!(group, parsed);
        }
    }

    #[test]
    fn test_is_electricity() {
        assert!(ConcessionFeeCustomerGroup::ElectricityOffPeak.is_electricity());
        assert!(ConcessionFeeCustomerGroup::SpecialTSS.is_electricity());
        assert!(!ConcessionFeeCustomerGroup::GasTariff25000.is_electricity());
    }

    #[test]
    fn test_is_gas() {
        assert!(ConcessionFeeCustomerGroup::GasTariff25000.is_gas());
        assert!(ConcessionFeeCustomerGroup::SpecialTKS.is_gas());
        assert!(!ConcessionFeeCustomerGroup::ElectricityOffPeak.is_gas());
    }
}
