//! Measured value status (Messwertstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status of a measured value.
///
/// The status of a meter reading.
///
/// German: Messwertstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Messwertstatus"))]
#[non_exhaustive]
pub enum MeasuredValueStatus {
    /// Read/collected value (Abgelesen)
    #[serde(rename = "ABGELESEN")]
    Read,

    /// Substitute/replacement value (Ersatzwert)
    #[serde(rename = "ERSATZWERT")]
    Substitute,

    /// Information for delivery note (Angabe für Lieferschein)
    #[serde(rename = "ANGABE_FUER_LIEFERSCHEIN")]
    DeliveryNoteInfo,

    /// Proposed/suggested value (Vorschlagswert)
    #[serde(rename = "VORSCHLAGSWERT")]
    Proposed,

    /// Not usable (Nicht verwendbar)
    #[serde(rename = "NICHT_VERWENDBAR")]
    NotUsable,

    /// Forecast/prognosis value (Prognosewert)
    #[serde(rename = "PROGNOSEWERT")]
    Forecast,

    /// Preliminary/provisional value (Vorläufiger Wert)
    #[serde(rename = "VORLAEUFIGERWERT")]
    Preliminary,

    /// Summed energy amount (Energiemenge summiert)
    #[serde(rename = "ENERGIEMENGESUMMIERT")]
    EnergySummed,

    /// Missing (Fehlt)
    #[serde(rename = "FEHLT")]
    Missing,
}

impl MeasuredValueStatus {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Read => "Abgelesen",
            Self::Substitute => "Ersatzwert",
            Self::DeliveryNoteInfo => "Angabe für Lieferschein",
            Self::Proposed => "Vorschlagswert",
            Self::NotUsable => "Nicht verwendbar",
            Self::Forecast => "Prognosewert",
            Self::Preliminary => "Vorläufiger Wert",
            Self::EnergySummed => "Energiemenge summiert",
            Self::Missing => "Fehlt",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeasuredValueStatus::Read).unwrap(),
            r#""ABGELESEN""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for status in [
            MeasuredValueStatus::Read,
            MeasuredValueStatus::Substitute,
            MeasuredValueStatus::DeliveryNoteInfo,
            MeasuredValueStatus::Proposed,
            MeasuredValueStatus::NotUsable,
            MeasuredValueStatus::Forecast,
            MeasuredValueStatus::Preliminary,
            MeasuredValueStatus::EnergySummed,
            MeasuredValueStatus::Missing,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: MeasuredValueStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
