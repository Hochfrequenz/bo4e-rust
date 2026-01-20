//! Validity type (Gueltigkeitstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Classification of validity for implementing positive or negative lists.
///
/// German: Gueltigkeitstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Gueltigkeitstyp"))]
#[non_exhaustive]
pub enum ValidityType {
    /// Only in - criterion applies only with the specified values (NUR_IN)
    #[serde(rename = "NUR_IN")]
    OnlyIn,

    /// Not in - criterion does not apply with the specified values (NICHT_IN)
    #[serde(rename = "NICHT_IN")]
    NotIn,

    /// Only in combination with - criteria are combined with each other (NUR_IN_KOMBINATION_MIT)
    #[serde(rename = "NUR_IN_KOMBINATION_MIT")]
    OnlyInCombinationWith,
}

impl ValidityType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::OnlyIn => "Nur in",
            Self::NotIn => "Nicht in",
            Self::OnlyInCombinationWith => "Nur in Kombination mit",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ValidityType::OnlyIn).unwrap(),
            r#""NUR_IN""#
        );
        assert_eq!(
            serde_json::to_string(&ValidityType::NotIn).unwrap(),
            r#""NICHT_IN""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<ValidityType>(r#""NUR_IN""#).unwrap(),
            ValidityType::OnlyIn
        );
        assert_eq!(
            serde_json::from_str::<ValidityType>(r#""NUR_IN_KOMBINATION_MIT""#).unwrap(),
            ValidityType::OnlyInCombinationWith
        );
    }

    #[test]
    fn test_roundtrip() {
        for validity in [
            ValidityType::OnlyIn,
            ValidityType::NotIn,
            ValidityType::OnlyInCombinationWith,
        ] {
            let json = serde_json::to_string(&validity).unwrap();
            let parsed: ValidityType = serde_json::from_str(&json).unwrap();
            assert_eq!(validity, parsed);
        }
    }
}
