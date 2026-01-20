//! Reading type (Ablesetyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of meter reading.
///
/// Indicates how the meter was read.
///
/// German: Ablesetyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Ableseart"))]
#[non_exhaustive]
pub enum ReadingType {
    /// Reading by customer (Kundenselbstablesung)
    #[serde(rename = "KUNDENSELBSTABLESUNG")]
    CustomerSelfReading,

    /// Remote reading (Fernauslesung)
    #[serde(rename = "FERNAUSLESUNG")]
    RemoteReading,

    /// Reading by metering point operator (Ablesung durch MSB)
    #[serde(rename = "MSB_ABLESUNG")]
    MeteringOperatorReading,

    /// Estimated reading (Schätzung)
    #[serde(rename = "SCHAETZUNG")]
    Estimated,

    /// Reading by network operator (Ablesung durch NB)
    #[serde(rename = "NB_ABLESUNG")]
    NetworkOperatorReading,
}

impl ReadingType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::CustomerSelfReading => "Kundenselbstablesung",
            Self::RemoteReading => "Fernauslesung",
            Self::MeteringOperatorReading => "Ablesung durch MSB",
            Self::Estimated => "Schätzung",
            Self::NetworkOperatorReading => "Ablesung durch NB",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ReadingType::RemoteReading).unwrap(),
            r#""FERNAUSLESUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for rtype in [
            ReadingType::CustomerSelfReading,
            ReadingType::RemoteReading,
            ReadingType::MeteringOperatorReading,
            ReadingType::Estimated,
            ReadingType::NetworkOperatorReading,
        ] {
            let json = serde_json::to_string(&rtype).unwrap();
            let parsed: ReadingType = serde_json::from_str(&json).unwrap();
            assert_eq!(rtype, parsed);
        }
    }
}
