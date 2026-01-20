//! Additional attributes for extensibility.

use serde::{Deserialize, Serialize};

/// Value type for additional attributes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    /// String value
    String(String),
    /// Numeric value (integer or float)
    Number(f64),
    /// Boolean value
    Boolean(bool),
    /// Nested object with more attributes
    Object(Vec<AdditionalAttribute>),
    /// Array of values
    Array(Vec<AttributeValue>),
    /// Null value
    Null,
}

/// Additional attribute for external system IDs and custom metadata.
///
/// This enables interoperability with external systems that need to attach
/// their own identifiers or metadata to BO4E objects.
///
/// # Example
///
/// ```rust
/// use bo4e_core::AdditionalAttribute;
/// use bo4e_core::additional_attribute::AttributeValue;
///
/// let attr = AdditionalAttribute {
///     name: "sap_id".to_string(),
///     value: Some(AttributeValue::String("SAP123".to_string())),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalAttribute {
    /// Name/key of the attribute
    pub name: String,
    /// Value of the attribute (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

impl AdditionalAttribute {
    /// Create a new additional attribute with a string value.
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(AttributeValue::String(value.into())),
        }
    }

    /// Create a new additional attribute with a numeric value.
    pub fn number(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value: Some(AttributeValue::Number(value)),
        }
    }

    /// Create a new additional attribute with a boolean value.
    pub fn boolean(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            value: Some(AttributeValue::Boolean(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_attribute() {
        let attr = AdditionalAttribute::string("system_id", "ABC123");
        assert_eq!(attr.name, "system_id");
        assert_eq!(
            attr.value,
            Some(AttributeValue::String("ABC123".to_string()))
        );
    }

    #[test]
    fn test_number_attribute() {
        let attr = AdditionalAttribute::number("priority", 42.0);
        assert_eq!(attr.name, "priority");
        assert_eq!(attr.value, Some(AttributeValue::Number(42.0)));
    }

    #[test]
    fn test_serialize_attribute() {
        let attr = AdditionalAttribute::string("key", "value");
        let json = serde_json::to_string(&attr).unwrap();
        assert_eq!(json, r#"{"name":"key","value":"value"}"#);
    }

    #[test]
    fn test_deserialize_attribute() {
        let json = r#"{"name":"key","value":"value"}"#;
        let attr: AdditionalAttribute = serde_json::from_str(json).unwrap();
        assert_eq!(attr.name, "key");
        assert_eq!(
            attr.value,
            Some(AttributeValue::String("value".to_string()))
        );
    }

    #[test]
    fn test_deserialize_nested_attribute() {
        let json = r#"{"name":"parent","value":[{"name":"child","value":123}]}"#;
        let attr: AdditionalAttribute = serde_json::from_str(json).unwrap();
        assert_eq!(attr.name, "parent");
        match attr.value {
            Some(AttributeValue::Object(children)) => {
                assert_eq!(children.len(), 1);
                assert_eq!(children[0].name, "child");
            }
            _ => panic!("Expected nested object"),
        }
    }
}
