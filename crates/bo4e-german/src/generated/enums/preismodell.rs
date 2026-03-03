#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Preismodell {
    #[serde(rename = "FESTPREIS")]
    Festpreis,
    #[serde(rename = "TRANCHE")]
    Tranche,
}
impl From<bo4e_core::enums::PriceModel> for Preismodell {
    fn from(v: bo4e_core::enums::PriceModel) -> Self {
        match v {
            bo4e_core::enums::PriceModel::FixedPrice => Preismodell::Festpreis,
            bo4e_core::enums::PriceModel::Tranche => Preismodell::Tranche,
            _ => panic!("Unknown {} variant", stringify!(PriceModel)),
        }
    }
}
impl From<Preismodell> for bo4e_core::enums::PriceModel {
    fn from(v: Preismodell) -> Self {
        match v {
            Preismodell::Festpreis => bo4e_core::enums::PriceModel::FixedPrice,
            Preismodell::Tranche => bo4e_core::enums::PriceModel::Tranche,
            _ => panic!("Unknown {} variant", stringify!(Preismodell)),
        }
    }
}
