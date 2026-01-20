//! Salutation (Anrede) enumeration.

use serde::{Deserialize, Serialize};

/// Salutation/form of address for a person or business partner.
///
/// German: Anrede
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Salutation {
    /// Mr. (Herr)
    #[serde(rename = "HERR")]
    Mr,

    /// Ms./Mrs. (Frau)
    #[serde(rename = "FRAU")]
    Ms,

    /// Married couple (Eheleute)
    #[serde(rename = "EHELEUTE")]
    MarriedCouple,

    /// Company (Firma)
    #[serde(rename = "FIRMA")]
    Company,

    /// Family (Familie)
    #[serde(rename = "FAMILIE")]
    Family,

    /// Heirs community (Erbengemeinschaft)
    #[serde(rename = "ERBENGEMEINSCHAFT")]
    HeirsCommunity,

    /// Property community (Grundstuecksgemeinschaft)
    #[serde(rename = "GRUNDSTUECKSGEMEINSCHAFT")]
    PropertyCommunity,
}

impl Salutation {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Mr => "Herr",
            Self::Ms => "Frau",
            Self::MarriedCouple => "Eheleute",
            Self::Company => "Firma",
            Self::Family => "Familie",
            Self::HeirsCommunity => "Erbengemeinschaft",
            Self::PropertyCommunity => "Grundstuecksgemeinschaft",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Salutation::Mr).unwrap(), r#""HERR""#);
        assert_eq!(serde_json::to_string(&Salutation::Ms).unwrap(), r#""FRAU""#);
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Salutation>(r#""HERR""#).unwrap(),
            Salutation::Mr
        );
        assert_eq!(
            serde_json::from_str::<Salutation>(r#""FIRMA""#).unwrap(),
            Salutation::Company
        );
    }

    #[test]
    fn test_roundtrip() {
        for salutation in [
            Salutation::Mr,
            Salutation::Ms,
            Salutation::MarriedCouple,
            Salutation::Company,
            Salutation::Family,
            Salutation::HeirsCommunity,
            Salutation::PropertyCommunity,
        ] {
            let json = serde_json::to_string(&salutation).unwrap();
            let parsed: Salutation = serde_json::from_str(&json).unwrap();
            assert_eq!(salutation, parsed);
        }
    }
}
