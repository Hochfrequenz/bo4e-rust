//! Title (Titel) enumeration.

use serde::{Deserialize, Serialize};

/// Academic or professional title of a person.
///
/// German: Titel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Titel"))]
#[non_exhaustive]
pub enum Title {
    /// Doctor (Doktor)
    #[serde(rename = "DR")]
    Dr,

    /// Professor
    #[serde(rename = "PROF")]
    Prof,

    /// Professor Doctor
    #[serde(rename = "PROF_DR")]
    ProfDr,
}

impl Title {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Dr => "Dr.",
            Self::Prof => "Prof.",
            Self::ProfDr => "Prof. Dr.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Title::Dr).unwrap(), r#""DR""#);
        assert_eq!(serde_json::to_string(&Title::Prof).unwrap(), r#""PROF""#);
        assert_eq!(
            serde_json::to_string(&Title::ProfDr).unwrap(),
            r#""PROF_DR""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(serde_json::from_str::<Title>(r#""DR""#).unwrap(), Title::Dr);
        assert_eq!(
            serde_json::from_str::<Title>(r#""PROF_DR""#).unwrap(),
            Title::ProfDr
        );
    }

    #[test]
    fn test_roundtrip() {
        for title in [Title::Dr, Title::Prof, Title::ProfDr] {
            let json = serde_json::to_string(&title).unwrap();
            let parsed: Title = serde_json::from_str(&json).unwrap();
            assert_eq!(title, parsed);
        }
    }
}
