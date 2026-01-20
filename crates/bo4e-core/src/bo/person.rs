//! Person (Person) business object.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::com::{Address, ContactMethod};
use crate::enums::{Salutation, Title};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A natural person.
///
/// German: Person
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Person;
/// use bo4e_core::enums::Salutation;
///
/// let person = Person {
///     salutation: Some(Salutation::Mr),
///     first_name: Some("Max".to_string()),
///     last_name: Some("Mustermann".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Salutation (Anrede)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salutation: Option<Salutation>,

    /// Title (Titel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,

    /// First name (Vorname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name (Nachname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Name suffix (Namenszusatz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// Name prefix (Namenspraefix)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,

    /// Company name if representing a company (Firma)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,

    /// Birth date (Geburtsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,

    /// Primary address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Contact methods (Kontaktwege)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact_methods: Vec<ContactMethod>,
}

impl Bo4eObject for Person {
    fn type_name_german() -> &'static str {
        "Person"
    }

    fn type_name_english() -> &'static str {
        "Person"
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
    fn test_person_creation() {
        let person = Person {
            salutation: Some(Salutation::Mr),
            first_name: Some("Max".to_string()),
            last_name: Some("Mustermann".to_string()),
            ..Default::default()
        };

        assert_eq!(person.first_name, Some("Max".to_string()));
        assert_eq!(person.last_name, Some("Mustermann".to_string()));
    }

    #[test]
    fn test_person_with_title() {
        let person = Person {
            salutation: Some(Salutation::Mr),
            title: Some(Title::Dr),
            first_name: Some("Hans".to_string()),
            last_name: Some("Mueller".to_string()),
            ..Default::default()
        };

        assert_eq!(person.title, Some(Title::Dr));
    }

    #[test]
    fn test_serialize() {
        let person = Person {
            meta: Bo4eMeta::with_type("Person"),
            salutation: Some(Salutation::Ms),
            first_name: Some("Erika".to_string()),
            last_name: Some("Musterfrau".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&person).unwrap();
        assert!(json.contains(r#""firstName":"Erika""#));
        assert!(json.contains(r#""lastName":"Musterfrau""#));
    }

    #[test]
    fn test_roundtrip() {
        let person = Person {
            meta: Bo4eMeta::with_type("Person"),
            salutation: Some(Salutation::Mr),
            title: Some(Title::ProfDr),
            first_name: Some("Klaus".to_string()),
            last_name: Some("Schmidt".to_string()),
            name_suffix: Some("Jr.".to_string()),
            company_name: Some("Schmidt GmbH".to_string()),
            birth_date: Some(NaiveDate::from_ymd_opt(1970, 5, 15).unwrap()),
            ..Default::default()
        };

        let json = serde_json::to_string(&person).unwrap();
        let parsed: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(person, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Person::type_name_german(), "Person");
        assert_eq!(Person::type_name_english(), "Person");
    }
}
