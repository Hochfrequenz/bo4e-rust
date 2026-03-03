#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Energierichtung {
    #[serde(rename = "AUSSP")]
    Ausspeisung,
    #[serde(rename = "EINSP")]
    Einspeisung,
}
impl From<bo4e_core::enums::EnergyDirection> for Energierichtung {
    fn from(v: bo4e_core::enums::EnergyDirection) -> Self {
        match v {
            bo4e_core::enums::EnergyDirection::FeedOut => Energierichtung::Ausspeisung,
            bo4e_core::enums::EnergyDirection::FeedIn => Energierichtung::Einspeisung,
            _ => panic!("Unknown {} variant", stringify!(EnergyDirection)),
        }
    }
}
impl From<Energierichtung> for bo4e_core::enums::EnergyDirection {
    fn from(v: Energierichtung) -> Self {
        match v {
            Energierichtung::Ausspeisung => bo4e_core::enums::EnergyDirection::FeedOut,
            Energierichtung::Einspeisung => bo4e_core::enums::EnergyDirection::FeedIn,
            _ => panic!("Unknown {} variant", stringify!(Energierichtung)),
        }
    }
}
