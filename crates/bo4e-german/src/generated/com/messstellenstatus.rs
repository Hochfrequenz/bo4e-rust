#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Messstellenstatus {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "statusTimestamp")]
    pub statuszeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isActive")]
    pub aktiv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "statusCode")]
    pub statuscode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "statusDescription")]
    pub statusbeschreibung: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "dataTransmissionActive"
    )]
    pub datenuebertragung: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "installationStatus")]
    pub installationsstatus: Option<String>,
}
impl From<bo4e_core::com::MeteringPointStatus> for Messstellenstatus {
    fn from(v: bo4e_core::com::MeteringPointStatus) -> Self {
        Self {
            meta: v.meta,
            statuszeitpunkt: v.status_timestamp,
            aktiv: v.is_active,
            statuscode: v.status_code,
            statusbeschreibung: v.status_description,
            datenuebertragung: v.data_transmission_active,
            installationsstatus: v.installation_status,
        }
    }
}
impl From<Messstellenstatus> for bo4e_core::com::MeteringPointStatus {
    fn from(v: Messstellenstatus) -> Self {
        Self {
            meta: v.meta,
            status_timestamp: v.statuszeitpunkt,
            is_active: v.aktiv,
            status_code: v.statuscode,
            status_description: v.statusbeschreibung,
            data_transmission_active: v.datenuebertragung,
            installation_status: v.installationsstatus,
        }
    }
}
