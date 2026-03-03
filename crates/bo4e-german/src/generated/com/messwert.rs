#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Messwert {
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
}
impl From<bo4e_core::com::MeasuredValue> for Messwert {
    fn from(v: bo4e_core::com::MeasuredValue) -> Self {
        Self {
            meta: v.meta,
            zeitpunkt: v.timestamp,
            wert: v.value,
            einheit: v.unit.map(Into::into),
            status: v.status.map(Into::into),
            obis_kennzahl: v.obis_code,
        }
    }
}
impl From<Messwert> for bo4e_core::com::MeasuredValue {
    fn from(v: Messwert) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.zeitpunkt,
            value: v.wert,
            unit: v.einheit.map(Into::into),
            status: v.status.map(Into::into),
            obis_code: v.obis_kennzahl,
        }
    }
}
