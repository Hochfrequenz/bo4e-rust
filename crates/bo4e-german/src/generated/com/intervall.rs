#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intervall {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "duration")]
    pub dauer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub zeiteinheit: Option<crate::Zeiteinheit>,
}
impl From<bo4e_core::com::Interval> for Intervall {
    fn from(v: bo4e_core::com::Interval) -> Self {
        Self {
            meta: v.meta,
            dauer: v.duration,
            zeiteinheit: v.unit.map(Into::into),
        }
    }
}
impl From<Intervall> for bo4e_core::com::Interval {
    fn from(v: Intervall) -> Self {
        Self {
            meta: v.meta,
            duration: v.dauer,
            unit: v.zeiteinheit.map(Into::into),
        }
    }
}
