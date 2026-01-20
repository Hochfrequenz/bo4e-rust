//! Device category (Geraeteklasse) enumeration.

use serde::{Deserialize, Serialize};

/// Category/class of device.
///
/// Lists possible overarching device categories.
///
/// German: Geraeteklasse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeviceCategory {
    /// Transformer/converter (Wandler)
    #[serde(rename = "WANDLER")]
    Transformer,

    /// Communication equipment (Kommunikationseinrichtung)
    #[serde(rename = "KOMMUNIKATIONSEINRICHTUNG")]
    CommunicationEquipment,

    /// Technical control equipment (Technische Steuereinrichtung)
    #[serde(rename = "TECHNISCHE_STEUEREINRICHTUNG")]
    TechnicalControlEquipment,

    /// Volume converter (Mengenumwerter)
    #[serde(rename = "MENGENUMWERTER")]
    VolumeConverter,

    /// Smart meter gateway
    #[serde(rename = "SMARTMETER_GATEWAY")]
    SmartMeterGateway,

    /// Control box (Steuerbox)
    #[serde(rename = "STEUERBOX")]
    ControlBox,

    /// Metering device (Zähleinrichtung)
    #[serde(rename = "ZAEHLEINRICHTUNG")]
    MeteringDevice,
}

impl DeviceCategory {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Transformer => "Wandler",
            Self::CommunicationEquipment => "Kommunikationseinrichtung",
            Self::TechnicalControlEquipment => "Technische Steuereinrichtung",
            Self::VolumeConverter => "Mengenumwerter",
            Self::SmartMeterGateway => "Smartmeter-Gateway",
            Self::ControlBox => "Steuerbox",
            Self::MeteringDevice => "Zähleinrichtung",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&DeviceCategory::SmartMeterGateway).unwrap(),
            r#""SMARTMETER_GATEWAY""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for cat in [
            DeviceCategory::Transformer,
            DeviceCategory::CommunicationEquipment,
            DeviceCategory::TechnicalControlEquipment,
            DeviceCategory::VolumeConverter,
            DeviceCategory::SmartMeterGateway,
            DeviceCategory::ControlBox,
            DeviceCategory::MeteringDevice,
        ] {
            let json = serde_json::to_string(&cat).unwrap();
            let parsed: DeviceCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(cat, parsed);
        }
    }
}
