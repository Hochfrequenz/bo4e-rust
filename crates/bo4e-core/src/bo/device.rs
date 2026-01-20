//! Device (Geraet) business object.
//!
//! Represents a technical device in the energy infrastructure.

use serde::{Deserialize, Serialize};

use crate::enums::{DeviceCategory, DeviceType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A technical device used in the energy infrastructure.
///
/// German: Geraet
///
/// Devices are technical equipment like transformers, switches,
/// or other equipment in the energy network.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Device;
/// use bo4e_core::enums::DeviceCategory;
///
/// let device = Device {
///     device_id: Some("DEV001".to_string()),
///     device_category: Some(DeviceCategory::MeteringDevice),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Device identification (Geraetkennung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    /// Serial number (Seriennummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// Device category (Geraeteklasse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_category: Option<DeviceCategory>,

    /// Device type (Geraetetyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<DeviceType>,

    /// Manufacturer (Hersteller)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,

    /// Model name (Modellbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Manufacturing year (Baujahr)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturing_year: Option<i32>,

    /// Installation date (Einbaudatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Removal date (Ausbaudatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removal_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Firmware version (Firmware-Version)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Associated metering location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_location_id: Option<String>,
}

impl Bo4eObject for Device {
    fn type_name_german() -> &'static str {
        "Geraet"
    }

    fn type_name_english() -> &'static str {
        "Device"
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
    fn test_device_creation() {
        let device = Device {
            device_id: Some("DEV001".to_string()),
            device_category: Some(DeviceCategory::MeteringDevice),
            ..Default::default()
        };

        assert_eq!(device.device_id, Some("DEV001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let device = Device {
            meta: Bo4eMeta::with_type("Geraet"),
            device_id: Some("DEV001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&device).unwrap();
        assert!(json.contains(r#""_typ":"Geraet""#));
    }

    #[test]
    fn test_roundtrip() {
        let device = Device {
            meta: Bo4eMeta::with_type("Geraet"),
            device_id: Some("DEV001".to_string()),
            serial_number: Some("SN123456".to_string()),
            device_category: Some(DeviceCategory::MeteringDevice),
            manufacturer: Some("Acme Corp".to_string()),
            manufacturing_year: Some(2023),
            ..Default::default()
        };

        let json = serde_json::to_string(&device).unwrap();
        let parsed: Device = serde_json::from_str(&json).unwrap();
        assert_eq!(device, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Device::type_name_german(), "Geraet");
        assert_eq!(Device::type_name_english(), "Device");
    }
}
