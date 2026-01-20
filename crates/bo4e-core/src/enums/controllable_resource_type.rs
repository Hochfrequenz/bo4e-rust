//! Controllable resource type (SteuerbareRessourceTyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of controllable resource.
///
/// Describes the type of a controllable resource in the energy grid.
///
/// German: SteuerbareRessourceTyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ControllableResourceType {
    /// On/Off control (An/Aus)
    #[serde(rename = "AN_AUS")]
    OnOff,

    /// Graduated/stepped control (Gestuft)
    #[serde(rename = "GESTUFT")]
    Graduated,
}

impl ControllableResourceType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::OnOff => "An/Aus",
            Self::Graduated => "Gestuft",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ControllableResourceType::OnOff).unwrap(),
            r#""AN_AUS""#
        );
        assert_eq!(
            serde_json::to_string(&ControllableResourceType::Graduated).unwrap(),
            r#""GESTUFT""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for crt in [
            ControllableResourceType::OnOff,
            ControllableResourceType::Graduated,
        ] {
            let json = serde_json::to_string(&crt).unwrap();
            let parsed: ControllableResourceType = serde_json::from_str(&json).unwrap();
            assert_eq!(crt, parsed);
        }
    }
}
