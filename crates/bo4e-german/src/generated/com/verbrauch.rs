#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verbrauch {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startDate")]
    pub startdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endDate")]
    pub enddatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "measuredValueStatus")]
    pub messwertstatus: Option<crate::Messwertstatus>,
}
impl From<bo4e_core::com::Consumption> for Verbrauch {
    fn from(v: bo4e_core::com::Consumption) -> Self {
        Self {
            meta: v.meta,
            wert: v.value,
            einheit: v.unit.map(Into::into),
            startdatum: v.start_date,
            enddatum: v.end_date,
            obis_kennzahl: v.obis_code,
            messwertstatus: v.measured_value_status.map(Into::into),
        }
    }
}
impl From<Verbrauch> for bo4e_core::com::Consumption {
    fn from(v: Verbrauch) -> Self {
        Self {
            meta: v.meta,
            value: v.wert,
            unit: v.einheit.map(Into::into),
            start_date: v.startdatum,
            end_date: v.enddatum,
            obis_code: v.obis_kennzahl,
            measured_value_status: v.messwertstatus.map(Into::into),
        }
    }
}
