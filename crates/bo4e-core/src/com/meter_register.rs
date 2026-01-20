//! Meter register (Zaehlwerk) component.

use serde::{Deserialize, Serialize};

use crate::enums::{EnergyDirection, RegisterType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A register on a meter that records consumption.
///
/// German: Zaehlwerk
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::MeterRegister;
/// use bo4e_core::enums::{EnergyDirection, Unit};
///
/// let register = MeterRegister {
///     obis_code: Some("1-0:1.8.0".to_string()),
///     energy_direction: Some(EnergyDirection::FeedOut),
///     unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehlwerk"))]
#[serde(rename_all = "camelCase")]
pub struct MeterRegister {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Register number/ID (Zaehlwerkskennung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehlwerkskennung"))]
    pub register_id: Option<String>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,

    /// Type of register (Registerart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "registerart"))]
    pub register_type: Option<RegisterType>,

    /// Direction of energy flow (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energierichtung"))]
    pub energy_direction: Option<EnergyDirection>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Number of decimal places (Nachkommastellen)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "nachkommastellen"))]
    pub decimal_places: Option<i32>,

    /// Multiplier/transformer ratio (Wandlerfaktor)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wandlerfaktor"))]
    pub transformer_ratio: Option<f64>,

    /// Description (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub description: Option<String>,
}

impl Bo4eObject for MeterRegister {
    fn type_name_german() -> &'static str {
        "Zaehlwerk"
    }

    fn type_name_english() -> &'static str {
        "MeterRegister"
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
    fn test_electricity_register() {
        let register = MeterRegister {
            obis_code: Some("1-0:1.8.0".to_string()),
            energy_direction: Some(EnergyDirection::FeedOut),
            unit: Some(Unit::KilowattHour),
            description: Some("Bezug".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&register).unwrap();
        assert!(json.contains("1-0:1.8.0"));
    }

    #[test]
    fn test_roundtrip() {
        let register = MeterRegister {
            obis_code: Some("1-0:2.8.0".to_string()),
            energy_direction: Some(EnergyDirection::FeedIn),
            unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        let json = serde_json::to_string(&register).unwrap();
        let parsed: MeterRegister = serde_json::from_str(&json).unwrap();
        assert_eq!(register, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MeterRegister::type_name_german(), "Zaehlwerk");
        assert_eq!(MeterRegister::type_name_english(), "MeterRegister");
    }
}
