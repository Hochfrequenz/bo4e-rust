#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExterneReferenz {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "externalRefName")]
    pub ex_ref_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "externalRefValue")]
    pub ex_ref_wert: Option<String>,
}
impl From<bo4e_core::com::ExternalReference> for ExterneReferenz {
    fn from(v: bo4e_core::com::ExternalReference) -> Self {
        Self {
            meta: v.meta,
            ex_ref_name: v.external_ref_name,
            ex_ref_wert: v.external_ref_value,
        }
    }
}
impl From<ExterneReferenz> for bo4e_core::com::ExternalReference {
    fn from(v: ExterneReferenz) -> Self {
        Self {
            meta: v.meta,
            external_ref_name: v.ex_ref_name,
            external_ref_value: v.ex_ref_wert,
        }
    }
}
