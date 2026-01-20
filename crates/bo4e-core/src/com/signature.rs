//! Signature (Unterschrift) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Digital signature for contracts, offers, etc.
///
/// German: Unterschrift
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Signature;
///
/// let signature = Signature {
///     name: Some("Max Mustermann".to_string()),
///     location: Some("Köln".to_string()),
///     date: Some("2024-01-15T10:30:00+01:00".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Unterschrift"))]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name of the signatory (Name)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "name"))]
    pub name: Option<String>,

    /// Location where the signature was made (Ort)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ort"))]
    pub location: Option<String>,

    /// Date of the signature (Datum) - ISO 8601 datetime string
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "datum"))]
    pub date: Option<String>,
}

impl Bo4eObject for Signature {
    fn type_name_german() -> &'static str {
        "Unterschrift"
    }

    fn type_name_english() -> &'static str {
        "Signature"
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
    fn test_signature_default() {
        let sig = Signature::default();
        assert!(sig.name.is_none());
        assert!(sig.location.is_none());
        assert!(sig.date.is_none());
    }

    #[test]
    fn test_signature_serialize() {
        let sig = Signature {
            name: Some("Hans Schmidt".to_string()),
            location: Some("Berlin".to_string()),
            date: Some("2024-06-15T14:00:00+02:00".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&sig).unwrap();
        assert!(json.contains(r#""name":"Hans Schmidt""#));
        assert!(json.contains(r#""location":"Berlin""#));
        assert!(json.contains(r#""date":"2024-06-15T14:00:00+02:00""#));
    }

    #[test]
    fn test_signature_roundtrip() {
        let sig = Signature {
            meta: Bo4eMeta::with_type("Unterschrift"),
            name: Some("Erika Musterfrau".to_string()),
            location: Some("München".to_string()),
            date: Some("2024-03-20T09:15:00+01:00".to_string()),
        };

        let json = serde_json::to_string(&sig).unwrap();
        let parsed: Signature = serde_json::from_str(&json).unwrap();
        assert_eq!(sig, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Signature::type_name_german(), "Unterschrift");
        assert_eq!(Signature::type_name_english(), "Signature");
    }
}
