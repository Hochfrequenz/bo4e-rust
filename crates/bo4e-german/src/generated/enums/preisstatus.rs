#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Preisstatus {
    #[serde(rename = "VORLAEUFIG")]
    VorlUfig,
    #[serde(rename = "ENDGUELTIG")]
    EndgLtig,
}
impl From<bo4e_core::enums::PriceStatus> for Preisstatus {
    fn from(v: bo4e_core::enums::PriceStatus) -> Self {
        match v {
            bo4e_core::enums::PriceStatus::Preliminary => Preisstatus::VorlUfig,
            bo4e_core::enums::PriceStatus::Final => Preisstatus::EndgLtig,
            _ => panic!("Unknown {} variant", stringify!(PriceStatus)),
        }
    }
}
impl From<Preisstatus> for bo4e_core::enums::PriceStatus {
    fn from(v: Preisstatus) -> Self {
        match v {
            Preisstatus::VorlUfig => bo4e_core::enums::PriceStatus::Preliminary,
            Preisstatus::EndgLtig => bo4e_core::enums::PriceStatus::Final,
            _ => panic!("Unknown {} variant", stringify!(Preisstatus)),
        }
    }
}
