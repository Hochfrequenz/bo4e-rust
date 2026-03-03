#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bilanzierung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "balanceGroupId")]
    pub bilanzkreis_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "balanceGroupName")]
    pub bilanzkreisname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketArea")]
    pub marktgebiet: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "balanceResponsibleParty"
    )]
    pub bilanzkreisverantwortlicher: Option<Box<crate::Marktteilnehmer>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startDate")]
    pub startdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endDate")]
    pub enddatum: Option<chrono::DateTime<chrono::Utc>>,
}
impl From<bo4e_core::bo::Balancing> for Bilanzierung {
    fn from(v: bo4e_core::bo::Balancing) -> Self {
        Self {
            meta: v.meta,
            bilanzkreis_id: v.balance_group_id,
            bilanzkreisname: v.balance_group_name,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            marktgebiet: v.market_area,
            bilanzkreisverantwortlicher: v.balance_responsible_party.map(|b| Box::new((*b).into())),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            startdatum: v.start_date,
            enddatum: v.end_date,
        }
    }
}
impl From<Bilanzierung> for bo4e_core::bo::Balancing {
    fn from(v: Bilanzierung) -> Self {
        Self {
            meta: v.meta,
            balance_group_id: v.bilanzkreis_id,
            balance_group_name: v.bilanzkreisname,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            market_area: v.marktgebiet,
            balance_responsible_party: v.bilanzkreisverantwortlicher.map(|b| Box::new((*b).into())),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            start_date: v.startdatum,
            end_date: v.enddatum,
        }
    }
}
