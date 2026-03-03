#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Tarifkalkulationsmethode {
    #[serde(rename = "KEINE")]
    None,
    #[serde(rename = "STAFFELN")]
    Tiers,
    #[serde(rename = "ZONEN")]
    Zones,
    #[serde(rename = "BESTABRECHNUNG_STAFFEL")]
    BestBillingTier,
    #[serde(rename = "PAKETPREIS")]
    PriceForAQuantityPackage,
}
impl From<bo4e_core::enums::TariffCalculationMethod> for Tarifkalkulationsmethode {
    fn from(v: bo4e_core::enums::TariffCalculationMethod) -> Self {
        match v {
            bo4e_core::enums::TariffCalculationMethod::None => {
                Tarifkalkulationsmethode::None
            }
            bo4e_core::enums::TariffCalculationMethod::Tiers => {
                Tarifkalkulationsmethode::Tiers
            }
            bo4e_core::enums::TariffCalculationMethod::Zones => {
                Tarifkalkulationsmethode::Zones
            }
            bo4e_core::enums::TariffCalculationMethod::BestBillingTier => {
                Tarifkalkulationsmethode::BestBillingTier
            }
            bo4e_core::enums::TariffCalculationMethod::PackagePrice => {
                Tarifkalkulationsmethode::PriceForAQuantityPackage
            }
            _ => panic!("Unknown {} variant", stringify!(TariffCalculationMethod)),
        }
    }
}
impl From<Tarifkalkulationsmethode> for bo4e_core::enums::TariffCalculationMethod {
    fn from(v: Tarifkalkulationsmethode) -> Self {
        match v {
            Tarifkalkulationsmethode::None => {
                bo4e_core::enums::TariffCalculationMethod::None
            }
            Tarifkalkulationsmethode::Tiers => {
                bo4e_core::enums::TariffCalculationMethod::Tiers
            }
            Tarifkalkulationsmethode::Zones => {
                bo4e_core::enums::TariffCalculationMethod::Zones
            }
            Tarifkalkulationsmethode::BestBillingTier => {
                bo4e_core::enums::TariffCalculationMethod::BestBillingTier
            }
            Tarifkalkulationsmethode::PriceForAQuantityPackage => {
                bo4e_core::enums::TariffCalculationMethod::PackagePrice
            }
            _ => panic!("Unknown {} variant", stringify!(Tarifkalkulationsmethode)),
        }
    }
}
