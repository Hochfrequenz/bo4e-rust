//! Phase type (Phasenart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of electrical phase configuration.
///
/// Describes the phase configuration of electrical installations.
///
/// German: Phasenart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PhaseType {
    /// Single-phase (Einphasig)
    #[serde(rename = "EINPHASIG")]
    SinglePhase,

    /// Two-phase (Zweiphasig)
    #[serde(rename = "ZWEIPHASIG")]
    TwoPhase,

    /// Three-phase (Dreiphasig)
    #[serde(rename = "DREIPHASIG")]
    ThreePhase,
}

impl PhaseType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::SinglePhase => "Einphasig",
            Self::TwoPhase => "Zweiphasig",
            Self::ThreePhase => "Dreiphasig",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&PhaseType::ThreePhase).unwrap(),
            r#""DREIPHASIG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for phase in [
            PhaseType::SinglePhase,
            PhaseType::TwoPhase,
            PhaseType::ThreePhase,
        ] {
            let json = serde_json::to_string(&phase).unwrap();
            let parsed: PhaseType = serde_json::from_str(&json).unwrap();
            assert_eq!(phase, parsed);
        }
    }
}
