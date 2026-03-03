#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Rechnungstyp {
    #[serde(rename = "ENDKUNDENRECHNUNG")]
    Endkundenrechnung,
    #[serde(rename = "NETZNUTZUNGSRECHNUNG")]
    Netznutzungsrechnung,
    #[serde(rename = "MEHRMINDERMENGENRECHNUNG")]
    Mehrmindermengenrechnung,
    #[serde(rename = "MESSSTELLENBETRIEBSRECHNUNG")]
    Messstellenbetriebsrechnung,
    #[serde(rename = "BESCHAFFUNGSRECHNUNG")]
    Beschaffungsrechnung,
    #[serde(rename = "AUSGLEICHSENERGIERECHNUNG")]
    Ausgleichsenergierechnung,
    #[serde(rename = "ABSCHLUSSRECHNUNG")]
    Abschlussrechnung,
    #[serde(rename = "ABSCHLAGSRECHNUNG")]
    Abschlagsrechnung,
    #[serde(rename = "TURNUSRECHNUNG")]
    Turnusrechnung,
    #[serde(rename = "MONATSRECHNUNG")]
    Monatsrechnung,
    #[serde(rename = "ZWISCHENRECHNUNG")]
    Zwischenrechnung,
    #[serde(rename = "INTEGRIERTE_13TE_RECHNUNG")]
    Integrierte13teRechnung,
    #[serde(rename = "ZUSAETZLICHE_13TE_RECHNUNG")]
    Zusaetzliche13teRechnung,
}
impl From<bo4e_core::enums::InvoiceType> for Rechnungstyp {
    fn from(v: bo4e_core::enums::InvoiceType) -> Self {
        match v {
            bo4e_core::enums::InvoiceType::EndCustomerInvoice => {
                Rechnungstyp::Endkundenrechnung
            }
            bo4e_core::enums::InvoiceType::NetworkUsageInvoice => {
                Rechnungstyp::Netznutzungsrechnung
            }
            bo4e_core::enums::InvoiceType::SurplusDeficitInvoice => {
                Rechnungstyp::Mehrmindermengenrechnung
            }
            bo4e_core::enums::InvoiceType::MeteringPointOperationInvoice => {
                Rechnungstyp::Messstellenbetriebsrechnung
            }
            bo4e_core::enums::InvoiceType::ProcurementInvoice => {
                Rechnungstyp::Beschaffungsrechnung
            }
            bo4e_core::enums::InvoiceType::BalancingEnergyInvoice => {
                Rechnungstyp::Ausgleichsenergierechnung
            }
            bo4e_core::enums::InvoiceType::FinalInvoice => {
                Rechnungstyp::Abschlussrechnung
            }
            bo4e_core::enums::InvoiceType::InstalmentInvoice => {
                Rechnungstyp::Abschlagsrechnung
            }
            bo4e_core::enums::InvoiceType::PeriodicInvoice => {
                Rechnungstyp::Turnusrechnung
            }
            bo4e_core::enums::InvoiceType::MonthlyInvoice => Rechnungstyp::Monatsrechnung,
            bo4e_core::enums::InvoiceType::InterimInvoice => {
                Rechnungstyp::Zwischenrechnung
            }
            bo4e_core::enums::InvoiceType::Integrated13thInvoice => {
                Rechnungstyp::Integrierte13teRechnung
            }
            bo4e_core::enums::InvoiceType::Additional13thInvoice => {
                Rechnungstyp::Zusaetzliche13teRechnung
            }
            _ => panic!("Unknown {} variant", stringify!(InvoiceType)),
        }
    }
}
impl From<Rechnungstyp> for bo4e_core::enums::InvoiceType {
    fn from(v: Rechnungstyp) -> Self {
        match v {
            Rechnungstyp::Endkundenrechnung => {
                bo4e_core::enums::InvoiceType::EndCustomerInvoice
            }
            Rechnungstyp::Netznutzungsrechnung => {
                bo4e_core::enums::InvoiceType::NetworkUsageInvoice
            }
            Rechnungstyp::Mehrmindermengenrechnung => {
                bo4e_core::enums::InvoiceType::SurplusDeficitInvoice
            }
            Rechnungstyp::Messstellenbetriebsrechnung => {
                bo4e_core::enums::InvoiceType::MeteringPointOperationInvoice
            }
            Rechnungstyp::Beschaffungsrechnung => {
                bo4e_core::enums::InvoiceType::ProcurementInvoice
            }
            Rechnungstyp::Ausgleichsenergierechnung => {
                bo4e_core::enums::InvoiceType::BalancingEnergyInvoice
            }
            Rechnungstyp::Abschlussrechnung => {
                bo4e_core::enums::InvoiceType::FinalInvoice
            }
            Rechnungstyp::Abschlagsrechnung => {
                bo4e_core::enums::InvoiceType::InstalmentInvoice
            }
            Rechnungstyp::Turnusrechnung => {
                bo4e_core::enums::InvoiceType::PeriodicInvoice
            }
            Rechnungstyp::Monatsrechnung => bo4e_core::enums::InvoiceType::MonthlyInvoice,
            Rechnungstyp::Zwischenrechnung => {
                bo4e_core::enums::InvoiceType::InterimInvoice
            }
            Rechnungstyp::Integrierte13teRechnung => {
                bo4e_core::enums::InvoiceType::Integrated13thInvoice
            }
            Rechnungstyp::Zusaetzliche13teRechnung => {
                bo4e_core::enums::InvoiceType::Additional13thInvoice
            }
            _ => panic!("Unknown {} variant", stringify!(Rechnungstyp)),
        }
    }
}
