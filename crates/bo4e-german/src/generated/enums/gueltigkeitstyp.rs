#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Gueltigkeitstyp {
    #[serde(rename = "NUR_IN")]
    NurIn,
    #[serde(rename = "NICHT_IN")]
    NichtIn,
    #[serde(rename = "NUR_IN_KOMBINATION_MIT")]
    NurInKombinationMit,
}
impl From<bo4e_core::enums::ValidityType> for Gueltigkeitstyp {
    fn from(v: bo4e_core::enums::ValidityType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ValidityType::OnlyIn => Gueltigkeitstyp::NurIn,
            bo4e_core::enums::ValidityType::NotIn => Gueltigkeitstyp::NichtIn,
            bo4e_core::enums::ValidityType::OnlyInCombinationWith => {
                Gueltigkeitstyp::NurInKombinationMit
            }
            _ => panic!("Unknown {} variant", stringify!(ValidityType)),
        }
    }
}
impl From<Gueltigkeitstyp> for bo4e_core::enums::ValidityType {
    fn from(v: Gueltigkeitstyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Gueltigkeitstyp::NurIn => bo4e_core::enums::ValidityType::OnlyIn,
            Gueltigkeitstyp::NichtIn => bo4e_core::enums::ValidityType::NotIn,
            Gueltigkeitstyp::NurInKombinationMit => {
                bo4e_core::enums::ValidityType::OnlyInCombinationWith
            }
        }
    }
}
