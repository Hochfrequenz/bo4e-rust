#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum AufAbschlagstyp {
    #[serde(rename = "RELATIV")]
    Relative,
    #[serde(rename = "ABSOLUT")]
    Absolute,
}
impl From<bo4e_core::enums::SurchargeType> for AufAbschlagstyp {
    fn from(v: bo4e_core::enums::SurchargeType) -> Self {
        match v {
            bo4e_core::enums::SurchargeType::Relative => AufAbschlagstyp::Relative,
            bo4e_core::enums::SurchargeType::Absolute => AufAbschlagstyp::Absolute,
            _ => panic!("Unknown {} variant", stringify!(SurchargeType)),
        }
    }
}
impl From<AufAbschlagstyp> for bo4e_core::enums::SurchargeType {
    fn from(v: AufAbschlagstyp) -> Self {
        match v {
            AufAbschlagstyp::Relative => bo4e_core::enums::SurchargeType::Relative,
            AufAbschlagstyp::Absolute => bo4e_core::enums::SurchargeType::Absolute,
            _ => panic!("Unknown {} variant", stringify!(AufAbschlagstyp)),
        }
    }
}
