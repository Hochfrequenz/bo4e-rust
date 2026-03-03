#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lastprofilwert {
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
    #[serde(skip_serializing_if = "Option::is_none", alias = "intervalMinutes")]
    pub intervalllaenge: Option<i32>,
}
impl From<bo4e_core::com::LoadProfileValue> for Lastprofilwert {
    fn from(v: bo4e_core::com::LoadProfileValue) -> Self {
        Self {
            meta: v.meta,
            zeitpunkt: v.timestamp,
            wert: v.value,
            einheit: v.unit.map(Into::into),
            status: v.status.map(Into::into),
            obis_kennzahl: v.obis_code,
            intervalllaenge: v.interval_minutes,
        }
    }
}
impl From<Lastprofilwert> for bo4e_core::com::LoadProfileValue {
    fn from(v: Lastprofilwert) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.zeitpunkt,
            value: v.wert,
            unit: v.einheit.map(Into::into),
            status: v.status.map(Into::into),
            obis_code: v.obis_kennzahl,
            interval_minutes: v.intervalllaenge,
        }
    }
}
