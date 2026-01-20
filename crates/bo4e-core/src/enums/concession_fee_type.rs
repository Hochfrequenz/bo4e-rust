//! Concession fee type (Abgabeart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of concession fee.
///
/// Defines different types of concession fees according to German concession fee regulations (KAV).
///
/// German: Abgabeart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Abgabeart"))]
#[non_exhaustive]
pub enum ConcessionFeeType {
    /// KAS: Special concession contract regulations not integrated into KAV system
    #[serde(rename = "KAS")]
    SpecialConcessionContract,

    /// SA: Special contract customers, price according to § 2 (3) (electricity 0.11 ct/kWh, gas 0.03 ct/kWh)
    #[serde(rename = "SA")]
    SpecialContractCustomer,

    /// SAS: Indication of a deviating price for special contract customers
    #[serde(rename = "SAS")]
    SpecialContractCustomerDeviating,

    /// TA: Tariff customers, for electricity § 2 (2) 1b HT/ET (high concession fee), for gas § 2 (2) 2b
    #[serde(rename = "TA")]
    TariffCustomer,

    /// TAS: Indication of a deviating price for tariff customers
    #[serde(rename = "TAS")]
    TariffCustomerDeviating,

    /// TK: For gas according to KAV § 2 (2) 2a when used exclusively for cooking and hot water
    #[serde(rename = "TK")]
    GasCookingHotWater,

    /// TKS: Indication when a different price is to be used according to KAV § 2 (2) 2a
    #[serde(rename = "TKS")]
    GasCookingHotWaterDeviating,

    /// TS: For electricity with off-peak load § 2 (2) 1a NT (low concession fee, 0.61 ct/kWh)
    #[serde(rename = "TS")]
    ElectricityOffPeak,

    /// TSS: Indication of a deviating price for off-peak load
    #[serde(rename = "TSS")]
    ElectricityOffPeakDeviating,
}

impl ConcessionFeeType {
    /// Returns the German name/description.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::SpecialConcessionContract => "KAS - Konzessionsvertragliche Sonderregelungen",
            Self::SpecialContractCustomer => "SA - Sondervertragskunden",
            Self::SpecialContractCustomerDeviating => {
                "SAS - Abweichender Preis für Sondervertragskunden"
            }
            Self::TariffCustomer => "TA - Tarifkunden",
            Self::TariffCustomerDeviating => "TAS - Abweichender Preis für Tarifkunden",
            Self::GasCookingHotWater => "TK - Gas für Kochen und Warmwasser",
            Self::GasCookingHotWaterDeviating => "TKS - Abweichender Preis nach KAV § 2 (2) 2a",
            Self::ElectricityOffPeak => "TS - Strom Schwachlast",
            Self::ElectricityOffPeakDeviating => "TSS - Abweichender Preis für Schwachlast",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ConcessionFeeType::TariffCustomer).unwrap(),
            r#""TA""#
        );
        assert_eq!(
            serde_json::to_string(&ConcessionFeeType::SpecialContractCustomer).unwrap(),
            r#""SA""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for fee_type in [
            ConcessionFeeType::SpecialConcessionContract,
            ConcessionFeeType::SpecialContractCustomer,
            ConcessionFeeType::SpecialContractCustomerDeviating,
            ConcessionFeeType::TariffCustomer,
            ConcessionFeeType::TariffCustomerDeviating,
            ConcessionFeeType::GasCookingHotWater,
            ConcessionFeeType::GasCookingHotWaterDeviating,
            ConcessionFeeType::ElectricityOffPeak,
            ConcessionFeeType::ElectricityOffPeakDeviating,
        ] {
            let json = serde_json::to_string(&fee_type).unwrap();
            let parsed: ConcessionFeeType = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_type, parsed);
        }
    }
}
