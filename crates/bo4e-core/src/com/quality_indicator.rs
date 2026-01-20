//! Quality indicator (Qualitaetsindikator) component.

use serde::{Deserialize, Serialize};

use crate::enums::MeasuredValueStatus;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Quality indicator for measured data.
///
/// German: Qualitaetsindikator
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::QualityIndicator;
/// use bo4e_core::enums::MeasuredValueStatus;
///
/// let indicator = QualityIndicator {
///     quality_code: Some("G1".to_string()),
///     status: Some(MeasuredValueStatus::Read),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityIndicator {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Quality code (Qualitätscode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_code: Option<String>,

    /// Status of the measured value (Status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MeasuredValueStatus>,

    /// Quality description (Qualitätsbeschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_description: Option<String>,

    /// Confidence percentage (Konfidenz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_percent: Option<f64>,

    /// Data source (Datenquelle)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,

    /// Whether the value was substituted (Ersetzt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_substituted: Option<bool>,
}

impl Bo4eObject for QualityIndicator {
    fn type_name_german() -> &'static str {
        "Qualitaetsindikator"
    }

    fn type_name_english() -> &'static str {
        "QualityIndicator"
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
    fn test_quality_indicator() {
        let indicator = QualityIndicator {
            quality_code: Some("G1".to_string()),
            status: Some(MeasuredValueStatus::Read),
            confidence_percent: Some(99.5),
            is_substituted: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&indicator).unwrap();
        assert!(json.contains("G1"));
        assert!(json.contains("99.5"));
    }

    #[test]
    fn test_substituted_value() {
        let indicator = QualityIndicator {
            status: Some(MeasuredValueStatus::Substitute),
            is_substituted: Some(true),
            data_source: Some("INTERPOLATION".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&indicator).unwrap();
        assert!(json.contains("INTERPOLATION"));
    }

    #[test]
    fn test_roundtrip() {
        let indicator = QualityIndicator {
            quality_code: Some("G2".to_string()),
            status: Some(MeasuredValueStatus::Forecast),
            ..Default::default()
        };

        let json = serde_json::to_string(&indicator).unwrap();
        let parsed: QualityIndicator = serde_json::from_str(&json).unwrap();
        assert_eq!(indicator, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(QualityIndicator::type_name_german(), "Qualitaetsindikator");
        assert_eq!(QualityIndicator::type_name_english(), "QualityIndicator");
    }
}
