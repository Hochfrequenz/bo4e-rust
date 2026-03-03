#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Registertyp {
    #[serde(rename = "EINTARIF")]
    Eintarif,
    #[serde(rename = "ZWEITARIF")]
    Zweitarif,
    #[serde(rename = "MEHRTARIF")]
    Mehrtarif,
}
impl From<bo4e_core::enums::RegisterType> for Registertyp {
    fn from(v: bo4e_core::enums::RegisterType) -> Self {
        match v {
            bo4e_core::enums::RegisterType::SingleTariff => Registertyp::Eintarif,
            bo4e_core::enums::RegisterType::DualTariff => Registertyp::Zweitarif,
            bo4e_core::enums::RegisterType::MultiTariff => Registertyp::Mehrtarif,
            _ => panic!("Unknown {} variant", stringify!(RegisterType)),
        }
    }
}
impl From<Registertyp> for bo4e_core::enums::RegisterType {
    fn from(v: Registertyp) -> Self {
        match v {
            Registertyp::Eintarif => bo4e_core::enums::RegisterType::SingleTariff,
            Registertyp::Zweitarif => bo4e_core::enums::RegisterType::DualTariff,
            Registertyp::Mehrtarif => bo4e_core::enums::RegisterType::MultiTariff,
            _ => panic!("Unknown {} variant", stringify!(Registertyp)),
        }
    }
}
