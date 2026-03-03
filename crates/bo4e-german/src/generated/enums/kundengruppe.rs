#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Kundengruppe {
    #[serde(rename = "RLM")]
    NoSlp,
    #[serde(rename = "RLM_KOMMUNAL")]
    RlmMunicipal,
    #[serde(rename = "SLP_KOMMUNAL")]
    SlpMunicipal,
    #[serde(rename = "SLP_S_G0")]
    G0,
    #[serde(rename = "SLP_S_G1")]
    G1,
    #[serde(rename = "SLP_S_G2")]
    G2,
    #[serde(rename = "SLP_S_G3")]
    G3,
    #[serde(rename = "SLP_S_G4")]
    G4,
    #[serde(rename = "SLP_S_G5")]
    G5,
    #[serde(rename = "SLP_S_G6")]
    G6,
    #[serde(rename = "SLP_S_G7")]
    G7,
    #[serde(rename = "SLP_S_L0")]
    L0,
    #[serde(rename = "SLP_S_L1")]
    L1,
    #[serde(rename = "SLP_S_L2")]
    L2,
    #[serde(rename = "SLP_S_H0")]
    H0,
    #[serde(rename = "SLP_S_SB")]
    SB,
    #[serde(rename = "SLP_S_HZ")]
    HZ,
    #[serde(rename = "SLP_S_WP")]
    WP,
    #[serde(rename = "SLP_S_EM")]
    EM,
    #[serde(rename = "SLP_S_HZ_GEM")]
    HzGem,
    #[serde(rename = "SLP_G_GKO")]
    SlpGasGko,
    #[serde(rename = "SLP_G_STANDARD")]
    SlpGasStandard,
    #[serde(rename = "SLP_G_GHA")]
    SlpGasGha,
    #[serde(rename = "SLP_G_GMK")]
    SlpGasGmk,
    #[serde(rename = "SLP_G_GBD")]
    SlpGasGbd,
    #[serde(rename = "SLP_G_GGA")]
    SlpGasGga,
    #[serde(rename = "SLP_G_GBH")]
    SlpGasGbh,
    #[serde(rename = "SLP_G_GBA")]
    SlpGasGba,
    #[serde(rename = "SLP_G_GWA")]
    SlpGasGwa,
    #[serde(rename = "SLP_G_GGB")]
    SlpGasGgb,
    #[serde(rename = "SLP_G_GPD")]
    SlpGasGpd,
    #[serde(rename = "SLP_G_GMF")]
    SlpGasGmf,
    #[serde(rename = "SLP_G_HEF")]
    SlpGasHef,
    #[serde(rename = "SLP_G_HMF")]
    SlpGasHmf,
    #[serde(rename = "SLP_G_HKO")]
    SlpGasHko,
}
impl From<bo4e_core::enums::CustomerGroup> for Kundengruppe {
    fn from(v: bo4e_core::enums::CustomerGroup) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::CustomerGroup::Rlm => Kundengruppe::NoSlp,
            bo4e_core::enums::CustomerGroup::RlmMunicipal => Kundengruppe::RlmMunicipal,
            bo4e_core::enums::CustomerGroup::SlpMunicipal => Kundengruppe::SlpMunicipal,
            bo4e_core::enums::CustomerGroup::SlpElectricityG0 => Kundengruppe::G0,
            bo4e_core::enums::CustomerGroup::SlpElectricityG1 => Kundengruppe::G1,
            bo4e_core::enums::CustomerGroup::SlpElectricityG2 => Kundengruppe::G2,
            bo4e_core::enums::CustomerGroup::SlpElectricityG3 => Kundengruppe::G3,
            bo4e_core::enums::CustomerGroup::SlpElectricityG4 => Kundengruppe::G4,
            bo4e_core::enums::CustomerGroup::SlpElectricityG5 => Kundengruppe::G5,
            bo4e_core::enums::CustomerGroup::SlpElectricityG6 => Kundengruppe::G6,
            bo4e_core::enums::CustomerGroup::SlpElectricityG7 => Kundengruppe::G7,
            bo4e_core::enums::CustomerGroup::SlpElectricityL0 => Kundengruppe::L0,
            bo4e_core::enums::CustomerGroup::SlpElectricityL1 => Kundengruppe::L1,
            bo4e_core::enums::CustomerGroup::SlpElectricityL2 => Kundengruppe::L2,
            bo4e_core::enums::CustomerGroup::SlpElectricityH0 => Kundengruppe::H0,
            bo4e_core::enums::CustomerGroup::SlpElectricitySb => Kundengruppe::SB,
            bo4e_core::enums::CustomerGroup::SlpElectricityHz => Kundengruppe::HZ,
            bo4e_core::enums::CustomerGroup::SlpElectricityWp => Kundengruppe::WP,
            bo4e_core::enums::CustomerGroup::SlpElectricityEm => Kundengruppe::EM,
            bo4e_core::enums::CustomerGroup::SlpElectricityHzGem => Kundengruppe::HzGem,
            bo4e_core::enums::CustomerGroup::SlpGasGko => Kundengruppe::SlpGasGko,
            bo4e_core::enums::CustomerGroup::SlpGasStandard => Kundengruppe::SlpGasStandard,
            bo4e_core::enums::CustomerGroup::SlpGasGha => Kundengruppe::SlpGasGha,
            bo4e_core::enums::CustomerGroup::SlpGasGmk => Kundengruppe::SlpGasGmk,
            bo4e_core::enums::CustomerGroup::SlpGasGbd => Kundengruppe::SlpGasGbd,
            bo4e_core::enums::CustomerGroup::SlpGasGga => Kundengruppe::SlpGasGga,
            bo4e_core::enums::CustomerGroup::SlpGasGbh => Kundengruppe::SlpGasGbh,
            bo4e_core::enums::CustomerGroup::SlpGasGba => Kundengruppe::SlpGasGba,
            bo4e_core::enums::CustomerGroup::SlpGasGwa => Kundengruppe::SlpGasGwa,
            bo4e_core::enums::CustomerGroup::SlpGasGgb => Kundengruppe::SlpGasGgb,
            bo4e_core::enums::CustomerGroup::SlpGasGpd => Kundengruppe::SlpGasGpd,
            bo4e_core::enums::CustomerGroup::SlpGasGmf => Kundengruppe::SlpGasGmf,
            bo4e_core::enums::CustomerGroup::SlpGasHef => Kundengruppe::SlpGasHef,
            bo4e_core::enums::CustomerGroup::SlpGasHmf => Kundengruppe::SlpGasHmf,
            bo4e_core::enums::CustomerGroup::SlpGasHko => Kundengruppe::SlpGasHko,
            _ => panic!("Unknown {} variant", stringify!(CustomerGroup)),
        }
    }
}
impl From<Kundengruppe> for bo4e_core::enums::CustomerGroup {
    fn from(v: Kundengruppe) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Kundengruppe::NoSlp => bo4e_core::enums::CustomerGroup::Rlm,
            Kundengruppe::RlmMunicipal => bo4e_core::enums::CustomerGroup::RlmMunicipal,
            Kundengruppe::SlpMunicipal => bo4e_core::enums::CustomerGroup::SlpMunicipal,
            Kundengruppe::G0 => bo4e_core::enums::CustomerGroup::SlpElectricityG0,
            Kundengruppe::G1 => bo4e_core::enums::CustomerGroup::SlpElectricityG1,
            Kundengruppe::G2 => bo4e_core::enums::CustomerGroup::SlpElectricityG2,
            Kundengruppe::G3 => bo4e_core::enums::CustomerGroup::SlpElectricityG3,
            Kundengruppe::G4 => bo4e_core::enums::CustomerGroup::SlpElectricityG4,
            Kundengruppe::G5 => bo4e_core::enums::CustomerGroup::SlpElectricityG5,
            Kundengruppe::G6 => bo4e_core::enums::CustomerGroup::SlpElectricityG6,
            Kundengruppe::G7 => bo4e_core::enums::CustomerGroup::SlpElectricityG7,
            Kundengruppe::L0 => bo4e_core::enums::CustomerGroup::SlpElectricityL0,
            Kundengruppe::L1 => bo4e_core::enums::CustomerGroup::SlpElectricityL1,
            Kundengruppe::L2 => bo4e_core::enums::CustomerGroup::SlpElectricityL2,
            Kundengruppe::H0 => bo4e_core::enums::CustomerGroup::SlpElectricityH0,
            Kundengruppe::SB => bo4e_core::enums::CustomerGroup::SlpElectricitySb,
            Kundengruppe::HZ => bo4e_core::enums::CustomerGroup::SlpElectricityHz,
            Kundengruppe::WP => bo4e_core::enums::CustomerGroup::SlpElectricityWp,
            Kundengruppe::EM => bo4e_core::enums::CustomerGroup::SlpElectricityEm,
            Kundengruppe::HzGem => bo4e_core::enums::CustomerGroup::SlpElectricityHzGem,
            Kundengruppe::SlpGasGko => bo4e_core::enums::CustomerGroup::SlpGasGko,
            Kundengruppe::SlpGasStandard => bo4e_core::enums::CustomerGroup::SlpGasStandard,
            Kundengruppe::SlpGasGha => bo4e_core::enums::CustomerGroup::SlpGasGha,
            Kundengruppe::SlpGasGmk => bo4e_core::enums::CustomerGroup::SlpGasGmk,
            Kundengruppe::SlpGasGbd => bo4e_core::enums::CustomerGroup::SlpGasGbd,
            Kundengruppe::SlpGasGga => bo4e_core::enums::CustomerGroup::SlpGasGga,
            Kundengruppe::SlpGasGbh => bo4e_core::enums::CustomerGroup::SlpGasGbh,
            Kundengruppe::SlpGasGba => bo4e_core::enums::CustomerGroup::SlpGasGba,
            Kundengruppe::SlpGasGwa => bo4e_core::enums::CustomerGroup::SlpGasGwa,
            Kundengruppe::SlpGasGgb => bo4e_core::enums::CustomerGroup::SlpGasGgb,
            Kundengruppe::SlpGasGpd => bo4e_core::enums::CustomerGroup::SlpGasGpd,
            Kundengruppe::SlpGasGmf => bo4e_core::enums::CustomerGroup::SlpGasGmf,
            Kundengruppe::SlpGasHef => bo4e_core::enums::CustomerGroup::SlpGasHef,
            Kundengruppe::SlpGasHmf => bo4e_core::enums::CustomerGroup::SlpGasHmf,
            Kundengruppe::SlpGasHko => bo4e_core::enums::CustomerGroup::SlpGasHko,
        }
    }
}
