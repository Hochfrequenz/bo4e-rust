//! Profile data (Profildaten) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Profile data for standard load profiles or individual consumption profiles.
///
/// German: Profildaten
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ProfileData;
/// use chrono::Utc;
///
/// let profile = ProfileData {
///     profile_type: Some("H0".to_string()),
///     timestamp: Some(Utc::now()),
///     value: Some(0.000125),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Profildaten"))]
#[serde(rename_all = "camelCase")]
pub struct ProfileData {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Profile type/identifier (Profiltyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "profiltyp"))]
    pub profile_type: Option<String>,

    /// Timestamp of the profile value (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitpunkt"))]
    pub timestamp: Option<DateTime<Utc>>,

    /// Profile value (Profilwert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "profilwert"))]
    pub value: Option<f64>,

    /// Unit of the profile value (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Profile name (Profilname)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "profilname"))]
    pub profile_name: Option<String>,

    /// Profile version (Profilversion)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "profilversion"))]
    pub profile_version: Option<String>,

    /// Temperature zone (Temperaturzone)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "temperaturzone"))]
    pub temperature_zone: Option<String>,
}

impl Bo4eObject for ProfileData {
    fn type_name_german() -> &'static str {
        "Profildaten"
    }

    fn type_name_english() -> &'static str {
        "ProfileData"
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
    fn test_profile_data() {
        let profile = ProfileData {
            profile_type: Some("H0".to_string()),
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 15, 0).unwrap()),
            value: Some(0.000125),
            profile_name: Some("Haushalt".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("H0"));
        assert!(json.contains("0.000125"));
    }

    #[test]
    fn test_business_profile() {
        let profile = ProfileData {
            profile_type: Some("G0".to_string()),
            profile_name: Some("Gewerbe allgemein".to_string()),
            profile_version: Some("2024-01".to_string()),
            temperature_zone: Some("TZ1".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("G0"));
        assert!(json.contains("TZ1"));
    }

    #[test]
    fn test_roundtrip() {
        let profile = ProfileData {
            profile_type: Some("L0".to_string()),
            value: Some(0.000089),
            ..Default::default()
        };

        let json = serde_json::to_string(&profile).unwrap();
        let parsed: ProfileData = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ProfileData::type_name_german(), "Profildaten");
        assert_eq!(ProfileData::type_name_english(), "ProfileData");
    }
}
