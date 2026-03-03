#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buendelvertrag {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "bundleContractNumber")]
    pub buendelvertragsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "status")]
    pub vertragsstatus: Option<crate::Vertragsstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractStart")]
    pub vertragsbeginn: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractEnd")]
    pub vertragsende: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "individualContracts"
    )]
    pub einzelvertraege: Vec<crate::Vertrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractPartner")]
    pub vertragspartner: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::BundleContract> for Buendelvertrag {
    fn from(v: bo4e_core::bo::BundleContract) -> Self {
        Self {
            meta: v.meta,
            buendelvertragsnummer: v.bundle_contract_number,
            beschreibung: v.description,
            vertragsstatus: v.status.map(Into::into),
            sparte: v.division.map(Into::into),
            vertragsbeginn: v.contract_start,
            vertragsende: v.contract_end,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            einzelvertraege: v
                .individual_contracts
                .into_iter()
                .map(Into::into)
                .collect(),
            vertragspartner: v.contract_partner.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Buendelvertrag> for bo4e_core::bo::BundleContract {
    fn from(v: Buendelvertrag) -> Self {
        Self {
            meta: v.meta,
            bundle_contract_number: v.buendelvertragsnummer,
            description: v.beschreibung,
            status: v.vertragsstatus.map(Into::into),
            division: v.sparte.map(Into::into),
            contract_start: v.vertragsbeginn,
            contract_end: v.vertragsende,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            individual_contracts: v
                .einzelvertraege
                .into_iter()
                .map(Into::into)
                .collect(),
            contract_partner: v.vertragspartner.map(|b| Box::new((*b).into())),
        }
    }
}
