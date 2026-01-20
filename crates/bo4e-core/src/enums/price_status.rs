//! Price status (Preisstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status information for prices.
///
/// German: Preisstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Preisstatus"))]
#[non_exhaustive]
pub enum PriceStatus {
    /// Preliminary (Vorläufig)
    #[serde(rename = "VORLAEUFIG")]
    Preliminary,

    /// Final (Endgültig)
    #[serde(rename = "ENDGUELTIG")]
    Final,
}

impl PriceStatus {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Preliminary => "Vorläufig",
            Self::Final => "Endgültig",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&PriceStatus::Preliminary).unwrap(),
            r#""VORLAEUFIG""#
        );
        assert_eq!(
            serde_json::to_string(&PriceStatus::Final).unwrap(),
            r#""ENDGUELTIG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for status in [PriceStatus::Preliminary, PriceStatus::Final] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: PriceStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
