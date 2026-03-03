#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreisblattKonzessionsabgabe {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customerGroup")]
    pub kundengruppe: Option<crate::KundengruppeKA>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceSheetNumber")]
    pub preisblattnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validFrom")]
    pub gueltig_ab: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub gueltig_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "concessionFees"
    )]
    pub konzessionsabgaben: Vec<crate::Konzessionsabgabe>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "municipality")]
    pub gemeindebezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "operator")]
    pub netzbetreiber: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::ConcessionFeePriceSheet> for PreisblattKonzessionsabgabe {
    fn from(v: bo4e_core::bo::ConcessionFeePriceSheet) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            kundengruppe: v.customer_group.map(Into::into),
            preisblattnummer: v.price_sheet_number,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            gueltig_ab: v.valid_from,
            gueltig_bis: v.valid_until,
            konzessionsabgaben: v.concession_fees.into_iter().map(Into::into).collect(),
            gemeindebezeichnung: v.municipality,
            netzbetreiber: v.operator.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<PreisblattKonzessionsabgabe> for bo4e_core::bo::ConcessionFeePriceSheet {
    fn from(v: PreisblattKonzessionsabgabe) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            customer_group: v.kundengruppe.map(Into::into),
            price_sheet_number: v.preisblattnummer,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            valid_from: v.gueltig_ab,
            valid_until: v.gueltig_bis,
            concession_fees: v.konzessionsabgaben.into_iter().map(Into::into).collect(),
            municipality: v.gemeindebezeichnung,
            operator: v.netzbetreiber.map(|b| Box::new((*b).into())),
        }
    }
}
