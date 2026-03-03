#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rechnung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "invoiceNumber")]
    pub rechnungsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "invoiceType")]
    pub rechnungstyp: Option<crate::Rechnungstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "status")]
    pub rechnungsstatus: Option<crate::Rechnungsstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "invoiceDate")]
    pub rechnungsdatum: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "dueDate")]
    pub faelligkeitsdatum: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "billingPeriod")]
    pub abrechnungszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "netAmount")]
    pub nettobetrag: Option<crate::Betrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "taxAmount")]
    pub steuerbetrag: Option<crate::Betrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "grossAmount")]
    pub bruttobetrag: Option<crate::Betrag>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "positions")]
    pub rechnungspositionen: Vec<crate::Rechnungsposition>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "recipient")]
    pub rechnungsempfaenger: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::Invoice> for Rechnung {
    fn from(v: bo4e_core::bo::Invoice) -> Self {
        Self {
            meta: v.meta,
            rechnungsnummer: v.invoice_number,
            rechnungstyp: v.invoice_type.map(Into::into),
            rechnungsstatus: v.status.map(Into::into),
            sparte: v.division.map(Into::into),
            rechnungsdatum: v.invoice_date,
            faelligkeitsdatum: v.due_date,
            abrechnungszeitraum: v.billing_period.map(Into::into),
            nettobetrag: v.net_amount.map(Into::into),
            steuerbetrag: v.tax_amount.map(Into::into),
            bruttobetrag: v.gross_amount.map(Into::into),
            rechnungspositionen: v.positions.into_iter().map(Into::into).collect(),
            rechnungsempfaenger: v.recipient.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Rechnung> for bo4e_core::bo::Invoice {
    fn from(v: Rechnung) -> Self {
        Self {
            meta: v.meta,
            invoice_number: v.rechnungsnummer,
            invoice_type: v.rechnungstyp.map(Into::into),
            status: v.rechnungsstatus.map(Into::into),
            division: v.sparte.map(Into::into),
            invoice_date: v.rechnungsdatum,
            due_date: v.faelligkeitsdatum,
            billing_period: v.abrechnungszeitraum.map(Into::into),
            net_amount: v.nettobetrag.map(Into::into),
            tax_amount: v.steuerbetrag.map(Into::into),
            gross_amount: v.bruttobetrag.map(Into::into),
            positions: v.rechnungspositionen.into_iter().map(Into::into).collect(),
            recipient: v.rechnungsempfaenger.map(|b| Box::new((*b).into())),
        }
    }
}
