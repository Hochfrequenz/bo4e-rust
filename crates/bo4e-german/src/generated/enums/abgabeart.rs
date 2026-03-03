#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Abgabeart {
    #[serde(rename = "KAS")]
    SpecialConcessionContract,
    #[serde(rename = "SA")]
    SpecialContractCustomer,
    #[serde(rename = "SAS")]
    SpecialContractCustomerDeviating,
    #[serde(rename = "TA")]
    TariffCustomer,
    #[serde(rename = "TAS")]
    TariffCustomerDeviating,
    #[serde(rename = "TK")]
    GasCookingHotWater,
    #[serde(rename = "TKS")]
    GasCookingHotWaterDeviating,
    #[serde(rename = "TS")]
    ElectricityOffPeak,
    #[serde(rename = "TSS")]
    ElectricityOffPeakDeviating,
}
impl From<bo4e_core::enums::ConcessionFeeType> for Abgabeart {
    fn from(v: bo4e_core::enums::ConcessionFeeType) -> Self {
        match v {
            bo4e_core::enums::ConcessionFeeType::SpecialConcessionContract => {
                Abgabeart::SpecialConcessionContract
            }
            bo4e_core::enums::ConcessionFeeType::SpecialContractCustomer => {
                Abgabeart::SpecialContractCustomer
            }
            bo4e_core::enums::ConcessionFeeType::SpecialContractCustomerDeviating => {
                Abgabeart::SpecialContractCustomerDeviating
            }
            bo4e_core::enums::ConcessionFeeType::TariffCustomer => {
                Abgabeart::TariffCustomer
            }
            bo4e_core::enums::ConcessionFeeType::TariffCustomerDeviating => {
                Abgabeart::TariffCustomerDeviating
            }
            bo4e_core::enums::ConcessionFeeType::GasCookingHotWater => {
                Abgabeart::GasCookingHotWater
            }
            bo4e_core::enums::ConcessionFeeType::GasCookingHotWaterDeviating => {
                Abgabeart::GasCookingHotWaterDeviating
            }
            bo4e_core::enums::ConcessionFeeType::ElectricityOffPeak => {
                Abgabeart::ElectricityOffPeak
            }
            bo4e_core::enums::ConcessionFeeType::ElectricityOffPeakDeviating => {
                Abgabeart::ElectricityOffPeakDeviating
            }
            _ => panic!("Unknown {} variant", stringify!(ConcessionFeeType)),
        }
    }
}
impl From<Abgabeart> for bo4e_core::enums::ConcessionFeeType {
    fn from(v: Abgabeart) -> Self {
        match v {
            Abgabeart::SpecialConcessionContract => {
                bo4e_core::enums::ConcessionFeeType::SpecialConcessionContract
            }
            Abgabeart::SpecialContractCustomer => {
                bo4e_core::enums::ConcessionFeeType::SpecialContractCustomer
            }
            Abgabeart::SpecialContractCustomerDeviating => {
                bo4e_core::enums::ConcessionFeeType::SpecialContractCustomerDeviating
            }
            Abgabeart::TariffCustomer => {
                bo4e_core::enums::ConcessionFeeType::TariffCustomer
            }
            Abgabeart::TariffCustomerDeviating => {
                bo4e_core::enums::ConcessionFeeType::TariffCustomerDeviating
            }
            Abgabeart::GasCookingHotWater => {
                bo4e_core::enums::ConcessionFeeType::GasCookingHotWater
            }
            Abgabeart::GasCookingHotWaterDeviating => {
                bo4e_core::enums::ConcessionFeeType::GasCookingHotWaterDeviating
            }
            Abgabeart::ElectricityOffPeak => {
                bo4e_core::enums::ConcessionFeeType::ElectricityOffPeak
            }
            Abgabeart::ElectricityOffPeakDeviating => {
                bo4e_core::enums::ConcessionFeeType::ElectricityOffPeakDeviating
            }
            _ => panic!("Unknown {} variant", stringify!(Abgabeart)),
        }
    }
}
