#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Ausschreibungsstatus {
    #[serde(rename = "PHASE1")]
    Teilnahmewettbewerb,
    #[serde(rename = "PHASE2")]
    Angebotsphase,
    #[serde(rename = "PHASE3")]
    Verhandlungsphase,
    #[serde(rename = "PHASE4")]
    Zuschlagserteilung,
}
impl From<bo4e_core::enums::TenderStatus> for Ausschreibungsstatus {
    fn from(v: bo4e_core::enums::TenderStatus) -> Self {
        match v {
            bo4e_core::enums::TenderStatus::Phase1 => {
                Ausschreibungsstatus::Teilnahmewettbewerb
            }
            bo4e_core::enums::TenderStatus::Phase2 => Ausschreibungsstatus::Angebotsphase,
            bo4e_core::enums::TenderStatus::Phase3 => {
                Ausschreibungsstatus::Verhandlungsphase
            }
            bo4e_core::enums::TenderStatus::Phase4 => {
                Ausschreibungsstatus::Zuschlagserteilung
            }
            _ => panic!("Unknown {} variant", stringify!(TenderStatus)),
        }
    }
}
impl From<Ausschreibungsstatus> for bo4e_core::enums::TenderStatus {
    fn from(v: Ausschreibungsstatus) -> Self {
        match v {
            Ausschreibungsstatus::Teilnahmewettbewerb => {
                bo4e_core::enums::TenderStatus::Phase1
            }
            Ausschreibungsstatus::Angebotsphase => bo4e_core::enums::TenderStatus::Phase2,
            Ausschreibungsstatus::Verhandlungsphase => {
                bo4e_core::enums::TenderStatus::Phase3
            }
            Ausschreibungsstatus::Zuschlagserteilung => {
                bo4e_core::enums::TenderStatus::Phase4
            }
            _ => panic!("Unknown {} variant", stringify!(Ausschreibungsstatus)),
        }
    }
}
