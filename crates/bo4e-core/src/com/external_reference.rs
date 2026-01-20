//! External reference (ExterneReferenz) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Reference to an external system.
///
/// Used to link BO4E objects to identifiers in other systems (e.g., SAP, CRM).
///
/// German: ExterneReferenz
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ExternalReference;
///
/// let ext_ref = ExternalReference::new("SAP", "4711");
/// assert_eq!(ext_ref.external_ref_name, Some("SAP".to_string()));
/// assert_eq!(ext_ref.external_ref_value, Some("4711".to_string()));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "ExterneReferenz"))]
#[serde(rename_all = "camelCase")]
pub struct ExternalReference {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name of the external system (ExRefName)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "exRefName"))]
    pub external_ref_name: Option<String>,

    /// Value/ID in the external system (ExRefWert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "exRefWert"))]
    pub external_ref_value: Option<String>,
}

impl Bo4eObject for ExternalReference {
    fn type_name_german() -> &'static str {
        "ExterneReferenz"
    }

    fn type_name_english() -> &'static str {
        "ExternalReference"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl ExternalReference {
    /// Create a new external reference.
    ///
    /// # Arguments
    ///
    /// * `system` - Name of the external system
    /// * `value` - The identifier/value in that system
    pub fn new(system: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            external_ref_name: Some(system.into()),
            external_ref_value: Some(value.into()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_reference_default() {
        let ext_ref = ExternalReference::default();
        assert!(ext_ref.external_ref_name.is_none());
        assert!(ext_ref.external_ref_value.is_none());
    }

    #[test]
    fn test_sap_reference() {
        let ext_ref = ExternalReference::new("SAP", "4711");
        assert_eq!(ext_ref.external_ref_name, Some("SAP".to_string()));
        assert_eq!(ext_ref.external_ref_value, Some("4711".to_string()));
    }

    #[test]
    fn test_serialize() {
        let ext_ref = ExternalReference::new("CRM", "CUST-12345");
        let json = serde_json::to_string(&ext_ref).unwrap();
        assert!(json.contains(r#""externalRefName":"CRM""#));
        assert!(json.contains(r#""externalRefValue":"CUST-12345""#));
    }

    #[test]
    fn test_external_reference_roundtrip() {
        let ext_ref = ExternalReference {
            meta: Bo4eMeta::with_type("ExterneReferenz"),
            external_ref_name: Some("ERP".to_string()),
            external_ref_value: Some("ORDER-2024-001".to_string()),
        };

        let json = serde_json::to_string(&ext_ref).unwrap();
        let parsed: ExternalReference = serde_json::from_str(&json).unwrap();
        assert_eq!(ext_ref, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ExternalReference::type_name_german(), "ExterneReferenz");
        assert_eq!(ExternalReference::type_name_english(), "ExternalReference");
    }
}
