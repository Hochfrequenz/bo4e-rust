#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Tarifmerkmal {
    #[serde(rename = "STANDARD")]
    Standardprodukt,
    #[serde(rename = "VORKASSE")]
    Vorkassenprodukt,
    #[serde(rename = "PAKET")]
    Paketpreisprodukt,
    #[serde(rename = "KOMBI")]
    Kombiprodukt,
    #[serde(rename = "FESTPREIS")]
    Festpreisprodukt,
    #[serde(rename = "BAUSTROM")]
    Baustromprodukt,
    #[serde(rename = "HAUSLICHT")]
    Hauslichtprodukt,
    #[serde(rename = "HEIZSTROM")]
    Heizstromprodukt,
    #[serde(rename = "ONLINE")]
    Onlineprodukt,
}
impl From<bo4e_core::enums::TariffFeature> for Tarifmerkmal {
    fn from(v: bo4e_core::enums::TariffFeature) -> Self {
        match v {
            bo4e_core::enums::TariffFeature::Standard => Tarifmerkmal::Standardprodukt,
            bo4e_core::enums::TariffFeature::Prepayment => Tarifmerkmal::Vorkassenprodukt,
            bo4e_core::enums::TariffFeature::Package => Tarifmerkmal::Paketpreisprodukt,
            bo4e_core::enums::TariffFeature::Combined => Tarifmerkmal::Kombiprodukt,
            bo4e_core::enums::TariffFeature::FixedPrice => Tarifmerkmal::Festpreisprodukt,
            bo4e_core::enums::TariffFeature::ConstructionPower => {
                Tarifmerkmal::Baustromprodukt
            }
            bo4e_core::enums::TariffFeature::BuildingLighting => {
                Tarifmerkmal::Hauslichtprodukt
            }
            bo4e_core::enums::TariffFeature::HeatingPower => {
                Tarifmerkmal::Heizstromprodukt
            }
            bo4e_core::enums::TariffFeature::Online => Tarifmerkmal::Onlineprodukt,
            _ => panic!("Unknown {} variant", stringify!(TariffFeature)),
        }
    }
}
impl From<Tarifmerkmal> for bo4e_core::enums::TariffFeature {
    fn from(v: Tarifmerkmal) -> Self {
        match v {
            Tarifmerkmal::Standardprodukt => bo4e_core::enums::TariffFeature::Standard,
            Tarifmerkmal::Vorkassenprodukt => bo4e_core::enums::TariffFeature::Prepayment,
            Tarifmerkmal::Paketpreisprodukt => bo4e_core::enums::TariffFeature::Package,
            Tarifmerkmal::Kombiprodukt => bo4e_core::enums::TariffFeature::Combined,
            Tarifmerkmal::Festpreisprodukt => bo4e_core::enums::TariffFeature::FixedPrice,
            Tarifmerkmal::Baustromprodukt => {
                bo4e_core::enums::TariffFeature::ConstructionPower
            }
            Tarifmerkmal::Hauslichtprodukt => {
                bo4e_core::enums::TariffFeature::BuildingLighting
            }
            Tarifmerkmal::Heizstromprodukt => {
                bo4e_core::enums::TariffFeature::HeatingPower
            }
            Tarifmerkmal::Onlineprodukt => bo4e_core::enums::TariffFeature::Online,
            _ => panic!("Unknown {} variant", stringify!(Tarifmerkmal)),
        }
    }
}
