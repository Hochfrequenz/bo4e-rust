//! Surcharge type (AufAbschlagstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of surcharge or discount.
///
/// Determines whether the surcharge/discount is relative (percentage) or absolute.
///
/// German: AufAbschlagstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SurchargeType {
    /// Relative (percentage-based) surcharge/discount
    #[serde(rename = "RELATIV")]
    Relative,

    /// Absolute surcharge/discount
    #[serde(rename = "ABSOLUT")]
    Absolute,
}

impl SurchargeType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Relative => "Prozentualer Auf-/Abschlag",
            Self::Absolute => "Absoluter Auf-/Abschlag",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&SurchargeType::Relative).unwrap(),
            r#""RELATIV""#
        );
        assert_eq!(
            serde_json::to_string(&SurchargeType::Absolute).unwrap(),
            r#""ABSOLUT""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for surcharge_type in [SurchargeType::Relative, SurchargeType::Absolute] {
            let json = serde_json::to_string(&surcharge_type).unwrap();
            let parsed: SurchargeType = serde_json::from_str(&json).unwrap();
            assert_eq!(surcharge_type, parsed);
        }
    }
}
