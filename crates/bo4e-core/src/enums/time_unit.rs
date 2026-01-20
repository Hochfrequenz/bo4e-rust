//! Time unit (Zeiteinheit) enumeration.

use serde::{Deserialize, Serialize};

/// Unit of time.
///
/// Time periods used for measurements, billing, or contracts.
///
/// German: Zeiteinheit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TimeUnit {
    /// Second
    #[serde(rename = "SEKUNDE")]
    Second,

    /// Minute
    #[serde(rename = "MINUTE")]
    Minute,

    /// Hour
    #[serde(rename = "STUNDE")]
    Hour,

    /// Quarter hour (15 minutes)
    #[serde(rename = "VIERTEL_STUNDE")]
    QuarterHour,

    /// Day
    #[serde(rename = "TAG")]
    Day,

    /// Week
    #[serde(rename = "WOCHE")]
    Week,

    /// Month
    #[serde(rename = "MONAT")]
    Month,

    /// Quarter (3 months)
    #[serde(rename = "QUARTAL")]
    Quarter,

    /// Half year (6 months)
    #[serde(rename = "HALBJAHR")]
    HalfYear,

    /// Year
    #[serde(rename = "JAHR")]
    Year,
}

impl TimeUnit {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Second => "Sekunde",
            Self::Minute => "Minute",
            Self::Hour => "Stunde",
            Self::QuarterHour => "Viertelstunde",
            Self::Day => "Tag",
            Self::Week => "Woche",
            Self::Month => "Monat",
            Self::Quarter => "Quartal",
            Self::HalfYear => "Halbjahr",
            Self::Year => "Jahr",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TimeUnit::Hour).unwrap(),
            r#""STUNDE""#
        );
        assert_eq!(serde_json::to_string(&TimeUnit::Year).unwrap(), r#""JAHR""#);
    }

    #[test]
    fn test_roundtrip() {
        for unit in [
            TimeUnit::Second,
            TimeUnit::Minute,
            TimeUnit::Hour,
            TimeUnit::QuarterHour,
            TimeUnit::Day,
            TimeUnit::Week,
            TimeUnit::Month,
            TimeUnit::Quarter,
            TimeUnit::HalfYear,
            TimeUnit::Year,
        ] {
            let json = serde_json::to_string(&unit).unwrap();
            let parsed: TimeUnit = serde_json::from_str(&json).unwrap();
            assert_eq!(unit, parsed);
        }
    }
}
