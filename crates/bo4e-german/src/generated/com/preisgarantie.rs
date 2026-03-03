#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preisgarantie {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "guaranteeType")]
    pub preisgarantietyp: Option<crate::Preisgarantietyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validFrom")]
    pub zeitliche_gueltigkeit: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub zeitliche_gueltigkeit_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
}
impl From<bo4e_core::com::PriceGuarantee> for Preisgarantie {
    fn from(v: bo4e_core::com::PriceGuarantee) -> Self {
        Self {
            meta: v.meta,
            preisgarantietyp: v.guarantee_type.map(Into::into),
            zeitliche_gueltigkeit: v.valid_from,
            zeitliche_gueltigkeit_bis: v.valid_until,
            beschreibung: v.description,
        }
    }
}
impl From<Preisgarantie> for bo4e_core::com::PriceGuarantee {
    fn from(v: Preisgarantie) -> Self {
        Self {
            meta: v.meta,
            guarantee_type: v.preisgarantietyp.map(Into::into),
            valid_from: v.zeitliche_gueltigkeit,
            valid_until: v.zeitliche_gueltigkeit_bis,
            description: v.beschreibung,
        }
    }
}
