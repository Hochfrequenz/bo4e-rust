//! Meter category (Zaehlerauspraegung) enumeration.

use serde::{Deserialize, Serialize};

/// Category/configuration of meter.
///
/// Indicates whether it is a unidirectional or bidirectional meter.
///
/// German: Zaehlerauspraegung
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MeterCategory {
    /// Unidirectional meter (Einrichtungszähler)
    #[serde(rename = "EINRICHTUNGSZAEHLER")]
    Unidirectional,

    /// Bidirectional meter (Zweirichtungszähler)
    #[serde(rename = "ZWEIRICHTUNGSZAEHLER")]
    Bidirectional,
}

impl MeterCategory {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Unidirectional => "Einrichtungszähler",
            Self::Bidirectional => "Zweirichtungszähler",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeterCategory::Unidirectional).unwrap(),
            r#""EINRICHTUNGSZAEHLER""#
        );
        assert_eq!(
            serde_json::to_string(&MeterCategory::Bidirectional).unwrap(),
            r#""ZWEIRICHTUNGSZAEHLER""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for cat in [MeterCategory::Unidirectional, MeterCategory::Bidirectional] {
            let json = serde_json::to_string(&cat).unwrap();
            let parsed: MeterCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(cat, parsed);
        }
    }
}
