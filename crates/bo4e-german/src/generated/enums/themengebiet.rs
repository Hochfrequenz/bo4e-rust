#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Themengebiet {
    #[serde(rename = "ALLGEMEINER_INFORMATIONSAUSTAUSCH")]
    AllgemeinerInformationsaustausch,
    #[serde(rename = "AN_UND_ABMELDUNG")]
    AnUndAbmeldung,
    #[serde(rename = "ANSPRECHPARTNER_ALLGEMEIN")]
    AnsprechpartnerAllgemein,
    #[serde(rename = "ANSPRECHPARTNER_BDEW_DVGW")]
    BdewDvgwContact,
    #[serde(rename = "ANSPRECHPARTNER_IT_TECHNIK")]
    ItTechContact,
    #[serde(rename = "BILANZIERUNG")]
    Bilanzierung,
    #[serde(rename = "BILANZKREISKOORDINATOR")]
    Bilanzkreiskoordinator,
    #[serde(rename = "BILANZKREISVERANTWORTLICHER")]
    Bilanzkreisverantwortlicher,
    #[serde(rename = "DATENFORMATE_ZERTIFIKATE_VERSCHLUESSELUNGEN")]
    DatenformateZertifikateVerschlSselungen,
    #[serde(rename = "DEBITORENMANAGEMENT")]
    Debitorenmanagement,
    #[serde(rename = "DEMAND_SIDE_MANAGEMENT")]
    DemandSideManagement,
    #[serde(rename = "EDI_VEREINBARUNG")]
    EDIVereinbarung,
    #[serde(rename = "EDIFACT")]
    Edifact,
    #[serde(rename = "ENERGIEDATENMANAGEMENT")]
    Energiedatenmanagement,
    #[serde(rename = "FAHRPLANMANAGEMENT")]
    Fahrplanmanagement,
    #[serde(rename = "ALOCAT")]
    Alocat,
    #[serde(rename = "APERAK")]
    Aperak,
    #[serde(rename = "CONTRL")]
    Contrl,
    #[serde(rename = "INVOIC")]
    Invoic,
    #[serde(rename = "MSCONS")]
    Mscons,
    #[serde(rename = "ORDERS")]
    Orders,
    #[serde(rename = "ORDERSP")]
    Ordersp,
    #[serde(rename = "REMADV")]
    Remadv,
    #[serde(rename = "UTILMD")]
    Utilmd,
    #[serde(rename = "GABI")]
    GabiGas,
    #[serde(rename = "GELI")]
    GeliGas,
    #[serde(rename = "GERAETERUECKGABE")]
    GerTerCkgabe,
    #[serde(rename = "GERAETEWECHSEL")]
    GerTewechsel,
    #[serde(rename = "GPKE")]
    GeschFtsprozesseZurKundenbelieferungMitElektrizitT,
    #[serde(rename = "INBETRIEBNAHME")]
    Inbetriebnahme,
    #[serde(rename = "KAPAZITAETSMANAGEMENT")]
    KapazitTsmanagement,
    #[serde(rename = "KLAERFAELLE")]
    KlRfLle,
    #[serde(rename = "LASTGAENGE_RLM")]
    LastgNgeRLM,
    #[serde(rename = "LIEFERANTENRAHMENVERTRAG")]
    Lieferantenrahmenvertrag,
    #[serde(rename = "LIEFERANTENWECHSEL")]
    Lieferantenwechsel,
    #[serde(rename = "MABIS")]
    MarktregelnFRBilanzkreisabrechnungStrom,
    #[serde(rename = "MAHNWESEN")]
    Mahnwesen,
    #[serde(rename = "MARKTGEBIETSVERANTWORTLICHER")]
    Marktgebietsverantwortlicher,
    #[serde(rename = "MARKTKOMMUNIKATION")]
    Marktkommunikation,
    #[serde(rename = "MEHR_MINDERMENGEN")]
    MoreLessQuantities,
    #[serde(rename = "MSB_MDL")]
    MsbMdl,
    #[serde(rename = "NETZABRECHNUNG")]
    Netzabrechnung,
    #[serde(rename = "NETZENTGELTE")]
    Netzentgelte,
    #[serde(rename = "NETZMANAGEMENT")]
    Netzmanagement,
    #[serde(rename = "RECHT")]
    Recht,
    #[serde(rename = "REGULIERUNGSMANAGEMENT")]
    Regulierungsmanagement,
    #[serde(rename = "REKLAMATIONEN")]
    Reklamationen,
    #[serde(rename = "SPERREN_ENTSPERREN_INKASSO")]
    BlockingUnblockingCollection,
    #[serde(rename = "STAMMDATEN")]
    Stammdaten,
    #[serde(rename = "STOERUNGSFAELLE")]
    StRungsfLle,
    #[serde(rename = "TECHNISCHE_FRAGEN")]
    TechnischeFragen,
    #[serde(rename = "UMSTELLUNG_INVOIC")]
    UmstellungINVOIC,
    #[serde(rename = "VERSCHLUESSELUNG_SIGNATUR")]
    EncryptionSignature,
    #[serde(rename = "VERTRAGSMANAGEMENT")]
    Vertragsmanagement,
    #[serde(rename = "VERTRIEB")]
    Vertrieb,
    #[serde(rename = "WIM")]
    WechselprozesseImMesswesen,
    #[serde(rename = "ZAEHLERSTAENDE_SLP")]
    ZHlerstNdeSLP,
    #[serde(rename = "ZAHLUNGSVERKEHR")]
    Zahlungsverkehr,
    #[serde(rename = "ZUORDNUNGSVEREINBARUNG")]
    Zuordnungsvereinbarung,
    #[serde(rename = "EINSPEISUNG")]
    Einspeisung,
    #[serde(rename = "BEWEGUNGSDATEN")]
    Bewegungsdaten,
}
impl From<bo4e_core::enums::SubjectArea> for Themengebiet {
    fn from(v: bo4e_core::enums::SubjectArea) -> Self {
        match v {
            bo4e_core::enums::SubjectArea::GeneralInformationExchange => {
                Themengebiet::AllgemeinerInformationsaustausch
            }
            bo4e_core::enums::SubjectArea::RegistrationDeregistration => {
                Themengebiet::AnUndAbmeldung
            }
            bo4e_core::enums::SubjectArea::GeneralContact => {
                Themengebiet::AnsprechpartnerAllgemein
            }
            bo4e_core::enums::SubjectArea::BdewDvgwContact => {
                Themengebiet::BdewDvgwContact
            }
            bo4e_core::enums::SubjectArea::ItTechContact => Themengebiet::ItTechContact,
            bo4e_core::enums::SubjectArea::Balancing => Themengebiet::Bilanzierung,
            bo4e_core::enums::SubjectArea::BalancingAreaCoordinator => {
                Themengebiet::Bilanzkreiskoordinator
            }
            bo4e_core::enums::SubjectArea::BalancingAreaResponsible => {
                Themengebiet::Bilanzkreisverantwortlicher
            }
            bo4e_core::enums::SubjectArea::DataFormatsCertificatesEncryption => {
                Themengebiet::DatenformateZertifikateVerschlSselungen
            }
            bo4e_core::enums::SubjectArea::DebtorManagement => {
                Themengebiet::Debitorenmanagement
            }
            bo4e_core::enums::SubjectArea::DemandSideManagement => {
                Themengebiet::DemandSideManagement
            }
            bo4e_core::enums::SubjectArea::EdiAgreement => Themengebiet::EDIVereinbarung,
            bo4e_core::enums::SubjectArea::Edifact => Themengebiet::Edifact,
            bo4e_core::enums::SubjectArea::EnergyDataManagement => {
                Themengebiet::Energiedatenmanagement
            }
            bo4e_core::enums::SubjectArea::ScheduleManagement => {
                Themengebiet::Fahrplanmanagement
            }
            bo4e_core::enums::SubjectArea::Alocat => Themengebiet::Alocat,
            bo4e_core::enums::SubjectArea::Aperak => Themengebiet::Aperak,
            bo4e_core::enums::SubjectArea::Contrl => Themengebiet::Contrl,
            bo4e_core::enums::SubjectArea::Invoic => Themengebiet::Invoic,
            bo4e_core::enums::SubjectArea::Mscons => Themengebiet::Mscons,
            bo4e_core::enums::SubjectArea::Orders => Themengebiet::Orders,
            bo4e_core::enums::SubjectArea::Ordersp => Themengebiet::Ordersp,
            bo4e_core::enums::SubjectArea::Remadv => Themengebiet::Remadv,
            bo4e_core::enums::SubjectArea::Utilmd => Themengebiet::Utilmd,
            bo4e_core::enums::SubjectArea::GabiGas => Themengebiet::GabiGas,
            bo4e_core::enums::SubjectArea::GeliGas => Themengebiet::GeliGas,
            bo4e_core::enums::SubjectArea::DeviceReturn => Themengebiet::GerTerCkgabe,
            bo4e_core::enums::SubjectArea::DeviceChange => Themengebiet::GerTewechsel,
            bo4e_core::enums::SubjectArea::Gpke => {
                Themengebiet::GeschFtsprozesseZurKundenbelieferungMitElektrizitT
            }
            bo4e_core::enums::SubjectArea::Commissioning => Themengebiet::Inbetriebnahme,
            bo4e_core::enums::SubjectArea::CapacityManagement => {
                Themengebiet::KapazitTsmanagement
            }
            bo4e_core::enums::SubjectArea::ClarificationCases => Themengebiet::KlRfLle,
            bo4e_core::enums::SubjectArea::LoadProfilesRlm => Themengebiet::LastgNgeRLM,
            bo4e_core::enums::SubjectArea::SupplierFrameworkContract => {
                Themengebiet::Lieferantenrahmenvertrag
            }
            bo4e_core::enums::SubjectArea::SupplierSwitch => {
                Themengebiet::Lieferantenwechsel
            }
            bo4e_core::enums::SubjectArea::Mabis => {
                Themengebiet::MarktregelnFRBilanzkreisabrechnungStrom
            }
            bo4e_core::enums::SubjectArea::Dunning => Themengebiet::Mahnwesen,
            bo4e_core::enums::SubjectArea::MarketAreaResponsible => {
                Themengebiet::Marktgebietsverantwortlicher
            }
            bo4e_core::enums::SubjectArea::MarketCommunication => {
                Themengebiet::Marktkommunikation
            }
            bo4e_core::enums::SubjectArea::MoreLessQuantities => {
                Themengebiet::MoreLessQuantities
            }
            bo4e_core::enums::SubjectArea::MsbMdl => Themengebiet::MsbMdl,
            bo4e_core::enums::SubjectArea::NetworkBilling => Themengebiet::Netzabrechnung,
            bo4e_core::enums::SubjectArea::NetworkCharges => Themengebiet::Netzentgelte,
            bo4e_core::enums::SubjectArea::NetworkManagement => {
                Themengebiet::Netzmanagement
            }
            bo4e_core::enums::SubjectArea::Legal => Themengebiet::Recht,
            bo4e_core::enums::SubjectArea::RegulatoryManagement => {
                Themengebiet::Regulierungsmanagement
            }
            bo4e_core::enums::SubjectArea::Complaints => Themengebiet::Reklamationen,
            bo4e_core::enums::SubjectArea::BlockingUnblockingCollection => {
                Themengebiet::BlockingUnblockingCollection
            }
            bo4e_core::enums::SubjectArea::MasterData => Themengebiet::Stammdaten,
            bo4e_core::enums::SubjectArea::FaultCases => Themengebiet::StRungsfLle,
            bo4e_core::enums::SubjectArea::TechnicalQuestions => {
                Themengebiet::TechnischeFragen
            }
            bo4e_core::enums::SubjectArea::InvoicConversion => {
                Themengebiet::UmstellungINVOIC
            }
            bo4e_core::enums::SubjectArea::EncryptionSignature => {
                Themengebiet::EncryptionSignature
            }
            bo4e_core::enums::SubjectArea::ContractManagement => {
                Themengebiet::Vertragsmanagement
            }
            bo4e_core::enums::SubjectArea::Sales => Themengebiet::Vertrieb,
            bo4e_core::enums::SubjectArea::Wim => {
                Themengebiet::WechselprozesseImMesswesen
            }
            bo4e_core::enums::SubjectArea::MeterReadingsSlp => {
                Themengebiet::ZHlerstNdeSLP
            }
            bo4e_core::enums::SubjectArea::PaymentTransactions => {
                Themengebiet::Zahlungsverkehr
            }
            bo4e_core::enums::SubjectArea::AssignmentAgreement => {
                Themengebiet::Zuordnungsvereinbarung
            }
            bo4e_core::enums::SubjectArea::FeedIn => Themengebiet::Einspeisung,
            bo4e_core::enums::SubjectArea::TransactionData => {
                Themengebiet::Bewegungsdaten
            }
            _ => panic!("Unknown {} variant", stringify!(SubjectArea)),
        }
    }
}
impl From<Themengebiet> for bo4e_core::enums::SubjectArea {
    fn from(v: Themengebiet) -> Self {
        match v {
            Themengebiet::AllgemeinerInformationsaustausch => {
                bo4e_core::enums::SubjectArea::GeneralInformationExchange
            }
            Themengebiet::AnUndAbmeldung => {
                bo4e_core::enums::SubjectArea::RegistrationDeregistration
            }
            Themengebiet::AnsprechpartnerAllgemein => {
                bo4e_core::enums::SubjectArea::GeneralContact
            }
            Themengebiet::BdewDvgwContact => {
                bo4e_core::enums::SubjectArea::BdewDvgwContact
            }
            Themengebiet::ItTechContact => bo4e_core::enums::SubjectArea::ItTechContact,
            Themengebiet::Bilanzierung => bo4e_core::enums::SubjectArea::Balancing,
            Themengebiet::Bilanzkreiskoordinator => {
                bo4e_core::enums::SubjectArea::BalancingAreaCoordinator
            }
            Themengebiet::Bilanzkreisverantwortlicher => {
                bo4e_core::enums::SubjectArea::BalancingAreaResponsible
            }
            Themengebiet::DatenformateZertifikateVerschlSselungen => {
                bo4e_core::enums::SubjectArea::DataFormatsCertificatesEncryption
            }
            Themengebiet::Debitorenmanagement => {
                bo4e_core::enums::SubjectArea::DebtorManagement
            }
            Themengebiet::DemandSideManagement => {
                bo4e_core::enums::SubjectArea::DemandSideManagement
            }
            Themengebiet::EDIVereinbarung => bo4e_core::enums::SubjectArea::EdiAgreement,
            Themengebiet::Edifact => bo4e_core::enums::SubjectArea::Edifact,
            Themengebiet::Energiedatenmanagement => {
                bo4e_core::enums::SubjectArea::EnergyDataManagement
            }
            Themengebiet::Fahrplanmanagement => {
                bo4e_core::enums::SubjectArea::ScheduleManagement
            }
            Themengebiet::Alocat => bo4e_core::enums::SubjectArea::Alocat,
            Themengebiet::Aperak => bo4e_core::enums::SubjectArea::Aperak,
            Themengebiet::Contrl => bo4e_core::enums::SubjectArea::Contrl,
            Themengebiet::Invoic => bo4e_core::enums::SubjectArea::Invoic,
            Themengebiet::Mscons => bo4e_core::enums::SubjectArea::Mscons,
            Themengebiet::Orders => bo4e_core::enums::SubjectArea::Orders,
            Themengebiet::Ordersp => bo4e_core::enums::SubjectArea::Ordersp,
            Themengebiet::Remadv => bo4e_core::enums::SubjectArea::Remadv,
            Themengebiet::Utilmd => bo4e_core::enums::SubjectArea::Utilmd,
            Themengebiet::GabiGas => bo4e_core::enums::SubjectArea::GabiGas,
            Themengebiet::GeliGas => bo4e_core::enums::SubjectArea::GeliGas,
            Themengebiet::GerTerCkgabe => bo4e_core::enums::SubjectArea::DeviceReturn,
            Themengebiet::GerTewechsel => bo4e_core::enums::SubjectArea::DeviceChange,
            Themengebiet::GeschFtsprozesseZurKundenbelieferungMitElektrizitT => {
                bo4e_core::enums::SubjectArea::Gpke
            }
            Themengebiet::Inbetriebnahme => bo4e_core::enums::SubjectArea::Commissioning,
            Themengebiet::KapazitTsmanagement => {
                bo4e_core::enums::SubjectArea::CapacityManagement
            }
            Themengebiet::KlRfLle => bo4e_core::enums::SubjectArea::ClarificationCases,
            Themengebiet::LastgNgeRLM => bo4e_core::enums::SubjectArea::LoadProfilesRlm,
            Themengebiet::Lieferantenrahmenvertrag => {
                bo4e_core::enums::SubjectArea::SupplierFrameworkContract
            }
            Themengebiet::Lieferantenwechsel => {
                bo4e_core::enums::SubjectArea::SupplierSwitch
            }
            Themengebiet::MarktregelnFRBilanzkreisabrechnungStrom => {
                bo4e_core::enums::SubjectArea::Mabis
            }
            Themengebiet::Mahnwesen => bo4e_core::enums::SubjectArea::Dunning,
            Themengebiet::Marktgebietsverantwortlicher => {
                bo4e_core::enums::SubjectArea::MarketAreaResponsible
            }
            Themengebiet::Marktkommunikation => {
                bo4e_core::enums::SubjectArea::MarketCommunication
            }
            Themengebiet::MoreLessQuantities => {
                bo4e_core::enums::SubjectArea::MoreLessQuantities
            }
            Themengebiet::MsbMdl => bo4e_core::enums::SubjectArea::MsbMdl,
            Themengebiet::Netzabrechnung => bo4e_core::enums::SubjectArea::NetworkBilling,
            Themengebiet::Netzentgelte => bo4e_core::enums::SubjectArea::NetworkCharges,
            Themengebiet::Netzmanagement => {
                bo4e_core::enums::SubjectArea::NetworkManagement
            }
            Themengebiet::Recht => bo4e_core::enums::SubjectArea::Legal,
            Themengebiet::Regulierungsmanagement => {
                bo4e_core::enums::SubjectArea::RegulatoryManagement
            }
            Themengebiet::Reklamationen => bo4e_core::enums::SubjectArea::Complaints,
            Themengebiet::BlockingUnblockingCollection => {
                bo4e_core::enums::SubjectArea::BlockingUnblockingCollection
            }
            Themengebiet::Stammdaten => bo4e_core::enums::SubjectArea::MasterData,
            Themengebiet::StRungsfLle => bo4e_core::enums::SubjectArea::FaultCases,
            Themengebiet::TechnischeFragen => {
                bo4e_core::enums::SubjectArea::TechnicalQuestions
            }
            Themengebiet::UmstellungINVOIC => {
                bo4e_core::enums::SubjectArea::InvoicConversion
            }
            Themengebiet::EncryptionSignature => {
                bo4e_core::enums::SubjectArea::EncryptionSignature
            }
            Themengebiet::Vertragsmanagement => {
                bo4e_core::enums::SubjectArea::ContractManagement
            }
            Themengebiet::Vertrieb => bo4e_core::enums::SubjectArea::Sales,
            Themengebiet::WechselprozesseImMesswesen => {
                bo4e_core::enums::SubjectArea::Wim
            }
            Themengebiet::ZHlerstNdeSLP => {
                bo4e_core::enums::SubjectArea::MeterReadingsSlp
            }
            Themengebiet::Zahlungsverkehr => {
                bo4e_core::enums::SubjectArea::PaymentTransactions
            }
            Themengebiet::Zuordnungsvereinbarung => {
                bo4e_core::enums::SubjectArea::AssignmentAgreement
            }
            Themengebiet::Einspeisung => bo4e_core::enums::SubjectArea::FeedIn,
            Themengebiet::Bewegungsdaten => {
                bo4e_core::enums::SubjectArea::TransactionData
            }
            _ => panic!("Unknown {} variant", stringify!(Themengebiet)),
        }
    }
}
