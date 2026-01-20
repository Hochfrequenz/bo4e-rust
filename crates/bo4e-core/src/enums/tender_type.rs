//! Tender type (Ausschreibungstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of tender/procurement.
///
/// German: Ausschreibungstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TenderType {
    /// Private law (Privatrechtlich)
    #[serde(rename = "PRIVATRECHTLICH")]
    PrivateLaw,

    /// Public law (Oeffentlichrechtlich)
    #[serde(rename = "OEFFENTLICHRECHTLICH")]
    PublicLaw,

    /// Europe-wide (Europaweit)
    #[serde(rename = "EUROPAWEIT")]
    EuropeWide,
}

impl TenderType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::PrivateLaw => "Privatrechtlich",
            Self::PublicLaw => "Oeffentlichrechtlich",
            Self::EuropeWide => "Europaweit",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TenderType::PrivateLaw).unwrap(),
            r#""PRIVATRECHTLICH""#
        );
        assert_eq!(
            serde_json::to_string(&TenderType::EuropeWide).unwrap(),
            r#""EUROPAWEIT""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<TenderType>(r#""PRIVATRECHTLICH""#).unwrap(),
            TenderType::PrivateLaw
        );
        assert_eq!(
            serde_json::from_str::<TenderType>(r#""OEFFENTLICHRECHTLICH""#).unwrap(),
            TenderType::PublicLaw
        );
    }

    #[test]
    fn test_roundtrip() {
        for tender_type in [
            TenderType::PrivateLaw,
            TenderType::PublicLaw,
            TenderType::EuropeWide,
        ] {
            let json = serde_json::to_string(&tender_type).unwrap();
            let parsed: TenderType = serde_json::from_str(&json).unwrap();
            assert_eq!(tender_type, parsed);
        }
    }
}
