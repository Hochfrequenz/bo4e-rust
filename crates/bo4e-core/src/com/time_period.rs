//! Time period (Zeitraum) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// A time period with start and end timestamps.
///
/// German: Zeitraum
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TimePeriod;
/// use chrono::Utc;
///
/// let period = TimePeriod {
///     start: Some(Utc::now()),
///     end: None, // Open-ended
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriod {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Start of the period (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,

    /// End of the period (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
}

impl Bo4eObject for TimePeriod {
    fn type_name_german() -> &'static str {
        "Zeitraum"
    }

    fn type_name_english() -> &'static str {
        "TimePeriod"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl TimePeriod {
    /// Create a time period from start to end.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
            ..Default::default()
        }
    }

    /// Create an open-ended period starting from a given time.
    pub fn starting_from(start: DateTime<Utc>) -> Self {
        Self {
            start: Some(start),
            end: None,
            ..Default::default()
        }
    }

    /// Check if this period contains a given timestamp.
    pub fn contains(&self, timestamp: DateTime<Utc>) -> bool {
        let after_start = self.start.map_or(true, |s| timestamp >= s);
        let before_end = self.end.map_or(true, |e| timestamp < e);
        after_start && before_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_time_period_creation() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();

        let period = TimePeriod::new(start, end);
        assert_eq!(period.start, Some(start));
        assert_eq!(period.end, Some(end));
    }

    #[test]
    fn test_contains() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let period = TimePeriod::new(start, end);

        let mid = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        assert!(period.contains(mid));

        let before = Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
        assert!(!period.contains(before));
    }

    #[test]
    fn test_serialize_iso8601() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let period = TimePeriod::starting_from(start);

        let json = serde_json::to_string(&period).unwrap();
        assert!(json.contains("2024-01-01"));
    }

    #[test]
    fn test_roundtrip() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let period = TimePeriod::new(start, end);

        let json = serde_json::to_string(&period).unwrap();
        let parsed: TimePeriod = serde_json::from_str(&json).unwrap();
        assert_eq!(period, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TimePeriod::type_name_german(), "Zeitraum");
        assert_eq!(TimePeriod::type_name_english(), "TimePeriod");
    }
}
