//! Subject area (Themengebiet) enumeration.

use serde::{Deserialize, Serialize};

/// Subject area classification for assigning contacts or responsibilities.
///
/// German: Themengebiet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SubjectArea {
    /// General information exchange (Allgemeiner Informationsaustausch)
    #[serde(rename = "ALLGEMEINER_INFORMATIONSAUSTAUSCH")]
    GeneralInformationExchange,

    /// Registration and deregistration (An- und Abmeldung)
    #[serde(rename = "AN_UND_ABMELDUNG")]
    RegistrationDeregistration,

    /// General contact person (Ansprechpartner Allgemein)
    #[serde(rename = "ANSPRECHPARTNER_ALLGEMEIN")]
    GeneralContact,

    /// BDEW/DVGW contact person (Ansprechpartner BDEW/DVGW)
    #[serde(rename = "ANSPRECHPARTNER_BDEW_DVGW")]
    BdewDvgwContact,

    /// IT/Technical contact person (Ansprechpartner IT/Technik)
    #[serde(rename = "ANSPRECHPARTNER_IT_TECHNIK")]
    ItTechContact,

    /// Balancing (Bilanzierung)
    #[serde(rename = "BILANZIERUNG")]
    Balancing,

    /// Balancing area coordinator (Bilanzkreiskoordinator)
    #[serde(rename = "BILANZKREISKOORDINATOR")]
    BalancingAreaCoordinator,

    /// Balancing area responsible (Bilanzkreisverantwortlicher)
    #[serde(rename = "BILANZKREISVERANTWORTLICHER")]
    BalancingAreaResponsible,

    /// Data formats, certificates, encryption (Datenformate, Zertifikate, Verschlüsselungen)
    #[serde(rename = "DATENFORMATE_ZERTIFIKATE_VERSCHLUESSELUNGEN")]
    DataFormatsCertificatesEncryption,

    /// Debtor management (Debitorenmanagement)
    #[serde(rename = "DEBITORENMANAGEMENT")]
    DebtorManagement,

    /// Demand-Side-Management
    #[serde(rename = "DEMAND_SIDE_MANAGEMENT")]
    DemandSideManagement,

    /// EDI agreement (EDI-Vereinbarung)
    #[serde(rename = "EDI_VEREINBARUNG")]
    EdiAgreement,

    /// EDIFACT format
    #[serde(rename = "EDIFACT")]
    Edifact,

    /// Energy data management (Energiedatenmanagement)
    #[serde(rename = "ENERGIEDATENMANAGEMENT")]
    EnergyDataManagement,

    /// Schedule management (Fahrplanmanagement)
    #[serde(rename = "FAHRPLANMANAGEMENT")]
    ScheduleManagement,

    /// Format: ALOCAT
    #[serde(rename = "ALOCAT")]
    Alocat,

    /// Format: APERAK
    #[serde(rename = "APERAK")]
    Aperak,

    /// Format: CONTRL
    #[serde(rename = "CONTRL")]
    Contrl,

    /// Format: INVOIC
    #[serde(rename = "INVOIC")]
    Invoic,

    /// Format: MSCONS
    #[serde(rename = "MSCONS")]
    Mscons,

    /// Format: ORDERS
    #[serde(rename = "ORDERS")]
    Orders,

    /// Format: ORDERSP
    #[serde(rename = "ORDERSP")]
    Ordersp,

    /// Format: REMADV
    #[serde(rename = "REMADV")]
    Remadv,

    /// Format: UTILMD
    #[serde(rename = "UTILMD")]
    Utilmd,

    /// GaBi Gas
    #[serde(rename = "GABI")]
    GabiGas,

    /// GeLi Gas
    #[serde(rename = "GELI")]
    GeliGas,

    /// Device return (Geräterückgabe)
    #[serde(rename = "GERAETERUECKGABE")]
    DeviceReturn,

    /// Device change (Gerätewechsel)
    #[serde(rename = "GERAETEWECHSEL")]
    DeviceChange,

    /// GPKE (Geschäftsprozesse zur Kundenbelieferung mit Elektrizität)
    #[serde(rename = "GPKE")]
    Gpke,

    /// Commissioning (Inbetriebnahme)
    #[serde(rename = "INBETRIEBNAHME")]
    Commissioning,

    /// Capacity management (Kapazitätsmanagement)
    #[serde(rename = "KAPAZITAETSMANAGEMENT")]
    CapacityManagement,

    /// Clarification cases (Klärfälle)
    #[serde(rename = "KLAERFAELLE")]
    ClarificationCases,

    /// Load profiles RLM (Lastgänge RLM)
    #[serde(rename = "LASTGAENGE_RLM")]
    LoadProfilesRlm,

    /// Supplier framework contract (Lieferantenrahmenvertrag)
    #[serde(rename = "LIEFERANTENRAHMENVERTRAG")]
    SupplierFrameworkContract,

    /// Supplier switch (Lieferantenwechsel)
    #[serde(rename = "LIEFERANTENWECHSEL")]
    SupplierSwitch,

    /// MaBiS (Marktregeln für Bilanzkreisabrechnung Strom)
    #[serde(rename = "MABIS")]
    Mabis,

    /// Dunning (Mahnwesen)
    #[serde(rename = "MAHNWESEN")]
    Dunning,

    /// Market area responsible (Marktgebietsverantwortlicher)
    #[serde(rename = "MARKTGEBIETSVERANTWORTLICHER")]
    MarketAreaResponsible,

    /// Market communication (Marktkommunikation)
    #[serde(rename = "MARKTKOMMUNIKATION")]
    MarketCommunication,

    /// More/less quantities (Mehr-/Mindermengen)
    #[serde(rename = "MEHR_MINDERMENGEN")]
    MoreLessQuantities,

    /// MSB - MDL
    #[serde(rename = "MSB_MDL")]
    MsbMdl,

    /// Network billing (Netzabrechnung)
    #[serde(rename = "NETZABRECHNUNG")]
    NetworkBilling,

    /// Network charges (Netzentgelte)
    #[serde(rename = "NETZENTGELTE")]
    NetworkCharges,

    /// Network management (Netzmanagement)
    #[serde(rename = "NETZMANAGEMENT")]
    NetworkManagement,

    /// Legal (Recht)
    #[serde(rename = "RECHT")]
    Legal,

    /// Regulatory management (Regulierungsmanagement)
    #[serde(rename = "REGULIERUNGSMANAGEMENT")]
    RegulatoryManagement,

    /// Complaints (Reklamationen)
    #[serde(rename = "REKLAMATIONEN")]
    Complaints,

    /// Blocking/unblocking/collection (Sperren/Entsperren/Inkasso)
    #[serde(rename = "SPERREN_ENTSPERREN_INKASSO")]
    BlockingUnblockingCollection,

    /// Master data (Stammdaten)
    #[serde(rename = "STAMMDATEN")]
    MasterData,

    /// Fault cases (Störungsfälle)
    #[serde(rename = "STOERUNGSFAELLE")]
    FaultCases,

    /// Technical questions (Technische Fragen)
    #[serde(rename = "TECHNISCHE_FRAGEN")]
    TechnicalQuestions,

    /// INVOIC conversion (Umstellung INVOIC)
    #[serde(rename = "UMSTELLUNG_INVOIC")]
    InvoicConversion,

    /// Encryption/Signature (Verschlüsselung/Signatur)
    #[serde(rename = "VERSCHLUESSELUNG_SIGNATUR")]
    EncryptionSignature,

    /// Contract management (Vertragsmanagement)
    #[serde(rename = "VERTRAGSMANAGEMENT")]
    ContractManagement,

    /// Sales (Vertrieb)
    #[serde(rename = "VERTRIEB")]
    Sales,

    /// WiM (Wechselprozesse im Messwesen)
    #[serde(rename = "WIM")]
    Wim,

    /// Meter readings SLP (Zählerstände SLP)
    #[serde(rename = "ZAEHLERSTAENDE_SLP")]
    MeterReadingsSlp,

    /// Payment transactions (Zahlungsverkehr)
    #[serde(rename = "ZAHLUNGSVERKEHR")]
    PaymentTransactions,

    /// Assignment agreement (Zuordnungsvereinbarung)
    #[serde(rename = "ZUORDNUNGSVEREINBARUNG")]
    AssignmentAgreement,

    /// Feed-in (Einspeisung)
    #[serde(rename = "EINSPEISUNG")]
    FeedIn,

    /// Transaction data (Bewegungsdaten)
    #[serde(rename = "BEWEGUNGSDATEN")]
    TransactionData,
}

impl SubjectArea {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::GeneralInformationExchange => "Allgemeiner Informationsaustausch",
            Self::RegistrationDeregistration => "An- und Abmeldung",
            Self::GeneralContact => "Ansprechpartner Allgemein",
            Self::BdewDvgwContact => "Ansprechpartner BDEW/DVGW",
            Self::ItTechContact => "Ansprechpartner IT/Technik",
            Self::Balancing => "Bilanzierung",
            Self::BalancingAreaCoordinator => "Bilanzkreiskoordinator",
            Self::BalancingAreaResponsible => "Bilanzkreisverantwortlicher",
            Self::DataFormatsCertificatesEncryption => {
                "Datenformate, Zertifikate, Verschlüsselungen"
            }
            Self::DebtorManagement => "Debitorenmanagement",
            Self::DemandSideManagement => "Demand-Side-Management",
            Self::EdiAgreement => "EDI-Vereinbarung",
            Self::Edifact => "EDIFACT",
            Self::EnergyDataManagement => "Energiedatenmanagement",
            Self::ScheduleManagement => "Fahrplanmanagement",
            Self::Alocat => "Format:ALOCAT",
            Self::Aperak => "Format:APERAK",
            Self::Contrl => "Format:CONTRL",
            Self::Invoic => "Format:INVOIC",
            Self::Mscons => "Format:MSCONS",
            Self::Orders => "Format:ORDERS",
            Self::Ordersp => "Format:ORDERSP",
            Self::Remadv => "Format:REMADV",
            Self::Utilmd => "Format:UTILMD",
            Self::GabiGas => "GaBi Gas",
            Self::GeliGas => "GeLi Gas",
            Self::DeviceReturn => "Geräterückgabe",
            Self::DeviceChange => "Gerätewechsel",
            Self::Gpke => "GPKE",
            Self::Commissioning => "Inbetriebnahme",
            Self::CapacityManagement => "Kapazitätsmanagement",
            Self::ClarificationCases => "Klärfälle",
            Self::LoadProfilesRlm => "Lastgänge RLM",
            Self::SupplierFrameworkContract => "Lieferantenrahmenvertrag",
            Self::SupplierSwitch => "Lieferantenwechsel",
            Self::Mabis => "MaBiS",
            Self::Dunning => "Mahnwesen",
            Self::MarketAreaResponsible => "Marktgebietsverantwortlicher",
            Self::MarketCommunication => "Marktkommunikation",
            Self::MoreLessQuantities => "Mehr-/Mindermengen",
            Self::MsbMdl => "MSB - MDL",
            Self::NetworkBilling => "Netzabrechnung",
            Self::NetworkCharges => "Netzentgelte",
            Self::NetworkManagement => "Netzmanagement",
            Self::Legal => "Recht",
            Self::RegulatoryManagement => "Regulierungsmanagement",
            Self::Complaints => "Reklamationen",
            Self::BlockingUnblockingCollection => "Sperren/Entsperren/Inkasso",
            Self::MasterData => "Stammdaten",
            Self::FaultCases => "Störungsfälle",
            Self::TechnicalQuestions => "Technische Fragen",
            Self::InvoicConversion => "Umstellung INVOIC",
            Self::EncryptionSignature => "Verschlüsselung/Signatur",
            Self::ContractManagement => "Vertragsmanagement",
            Self::Sales => "Vertrieb",
            Self::Wim => "WiM",
            Self::MeterReadingsSlp => "Zählerstände SLP",
            Self::PaymentTransactions => "Zahlungsverkehr",
            Self::AssignmentAgreement => "Zuordnungsvereinbarung",
            Self::FeedIn => "Einspeisung",
            Self::TransactionData => "Bewegungsdaten",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&SubjectArea::Balancing).unwrap(),
            r#""BILANZIERUNG""#
        );
        assert_eq!(
            serde_json::to_string(&SubjectArea::MasterData).unwrap(),
            r#""STAMMDATEN""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<SubjectArea>(r#""MARKTKOMMUNIKATION""#).unwrap(),
            SubjectArea::MarketCommunication
        );
        assert_eq!(
            serde_json::from_str::<SubjectArea>(r#""VERTRIEB""#).unwrap(),
            SubjectArea::Sales
        );
    }

    #[test]
    fn test_roundtrip() {
        for area in [
            SubjectArea::GeneralInformationExchange,
            SubjectArea::Balancing,
            SubjectArea::MarketCommunication,
            SubjectArea::MasterData,
            SubjectArea::Sales,
        ] {
            let json = serde_json::to_string(&area).unwrap();
            let parsed: SubjectArea = serde_json::from_str(&json).unwrap();
            assert_eq!(area, parsed);
        }
    }
}
