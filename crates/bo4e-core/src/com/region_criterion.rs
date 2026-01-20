//! Region criterion (Regionskriterium) component.

use serde::{Deserialize, Serialize};

use crate::enums::{RegionCriterionType, ValidityType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Component for modeling a regional criterion.
///
/// Used to define criteria for regional delimitation, such as postal codes,
/// federal states, or network areas.
///
/// German: Regionskriterium
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::RegionCriterion;
/// use bo4e_core::enums::{RegionCriterionType, ValidityType};
///
/// let criterion = RegionCriterion {
///     validity_type: Some(ValidityType::OnlyIn),
///     criterion_type: Some(RegionCriterionType::PostalCode),
///     value: Some("50667".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Regionskriterium"))]
#[serde(rename_all = "camelCase")]
pub struct RegionCriterion {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Whether this is an inclusive or exclusive criterion (Gueltigkeitstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitstyp"))]
    pub validity_type: Option<ValidityType>,

    /// The type of criterion, e.g., federal state, postal code (Regionskriteriumtyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "regionskriteriumtyp"))]
    pub criterion_type: Option<RegionCriterionType>,

    /// The value the criterion takes, e.g., "NRW" or "50667" (Wert)
    /// Note: For BUNDESWEIT (nationwide), this value is not relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<String>,
}

impl Bo4eObject for RegionCriterion {
    fn type_name_german() -> &'static str {
        "Regionskriterium"
    }

    fn type_name_english() -> &'static str {
        "RegionCriterion"
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
    fn test_region_criterion_default() {
        let criterion = RegionCriterion::default();
        assert!(criterion.validity_type.is_none());
        assert!(criterion.criterion_type.is_none());
        assert!(criterion.value.is_none());
    }

    #[test]
    fn test_region_criterion_postal_code() {
        let criterion = RegionCriterion {
            validity_type: Some(ValidityType::OnlyIn),
            criterion_type: Some(RegionCriterionType::PostalCode),
            value: Some("50667".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&criterion).unwrap();
        assert!(json.contains(r#""validityType":"NUR_IN""#));
        assert!(json.contains(r#""criterionType":"POSTLEITZAHL""#));
        assert!(json.contains(r#""value":"50667""#));
    }

    #[test]
    fn test_region_criterion_federal_state() {
        let criterion = RegionCriterion {
            validity_type: Some(ValidityType::NotIn),
            criterion_type: Some(RegionCriterionType::FederalStateName),
            value: Some("Bayern".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&criterion).unwrap();
        assert!(json.contains(r#""validityType":"NICHT_IN""#));
        assert!(json.contains(r#""criterionType":"BUNDESLAND_NAME""#));
    }

    #[test]
    fn test_region_criterion_nationwide() {
        let criterion = RegionCriterion {
            validity_type: Some(ValidityType::OnlyIn),
            criterion_type: Some(RegionCriterionType::Nationwide),
            value: None, // Not relevant for nationwide
            ..Default::default()
        };

        let json = serde_json::to_string(&criterion).unwrap();
        assert!(json.contains(r#""criterionType":"BUNDESWEIT""#));
        assert!(!json.contains(r#""value""#)); // Should be skipped
    }

    #[test]
    fn test_region_criterion_roundtrip() {
        let criterion = RegionCriterion {
            meta: Bo4eMeta::with_type("Regionskriterium"),
            validity_type: Some(ValidityType::OnlyInCombinationWith),
            criterion_type: Some(RegionCriterionType::MunicipalityName),
            value: Some("Köln".to_string()),
        };

        let json = serde_json::to_string(&criterion).unwrap();
        let parsed: RegionCriterion = serde_json::from_str(&json).unwrap();
        assert_eq!(criterion, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(RegionCriterion::type_name_german(), "Regionskriterium");
        assert_eq!(RegionCriterion::type_name_english(), "RegionCriterion");
    }
}
