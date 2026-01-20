//! Contract conditions (Vertragskonditionen) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Contract conditions for contracts and tariffs.
///
/// German: Vertragskonditionen
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ContractConditions;
///
/// let conditions = ContractConditions {
///     description: Some("Standard conditions for gas supply".to_string()),
///     installment_count: Some(12),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractConditions {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Free text description of conditions (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Number of agreed installments per year, e.g., 12 (AnzahlAbschlaege)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_count: Option<i32>,

    // Note: The following fields would typically reference Zeitraum COM type.
    // Using simplified string representations for now.

    /// Contract duration (Vertragslaufzeit) - ISO 8601 duration or description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_duration: Option<String>,

    /// Notice period for termination (Kündigungsfrist)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_period: Option<String>,

    /// Automatic extension period if not terminated (Vertragsverlängerung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_period: Option<String>,

    /// Installment cycle (Abschlagszyklus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_cycle: Option<String>,
}

impl Bo4eObject for ContractConditions {
    fn type_name_german() -> &'static str {
        "Vertragskonditionen"
    }

    fn type_name_english() -> &'static str {
        "ContractConditions"
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
    fn test_contract_conditions_default() {
        let conditions = ContractConditions::default();
        assert!(conditions.description.is_none());
        assert!(conditions.installment_count.is_none());
    }

    #[test]
    fn test_contract_conditions_serialize() {
        let conditions = ContractConditions {
            description: Some("Standard electricity conditions".to_string()),
            installment_count: Some(12),
            contract_duration: Some("P12M".to_string()),
            notice_period: Some("P6W".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&conditions).unwrap();
        assert!(json.contains(r#""description":"Standard electricity conditions""#));
        assert!(json.contains(r#""installmentCount":12"#));
        assert!(json.contains(r#""contractDuration":"P12M""#));
    }

    #[test]
    fn test_contract_conditions_roundtrip() {
        let conditions = ContractConditions {
            meta: Bo4eMeta::with_type("Vertragskonditionen"),
            description: Some("Premium gas tariff conditions".to_string()),
            installment_count: Some(6),
            contract_duration: Some("P24M".to_string()),
            notice_period: Some("P3M".to_string()),
            extension_period: Some("P12M".to_string()),
            installment_cycle: Some("P2M".to_string()),
        };

        let json = serde_json::to_string(&conditions).unwrap();
        let parsed: ContractConditions = serde_json::from_str(&json).unwrap();
        assert_eq!(conditions, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ContractConditions::type_name_german(), "Vertragskonditionen");
        assert_eq!(
            ContractConditions::type_name_english(),
            "ContractConditions"
        );
    }
}
