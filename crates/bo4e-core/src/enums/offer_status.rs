//! Offer status (Angebotsstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status of an offer.
///
/// German: Angebotsstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OfferStatus {
    /// Concept phase (Konzeption)
    #[serde(rename = "KONZEPTION")]
    Concept,

    /// Non-binding (Unverbindlich)
    #[serde(rename = "UNVERBINDLICH")]
    NonBinding,

    /// Binding (Verbindlich)
    #[serde(rename = "VERBINDLICH")]
    Binding,

    /// Commissioned/ordered (Beauftragt)
    #[serde(rename = "BEAUFTRAGT")]
    Commissioned,

    /// Invalid (Ungueltig)
    #[serde(rename = "UNGUELTIG")]
    Invalid,

    /// Rejected (Abgelehnt)
    #[serde(rename = "ABGELEHNT")]
    Rejected,

    /// Followed up (Nachgefasst)
    #[serde(rename = "NACHGEFASST")]
    FollowedUp,

    /// Pending (Ausstehend)
    #[serde(rename = "AUSSTEHEND")]
    Pending,

    /// Completed (Erledigt)
    #[serde(rename = "ERLEDIGT")]
    Completed,
}

impl OfferStatus {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Concept => "Konzeption",
            Self::NonBinding => "Unverbindlich",
            Self::Binding => "Verbindlich",
            Self::Commissioned => "Beauftragt",
            Self::Invalid => "Ungueltig",
            Self::Rejected => "Abgelehnt",
            Self::FollowedUp => "Nachgefasst",
            Self::Pending => "Ausstehend",
            Self::Completed => "Erledigt",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&OfferStatus::Concept).unwrap(),
            r#""KONZEPTION""#
        );
        assert_eq!(
            serde_json::to_string(&OfferStatus::Binding).unwrap(),
            r#""VERBINDLICH""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<OfferStatus>(r#""KONZEPTION""#).unwrap(),
            OfferStatus::Concept
        );
        assert_eq!(
            serde_json::from_str::<OfferStatus>(r#""ERLEDIGT""#).unwrap(),
            OfferStatus::Completed
        );
    }

    #[test]
    fn test_roundtrip() {
        for status in [
            OfferStatus::Concept,
            OfferStatus::NonBinding,
            OfferStatus::Binding,
            OfferStatus::Commissioned,
            OfferStatus::Invalid,
            OfferStatus::Rejected,
            OfferStatus::FollowedUp,
            OfferStatus::Pending,
            OfferStatus::Completed,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: OfferStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
