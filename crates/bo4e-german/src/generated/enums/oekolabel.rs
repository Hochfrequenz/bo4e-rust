#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Oekolabel {
    #[serde(rename = "ENERGREEN")]
    Energreen,
    #[serde(rename = "GASGREEN_GRUENER_STROM")]
    GasgreenGruenerStrom,
    #[serde(rename = "GASGREEN")]
    Gasgreen,
    #[serde(rename = "GRUENER_STROM_GOLD")]
    GruenerStromGold,
    #[serde(rename = "GRUENER_STROM_SILBER")]
    GruenerStromSilber,
    #[serde(rename = "GRUENER_STROM")]
    GruenerStrom,
    #[serde(rename = "GRUENES_GAS")]
    GruenesGas,
    #[serde(rename = "NATURWATT_STROM")]
    NaturwattStrom,
    #[serde(rename = "OK_POWER")]
    OkPower,
    #[serde(rename = "RENEWABLE_PLUS")]
    RenewablePlus,
    #[serde(rename = "WATERGREEN")]
    Watergreen,
    #[serde(rename = "WATERGREEN_PLUS")]
    WatergreenPlus,
}
impl From<bo4e_core::enums::EcoLabel> for Oekolabel {
    fn from(v: bo4e_core::enums::EcoLabel) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::EcoLabel::Energreen => Oekolabel::Energreen,
            bo4e_core::enums::EcoLabel::GasgreenGruenerStrom => Oekolabel::GasgreenGruenerStrom,
            bo4e_core::enums::EcoLabel::Gasgreen => Oekolabel::Gasgreen,
            bo4e_core::enums::EcoLabel::GruenerStromGold => Oekolabel::GruenerStromGold,
            bo4e_core::enums::EcoLabel::GruenerStromSilber => Oekolabel::GruenerStromSilber,
            bo4e_core::enums::EcoLabel::GruenerStrom => Oekolabel::GruenerStrom,
            bo4e_core::enums::EcoLabel::GruenesGas => Oekolabel::GruenesGas,
            bo4e_core::enums::EcoLabel::NaturwattStrom => Oekolabel::NaturwattStrom,
            bo4e_core::enums::EcoLabel::OkPower => Oekolabel::OkPower,
            bo4e_core::enums::EcoLabel::RenewablePlus => Oekolabel::RenewablePlus,
            bo4e_core::enums::EcoLabel::Watergreen => Oekolabel::Watergreen,
            bo4e_core::enums::EcoLabel::WatergreenPlus => Oekolabel::WatergreenPlus,
            _ => panic!("Unknown {} variant", stringify!(EcoLabel)),
        }
    }
}
impl From<Oekolabel> for bo4e_core::enums::EcoLabel {
    fn from(v: Oekolabel) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Oekolabel::Energreen => bo4e_core::enums::EcoLabel::Energreen,
            Oekolabel::GasgreenGruenerStrom => bo4e_core::enums::EcoLabel::GasgreenGruenerStrom,
            Oekolabel::Gasgreen => bo4e_core::enums::EcoLabel::Gasgreen,
            Oekolabel::GruenerStromGold => bo4e_core::enums::EcoLabel::GruenerStromGold,
            Oekolabel::GruenerStromSilber => bo4e_core::enums::EcoLabel::GruenerStromSilber,
            Oekolabel::GruenerStrom => bo4e_core::enums::EcoLabel::GruenerStrom,
            Oekolabel::GruenesGas => bo4e_core::enums::EcoLabel::GruenesGas,
            Oekolabel::NaturwattStrom => bo4e_core::enums::EcoLabel::NaturwattStrom,
            Oekolabel::OkPower => bo4e_core::enums::EcoLabel::OkPower,
            Oekolabel::RenewablePlus => bo4e_core::enums::EcoLabel::RenewablePlus,
            Oekolabel::Watergreen => bo4e_core::enums::EcoLabel::Watergreen,
            Oekolabel::WatergreenPlus => bo4e_core::enums::EcoLabel::WatergreenPlus,
        }
    }
}
