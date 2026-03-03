#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum KundengruppeKA {
    #[serde(rename = "S_SCHWACHLAST")]
    StromSchwachlast,
    #[serde(rename = "S_TARIF_25000")]
    ElectricityTariff25000,
    #[serde(rename = "S_TARIF_100000")]
    ElectricityTariff100000,
    #[serde(rename = "S_TARIF_500000")]
    ElectricityTariff500000,
    #[serde(rename = "S_TARIF_G_500000")]
    ElectricityTariffAbove500000,
    #[serde(rename = "S_SONDERKUNDE")]
    StromSonderkunde,
    #[serde(rename = "G_KOWA_25000")]
    GasCookingHotWater25000,
    #[serde(rename = "G_KOWA_100000")]
    GasCookingHotWater100000,
    #[serde(rename = "G_KOWA_500000")]
    GasCookingHotWater500000,
    #[serde(rename = "G_KOWA_G_500000")]
    GasCookingHotWaterAbove500000,
    #[serde(rename = "G_TARIF_25000")]
    GasTariff25000,
    #[serde(rename = "G_TARIF_100000")]
    GasTariff100000,
    #[serde(rename = "G_TARIF_500000")]
    GasTariff500000,
    #[serde(rename = "G_TARIF_G_500000")]
    GasTariffAbove500000,
    #[serde(rename = "G_SONDERKUNDE")]
    GasSonderkunde,
    #[serde(rename = "SONDER_KAS")]
    SpecialKAS,
    #[serde(rename = "SONDER_SAS")]
    SpecialSAS,
    #[serde(rename = "SONDER_TAS")]
    SpecialTAS,
    #[serde(rename = "SONDER_TKS")]
    SpecialTKS,
    #[serde(rename = "SONDER_TSS")]
    SpecialTSS,
}
impl From<bo4e_core::enums::ConcessionFeeCustomerGroup> for KundengruppeKA {
    fn from(v: bo4e_core::enums::ConcessionFeeCustomerGroup) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityOffPeak => {
                KundengruppeKA::StromSchwachlast
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariff25000 => {
                KundengruppeKA::ElectricityTariff25000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariff100000 => {
                KundengruppeKA::ElectricityTariff100000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariff500000 => {
                KundengruppeKA::ElectricityTariff500000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariffAbove500000 => {
                KundengruppeKA::ElectricityTariffAbove500000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricitySpecialCustomer => {
                KundengruppeKA::StromSonderkunde
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWater25000 => {
                KundengruppeKA::GasCookingHotWater25000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWater100000 => {
                KundengruppeKA::GasCookingHotWater100000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWater500000 => {
                KundengruppeKA::GasCookingHotWater500000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWaterAbove500000 => {
                KundengruppeKA::GasCookingHotWaterAbove500000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariff25000 => {
                KundengruppeKA::GasTariff25000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariff100000 => {
                KundengruppeKA::GasTariff100000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariff500000 => {
                KundengruppeKA::GasTariff500000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariffAbove500000 => {
                KundengruppeKA::GasTariffAbove500000
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::GasSpecialCustomer => {
                KundengruppeKA::GasSonderkunde
            }
            bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialKAS => KundengruppeKA::SpecialKAS,
            bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialSAS => KundengruppeKA::SpecialSAS,
            bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialTAS => KundengruppeKA::SpecialTAS,
            bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialTKS => KundengruppeKA::SpecialTKS,
            bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialTSS => KundengruppeKA::SpecialTSS,
            _ => panic!("Unknown {} variant", stringify!(ConcessionFeeCustomerGroup)),
        }
    }
}
impl From<KundengruppeKA> for bo4e_core::enums::ConcessionFeeCustomerGroup {
    fn from(v: KundengruppeKA) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            KundengruppeKA::StromSchwachlast => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityOffPeak
            }
            KundengruppeKA::ElectricityTariff25000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariff25000
            }
            KundengruppeKA::ElectricityTariff100000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariff100000
            }
            KundengruppeKA::ElectricityTariff500000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariff500000
            }
            KundengruppeKA::ElectricityTariffAbove500000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricityTariffAbove500000
            }
            KundengruppeKA::StromSonderkunde => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::ElectricitySpecialCustomer
            }
            KundengruppeKA::GasCookingHotWater25000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWater25000
            }
            KundengruppeKA::GasCookingHotWater100000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWater100000
            }
            KundengruppeKA::GasCookingHotWater500000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWater500000
            }
            KundengruppeKA::GasCookingHotWaterAbove500000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasCookingHotWaterAbove500000
            }
            KundengruppeKA::GasTariff25000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariff25000
            }
            KundengruppeKA::GasTariff100000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariff100000
            }
            KundengruppeKA::GasTariff500000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariff500000
            }
            KundengruppeKA::GasTariffAbove500000 => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasTariffAbove500000
            }
            KundengruppeKA::GasSonderkunde => {
                bo4e_core::enums::ConcessionFeeCustomerGroup::GasSpecialCustomer
            }
            KundengruppeKA::SpecialKAS => bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialKAS,
            KundengruppeKA::SpecialSAS => bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialSAS,
            KundengruppeKA::SpecialTAS => bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialTAS,
            KundengruppeKA::SpecialTKS => bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialTKS,
            KundengruppeKA::SpecialTSS => bo4e_core::enums::ConcessionFeeCustomerGroup::SpecialTSS,
        }
    }
}
