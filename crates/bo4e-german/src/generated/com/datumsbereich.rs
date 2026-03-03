#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datumsbereich {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startDate")]
    pub startdatum: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endDate")]
    pub enddatum: Option<NaiveDate>,
}
impl From<bo4e_core::com::DateRange> for Datumsbereich {
    fn from(v: bo4e_core::com::DateRange) -> Self {
        Self {
            meta: v.meta,
            startdatum: v.start_date,
            enddatum: v.end_date,
        }
    }
}
impl From<Datumsbereich> for bo4e_core::com::DateRange {
    fn from(v: Datumsbereich) -> Self {
        Self {
            meta: v.meta,
            start_date: v.startdatum,
            end_date: v.enddatum,
        }
    }
}
