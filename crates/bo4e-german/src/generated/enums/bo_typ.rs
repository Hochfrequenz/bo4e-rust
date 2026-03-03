#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum BoTyp {
    #[serde(rename = "Angebot")]
    Offer,
    #[serde(rename = "Ausschreibung")]
    Tender,
    #[serde(rename = "Buendelvertrag")]
    BundleContract,
    #[serde(rename = "Vertrag")]
    Contract,
    #[serde(rename = "Lokationszuordnung")]
    LocationAssignment,
    #[serde(rename = "Marktlokation")]
    MarketLocation,
    #[serde(rename = "Messlokation")]
    MeteringLocation,
    #[serde(rename = "Netzlokation")]
    NetworkLocation,
    #[serde(rename = "Geschaeftspartner")]
    BusinessPartner,
    #[serde(rename = "Marktteilnehmer")]
    MarketParticipant,
    #[serde(rename = "Person")]
    Person,
    #[serde(rename = "Fremdkosten")]
    ExternalCosts,
    #[serde(rename = "Kosten")]
    Costs,
    #[serde(rename = "Rechnung")]
    Invoice,
    #[serde(rename = "Tarifkosten")]
    TariffCosts,
    #[serde(rename = "Preisblatt")]
    PriceSheet,
    #[serde(rename = "PreisblattDienstleistung")]
    ServicePriceSheet,
    #[serde(rename = "PreisblattHardware")]
    HardwarePriceSheet,
    #[serde(rename = "PreisblattKonzessionsabgabe")]
    ConcessionFeePriceSheet,
    #[serde(rename = "PreisblattMessung")]
    MeteringPriceSheet,
    #[serde(rename = "PreisblattNetznutzung")]
    NetworkUsagePriceSheet,
    #[serde(rename = "Tarif")]
    Tariff,
    #[serde(rename = "Tarifinfo")]
    TariffInfo,
    #[serde(rename = "Tarifpreisblatt")]
    TariffPriceSheet,
    #[serde(rename = "Energiemenge")]
    EnergyAmount,
    #[serde(rename = "Geraet")]
    Device,
    #[serde(rename = "Lastgang")]
    LoadProfile,
    #[serde(rename = "SteuerbareRessource")]
    ControllableResource,
    #[serde(rename = "TechnischeRessource")]
    TechnicalResource,
    #[serde(rename = "Zaehler")]
    Meter,
    #[serde(rename = "Zeitreihe")]
    TimeSeries,
    #[serde(rename = "Bilanzierung")]
    Balancing,
    #[serde(rename = "Region")]
    Region,
    #[serde(rename = "Regionaltarif")]
    RegionalTariff,
    #[serde(rename = "Standorteigenschaften")]
    LocationProperties,
}
impl From<bo4e_core::enums::BoType> for BoTyp {
    fn from(v: bo4e_core::enums::BoType) -> Self {
        match v {
            bo4e_core::enums::BoType::Offer => BoTyp::Offer,
            bo4e_core::enums::BoType::Tender => BoTyp::Tender,
            bo4e_core::enums::BoType::BundleContract => BoTyp::BundleContract,
            bo4e_core::enums::BoType::Contract => BoTyp::Contract,
            bo4e_core::enums::BoType::LocationAssignment => BoTyp::LocationAssignment,
            bo4e_core::enums::BoType::MarketLocation => BoTyp::MarketLocation,
            bo4e_core::enums::BoType::MeteringLocation => BoTyp::MeteringLocation,
            bo4e_core::enums::BoType::NetworkLocation => BoTyp::NetworkLocation,
            bo4e_core::enums::BoType::BusinessPartner => BoTyp::BusinessPartner,
            bo4e_core::enums::BoType::MarketParticipant => BoTyp::MarketParticipant,
            bo4e_core::enums::BoType::Person => BoTyp::Person,
            bo4e_core::enums::BoType::ExternalCosts => BoTyp::ExternalCosts,
            bo4e_core::enums::BoType::Costs => BoTyp::Costs,
            bo4e_core::enums::BoType::Invoice => BoTyp::Invoice,
            bo4e_core::enums::BoType::TariffCosts => BoTyp::TariffCosts,
            bo4e_core::enums::BoType::PriceSheet => BoTyp::PriceSheet,
            bo4e_core::enums::BoType::ServicePriceSheet => BoTyp::ServicePriceSheet,
            bo4e_core::enums::BoType::HardwarePriceSheet => BoTyp::HardwarePriceSheet,
            bo4e_core::enums::BoType::ConcessionFeePriceSheet => {
                BoTyp::ConcessionFeePriceSheet
            }
            bo4e_core::enums::BoType::MeteringPriceSheet => BoTyp::MeteringPriceSheet,
            bo4e_core::enums::BoType::NetworkUsagePriceSheet => {
                BoTyp::NetworkUsagePriceSheet
            }
            bo4e_core::enums::BoType::Tariff => BoTyp::Tariff,
            bo4e_core::enums::BoType::TariffInfo => BoTyp::TariffInfo,
            bo4e_core::enums::BoType::TariffPriceSheet => BoTyp::TariffPriceSheet,
            bo4e_core::enums::BoType::EnergyAmount => BoTyp::EnergyAmount,
            bo4e_core::enums::BoType::Device => BoTyp::Device,
            bo4e_core::enums::BoType::LoadProfile => BoTyp::LoadProfile,
            bo4e_core::enums::BoType::ControllableResource => BoTyp::ControllableResource,
            bo4e_core::enums::BoType::TechnicalResource => BoTyp::TechnicalResource,
            bo4e_core::enums::BoType::Meter => BoTyp::Meter,
            bo4e_core::enums::BoType::TimeSeries => BoTyp::TimeSeries,
            bo4e_core::enums::BoType::Balancing => BoTyp::Balancing,
            bo4e_core::enums::BoType::Region => BoTyp::Region,
            bo4e_core::enums::BoType::RegionalTariff => BoTyp::RegionalTariff,
            bo4e_core::enums::BoType::LocationProperties => BoTyp::LocationProperties,
            _ => panic!("Unknown {} variant", stringify!(BoType)),
        }
    }
}
impl From<BoTyp> for bo4e_core::enums::BoType {
    fn from(v: BoTyp) -> Self {
        match v {
            BoTyp::Offer => bo4e_core::enums::BoType::Offer,
            BoTyp::Tender => bo4e_core::enums::BoType::Tender,
            BoTyp::BundleContract => bo4e_core::enums::BoType::BundleContract,
            BoTyp::Contract => bo4e_core::enums::BoType::Contract,
            BoTyp::LocationAssignment => bo4e_core::enums::BoType::LocationAssignment,
            BoTyp::MarketLocation => bo4e_core::enums::BoType::MarketLocation,
            BoTyp::MeteringLocation => bo4e_core::enums::BoType::MeteringLocation,
            BoTyp::NetworkLocation => bo4e_core::enums::BoType::NetworkLocation,
            BoTyp::BusinessPartner => bo4e_core::enums::BoType::BusinessPartner,
            BoTyp::MarketParticipant => bo4e_core::enums::BoType::MarketParticipant,
            BoTyp::Person => bo4e_core::enums::BoType::Person,
            BoTyp::ExternalCosts => bo4e_core::enums::BoType::ExternalCosts,
            BoTyp::Costs => bo4e_core::enums::BoType::Costs,
            BoTyp::Invoice => bo4e_core::enums::BoType::Invoice,
            BoTyp::TariffCosts => bo4e_core::enums::BoType::TariffCosts,
            BoTyp::PriceSheet => bo4e_core::enums::BoType::PriceSheet,
            BoTyp::ServicePriceSheet => bo4e_core::enums::BoType::ServicePriceSheet,
            BoTyp::HardwarePriceSheet => bo4e_core::enums::BoType::HardwarePriceSheet,
            BoTyp::ConcessionFeePriceSheet => {
                bo4e_core::enums::BoType::ConcessionFeePriceSheet
            }
            BoTyp::MeteringPriceSheet => bo4e_core::enums::BoType::MeteringPriceSheet,
            BoTyp::NetworkUsagePriceSheet => {
                bo4e_core::enums::BoType::NetworkUsagePriceSheet
            }
            BoTyp::Tariff => bo4e_core::enums::BoType::Tariff,
            BoTyp::TariffInfo => bo4e_core::enums::BoType::TariffInfo,
            BoTyp::TariffPriceSheet => bo4e_core::enums::BoType::TariffPriceSheet,
            BoTyp::EnergyAmount => bo4e_core::enums::BoType::EnergyAmount,
            BoTyp::Device => bo4e_core::enums::BoType::Device,
            BoTyp::LoadProfile => bo4e_core::enums::BoType::LoadProfile,
            BoTyp::ControllableResource => bo4e_core::enums::BoType::ControllableResource,
            BoTyp::TechnicalResource => bo4e_core::enums::BoType::TechnicalResource,
            BoTyp::Meter => bo4e_core::enums::BoType::Meter,
            BoTyp::TimeSeries => bo4e_core::enums::BoType::TimeSeries,
            BoTyp::Balancing => bo4e_core::enums::BoType::Balancing,
            BoTyp::Region => bo4e_core::enums::BoType::Region,
            BoTyp::RegionalTariff => bo4e_core::enums::BoType::RegionalTariff,
            BoTyp::LocationProperties => bo4e_core::enums::BoType::LocationProperties,
            _ => panic!("Unknown {} variant", stringify!(BoTyp)),
        }
    }
}
