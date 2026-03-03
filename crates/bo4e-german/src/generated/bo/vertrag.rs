#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vertrag {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractNumber")]
    pub vertragsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractType")]
    pub vertragsart: Option<crate::Vertragsart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "status")]
    pub vertragsstatus: Option<crate::Vertragsstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractStart")]
    pub vertragsbeginn: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractEnd")]
    pub vertragsende: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "signingDate")]
    pub unterzeichnungsdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "conditions")]
    pub vertragskonditionen: Option<crate::Vertragskonditionen>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "parts")]
    pub vertragsteile: Vec<crate::Vertragsteil>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractPartner")]
    pub vertragspartner: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::Contract> for Vertrag {
    fn from(v: bo4e_core::bo::Contract) -> Self {
        Self {
            meta: v.meta,
            vertragsnummer: v.contract_number,
            beschreibung: v.description,
            vertragsart: v.contract_type.map(Into::into),
            vertragsstatus: v.status.map(Into::into),
            sparte: v.division.map(Into::into),
            vertragsbeginn: v.contract_start,
            vertragsende: v.contract_end,
            unterzeichnungsdatum: v.signing_date,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            vertragskonditionen: v.conditions.map(Into::into),
            vertragsteile: v.parts.into_iter().map(Into::into).collect(),
            vertragspartner: v.contract_partner.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Vertrag> for bo4e_core::bo::Contract {
    fn from(v: Vertrag) -> Self {
        Self {
            meta: v.meta,
            contract_number: v.vertragsnummer,
            description: v.beschreibung,
            contract_type: v.vertragsart.map(Into::into),
            status: v.vertragsstatus.map(Into::into),
            division: v.sparte.map(Into::into),
            contract_start: v.vertragsbeginn,
            contract_end: v.vertragsende,
            signing_date: v.unterzeichnungsdatum,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            conditions: v.vertragskonditionen.map(Into::into),
            parts: v.vertragsteile.into_iter().map(Into::into).collect(),
            contract_partner: v.vertragspartner.map(|b| Box::new((*b).into())),
        }
    }
}
