//! Business Object type discriminator.

use serde::{Deserialize, Serialize};

/// Type discriminator for Business Objects.
///
/// Used in the `_typ` field to identify the concrete type of a BO4E object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "BoTyp"))]
#[non_exhaustive]
pub enum BoType {
    // Contracts
    #[serde(rename = "Angebot")]
    Offer,
    #[serde(rename = "Ausschreibung")]
    Tender,
    #[serde(rename = "Buendelvertrag")]
    BundleContract,
    #[serde(rename = "Vertrag")]
    Contract,

    // Locations
    #[serde(rename = "Lokationszuordnung")]
    LocationAssignment,
    #[serde(rename = "Marktlokation")]
    MarketLocation,
    #[serde(rename = "Messlokation")]
    MeteringLocation,
    #[serde(rename = "Netzlokation")]
    NetworkLocation,

    // Parties
    #[serde(rename = "Geschaeftspartner")]
    BusinessPartner,
    #[serde(rename = "Marktteilnehmer")]
    MarketParticipant,
    #[serde(rename = "Person")]
    Person,

    // Financial
    #[serde(rename = "Fremdkosten")]
    ExternalCosts,
    #[serde(rename = "Kosten")]
    Costs,
    #[serde(rename = "Rechnung")]
    Invoice,
    #[serde(rename = "Tarifkosten")]
    TariffCosts,

    // Pricing
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

    // Technical
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

    // Other
    #[serde(rename = "Bilanzierung")]
    Balancing,
    #[serde(rename = "Region")]
    Region,
    #[serde(rename = "Regionaltarif")]
    RegionalTariff,
    #[serde(rename = "Standorteigenschaften")]
    LocationProperties,
}

impl BoType {
    /// Returns the German type name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Offer => "Angebot",
            Self::Tender => "Ausschreibung",
            Self::BundleContract => "Buendelvertrag",
            Self::Contract => "Vertrag",
            Self::LocationAssignment => "Lokationszuordnung",
            Self::MarketLocation => "Marktlokation",
            Self::MeteringLocation => "Messlokation",
            Self::NetworkLocation => "Netzlokation",
            Self::BusinessPartner => "Geschaeftspartner",
            Self::MarketParticipant => "Marktteilnehmer",
            Self::Person => "Person",
            Self::ExternalCosts => "Fremdkosten",
            Self::Costs => "Kosten",
            Self::Invoice => "Rechnung",
            Self::TariffCosts => "Tarifkosten",
            Self::PriceSheet => "Preisblatt",
            Self::ServicePriceSheet => "PreisblattDienstleistung",
            Self::HardwarePriceSheet => "PreisblattHardware",
            Self::ConcessionFeePriceSheet => "PreisblattKonzessionsabgabe",
            Self::MeteringPriceSheet => "PreisblattMessung",
            Self::NetworkUsagePriceSheet => "PreisblattNetznutzung",
            Self::Tariff => "Tarif",
            Self::TariffInfo => "Tarifinfo",
            Self::TariffPriceSheet => "Tarifpreisblatt",
            Self::EnergyAmount => "Energiemenge",
            Self::Device => "Geraet",
            Self::LoadProfile => "Lastgang",
            Self::ControllableResource => "SteuerbareRessource",
            Self::TechnicalResource => "TechnischeRessource",
            Self::Meter => "Zaehler",
            Self::TimeSeries => "Zeitreihe",
            Self::Balancing => "Bilanzierung",
            Self::Region => "Region",
            Self::RegionalTariff => "Regionaltarif",
            Self::LocationProperties => "Standorteigenschaften",
        }
    }

    /// Returns the English type name.
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Offer => "Offer",
            Self::Tender => "Tender",
            Self::BundleContract => "BundleContract",
            Self::Contract => "Contract",
            Self::LocationAssignment => "LocationAssignment",
            Self::MarketLocation => "MarketLocation",
            Self::MeteringLocation => "MeteringLocation",
            Self::NetworkLocation => "NetworkLocation",
            Self::BusinessPartner => "BusinessPartner",
            Self::MarketParticipant => "MarketParticipant",
            Self::Person => "Person",
            Self::ExternalCosts => "ExternalCosts",
            Self::Costs => "Costs",
            Self::Invoice => "Invoice",
            Self::TariffCosts => "TariffCosts",
            Self::PriceSheet => "PriceSheet",
            Self::ServicePriceSheet => "ServicePriceSheet",
            Self::HardwarePriceSheet => "HardwarePriceSheet",
            Self::ConcessionFeePriceSheet => "ConcessionFeePriceSheet",
            Self::MeteringPriceSheet => "MeteringPriceSheet",
            Self::NetworkUsagePriceSheet => "NetworkUsagePriceSheet",
            Self::Tariff => "Tariff",
            Self::TariffInfo => "TariffInfo",
            Self::TariffPriceSheet => "TariffPriceSheet",
            Self::EnergyAmount => "EnergyAmount",
            Self::Device => "Device",
            Self::LoadProfile => "LoadProfile",
            Self::ControllableResource => "ControllableResource",
            Self::TechnicalResource => "TechnicalResource",
            Self::Meter => "Meter",
            Self::TimeSeries => "TimeSeries",
            Self::Balancing => "Balancing",
            Self::Region => "Region",
            Self::RegionalTariff => "RegionalTariff",
            Self::LocationProperties => "LocationProperties",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bo_type_serialize() {
        let typ = BoType::Meter;
        let json = serde_json::to_string(&typ).unwrap();
        assert_eq!(json, r#""Zaehler""#);
    }

    #[test]
    fn test_bo_type_deserialize() {
        let typ: BoType = serde_json::from_str(r#""Zaehler""#).unwrap();
        assert_eq!(typ, BoType::Meter);
    }

    #[test]
    fn test_bo_type_names() {
        let typ = BoType::MarketLocation;
        assert_eq!(typ.german_name(), "Marktlokation");
        assert_eq!(typ.english_name(), "MarketLocation");
    }
}
