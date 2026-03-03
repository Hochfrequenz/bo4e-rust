#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Tarifregionskriterium {
    #[serde(rename = "NETZ_NUMMER")]
    Netznummer,
    #[serde(rename = "POSTLEITZAHL")]
    Postleitzahl,
    #[serde(rename = "ORT")]
    Ort,
    #[serde(rename = "GRUNDVERSORGER_NUMMER")]
    Grundversorgernummer,
    #[serde(rename = "REGION")]
    URL,
}
impl From<bo4e_core::enums::TariffRegionCriterion> for Tarifregionskriterium {
    fn from(v: bo4e_core::enums::TariffRegionCriterion) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::TariffRegionCriterion::NetworkNumber => {
                Tarifregionskriterium::Netznummer
            }
            bo4e_core::enums::TariffRegionCriterion::PostalCode => {
                Tarifregionskriterium::Postleitzahl
            }
            bo4e_core::enums::TariffRegionCriterion::City => Tarifregionskriterium::Ort,
            bo4e_core::enums::TariffRegionCriterion::BasicSupplierNumber => {
                Tarifregionskriterium::Grundversorgernummer
            }
            bo4e_core::enums::TariffRegionCriterion::Region => Tarifregionskriterium::URL,
            _ => panic!("Unknown {} variant", stringify!(TariffRegionCriterion)),
        }
    }
}
impl From<Tarifregionskriterium> for bo4e_core::enums::TariffRegionCriterion {
    fn from(v: Tarifregionskriterium) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Tarifregionskriterium::Netznummer => {
                bo4e_core::enums::TariffRegionCriterion::NetworkNumber
            }
            Tarifregionskriterium::Postleitzahl => {
                bo4e_core::enums::TariffRegionCriterion::PostalCode
            }
            Tarifregionskriterium::Ort => bo4e_core::enums::TariffRegionCriterion::City,
            Tarifregionskriterium::Grundversorgernummer => {
                bo4e_core::enums::TariffRegionCriterion::BasicSupplierNumber
            }
            Tarifregionskriterium::URL => bo4e_core::enums::TariffRegionCriterion::Region,
        }
    }
}
