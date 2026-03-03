#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kontaktweg {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contactType")]
    pub kontaktart: Option<crate::Kontaktart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contactValue")]
    pub kontaktwert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isPreferred")]
    pub ist_bevorzugter_kontaktweg: Option<bool>,
}
impl From<bo4e_core::com::ContactMethod> for Kontaktweg {
    fn from(v: bo4e_core::com::ContactMethod) -> Self {
        Self {
            meta: v.meta,
            kontaktart: v.contact_type.map(Into::into),
            kontaktwert: v.contact_value,
            beschreibung: v.description,
            ist_bevorzugter_kontaktweg: v.is_preferred,
        }
    }
}
impl From<Kontaktweg> for bo4e_core::com::ContactMethod {
    fn from(v: Kontaktweg) -> Self {
        Self {
            meta: v.meta,
            contact_type: v.kontaktart.map(Into::into),
            contact_value: v.kontaktwert,
            description: v.beschreibung,
            is_preferred: v.ist_bevorzugter_kontaktweg,
        }
    }
}
