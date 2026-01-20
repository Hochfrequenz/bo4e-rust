//! Organization type (Organisationstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of organization a business partner represents.
///
/// German: Organisationstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Organisationstyp"))]
#[non_exhaustive]
pub enum OrganizationType {
    /// Private person (B2C)
    #[serde(rename = "PRIVATPERSON")]
    PrivatePerson,

    /// Company/enterprise (B2B)
    #[serde(rename = "UNTERNEHMEN")]
    Company,

    /// Municipal institution (B2A)
    #[serde(rename = "KOMMUNALE_EINRICHTUNG")]
    MunicipalInstitution,

    /// Government authority (B2G)
    #[serde(rename = "STAATLICHE_BEHOERDE")]
    GovernmentAuthority,
}

impl OrganizationType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::PrivatePerson => "Privatperson",
            Self::Company => "Unternehmen",
            Self::MunicipalInstitution => "Kommunale Einrichtung",
            Self::GovernmentAuthority => "Staatliche Behoerde",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&OrganizationType::PrivatePerson).unwrap(),
            r#""PRIVATPERSON""#
        );
        assert_eq!(
            serde_json::to_string(&OrganizationType::Company).unwrap(),
            r#""UNTERNEHMEN""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<OrganizationType>(r#""PRIVATPERSON""#).unwrap(),
            OrganizationType::PrivatePerson
        );
        assert_eq!(
            serde_json::from_str::<OrganizationType>(r#""KOMMUNALE_EINRICHTUNG""#).unwrap(),
            OrganizationType::MunicipalInstitution
        );
    }

    #[test]
    fn test_roundtrip() {
        for org_type in [
            OrganizationType::PrivatePerson,
            OrganizationType::Company,
            OrganizationType::MunicipalInstitution,
            OrganizationType::GovernmentAuthority,
        ] {
            let json = serde_json::to_string(&org_type).unwrap();
            let parsed: OrganizationType = serde_json::from_str(&json).unwrap();
            assert_eq!(org_type, parsed);
        }
    }
}
