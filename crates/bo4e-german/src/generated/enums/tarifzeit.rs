#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Tarifzeit {
    #[serde(rename = "TZ_STANDARD")]
    Standard,
    #[serde(rename = "TZ_HT")]
    HtHochtarif,
    #[serde(rename = "TZ_NT")]
    NtNiedrigtarif,
}
impl From<bo4e_core::enums::TariffTime> for Tarifzeit {
    fn from(v: bo4e_core::enums::TariffTime) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::TariffTime::Standard => Tarifzeit::Standard,
            bo4e_core::enums::TariffTime::HighTariff => Tarifzeit::HtHochtarif,
            bo4e_core::enums::TariffTime::LowTariff => Tarifzeit::NtNiedrigtarif,
            _ => panic!("Unknown {} variant", stringify!(TariffTime)),
        }
    }
}
impl From<Tarifzeit> for bo4e_core::enums::TariffTime {
    fn from(v: Tarifzeit) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Tarifzeit::Standard => bo4e_core::enums::TariffTime::Standard,
            Tarifzeit::HtHochtarif => bo4e_core::enums::TariffTime::HighTariff,
            Tarifzeit::NtNiedrigtarif => bo4e_core::enums::TariffTime::LowTariff,
        }
    }
}
