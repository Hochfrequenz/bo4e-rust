#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zeitreihenwert {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timestamp")]
    pub zeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::Messwertstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "sequenceNumber")]
    pub sequenznummer: Option<i64>,
}
impl From<bo4e_core::com::TimeSeriesValue> for Zeitreihenwert {
    fn from(v: bo4e_core::com::TimeSeriesValue) -> Self {
        Self {
            meta: v.meta,
            zeitpunkt: v.timestamp,
            wert: v.value,
            einheit: v.unit.map(Into::into),
            status: v.status.map(Into::into),
            obis_kennzahl: v.obis_code,
            sequenznummer: v.sequence_number,
        }
    }
}
impl From<Zeitreihenwert> for bo4e_core::com::TimeSeriesValue {
    fn from(v: Zeitreihenwert) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.zeitpunkt,
            value: v.wert,
            unit: v.einheit.map(Into::into),
            status: v.status.map(Into::into),
            obis_code: v.obis_kennzahl,
            sequence_number: v.sequenznummer,
        }
    }
}
