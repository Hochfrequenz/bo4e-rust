//! Contact method (Kontaktweg) component.

use serde::{Deserialize, Serialize};

use crate::enums::ContactType;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Method of contact (email, phone, etc.).
///
/// Used to represent contact information for a person within business objects.
///
/// German: Kontaktweg
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ContactMethod;
/// use bo4e_core::enums::ContactType;
///
/// let contact = ContactMethod {
///     contact_type: Some(ContactType::Email),
///     contact_value: Some("info@example.com".to_string()),
///     is_preferred: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kontaktweg"))]
#[serde(rename_all = "camelCase")]
pub struct ContactMethod {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of contact (Kontaktart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kontaktart"))]
    pub contact_type: Option<ContactType>,

    /// Contact value - phone number, email address, etc. (Kontaktwert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kontaktwert"))]
    pub contact_value: Option<String>,

    /// Description/specification, e.g., "direct line", "switchboard" (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Whether this is the preferred contact method (IstBevorzugterKontaktweg)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "istBevorzugterKontaktweg"))]
    pub is_preferred: Option<bool>,
}

impl Bo4eObject for ContactMethod {
    fn type_name_german() -> &'static str {
        "Kontaktweg"
    }

    fn type_name_english() -> &'static str {
        "ContactMethod"
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
    fn test_contact_method_default() {
        let contact = ContactMethod::default();
        assert!(contact.contact_type.is_none());
        assert!(contact.contact_value.is_none());
        assert!(contact.is_preferred.is_none());
    }

    #[test]
    fn test_email_contact() {
        let contact = ContactMethod {
            contact_type: Some(ContactType::Email),
            contact_value: Some("test@example.com".to_string()),
            is_preferred: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&contact).unwrap();
        assert!(json.contains(r#""contactType":"E_MAIL""#));
        assert!(json.contains(r#""contactValue":"test@example.com""#));
        assert!(json.contains(r#""isPreferred":true"#));

        let parsed: ContactMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(contact, parsed);
    }

    #[test]
    fn test_phone_contact() {
        let contact = ContactMethod {
            contact_type: Some(ContactType::Phone),
            contact_value: Some("+49 221 1234567".to_string()),
            description: Some("Direct line".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&contact).unwrap();
        assert!(json.contains(r#""contactType":"TELEFON""#));
        assert!(json.contains(r#""description":"Direct line""#));

        let parsed: ContactMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(contact, parsed);
    }

    #[test]
    fn test_contact_method_roundtrip() {
        let contact = ContactMethod {
            meta: Bo4eMeta::with_type("Kontaktweg"),
            contact_type: Some(ContactType::Fax),
            contact_value: Some("+49 221 1234568".to_string()),
            description: Some("Fax number".to_string()),
            is_preferred: Some(false),
        };

        let json = serde_json::to_string(&contact).unwrap();
        let parsed: ContactMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(contact, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ContactMethod::type_name_german(), "Kontaktweg");
        assert_eq!(ContactMethod::type_name_english(), "ContactMethod");
    }
}
