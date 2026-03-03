#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unterschrift {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "location")]
    pub ort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "date")]
    pub datum: Option<String>,
}
impl From<bo4e_core::com::Signature> for Unterschrift {
    fn from(v: bo4e_core::com::Signature) -> Self {
        Self {
            meta: v.meta,
            name: v.name,
            ort: v.location,
            datum: v.date,
        }
    }
}
impl From<Unterschrift> for bo4e_core::com::Signature {
    fn from(v: Unterschrift) -> Self {
        Self {
            meta: v.meta,
            name: v.name,
            location: v.ort,
            date: v.datum,
        }
    }
}
