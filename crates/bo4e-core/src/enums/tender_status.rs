//! Tender status (Ausschreibungsstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status/phase of a tender process.
///
/// German: Ausschreibungsstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Ausschreibungsstatus"))]
#[non_exhaustive]
pub enum TenderStatus {
    /// Phase 1: Participation competition (Teilnahmewettbewerb)
    #[serde(rename = "PHASE1")]
    Phase1,

    /// Phase 2: Offer phase (Angebotsphase)
    #[serde(rename = "PHASE2")]
    Phase2,

    /// Phase 3: Negotiation phase (Verhandlungsphase)
    #[serde(rename = "PHASE3")]
    Phase3,

    /// Phase 4: Contract award (Zuschlagserteilung)
    #[serde(rename = "PHASE4")]
    Phase4,
}

impl TenderStatus {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Phase1 => "Teilnahmewettbewerb",
            Self::Phase2 => "Angebotsphase",
            Self::Phase3 => "Verhandlungsphase",
            Self::Phase4 => "Zuschlagserteilung",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TenderStatus::Phase1).unwrap(),
            r#""PHASE1""#
        );
        assert_eq!(
            serde_json::to_string(&TenderStatus::Phase4).unwrap(),
            r#""PHASE4""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<TenderStatus>(r#""PHASE1""#).unwrap(),
            TenderStatus::Phase1
        );
        assert_eq!(
            serde_json::from_str::<TenderStatus>(r#""PHASE3""#).unwrap(),
            TenderStatus::Phase3
        );
    }

    #[test]
    fn test_roundtrip() {
        for status in [
            TenderStatus::Phase1,
            TenderStatus::Phase2,
            TenderStatus::Phase3,
            TenderStatus::Phase4,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: TenderStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
