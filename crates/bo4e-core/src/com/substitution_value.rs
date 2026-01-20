//! Substitution value (Ersatzwert) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A substituted/replacement value for missing or invalid measurements.
///
/// German: Ersatzwert
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::SubstitutionValue;
/// use bo4e_core::enums::Unit;
/// use chrono::Utc;
///
/// let value = SubstitutionValue {
///     timestamp: Some(Utc::now()),
///     value: Some(125.5),
///     unit: Some(Unit::KilowattHour),
///     substitution_method: Some("Interpolation".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Ersatzwert"))]
#[serde(rename_all = "camelCase")]
pub struct SubstitutionValue {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp for the substituted value (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitpunkt"))]
    pub timestamp: Option<DateTime<Utc>>,

    /// The substituted value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Unit of the value (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,

    /// Method used for substitution (Ersatzwertmethode)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ersatzwertmethode"))]
    pub substitution_method: Option<String>,

    /// Reason for substitution (Grund)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "grund"))]
    pub reason: Option<String>,

    /// Original value that was replaced, if available (Originalwert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "originalwert"))]
    pub original_value: Option<f64>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "obisKennzahl"))]
    pub obis_code: Option<String>,
}

impl Bo4eObject for SubstitutionValue {
    fn type_name_german() -> &'static str {
        "Ersatzwert"
    }

    fn type_name_english() -> &'static str {
        "SubstitutionValue"
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
    fn test_substitution_value() {
        let value = SubstitutionValue {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap()),
            value: Some(125.5),
            unit: Some(Unit::KilowattHour),
            substitution_method: Some("Interpolation".to_string()),
            reason: Some("Meter communication failure".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("125.5"));
        assert!(json.contains("Interpolation"));
    }

    #[test]
    fn test_with_original() {
        let value = SubstitutionValue {
            value: Some(100.0),
            original_value: Some(-50.0),
            reason: Some("Negative value not allowed".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("-50"));
    }

    #[test]
    fn test_roundtrip() {
        let value = SubstitutionValue {
            timestamp: Some(Utc::now()),
            value: Some(999.99),
            substitution_method: Some("Historical average".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        let parsed: SubstitutionValue = serde_json::from_str(&json).unwrap();
        assert_eq!(value.value, parsed.value);
        assert_eq!(value.substitution_method, parsed.substitution_method);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(SubstitutionValue::type_name_german(), "Ersatzwert");
        assert_eq!(SubstitutionValue::type_name_english(), "SubstitutionValue");
    }
}
