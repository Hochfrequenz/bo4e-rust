#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum TechnischeRessourceNutzung {
    #[serde(rename = "STROMVERBRAUCHSART")]
    Stromverbrauchsart,
    #[serde(rename = "STROMERZEUGUNGSART")]
    Stromerzeugungsart,
    #[serde(rename = "SPEICHER")]
    Speicher,
}
impl From<bo4e_core::enums::TechnicalResourceUsage> for TechnischeRessourceNutzung {
    fn from(v: bo4e_core::enums::TechnicalResourceUsage) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::TechnicalResourceUsage::ElectricityConsumptionType => {
                TechnischeRessourceNutzung::Stromverbrauchsart
            }
            bo4e_core::enums::TechnicalResourceUsage::ElectricityGenerationType => {
                TechnischeRessourceNutzung::Stromerzeugungsart
            }
            bo4e_core::enums::TechnicalResourceUsage::Storage => {
                TechnischeRessourceNutzung::Speicher
            }
            _ => panic!("Unknown {} variant", stringify!(TechnicalResourceUsage)),
        }
    }
}
impl From<TechnischeRessourceNutzung> for bo4e_core::enums::TechnicalResourceUsage {
    fn from(v: TechnischeRessourceNutzung) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            TechnischeRessourceNutzung::Stromverbrauchsart => {
                bo4e_core::enums::TechnicalResourceUsage::ElectricityConsumptionType
            }
            TechnischeRessourceNutzung::Stromerzeugungsart => {
                bo4e_core::enums::TechnicalResourceUsage::ElectricityGenerationType
            }
            TechnischeRessourceNutzung::Speicher => {
                bo4e_core::enums::TechnicalResourceUsage::Storage
            }
        }
    }
}
