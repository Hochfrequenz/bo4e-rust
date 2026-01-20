//! Contract status (Vertragsstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status of a contract in its lifecycle.
///
/// German: Vertragsstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Vertragsstatus"))]
#[non_exhaustive]
pub enum ContractStatus {
    /// In progress/draft (In Arbeit)
    #[serde(rename = "IN_ARBEIT")]
    InProgress,

    /// Transmitted (Uebermittelt)
    #[serde(rename = "UEBERMITTELT")]
    Transmitted,

    /// Accepted (Angenommen)
    #[serde(rename = "ANGENOMMEN")]
    Accepted,

    /// Active (Aktiv)
    #[serde(rename = "AKTIV")]
    Active,

    /// Rejected (Abgelehnt)
    #[serde(rename = "ABGELEHNT")]
    Rejected,

    /// Revoked (Widerrufen)
    #[serde(rename = "WIDERRUFEN")]
    Revoked,

    /// Cancelled (Storniert)
    #[serde(rename = "STORNIERT")]
    Cancelled,

    /// Terminated (Gekuendigt)
    #[serde(rename = "GEKUENDIGT")]
    Terminated,

    /// Ended (Beendet)
    #[serde(rename = "BEENDET")]
    Ended,
}

impl ContractStatus {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::InProgress => "In Arbeit",
            Self::Transmitted => "Uebermittelt",
            Self::Accepted => "Angenommen",
            Self::Active => "Aktiv",
            Self::Rejected => "Abgelehnt",
            Self::Revoked => "Widerrufen",
            Self::Cancelled => "Storniert",
            Self::Terminated => "Gekuendigt",
            Self::Ended => "Beendet",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ContractStatus::Active).unwrap(),
            r#""AKTIV""#
        );
        assert_eq!(
            serde_json::to_string(&ContractStatus::Terminated).unwrap(),
            r#""GEKUENDIGT""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<ContractStatus>(r#""IN_ARBEIT""#).unwrap(),
            ContractStatus::InProgress
        );
        assert_eq!(
            serde_json::from_str::<ContractStatus>(r#""BEENDET""#).unwrap(),
            ContractStatus::Ended
        );
    }

    #[test]
    fn test_roundtrip() {
        for status in [
            ContractStatus::InProgress,
            ContractStatus::Transmitted,
            ContractStatus::Accepted,
            ContractStatus::Active,
            ContractStatus::Rejected,
            ContractStatus::Revoked,
            ContractStatus::Cancelled,
            ContractStatus::Terminated,
            ContractStatus::Ended,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: ContractStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
