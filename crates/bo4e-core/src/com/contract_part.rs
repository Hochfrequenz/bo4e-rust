//! Contract part (Vertragsteil) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Part of a contract linking a service to a location.
///
/// Used to represent a contractual service in relation to a location
/// (market or metering location). Contracts for multiple locations are
/// modeled with multiple contract parts.
///
/// German: Vertragsteil
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ContractPart;
///
/// let part = ContractPart {
///     location_id: Some("DE0001234567890123456789012345678".to_string()),
///     contract_part_start: Some("2024-01-01T00:00:00+01:00".to_string()),
///     contract_part_end: Some("2024-12-31T23:59:59+01:00".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractPart {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Start of the contract part validity (inclusive) (Vertragsteilbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_part_start: Option<String>,

    /// End of the contract part validity (exclusive) (Vertragsteilende)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_part_end: Option<String>,

    /// Identifier for the market or metering location belonging to this contract part (Lokation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,

    // Note: The following fields would typically reference Menge COM type.
    // Using simplified f64 values for now.

    /// Contractually fixed consumption quantity (Vertraglich fixierte Menge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_quantity_value: Option<f64>,

    /// Minimum consumption quantity (inclusive) (Minimale Abnahmemenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity_value: Option<f64>,

    /// Maximum consumption quantity (exclusive) (Maximale Abnahmemenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity_value: Option<f64>,
}

impl Bo4eObject for ContractPart {
    fn type_name_german() -> &'static str {
        "Vertragsteil"
    }

    fn type_name_english() -> &'static str {
        "ContractPart"
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
    fn test_contract_part_default() {
        let part = ContractPart::default();
        assert!(part.location_id.is_none());
        assert!(part.contract_part_start.is_none());
    }

    #[test]
    fn test_contract_part_serialize() {
        let part = ContractPart {
            location_id: Some("DE00012345678901234567890123456".to_string()),
            contract_part_start: Some("2024-01-01T00:00:00+01:00".to_string()),
            contract_part_end: Some("2024-12-31T23:59:59+01:00".to_string()),
            fixed_quantity_value: Some(50000.0),
            ..Default::default()
        };

        let json = serde_json::to_string(&part).unwrap();
        assert!(json.contains(r#""locationId":"DE00012345678901234567890123456""#));
        assert!(json.contains(r#""contractPartStart":"#));
        assert!(json.contains(r#""fixedQuantityValue":50000"#));
    }

    #[test]
    fn test_contract_part_roundtrip() {
        let part = ContractPart {
            meta: Bo4eMeta::with_type("Vertragsteil"),
            contract_part_start: Some("2024-06-01T00:00:00+02:00".to_string()),
            contract_part_end: Some("2025-05-31T23:59:59+02:00".to_string()),
            location_id: Some("MALO-12345".to_string()),
            fixed_quantity_value: Some(100000.0),
            minimum_quantity_value: Some(80000.0),
            maximum_quantity_value: Some(120000.0),
        };

        let json = serde_json::to_string(&part).unwrap();
        let parsed: ContractPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ContractPart::type_name_german(), "Vertragsteil");
        assert_eq!(ContractPart::type_name_english(), "ContractPart");
    }
}
