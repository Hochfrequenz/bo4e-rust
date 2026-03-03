#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Marktrolle {
    #[serde(rename = "BTR")]
    BetreiberEinerTechnischenRessource,
    #[serde(rename = "BIKO")]
    BalanceCoordinator,
    #[serde(rename = "BKV")]
    Bilanzkreisverantwortlicher,
    #[serde(rename = "DP")]
    DataProvider,
    #[serde(rename = "EIV")]
    Einsatzverantwortlicher,
    #[serde(rename = "ESA")]
    EnergieserviceanbieterDesAnschlussnutzers,
    #[serde(rename = "KN")]
    Kapazitaetsnutzer,
    #[serde(rename = "LF")]
    Lieferant,
    #[serde(rename = "MGV")]
    Marktgebietsverantwortlicher,
    #[serde(rename = "MSB")]
    Messstellenbetreiber,
    #[serde(rename = "NB")]
    Netzbetreiber,
    #[serde(rename = "RB")]
    Registerbetreiber,
    #[serde(rename = "UENB")]
    Uebertragungsnetzbetreiber,
}
impl From<bo4e_core::enums::MarketRole> for Marktrolle {
    fn from(v: bo4e_core::enums::MarketRole) -> Self {
        match v {
            bo4e_core::enums::MarketRole::TechnicalResourceOperator => {
                Marktrolle::BetreiberEinerTechnischenRessource
            }
            bo4e_core::enums::MarketRole::BalanceCoordinator => {
                Marktrolle::BalanceCoordinator
            }
            bo4e_core::enums::MarketRole::BalanceResponsibleParty => {
                Marktrolle::Bilanzkreisverantwortlicher
            }
            bo4e_core::enums::MarketRole::DataProvider => Marktrolle::DataProvider,
            bo4e_core::enums::MarketRole::DeploymentResponsible => {
                Marktrolle::Einsatzverantwortlicher
            }
            bo4e_core::enums::MarketRole::EnergyServiceProvider => {
                Marktrolle::EnergieserviceanbieterDesAnschlussnutzers
            }
            bo4e_core::enums::MarketRole::CapacityUser => Marktrolle::Kapazitaetsnutzer,
            bo4e_core::enums::MarketRole::Supplier => Marktrolle::Lieferant,
            bo4e_core::enums::MarketRole::MarketAreaManager => {
                Marktrolle::Marktgebietsverantwortlicher
            }
            bo4e_core::enums::MarketRole::MeteringPointOperator => {
                Marktrolle::Messstellenbetreiber
            }
            bo4e_core::enums::MarketRole::NetworkOperator => Marktrolle::Netzbetreiber,
            bo4e_core::enums::MarketRole::RegisterOperator => {
                Marktrolle::Registerbetreiber
            }
            bo4e_core::enums::MarketRole::TransmissionSystemOperator => {
                Marktrolle::Uebertragungsnetzbetreiber
            }
            _ => panic!("Unknown {} variant", stringify!(MarketRole)),
        }
    }
}
impl From<Marktrolle> for bo4e_core::enums::MarketRole {
    fn from(v: Marktrolle) -> Self {
        match v {
            Marktrolle::BetreiberEinerTechnischenRessource => {
                bo4e_core::enums::MarketRole::TechnicalResourceOperator
            }
            Marktrolle::BalanceCoordinator => {
                bo4e_core::enums::MarketRole::BalanceCoordinator
            }
            Marktrolle::Bilanzkreisverantwortlicher => {
                bo4e_core::enums::MarketRole::BalanceResponsibleParty
            }
            Marktrolle::DataProvider => bo4e_core::enums::MarketRole::DataProvider,
            Marktrolle::Einsatzverantwortlicher => {
                bo4e_core::enums::MarketRole::DeploymentResponsible
            }
            Marktrolle::EnergieserviceanbieterDesAnschlussnutzers => {
                bo4e_core::enums::MarketRole::EnergyServiceProvider
            }
            Marktrolle::Kapazitaetsnutzer => bo4e_core::enums::MarketRole::CapacityUser,
            Marktrolle::Lieferant => bo4e_core::enums::MarketRole::Supplier,
            Marktrolle::Marktgebietsverantwortlicher => {
                bo4e_core::enums::MarketRole::MarketAreaManager
            }
            Marktrolle::Messstellenbetreiber => {
                bo4e_core::enums::MarketRole::MeteringPointOperator
            }
            Marktrolle::Netzbetreiber => bo4e_core::enums::MarketRole::NetworkOperator,
            Marktrolle::Registerbetreiber => {
                bo4e_core::enums::MarketRole::RegisterOperator
            }
            Marktrolle::Uebertragungsnetzbetreiber => {
                bo4e_core::enums::MarketRole::TransmissionSystemOperator
            }
            _ => panic!("Unknown {} variant", stringify!(Marktrolle)),
        }
    }
}
