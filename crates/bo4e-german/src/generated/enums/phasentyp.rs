#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Phasentyp {
    #[serde(rename = "EINPHASIG")]
    Einphasig,
    #[serde(rename = "ZWEIPHASIG")]
    Zweiphasig,
    #[serde(rename = "DREIPHASIG")]
    Dreiphasig,
}
impl From<bo4e_core::enums::PhaseType> for Phasentyp {
    fn from(v: bo4e_core::enums::PhaseType) -> Self {
        match v {
            bo4e_core::enums::PhaseType::SinglePhase => Phasentyp::Einphasig,
            bo4e_core::enums::PhaseType::TwoPhase => Phasentyp::Zweiphasig,
            bo4e_core::enums::PhaseType::ThreePhase => Phasentyp::Dreiphasig,
            _ => panic!("Unknown {} variant", stringify!(PhaseType)),
        }
    }
}
impl From<Phasentyp> for bo4e_core::enums::PhaseType {
    fn from(v: Phasentyp) -> Self {
        match v {
            Phasentyp::Einphasig => bo4e_core::enums::PhaseType::SinglePhase,
            Phasentyp::Zweiphasig => bo4e_core::enums::PhaseType::TwoPhase,
            Phasentyp::Dreiphasig => bo4e_core::enums::PhaseType::ThreePhase,
            _ => panic!("Unknown {} variant", stringify!(Phasentyp)),
        }
    }
}
