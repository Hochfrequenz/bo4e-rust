#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum AufAbschlagsziel {
    #[serde(rename = "ARBEITSPREIS_EINTARIF")]
    ArbeitspreisEintarif,
    #[serde(rename = "ARBEITSPREIS_HT")]
    ArbeitspreisHT,
    #[serde(rename = "ARBEITSPREIS_NT")]
    ArbeitspreisNT,
    #[serde(rename = "ARBEITSPREIS_HT_NT")]
    ArbeitspreisHTUndNT,
    #[serde(rename = "GRUNDPREIS")]
    Grundpreis,
    #[serde(rename = "GESAMTPREIS")]
    Gesamtpreis,
}
impl From<bo4e_core::enums::SurchargeTarget> for AufAbschlagsziel {
    fn from(v: bo4e_core::enums::SurchargeTarget) -> Self {
        match v {
            bo4e_core::enums::SurchargeTarget::WorkingPriceSingleTariff => {
                AufAbschlagsziel::ArbeitspreisEintarif
            }
            bo4e_core::enums::SurchargeTarget::WorkingPriceHT => {
                AufAbschlagsziel::ArbeitspreisHT
            }
            bo4e_core::enums::SurchargeTarget::WorkingPriceNT => {
                AufAbschlagsziel::ArbeitspreisNT
            }
            bo4e_core::enums::SurchargeTarget::WorkingPriceHTNT => {
                AufAbschlagsziel::ArbeitspreisHTUndNT
            }
            bo4e_core::enums::SurchargeTarget::BasePrice => AufAbschlagsziel::Grundpreis,
            bo4e_core::enums::SurchargeTarget::TotalPrice => {
                AufAbschlagsziel::Gesamtpreis
            }
            _ => panic!("Unknown {} variant", stringify!(SurchargeTarget)),
        }
    }
}
impl From<AufAbschlagsziel> for bo4e_core::enums::SurchargeTarget {
    fn from(v: AufAbschlagsziel) -> Self {
        match v {
            AufAbschlagsziel::ArbeitspreisEintarif => {
                bo4e_core::enums::SurchargeTarget::WorkingPriceSingleTariff
            }
            AufAbschlagsziel::ArbeitspreisHT => {
                bo4e_core::enums::SurchargeTarget::WorkingPriceHT
            }
            AufAbschlagsziel::ArbeitspreisNT => {
                bo4e_core::enums::SurchargeTarget::WorkingPriceNT
            }
            AufAbschlagsziel::ArbeitspreisHTUndNT => {
                bo4e_core::enums::SurchargeTarget::WorkingPriceHTNT
            }
            AufAbschlagsziel::Grundpreis => bo4e_core::enums::SurchargeTarget::BasePrice,
            AufAbschlagsziel::Gesamtpreis => {
                bo4e_core::enums::SurchargeTarget::TotalPrice
            }
            _ => panic!("Unknown {} variant", stringify!(AufAbschlagsziel)),
        }
    }
}
