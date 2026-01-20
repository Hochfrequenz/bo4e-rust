//! Contact type (Kontaktart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of contact method for reaching a person or business partner.
///
/// German: Kontaktart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kontaktart"))]
#[non_exhaustive]
pub enum ContactType {
    /// Postal mail (Postweg)
    #[serde(rename = "POSTWEG")]
    Mail,

    /// Telephone (Telefon)
    #[serde(rename = "TELEFON")]
    Phone,

    /// Fax
    #[serde(rename = "FAX")]
    Fax,

    /// Email (E-Mail)
    #[serde(rename = "E_MAIL")]
    Email,

    /// SMS
    #[serde(rename = "SMS")]
    Sms,
}

impl ContactType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Mail => "Postweg",
            Self::Phone => "Telefon",
            Self::Fax => "Fax",
            Self::Email => "E-Mail",
            Self::Sms => "SMS",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ContactType::Email).unwrap(),
            r#""E_MAIL""#
        );
        assert_eq!(
            serde_json::to_string(&ContactType::Phone).unwrap(),
            r#""TELEFON""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<ContactType>(r#""POSTWEG""#).unwrap(),
            ContactType::Mail
        );
        assert_eq!(
            serde_json::from_str::<ContactType>(r#""SMS""#).unwrap(),
            ContactType::Sms
        );
    }

    #[test]
    fn test_roundtrip() {
        for contact in [
            ContactType::Mail,
            ContactType::Phone,
            ContactType::Fax,
            ContactType::Email,
            ContactType::Sms,
        ] {
            let json = serde_json::to_string(&contact).unwrap();
            let parsed: ContactType = serde_json::from_str(&json).unwrap();
            assert_eq!(contact, parsed);
        }
    }
}
