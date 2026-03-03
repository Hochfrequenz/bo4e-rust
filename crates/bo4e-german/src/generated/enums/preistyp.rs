#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Preistyp {
    #[serde(rename = "GRUNDPREIS")]
    Grundpreis,
    #[serde(rename = "ARBEITSPREIS_EINTARIF")]
    ArbeitspreisEintarif,
    #[serde(rename = "ARBEITSPREIS_HT")]
    ArbeitspreisHT,
    #[serde(rename = "ARBEITSPREIS_NT")]
    ArbeitspreisNT,
    #[serde(rename = "LEISTUNGSPREIS")]
    Leistungspreis,
    #[serde(rename = "MESSPREIS")]
    Messpreis,
    #[serde(rename = "ENTGELT_ABLESUNG")]
    EntgeltFRAblesung,
    #[serde(rename = "ENTGELT_ABRECHNUNG")]
    EntgeltFRAbrechnung,
    #[serde(rename = "ENTGELT_MSB")]
    EntgeltFRMSB,
    #[serde(rename = "PROVISION")]
    Provision,
}
impl From<bo4e_core::enums::PriceType> for Preistyp {
    fn from(v: bo4e_core::enums::PriceType) -> Self {
        match v {
            bo4e_core::enums::PriceType::BasePrice => Preistyp::Grundpreis,
            bo4e_core::enums::PriceType::WorkingPriceSingleTariff => {
                Preistyp::ArbeitspreisEintarif
            }
            bo4e_core::enums::PriceType::WorkingPriceHT => Preistyp::ArbeitspreisHT,
            bo4e_core::enums::PriceType::WorkingPriceNT => Preistyp::ArbeitspreisNT,
            bo4e_core::enums::PriceType::CapacityPrice => Preistyp::Leistungspreis,
            bo4e_core::enums::PriceType::MeteringPrice => Preistyp::Messpreis,
            bo4e_core::enums::PriceType::MeterReadingFee => Preistyp::EntgeltFRAblesung,
            bo4e_core::enums::PriceType::BillingFee => Preistyp::EntgeltFRAbrechnung,
            bo4e_core::enums::PriceType::MeteringServiceFee => Preistyp::EntgeltFRMSB,
            bo4e_core::enums::PriceType::Commission => Preistyp::Provision,
            _ => panic!("Unknown {} variant", stringify!(PriceType)),
        }
    }
}
impl From<Preistyp> for bo4e_core::enums::PriceType {
    fn from(v: Preistyp) -> Self {
        match v {
            Preistyp::Grundpreis => bo4e_core::enums::PriceType::BasePrice,
            Preistyp::ArbeitspreisEintarif => {
                bo4e_core::enums::PriceType::WorkingPriceSingleTariff
            }
            Preistyp::ArbeitspreisHT => bo4e_core::enums::PriceType::WorkingPriceHT,
            Preistyp::ArbeitspreisNT => bo4e_core::enums::PriceType::WorkingPriceNT,
            Preistyp::Leistungspreis => bo4e_core::enums::PriceType::CapacityPrice,
            Preistyp::Messpreis => bo4e_core::enums::PriceType::MeteringPrice,
            Preistyp::EntgeltFRAblesung => bo4e_core::enums::PriceType::MeterReadingFee,
            Preistyp::EntgeltFRAbrechnung => bo4e_core::enums::PriceType::BillingFee,
            Preistyp::EntgeltFRMSB => bo4e_core::enums::PriceType::MeteringServiceFee,
            Preistyp::Provision => bo4e_core::enums::PriceType::Commission,
            _ => panic!("Unknown {} variant", stringify!(Preistyp)),
        }
    }
}
