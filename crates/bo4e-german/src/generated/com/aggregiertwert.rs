#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregiertwert {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timestamp")]
    pub zeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "aggregationMethod")]
    pub aggregationsmethode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "periodStart")]
    pub periodenbeginn: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "periodEnd")]
    pub periodenende: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "sourceCount")]
    pub anzahl_quellwerte: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
}
impl From<bo4e_core::com::AggregatedValue> for Aggregiertwert {
    fn from(v: bo4e_core::com::AggregatedValue) -> Self {
        Self {
            meta: v.meta,
            zeitpunkt: v.timestamp,
            wert: v.value,
            einheit: v.unit.map(Into::into),
            aggregationsmethode: v.aggregation_method,
            periodenbeginn: v.period_start,
            periodenende: v.period_end,
            anzahl_quellwerte: v.source_count,
            obis_kennzahl: v.obis_code,
        }
    }
}
impl From<Aggregiertwert> for bo4e_core::com::AggregatedValue {
    fn from(v: Aggregiertwert) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.zeitpunkt,
            value: v.wert,
            unit: v.einheit.map(Into::into),
            aggregation_method: v.aggregationsmethode,
            period_start: v.periodenbeginn,
            period_end: v.periodenende,
            source_count: v.anzahl_quellwerte,
            obis_code: v.obis_kennzahl,
        }
    }
}
