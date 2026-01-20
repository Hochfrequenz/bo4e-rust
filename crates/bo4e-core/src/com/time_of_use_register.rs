//! Time-of-use register (Zaehlzeitregister) component.

use serde::{Deserialize, Serialize};

use crate::enums::{TariffTime, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A time-of-use register on a meter for different tariff periods.
///
/// German: Zaehlzeitregister
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TimeOfUseRegister;
/// use bo4e_core::enums::{TariffTime, Unit};
///
/// let register = TimeOfUseRegister {
///     register_id: Some("HT".to_string()),
///     tariff_time: Some(TariffTime::HighTariff),
///     unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehlzeitregister"))]
#[serde(rename_all = "camelCase")]
pub struct TimeOfUseRegister {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Register ID (Zaehlwerkskennung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehlwerkskennung"))]
    pub register_id: Option<String>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,

    /// Tariff time period (Tarifzeit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifzeit"))]
    pub tariff_time: Option<TariffTime>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Description of the register (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub description: Option<String>,

    /// Active start time in HH:MM format (Aktivzeitbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aktivzeitbeginn"))]
    pub active_start_time: Option<String>,

    /// Active end time in HH:MM format (Aktivzeitende)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aktivzeitende"))]
    pub active_end_time: Option<String>,
}

impl Bo4eObject for TimeOfUseRegister {
    fn type_name_german() -> &'static str {
        "Zaehlzeitregister"
    }

    fn type_name_english() -> &'static str {
        "TimeOfUseRegister"
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
    fn test_high_tariff_register() {
        let register = TimeOfUseRegister {
            register_id: Some("HT".to_string()),
            tariff_time: Some(TariffTime::HighTariff),
            unit: Some(Unit::KilowattHour),
            active_start_time: Some("06:00".to_string()),
            active_end_time: Some("22:00".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&register).unwrap();
        assert!(json.contains("HT"));
    }

    #[test]
    fn test_roundtrip() {
        let register = TimeOfUseRegister {
            register_id: Some("NT".to_string()),
            tariff_time: Some(TariffTime::LowTariff),
            ..Default::default()
        };

        let json = serde_json::to_string(&register).unwrap();
        let parsed: TimeOfUseRegister = serde_json::from_str(&json).unwrap();
        assert_eq!(register, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TimeOfUseRegister::type_name_german(), "Zaehlzeitregister");
        assert_eq!(TimeOfUseRegister::type_name_english(), "TimeOfUseRegister");
    }
}
