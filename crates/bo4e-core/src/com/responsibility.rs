//! Responsibility (Zustaendigkeit) component.

use serde::{Deserialize, Serialize};

use crate::enums::SubjectArea;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Area of responsibility for a contact person.
///
/// Contains the temporal assignment of a contact person to departments and responsibilities.
///
/// German: Zustaendigkeit
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Responsibility;
/// use bo4e_core::enums::SubjectArea;
///
/// let resp = Responsibility {
///     subject_area: Some(SubjectArea::MarketCommunication),
///     position: Some("Manager".to_string()),
///     department: Some("IT".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zustaendigkeit"))]
#[serde(rename_all = "camelCase")]
pub struct Responsibility {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Subject area classification of the contact person (Themengebiet)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "themengebiet"))]
    pub subject_area: Option<SubjectArea>,

    /// Professional position/role of the contact person (Position)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "position"))]
    pub position: Option<String>,

    /// Department where the contact person works (Abteilung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "abteilung"))]
    pub department: Option<String>,
}

impl Bo4eObject for Responsibility {
    fn type_name_german() -> &'static str {
        "Zustaendigkeit"
    }

    fn type_name_english() -> &'static str {
        "Responsibility"
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
    fn test_responsibility_default() {
        let resp = Responsibility::default();
        assert!(resp.subject_area.is_none());
        assert!(resp.position.is_none());
        assert!(resp.department.is_none());
    }

    #[test]
    fn test_responsibility_serialize() {
        let resp = Responsibility {
            subject_area: Some(SubjectArea::MarketCommunication),
            position: Some("Team Lead".to_string()),
            department: Some("Market Communication".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains(r#""subjectArea":"MARKTKOMMUNIKATION""#));
        assert!(json.contains(r#""position":"Team Lead""#));
        assert!(json.contains(r#""department":"Market Communication""#));
    }

    #[test]
    fn test_responsibility_roundtrip() {
        let resp = Responsibility {
            meta: Bo4eMeta::with_type("Zustaendigkeit"),
            subject_area: Some(SubjectArea::Balancing),
            position: Some("Analyst".to_string()),
            department: Some("Energy Trading".to_string()),
        };

        let json = serde_json::to_string(&resp).unwrap();
        let parsed: Responsibility = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Responsibility::type_name_german(), "Zustaendigkeit");
        assert_eq!(Responsibility::type_name_english(), "Responsibility");
    }
}
