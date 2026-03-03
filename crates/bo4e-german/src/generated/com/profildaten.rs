#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profildaten {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "profileType")]
    pub profiltyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timestamp")]
    pub zeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub profilwert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "profileName")]
    pub profilname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "profileVersion")]
    pub profilversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "temperatureZone")]
    pub temperaturzone: Option<String>,
}
impl From<bo4e_core::com::ProfileData> for Profildaten {
    fn from(v: bo4e_core::com::ProfileData) -> Self {
        Self {
            meta: v.meta,
            profiltyp: v.profile_type,
            zeitpunkt: v.timestamp,
            profilwert: v.value,
            einheit: v.unit.map(Into::into),
            profilname: v.profile_name,
            profilversion: v.profile_version,
            temperaturzone: v.temperature_zone,
        }
    }
}
impl From<Profildaten> for bo4e_core::com::ProfileData {
    fn from(v: Profildaten) -> Self {
        Self {
            meta: v.meta,
            profile_type: v.profiltyp,
            timestamp: v.zeitpunkt,
            value: v.profilwert,
            unit: v.einheit.map(Into::into),
            profile_name: v.profilname,
            profile_version: v.profilversion,
            temperature_zone: v.temperaturzone,
        }
    }
}
