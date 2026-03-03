#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Tarifzeit {
    #[serde(rename = "TZ_STANDARD")]
    Standard,
    #[serde(rename = "TZ_HT")]
    HTHochtarif,
    #[serde(rename = "TZ_NT")]
    NTNiedrigtarif,
}
impl From<bo4e_core::enums::TariffTime> for Tarifzeit {
    fn from(v: bo4e_core::enums::TariffTime) -> Self {
        match v {
            bo4e_core::enums::TariffTime::Standard => Tarifzeit::Standard,
            bo4e_core::enums::TariffTime::HighTariff => Tarifzeit::HTHochtarif,
            bo4e_core::enums::TariffTime::LowTariff => Tarifzeit::NTNiedrigtarif,
            _ => panic!("Unknown {} variant", stringify!(TariffTime)),
        }
    }
}
impl From<Tarifzeit> for bo4e_core::enums::TariffTime {
    fn from(v: Tarifzeit) -> Self {
        match v {
            Tarifzeit::Standard => bo4e_core::enums::TariffTime::Standard,
            Tarifzeit::HTHochtarif => bo4e_core::enums::TariffTime::HighTariff,
            Tarifzeit::NTNiedrigtarif => bo4e_core::enums::TariffTime::LowTariff,
            _ => panic!("Unknown {} variant", stringify!(Tarifzeit)),
        }
    }
}
