#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Energierichtung {
    #[serde(rename = "AUSSP")]
    Ausspeisung,
    #[serde(rename = "EINSP")]
    Einspeisung,
}
impl From<bo4e_core::enums::EnergyDirection> for Energierichtung {
    fn from(v: bo4e_core::enums::EnergyDirection) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::EnergyDirection::FeedOut => Energierichtung::Ausspeisung,
            bo4e_core::enums::EnergyDirection::FeedIn => Energierichtung::Einspeisung,
            _ => panic!("Unknown {} variant", stringify!(EnergyDirection)),
        }
    }
}
impl From<Energierichtung> for bo4e_core::enums::EnergyDirection {
    fn from(v: Energierichtung) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Energierichtung::Ausspeisung => bo4e_core::enums::EnergyDirection::FeedOut,
            Energierichtung::Einspeisung => bo4e_core::enums::EnergyDirection::FeedIn,
        }
    }
}
