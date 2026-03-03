#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zeitraum {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "start")]
    pub startdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "end")]
    pub enddatum: Option<chrono::DateTime<chrono::Utc>>,
}
impl From<bo4e_core::com::TimePeriod> for Zeitraum {
    fn from(v: bo4e_core::com::TimePeriod) -> Self {
        Self {
            meta: v.meta,
            startdatum: v.start,
            enddatum: v.end,
        }
    }
}
impl From<Zeitraum> for bo4e_core::com::TimePeriod {
    fn from(v: Zeitraum) -> Self {
        Self {
            meta: v.meta,
            start: v.startdatum,
            end: v.enddatum,
        }
    }
}
