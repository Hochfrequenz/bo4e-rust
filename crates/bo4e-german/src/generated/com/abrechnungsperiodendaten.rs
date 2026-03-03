#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abrechnungsperiodendaten {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "periodStart")]
    pub abrechnungsbeginn: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "periodEnd")]
    pub abrechnungsende: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startReading")]
    pub anfangsstand: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "startReadingTimestamp"
    )]
    pub anfangsablesung: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endReading")]
    pub endstand: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endReadingTimestamp")]
    pub endablesung: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "consumptionValue")]
    pub verbrauchswert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "consumptionUnit")]
    pub verbrauchseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "daysInPeriod")]
    pub anzahl_tage: Option<i32>,
}
impl From<bo4e_core::com::BillingPeriodData> for Abrechnungsperiodendaten {
    fn from(v: bo4e_core::com::BillingPeriodData) -> Self {
        Self {
            meta: v.meta,
            abrechnungsbeginn: v.period_start,
            abrechnungsende: v.period_end,
            anfangsstand: v.start_reading,
            anfangsablesung: v.start_reading_timestamp,
            endstand: v.end_reading,
            endablesung: v.end_reading_timestamp,
            verbrauchswert: v.consumption_value,
            verbrauchseinheit: v.consumption_unit.map(Into::into),
            anzahl_tage: v.days_in_period,
        }
    }
}
impl From<Abrechnungsperiodendaten> for bo4e_core::com::BillingPeriodData {
    fn from(v: Abrechnungsperiodendaten) -> Self {
        Self {
            meta: v.meta,
            period_start: v.abrechnungsbeginn,
            period_end: v.abrechnungsende,
            start_reading: v.anfangsstand,
            start_reading_timestamp: v.anfangsablesung,
            end_reading: v.endstand,
            end_reading_timestamp: v.endablesung,
            consumption_value: v.verbrauchswert,
            consumption_unit: v.verbrauchseinheit.map(Into::into),
            days_in_period: v.anzahl_tage,
        }
    }
}
