//! Interval component.

use serde::{Deserialize, Serialize};

use crate::enums::TimeUnit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A time interval with duration and unit.
///
/// German: Intervall
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Interval;
/// use bo4e_core::enums::TimeUnit;
///
/// let interval = Interval {
///     duration: Some(15),
///     unit: Some(TimeUnit::Minute),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Duration value (Dauer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    /// Time unit (Zeiteinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<TimeUnit>,
}

impl Bo4eObject for Interval {
    fn type_name_german() -> &'static str {
        "Intervall"
    }

    fn type_name_english() -> &'static str {
        "Interval"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl Interval {
    /// Create a 15-minute interval (common for load profiles).
    pub fn minutes_15() -> Self {
        Self {
            duration: Some(15),
            unit: Some(TimeUnit::Minute),
            ..Default::default()
        }
    }

    /// Create an hourly interval.
    pub fn hourly() -> Self {
        Self {
            duration: Some(1),
            unit: Some(TimeUnit::Hour),
            ..Default::default()
        }
    }

    /// Create a daily interval.
    pub fn daily() -> Self {
        Self {
            duration: Some(1),
            unit: Some(TimeUnit::Day),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15_minute_interval() {
        let interval = Interval::minutes_15();
        assert_eq!(interval.duration, Some(15));
        assert_eq!(interval.unit, Some(TimeUnit::Minute));
    }

    #[test]
    fn test_hourly_interval() {
        let interval = Interval::hourly();
        assert_eq!(interval.duration, Some(1));
        assert_eq!(interval.unit, Some(TimeUnit::Hour));
    }

    #[test]
    fn test_roundtrip() {
        let interval = Interval::daily();
        let json = serde_json::to_string(&interval).unwrap();
        let parsed: Interval = serde_json::from_str(&json).unwrap();
        assert_eq!(interval, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Interval::type_name_german(), "Intervall");
        assert_eq!(Interval::type_name_english(), "Interval");
    }
}
