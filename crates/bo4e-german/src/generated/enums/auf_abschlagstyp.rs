#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum AufAbschlagstyp {
    #[serde(rename = "RELATIV")]
    Relative,
    #[serde(rename = "ABSOLUT")]
    Absolute,
}
impl From<bo4e_core::enums::SurchargeType> for AufAbschlagstyp {
    fn from(v: bo4e_core::enums::SurchargeType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::SurchargeType::Relative => AufAbschlagstyp::Relative,
            bo4e_core::enums::SurchargeType::Absolute => AufAbschlagstyp::Absolute,
            _ => panic!("Unknown {} variant", stringify!(SurchargeType)),
        }
    }
}
impl From<AufAbschlagstyp> for bo4e_core::enums::SurchargeType {
    fn from(v: AufAbschlagstyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            AufAbschlagstyp::Relative => bo4e_core::enums::SurchargeType::Relative,
            AufAbschlagstyp::Absolute => bo4e_core::enums::SurchargeType::Absolute,
        }
    }
}
