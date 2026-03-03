#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum SteuerbareRessourceTyp {
    #[serde(rename = "AN_AUS")]
    OnOff,
    #[serde(rename = "GESTUFT")]
    Gestuft,
}
impl From<bo4e_core::enums::ControllableResourceType> for SteuerbareRessourceTyp {
    fn from(v: bo4e_core::enums::ControllableResourceType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ControllableResourceType::OnOff => SteuerbareRessourceTyp::OnOff,
            bo4e_core::enums::ControllableResourceType::Graduated => {
                SteuerbareRessourceTyp::Gestuft
            }
            _ => panic!("Unknown {} variant", stringify!(ControllableResourceType)),
        }
    }
}
impl From<SteuerbareRessourceTyp> for bo4e_core::enums::ControllableResourceType {
    fn from(v: SteuerbareRessourceTyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            SteuerbareRessourceTyp::OnOff => bo4e_core::enums::ControllableResourceType::OnOff,
            SteuerbareRessourceTyp::Gestuft => {
                bo4e_core::enums::ControllableResourceType::Graduated
            }
        }
    }
}
