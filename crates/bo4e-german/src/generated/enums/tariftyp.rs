#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Tariftyp {
    #[serde(rename = "GRUND_ERSATZVERSORGUNG")]
    GrundUndErsatzversorgung,
    #[serde(rename = "GRUNDVERSORGUNG")]
    Grundversorgung,
    #[serde(rename = "ERSATZVERSORGUNG")]
    Ersatzversorgung,
    #[serde(rename = "SONDERTARIF")]
    Sondertarif,
}
impl From<bo4e_core::enums::TariffType> for Tariftyp {
    fn from(v: bo4e_core::enums::TariffType) -> Self {
        match v {
            bo4e_core::enums::TariffType::BasicAndBackupSupply => {
                Tariftyp::GrundUndErsatzversorgung
            }
            bo4e_core::enums::TariffType::BasicSupply => Tariftyp::Grundversorgung,
            bo4e_core::enums::TariffType::BackupSupply => Tariftyp::Ersatzversorgung,
            bo4e_core::enums::TariffType::SpecialTariff => Tariftyp::Sondertarif,
            _ => panic!("Unknown {} variant", stringify!(TariffType)),
        }
    }
}
impl From<Tariftyp> for bo4e_core::enums::TariffType {
    fn from(v: Tariftyp) -> Self {
        match v {
            Tariftyp::GrundUndErsatzversorgung => {
                bo4e_core::enums::TariffType::BasicAndBackupSupply
            }
            Tariftyp::Grundversorgung => bo4e_core::enums::TariffType::BasicSupply,
            Tariftyp::Ersatzversorgung => bo4e_core::enums::TariffType::BackupSupply,
            Tariftyp::Sondertarif => bo4e_core::enums::TariffType::SpecialTariff,
            _ => panic!("Unknown {} variant", stringify!(Tariftyp)),
        }
    }
}
