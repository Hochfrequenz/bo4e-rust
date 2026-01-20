//! Metering point status (Messstellenstatus) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Status information for a metering point.
///
/// German: Messstellenstatus
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::MeteringPointStatus;
/// use chrono::Utc;
///
/// let status = MeteringPointStatus {
///     status_timestamp: Some(Utc::now()),
///     is_active: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Messstellenstatus"))]
#[serde(rename_all = "camelCase")]
pub struct MeteringPointStatus {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of the status (Statuszeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "statuszeitpunkt"))]
    pub status_timestamp: Option<DateTime<Utc>>,

    /// Whether the metering point is active (Aktiv)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aktiv"))]
    pub is_active: Option<bool>,

    /// Status code (Statuscode)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "statuscode"))]
    pub status_code: Option<String>,

    /// Status description (Statusbeschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "statusbeschreibung"))]
    pub status_description: Option<String>,

    /// Whether data is being transmitted (Datenübertragung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "datenuebertragung"))]
    pub data_transmission_active: Option<bool>,

    /// Installation status (Installationsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "installationsstatus"))]
    pub installation_status: Option<String>,
}

impl Bo4eObject for MeteringPointStatus {
    fn type_name_german() -> &'static str {
        "Messstellenstatus"
    }

    fn type_name_english() -> &'static str {
        "MeteringPointStatus"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_metering_point_status() {
        let status = MeteringPointStatus {
            status_timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap()),
            is_active: Some(true),
            status_code: Some("ACTIVE".to_string()),
            data_transmission_active: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("ACTIVE"));
    }

    #[test]
    fn test_roundtrip() {
        let status = MeteringPointStatus {
            status_timestamp: Some(Utc::now()),
            is_active: Some(false),
            status_description: Some("Meter removed".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&status).unwrap();
        let parsed: MeteringPointStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status.is_active, parsed.is_active);
        assert_eq!(status.status_description, parsed.status_description);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MeteringPointStatus::type_name_german(), "Messstellenstatus");
        assert_eq!(
            MeteringPointStatus::type_name_english(),
            "MeteringPointStatus"
        );
    }
}
