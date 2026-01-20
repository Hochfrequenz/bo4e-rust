//! Hardware component.

use serde::{Deserialize, Serialize};

use crate::enums::{DeviceCategory, DeviceType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Hardware component for representing physical devices.
///
/// This component represents hardware devices that are not meters,
/// such as transformers, communication equipment, or other technical devices.
///
/// German: Hardware
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Hardware;
///
/// let hw = Hardware {
///     device_number: Some("HW-2024-001".to_string()),
///     description: Some("Current transformer".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Hardware"))]
#[serde(rename_all = "camelCase")]
pub struct Hardware {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Device number assigned by the metering service operator (Gerätenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geraetenummer"))]
    pub device_number: Option<String>,

    /// Description of the device (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub description: Option<String>,

    /// Category/class of the device (Geräteklasse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geraeteklasse"))]
    pub device_category: Option<DeviceCategory>,

    /// Specific type of the device (Gerätetyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geraetetyp"))]
    pub device_type: Option<DeviceType>,
}

impl Bo4eObject for Hardware {
    fn type_name_german() -> &'static str {
        "Hardware"
    }

    fn type_name_english() -> &'static str {
        "Hardware"
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

    #[test]
    fn test_hardware_default() {
        let hw = Hardware::default();
        assert!(hw.device_number.is_none());
        assert!(hw.description.is_none());
    }

    #[test]
    fn test_hardware_serialize() {
        let hw = Hardware {
            device_number: Some("HW-001".to_string()),
            description: Some("Transformer unit".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&hw).unwrap();
        assert!(json.contains(r#""deviceNumber":"HW-001""#));
        assert!(json.contains(r#""description":"Transformer unit""#));
    }

    #[test]
    fn test_hardware_roundtrip() {
        let hw = Hardware {
            meta: Bo4eMeta::with_type("Hardware"),
            device_number: Some("HW-2024-002".to_string()),
            description: Some("Communication gateway".to_string()),
            device_category: None,
            device_type: None,
        };

        let json = serde_json::to_string(&hw).unwrap();
        let parsed: Hardware = serde_json::from_str(&json).unwrap();
        assert_eq!(hw, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Hardware::type_name_german(), "Hardware");
        assert_eq!(Hardware::type_name_english(), "Hardware");
    }
}
