#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Titel {
    #[serde(rename = "DR")]
    Doktor,
    #[serde(rename = "PROF")]
    Prof,
    #[serde(rename = "PROF_DR")]
    ProfDr,
}
impl From<bo4e_core::enums::Title> for Titel {
    fn from(v: bo4e_core::enums::Title) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::Title::Dr => Titel::Doktor,
            bo4e_core::enums::Title::Prof => Titel::Prof,
            bo4e_core::enums::Title::ProfDr => Titel::ProfDr,
            _ => panic!("Unknown {} variant", stringify!(Title)),
        }
    }
}
impl From<Titel> for bo4e_core::enums::Title {
    fn from(v: Titel) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Titel::Doktor => bo4e_core::enums::Title::Dr,
            Titel::Prof => bo4e_core::enums::Title::Prof,
            Titel::ProfDr => bo4e_core::enums::Title::ProfDr,
        }
    }
}
