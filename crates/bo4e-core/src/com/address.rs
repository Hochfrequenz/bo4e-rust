//! Address (Adresse) component.

use serde::{Deserialize, Serialize};

use crate::enums::Country;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Physical or postal address.
///
/// German: Adresse
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Address;
///
/// let address = Address {
///     street: Some("Musterstraße".to_string()),
///     house_number: Some("42".to_string()),
///     postal_code: Some("50667".to_string()),
///     city: Some("Köln".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Street name (Strasse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    /// House number (Hausnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,

    /// Postal code (Postleitzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// City/town (Ort)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// District (Ortsteil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,

    /// PO Box number (Postfach)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub po_box: Option<String>,

    /// Address addition/note (Adresszusatz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_addition: Option<String>,

    /// Co-location info - c/o address (CoErgaenzung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co_ergaenzung: Option<String>,

    /// Country code (Landescode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Country>,
}

impl Bo4eObject for Address {
    fn type_name_german() -> &'static str {
        "Adresse"
    }

    fn type_name_english() -> &'static str {
        "Address"
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
    fn test_address_default() {
        let address = Address::default();
        assert!(address.street.is_none());
        assert!(address.city.is_none());
    }

    #[test]
    fn test_address_serialize() {
        let address = Address {
            street: Some("Hauptstraße".to_string()),
            house_number: Some("1".to_string()),
            postal_code: Some("12345".to_string()),
            city: Some("Berlin".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains(r#""street":"Hauptstraße""#));
        assert!(json.contains(r#""houseNumber":"1""#));
        assert!(json.contains(r#""postalCode":"12345""#));
        assert!(json.contains(r#""city":"Berlin""#));
    }

    #[test]
    fn test_address_deserialize() {
        let json = r#"{
            "street": "Musterweg",
            "houseNumber": "42a",
            "postalCode": "50667",
            "city": "Köln"
        }"#;

        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(address.street, Some("Musterweg".to_string()));
        assert_eq!(address.house_number, Some("42a".to_string()));
        assert_eq!(address.postal_code, Some("50667".to_string()));
        assert_eq!(address.city, Some("Köln".to_string()));
    }

    #[test]
    fn test_address_roundtrip() {
        let address = Address {
            meta: Bo4eMeta::with_type("Adresse"),
            street: Some("Teststraße".to_string()),
            house_number: Some("123".to_string()),
            postal_code: Some("99999".to_string()),
            city: Some("Teststadt".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&address).unwrap();
        let parsed: Address = serde_json::from_str(&json).unwrap();
        assert_eq!(address, parsed);
    }

    #[test]
    fn test_address_with_country() {
        let address = Address {
            street: Some("Musterstraße".to_string()),
            city: Some("München".to_string()),
            country_code: Some(Country::Germany),
            ..Default::default()
        };

        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains(r#""countryCode":"DE""#));

        let parsed: Address = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.country_code, Some(Country::Germany));
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Address::type_name_german(), "Adresse");
        assert_eq!(Address::type_name_english(), "Address");
    }
}
