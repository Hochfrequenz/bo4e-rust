#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum AufAbschlagsziel {
    #[serde(rename = "ARBEITSPREIS_EINTARIF")]
    ArbeitspreisEintarif,
    #[serde(rename = "ARBEITSPREIS_HT")]
    ArbeitspreisHt,
    #[serde(rename = "ARBEITSPREIS_NT")]
    ArbeitspreisNt,
    #[serde(rename = "ARBEITSPREIS_HT_NT")]
    ArbeitspreisHtUndNt,
    #[serde(rename = "GRUNDPREIS")]
    Grundpreis,
    #[serde(rename = "GESAMTPREIS")]
    Gesamtpreis,
}
impl From<bo4e_core::enums::SurchargeTarget> for AufAbschlagsziel {
    fn from(v: bo4e_core::enums::SurchargeTarget) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::SurchargeTarget::WorkingPriceSingleTariff => {
                AufAbschlagsziel::ArbeitspreisEintarif
            }
            bo4e_core::enums::SurchargeTarget::WorkingPriceHT => AufAbschlagsziel::ArbeitspreisHt,
            bo4e_core::enums::SurchargeTarget::WorkingPriceNT => AufAbschlagsziel::ArbeitspreisNt,
            bo4e_core::enums::SurchargeTarget::WorkingPriceHTNT => {
                AufAbschlagsziel::ArbeitspreisHtUndNt
            }
            bo4e_core::enums::SurchargeTarget::BasePrice => AufAbschlagsziel::Grundpreis,
            bo4e_core::enums::SurchargeTarget::TotalPrice => AufAbschlagsziel::Gesamtpreis,
            _ => panic!("Unknown {} variant", stringify!(SurchargeTarget)),
        }
    }
}
impl From<AufAbschlagsziel> for bo4e_core::enums::SurchargeTarget {
    fn from(v: AufAbschlagsziel) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            AufAbschlagsziel::ArbeitspreisEintarif => {
                bo4e_core::enums::SurchargeTarget::WorkingPriceSingleTariff
            }
            AufAbschlagsziel::ArbeitspreisHt => bo4e_core::enums::SurchargeTarget::WorkingPriceHT,
            AufAbschlagsziel::ArbeitspreisNt => bo4e_core::enums::SurchargeTarget::WorkingPriceNT,
            AufAbschlagsziel::ArbeitspreisHtUndNt => {
                bo4e_core::enums::SurchargeTarget::WorkingPriceHTNT
            }
            AufAbschlagsziel::Grundpreis => bo4e_core::enums::SurchargeTarget::BasePrice,
            AufAbschlagsziel::Gesamtpreis => bo4e_core::enums::SurchargeTarget::TotalPrice,
        }
    }
}
