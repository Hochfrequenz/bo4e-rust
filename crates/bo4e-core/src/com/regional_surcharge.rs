//! Regional surcharge (AufAbschlagRegional) component.

use serde::{Deserialize, Serialize};

use crate::enums::TariffRegionCriterion;
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::Surcharge;

/// A surcharge that applies to a specific regional criterion.
///
/// German: AufAbschlagRegional
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::RegionalSurcharge;
/// use bo4e_core::enums::TariffRegionCriterion;
///
/// let surcharge = RegionalSurcharge {
///     region_criterion: Some(TariffRegionCriterion::PostalCode),
///     region_code: Some("50*".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "AufAbschlagRegional"))]
#[serde(rename_all = "camelCase")]
pub struct RegionalSurcharge {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of regional criterion (Tarifregionskriterium)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifregionskriterium"))]
    pub region_criterion: Option<TariffRegionCriterion>,

    /// Region code/value (Regionscode)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "regionscode"))]
    pub region_code: Option<String>,

    /// Surcharges applicable to this region (AufAbschläge)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aufAbschlaege"))]
    pub surcharges: Vec<Surcharge>,
}

impl Bo4eObject for RegionalSurcharge {
    fn type_name_german() -> &'static str {
        "AufAbschlagRegional"
    }

    fn type_name_english() -> &'static str {
        "RegionalSurcharge"
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
    fn test_postal_region_surcharge() {
        let surcharge = RegionalSurcharge {
            region_criterion: Some(TariffRegionCriterion::PostalCode),
            region_code: Some("50*".to_string()),
            surcharges: vec![Surcharge {
                description: Some("Regional discount".to_string()),
                value: Some(-5.0),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(
            surcharge.region_criterion,
            Some(TariffRegionCriterion::PostalCode)
        );
        assert_eq!(surcharge.surcharges.len(), 1);
    }

    #[test]
    fn test_default() {
        let surcharge = RegionalSurcharge::default();
        assert!(surcharge.region_criterion.is_none());
        assert!(surcharge.surcharges.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let surcharge = RegionalSurcharge {
            region_criterion: Some(TariffRegionCriterion::NetworkNumber),
            region_code: Some("9990001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&surcharge).unwrap();
        let parsed: RegionalSurcharge = serde_json::from_str(&json).unwrap();
        assert_eq!(surcharge, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(RegionalSurcharge::type_name_german(), "AufAbschlagRegional");
        assert_eq!(RegionalSurcharge::type_name_english(), "RegionalSurcharge");
    }
}
