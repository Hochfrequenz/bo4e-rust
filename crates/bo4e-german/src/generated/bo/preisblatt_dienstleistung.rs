#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreisblattDienstleistung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "serviceType")]
    pub dienstleistungsart: Option<crate::Dienstleistungstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceSheetNumber")]
    pub preisblattnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validFrom")]
    pub gueltig_ab: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub gueltig_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "prices")]
    pub dienstleistungspreise: Vec<crate::Dienstleistungspreis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "provider")]
    pub dienstleister: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::ServicePriceSheet> for PreisblattDienstleistung {
    fn from(v: bo4e_core::bo::ServicePriceSheet) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            dienstleistungsart: v.service_type.map(Into::into),
            preisblattnummer: v.price_sheet_number,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            gueltig_ab: v.valid_from,
            gueltig_bis: v.valid_until,
            dienstleistungspreise: v.prices.into_iter().map(Into::into).collect(),
            dienstleister: v.provider.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<PreisblattDienstleistung> for bo4e_core::bo::ServicePriceSheet {
    fn from(v: PreisblattDienstleistung) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            service_type: v.dienstleistungsart.map(Into::into),
            price_sheet_number: v.preisblattnummer,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            valid_from: v.gueltig_ab,
            valid_until: v.gueltig_bis,
            prices: v.dienstleistungspreise.into_iter().map(Into::into).collect(),
            provider: v.dienstleister.map(|b| Box::new((*b).into())),
        }
    }
}
