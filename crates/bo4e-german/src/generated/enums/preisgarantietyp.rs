#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Preisgarantietyp {
    #[serde(rename = "ALLE_PREISBESTANDTEILE_BRUTTO")]
    AllePreisbestandteileBrutto,
    #[serde(rename = "ALLE_PREISBESTANDTEILE_NETTO")]
    AllePreisbestandteileNetto,
    #[serde(rename = "PREISBESTANDTEILE_OHNE_ABGABEN")]
    PreisbestandteileOhneAbgaben,
    #[serde(rename = "NUR_ENERGIEPREIS")]
    NurEnergiepreis,
}
impl From<bo4e_core::enums::PriceGuaranteeType> for Preisgarantietyp {
    fn from(v: bo4e_core::enums::PriceGuaranteeType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::PriceGuaranteeType::AllComponentsGross => {
                Preisgarantietyp::AllePreisbestandteileBrutto
            }
            bo4e_core::enums::PriceGuaranteeType::AllComponentsNet => {
                Preisgarantietyp::AllePreisbestandteileNetto
            }
            bo4e_core::enums::PriceGuaranteeType::ComponentsWithoutFees => {
                Preisgarantietyp::PreisbestandteileOhneAbgaben
            }
            bo4e_core::enums::PriceGuaranteeType::EnergyPriceOnly => {
                Preisgarantietyp::NurEnergiepreis
            }
            _ => panic!("Unknown {} variant", stringify!(PriceGuaranteeType)),
        }
    }
}
impl From<Preisgarantietyp> for bo4e_core::enums::PriceGuaranteeType {
    fn from(v: Preisgarantietyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Preisgarantietyp::AllePreisbestandteileBrutto => {
                bo4e_core::enums::PriceGuaranteeType::AllComponentsGross
            }
            Preisgarantietyp::AllePreisbestandteileNetto => {
                bo4e_core::enums::PriceGuaranteeType::AllComponentsNet
            }
            Preisgarantietyp::PreisbestandteileOhneAbgaben => {
                bo4e_core::enums::PriceGuaranteeType::ComponentsWithoutFees
            }
            Preisgarantietyp::NurEnergiepreis => {
                bo4e_core::enums::PriceGuaranteeType::EnergyPriceOnly
            }
        }
    }
}
