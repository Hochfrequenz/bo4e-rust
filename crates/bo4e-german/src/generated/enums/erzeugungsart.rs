#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Erzeugungsart {
    #[serde(rename = "FOSSIL")]
    Fossil,
    #[serde(rename = "KWK")]
    KraftWaermeKopplung,
    #[serde(rename = "WIND")]
    Wind,
    #[serde(rename = "SOLAR")]
    Solar,
    #[serde(rename = "KERNKRAFT")]
    Nuclear,
    #[serde(rename = "WASSER")]
    Hydro,
    #[serde(rename = "GEOTHERMIE")]
    Geothermal,
    #[serde(rename = "BIOMASSE")]
    Biomass,
    #[serde(rename = "KOHLE")]
    Coal,
    #[serde(rename = "GAS")]
    Gas,
    #[serde(rename = "SONSTIGE")]
    Other,
    #[serde(rename = "SONSTIGE_EEG")]
    RenewableEnergySourcesAct,
    #[serde(rename = "BIOGAS")]
    Biogas,
    #[serde(rename = "KLIMANEUTRALES_GAS")]
    ClimateNeutralGas,
}
impl From<bo4e_core::enums::GenerationType> for Erzeugungsart {
    fn from(v: bo4e_core::enums::GenerationType) -> Self {
        match v {
            bo4e_core::enums::GenerationType::Fossil => Erzeugungsart::Fossil,
            bo4e_core::enums::GenerationType::CombinedHeatPower => {
                Erzeugungsart::KraftWaermeKopplung
            }
            bo4e_core::enums::GenerationType::Wind => Erzeugungsart::Wind,
            bo4e_core::enums::GenerationType::Solar => Erzeugungsart::Solar,
            bo4e_core::enums::GenerationType::Nuclear => Erzeugungsart::Nuclear,
            bo4e_core::enums::GenerationType::Hydro => Erzeugungsart::Hydro,
            bo4e_core::enums::GenerationType::Geothermal => Erzeugungsart::Geothermal,
            bo4e_core::enums::GenerationType::Biomass => Erzeugungsart::Biomass,
            bo4e_core::enums::GenerationType::Coal => Erzeugungsart::Coal,
            bo4e_core::enums::GenerationType::Gas => Erzeugungsart::Gas,
            bo4e_core::enums::GenerationType::Other => Erzeugungsart::Other,
            bo4e_core::enums::GenerationType::OtherEeg => {
                Erzeugungsart::RenewableEnergySourcesAct
            }
            bo4e_core::enums::GenerationType::Biogas => Erzeugungsart::Biogas,
            bo4e_core::enums::GenerationType::ClimateNeutralGas => {
                Erzeugungsart::ClimateNeutralGas
            }
            _ => panic!("Unknown {} variant", stringify!(GenerationType)),
        }
    }
}
impl From<Erzeugungsart> for bo4e_core::enums::GenerationType {
    fn from(v: Erzeugungsart) -> Self {
        match v {
            Erzeugungsart::Fossil => bo4e_core::enums::GenerationType::Fossil,
            Erzeugungsart::KraftWaermeKopplung => {
                bo4e_core::enums::GenerationType::CombinedHeatPower
            }
            Erzeugungsart::Wind => bo4e_core::enums::GenerationType::Wind,
            Erzeugungsart::Solar => bo4e_core::enums::GenerationType::Solar,
            Erzeugungsart::Nuclear => bo4e_core::enums::GenerationType::Nuclear,
            Erzeugungsart::Hydro => bo4e_core::enums::GenerationType::Hydro,
            Erzeugungsart::Geothermal => bo4e_core::enums::GenerationType::Geothermal,
            Erzeugungsart::Biomass => bo4e_core::enums::GenerationType::Biomass,
            Erzeugungsart::Coal => bo4e_core::enums::GenerationType::Coal,
            Erzeugungsart::Gas => bo4e_core::enums::GenerationType::Gas,
            Erzeugungsart::Other => bo4e_core::enums::GenerationType::Other,
            Erzeugungsart::RenewableEnergySourcesAct => {
                bo4e_core::enums::GenerationType::OtherEeg
            }
            Erzeugungsart::Biogas => bo4e_core::enums::GenerationType::Biogas,
            Erzeugungsart::ClimateNeutralGas => {
                bo4e_core::enums::GenerationType::ClimateNeutralGas
            }
            _ => panic!("Unknown {} variant", stringify!(Erzeugungsart)),
        }
    }
}
