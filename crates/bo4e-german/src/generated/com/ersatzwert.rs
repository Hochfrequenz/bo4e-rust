#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ersatzwert {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timestamp")]
    pub zeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "substitutionMethod")]
    pub ersatzwertmethode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "reason")]
    pub grund: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "originalValue")]
    pub originalwert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
}
impl From<bo4e_core::com::SubstitutionValue> for Ersatzwert {
    fn from(v: bo4e_core::com::SubstitutionValue) -> Self {
        Self {
            meta: v.meta,
            zeitpunkt: v.timestamp,
            wert: v.value,
            einheit: v.unit.map(Into::into),
            ersatzwertmethode: v.substitution_method,
            grund: v.reason,
            originalwert: v.original_value,
            obis_kennzahl: v.obis_code,
        }
    }
}
impl From<Ersatzwert> for bo4e_core::com::SubstitutionValue {
    fn from(v: Ersatzwert) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.zeitpunkt,
            value: v.wert,
            unit: v.einheit.map(Into::into),
            substitution_method: v.ersatzwertmethode,
            reason: v.grund,
            original_value: v.originalwert,
            obis_code: v.obis_kennzahl,
        }
    }
}
