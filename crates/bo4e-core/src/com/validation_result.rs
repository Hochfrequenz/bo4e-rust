//! Validation result (Validierungsergebnis) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Result of a validation check on measured data.
///
/// German: Validierungsergebnis
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ValidationResult;
/// use chrono::Utc;
///
/// let result = ValidationResult {
///     validation_timestamp: Some(Utc::now()),
///     is_valid: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of validation (Validierungszeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_timestamp: Option<DateTime<Utc>>,

    /// Whether validation passed (Gültig)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,

    /// Validation rule ID (Validierungsregel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_rule_id: Option<String>,

    /// Validation rule name (Regelbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_rule_name: Option<String>,

    /// Error code if validation failed (Fehlercode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,

    /// Error message (Fehlermeldung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// Severity level (Schweregrad)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl Bo4eObject for ValidationResult {
    fn type_name_german() -> &'static str {
        "Validierungsergebnis"
    }

    fn type_name_english() -> &'static str {
        "ValidationResult"
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
    fn test_validation_passed() {
        let result = ValidationResult {
            validation_timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap()),
            is_valid: Some(true),
            validation_rule_id: Some("RULE_001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("RULE_001"));
    }

    #[test]
    fn test_validation_failed() {
        let result = ValidationResult {
            validation_timestamp: Some(Utc::now()),
            is_valid: Some(false),
            error_code: Some("E001".to_string()),
            error_message: Some("Value out of range".to_string()),
            severity: Some("ERROR".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("E001"));
    }

    #[test]
    fn test_roundtrip() {
        let result = ValidationResult {
            is_valid: Some(true),
            validation_rule_name: Some("Range check".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&result).unwrap();
        let parsed: ValidationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ValidationResult::type_name_german(), "Validierungsergebnis");
        assert_eq!(ValidationResult::type_name_english(), "ValidationResult");
    }
}
