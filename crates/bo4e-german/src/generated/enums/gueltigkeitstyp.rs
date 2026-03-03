#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Gueltigkeitstyp {
    #[serde(rename = "NUR_IN")]
    NUR_IN,
    #[serde(rename = "NICHT_IN")]
    NICHT_IN,
    #[serde(rename = "NUR_IN_KOMBINATION_MIT")]
    NUR_IN_KOMBINATION_MIT,
}
impl From<bo4e_core::enums::ValidityType> for Gueltigkeitstyp {
    fn from(v: bo4e_core::enums::ValidityType) -> Self {
        match v {
            bo4e_core::enums::ValidityType::OnlyIn => Gueltigkeitstyp::NUR_IN,
            bo4e_core::enums::ValidityType::NotIn => Gueltigkeitstyp::NICHT_IN,
            bo4e_core::enums::ValidityType::OnlyInCombinationWith => {
                Gueltigkeitstyp::NUR_IN_KOMBINATION_MIT
            }
            _ => panic!("Unknown {} variant", stringify!(ValidityType)),
        }
    }
}
impl From<Gueltigkeitstyp> for bo4e_core::enums::ValidityType {
    fn from(v: Gueltigkeitstyp) -> Self {
        match v {
            Gueltigkeitstyp::NUR_IN => bo4e_core::enums::ValidityType::OnlyIn,
            Gueltigkeitstyp::NICHT_IN => bo4e_core::enums::ValidityType::NotIn,
            Gueltigkeitstyp::NUR_IN_KOMBINATION_MIT => {
                bo4e_core::enums::ValidityType::OnlyInCombinationWith
            }
            _ => panic!("Unknown {} variant", stringify!(Gueltigkeitstyp)),
        }
    }
}
