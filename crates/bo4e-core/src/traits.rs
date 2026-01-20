//! Core traits and types for BO4E objects.

use crate::AdditionalAttribute;
use serde::{Deserialize, Serialize};

/// Metadata common to all BO4E objects.
///
/// This struct holds the standard BO4E metadata fields:
/// - `_typ`: Type discriminator
/// - `_version`: BO4E schema version
/// - `_id`: External system ID
/// - `zusatzAttribute`: Additional attributes for extensibility
///
/// # Example
///
/// ```rust
/// use bo4e_core::Bo4eMeta;
///
/// let meta = Bo4eMeta {
///     typ: Some("Zaehler".to_string()),
///     version: Some("202401.0.1".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Bo4eMeta {
    /// Type discriminator (maps to `_typ` in JSON)
    #[serde(rename = "_typ", skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,

    /// BO4E schema version (maps to `_version` in JSON)
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// External system ID (maps to `_id` in JSON)
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Additional attributes for extensibility
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zusatz_attribute: Vec<AdditionalAttribute>,
}

impl Bo4eMeta {
    /// Create metadata with type name.
    pub fn with_type(typ: impl Into<String>) -> Self {
        Self {
            typ: Some(typ.into()),
            ..Default::default()
        }
    }

    /// Set the version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the external ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add an additional attribute.
    pub fn with_attribute(mut self, attr: AdditionalAttribute) -> Self {
        self.zusatz_attribute.push(attr);
        self
    }
}

/// Trait implemented by all BO4E types.
///
/// This trait provides a common interface for accessing type metadata
/// and enables generic programming over BO4E types.
pub trait Bo4eObject {
    /// Returns the German type name as used in the `_typ` field.
    ///
    /// Example: `"Zaehler"` for Meter, `"Marktlokation"` for MarketLocation
    fn type_name_german() -> &'static str;

    /// Returns the English type name.
    ///
    /// Example: `"Meter"`, `"MarketLocation"`
    fn type_name_english() -> &'static str;

    /// Returns a reference to the metadata.
    fn meta(&self) -> &Bo4eMeta;

    /// Returns a mutable reference to the metadata.
    fn meta_mut(&mut self) -> &mut Bo4eMeta;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_default() {
        let meta = Bo4eMeta::default();
        assert!(meta.typ.is_none());
        assert!(meta.version.is_none());
        assert!(meta.id.is_none());
        assert!(meta.zusatz_attribute.is_empty());
    }

    #[test]
    fn test_meta_builder() {
        let meta = Bo4eMeta::with_type("Zaehler")
            .version("202401.0.1")
            .id("ext-123");

        assert_eq!(meta.typ, Some("Zaehler".to_string()));
        assert_eq!(meta.version, Some("202401.0.1".to_string()));
        assert_eq!(meta.id, Some("ext-123".to_string()));
    }

    #[test]
    fn test_meta_serialize() {
        let meta = Bo4eMeta::with_type("Zaehler").version("202401.0.1");
        let json = serde_json::to_string(&meta).unwrap();

        assert!(json.contains(r#""_typ":"Zaehler""#));
        assert!(json.contains(r#""_version":"202401.0.1""#));
        assert!(!json.contains("zusatzAttribute")); // Empty vec skipped
    }

    #[test]
    fn test_meta_deserialize() {
        let json = r#"{"_typ":"Zaehler","_version":"202401.0.1","_id":"123"}"#;
        let meta: Bo4eMeta = serde_json::from_str(json).unwrap();

        assert_eq!(meta.typ, Some("Zaehler".to_string()));
        assert_eq!(meta.version, Some("202401.0.1".to_string()));
        assert_eq!(meta.id, Some("123".to_string()));
    }

    #[test]
    fn test_meta_with_zusatz_attribute() {
        let meta = Bo4eMeta::with_type("Zaehler")
            .with_attribute(crate::AdditionalAttribute::string("sap_id", "SAP001"));

        assert_eq!(meta.zusatz_attribute.len(), 1);
        assert_eq!(meta.zusatz_attribute[0].name, "sap_id");
    }
}
