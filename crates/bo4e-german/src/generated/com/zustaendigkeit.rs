#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zustaendigkeit {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "subjectArea")]
    pub themengebiet: Option<crate::Themengebiet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "department")]
    pub abteilung: Option<String>,
}
impl From<bo4e_core::com::Responsibility> for Zustaendigkeit {
    fn from(v: bo4e_core::com::Responsibility) -> Self {
        Self {
            meta: v.meta,
            themengebiet: v.subject_area.map(Into::into),
            position: v.position,
            abteilung: v.department,
        }
    }
}
impl From<Zustaendigkeit> for bo4e_core::com::Responsibility {
    fn from(v: Zustaendigkeit) -> Self {
        Self {
            meta: v.meta,
            subject_area: v.themengebiet.map(Into::into),
            position: v.position,
            department: v.abteilung,
        }
    }
}
