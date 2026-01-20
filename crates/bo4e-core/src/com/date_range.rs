//! Date range (Datumsbereich) component.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// A date range with start and end dates (without time).
///
/// German: Datumsbereich
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::DateRange;
/// use chrono::NaiveDate;
///
/// let range = DateRange {
///     start_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
///     end_date: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Start date (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,

    /// End date (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
}

impl Bo4eObject for DateRange {
    fn type_name_german() -> &'static str {
        "Datumsbereich"
    }

    fn type_name_english() -> &'static str {
        "DateRange"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl DateRange {
    /// Create a date range from start to end.
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            start_date: Some(start),
            end_date: Some(end),
            ..Default::default()
        }
    }

    /// Create a date range for a full year.
    pub fn year(year: i32) -> Self {
        Self {
            start_date: NaiveDate::from_ymd_opt(year, 1, 1),
            end_date: NaiveDate::from_ymd_opt(year, 12, 31),
            ..Default::default()
        }
    }

    /// Check if a date falls within this range.
    pub fn contains(&self, date: NaiveDate) -> bool {
        let after_start = self.start_date.map_or(true, |s| date >= s);
        let before_end = self.end_date.map_or(true, |e| date <= e);
        after_start && before_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_creation() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let range = DateRange::new(start, end);
        assert_eq!(range.start_date, Some(start));
        assert_eq!(range.end_date, Some(end));
    }

    #[test]
    fn test_year_range() {
        let range = DateRange::year(2024);
        assert_eq!(
            range.start_date,
            NaiveDate::from_ymd_opt(2024, 1, 1)
        );
        assert_eq!(
            range.end_date,
            NaiveDate::from_ymd_opt(2024, 12, 31)
        );
    }

    #[test]
    fn test_contains() {
        let range = DateRange::year(2024);
        let mid = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        assert!(range.contains(mid));

        let before = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        assert!(!range.contains(before));
    }

    #[test]
    fn test_roundtrip() {
        let range = DateRange::year(2024);
        let json = serde_json::to_string(&range).unwrap();
        let parsed: DateRange = serde_json::from_str(&json).unwrap();
        assert_eq!(range, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(DateRange::type_name_german(), "Datumsbereich");
        assert_eq!(DateRange::type_name_english(), "DateRange");
    }
}
