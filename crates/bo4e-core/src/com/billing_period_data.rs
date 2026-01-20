//! Billing period data (Abrechnungsperiodendaten) component.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Data for a billing period.
///
/// German: Abrechnungsperiodendaten
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::BillingPeriodData;
/// use bo4e_core::enums::Unit;
/// use chrono::NaiveDate;
///
/// let data = BillingPeriodData {
///     period_start: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
///     period_end: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
///     consumption_value: Some(3500.0),
///     consumption_unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingPeriodData {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Start of billing period (Abrechnungsbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<NaiveDate>,

    /// End of billing period (Abrechnungsende)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<NaiveDate>,

    /// Starting meter reading (Anfangsstand)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_reading: Option<f64>,

    /// Starting reading timestamp (Anfangsablesung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_reading_timestamp: Option<DateTime<Utc>>,

    /// Ending meter reading (Endstand)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reading: Option<f64>,

    /// Ending reading timestamp (Endablesung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reading_timestamp: Option<DateTime<Utc>>,

    /// Consumption value for the period (Verbrauchswert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_value: Option<f64>,

    /// Unit of consumption (Verbrauchseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_unit: Option<Unit>,

    /// Number of days in period (Anzahl Tage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_in_period: Option<i32>,
}

impl Bo4eObject for BillingPeriodData {
    fn type_name_german() -> &'static str {
        "Abrechnungsperiodendaten"
    }

    fn type_name_english() -> &'static str {
        "BillingPeriodData"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_period_data() {
        let data = BillingPeriodData {
            period_start: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            period_end: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
            start_reading: Some(10000.0),
            end_reading: Some(13500.0),
            consumption_value: Some(3500.0),
            consumption_unit: Some(Unit::KilowattHour),
            days_in_period: Some(366),
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("3500"));
    }

    #[test]
    fn test_roundtrip() {
        let data = BillingPeriodData {
            period_start: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            period_end: Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
            consumption_value: Some(1750.0),
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        let parsed: BillingPeriodData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            BillingPeriodData::type_name_german(),
            "Abrechnungsperiodendaten"
        );
        assert_eq!(BillingPeriodData::type_name_english(), "BillingPeriodData");
    }
}
