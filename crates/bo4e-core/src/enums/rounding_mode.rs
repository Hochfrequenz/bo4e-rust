//! Rounding mode (Rundungsverfahren) enumeration.

use serde::{Deserialize, Serialize};

/// Rounding mode/method.
///
/// Defines how numerical values should be rounded.
///
/// German: Rundungsverfahren
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Rundungsverfahren"))]
#[non_exhaustive]
pub enum RoundingMode {
    /// No rounding (Keine Rundung)
    #[serde(rename = "KEINE")]
    None,

    /// Commercial rounding / round half up (Kaufmännische Rundung)
    #[serde(rename = "KAUFMAENNISCH")]
    Commercial,

    /// Round down / floor (Abrunden)
    #[serde(rename = "ABRUNDEN")]
    Floor,

    /// Round up / ceiling (Aufrunden)
    #[serde(rename = "AUFRUNDEN")]
    Ceiling,
}

impl RoundingMode {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::None => "Keine Rundung",
            Self::Commercial => "Kaufmännische Rundung",
            Self::Floor => "Abrunden",
            Self::Ceiling => "Aufrunden",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&RoundingMode::Commercial).unwrap(),
            r#""KAUFMAENNISCH""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for mode in [
            RoundingMode::None,
            RoundingMode::Commercial,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: RoundingMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, parsed);
        }
    }
}
