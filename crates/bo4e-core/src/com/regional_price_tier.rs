//! Regional price tier (RegionalePreisstaffel) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

use super::PriceTier;

/// A price tier that applies to a specific region.
///
/// German: RegionalePreisstaffel
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{RegionalPriceTier, PriceTier};
///
/// let regional_tier = RegionalPriceTier {
///     region_code: Some("50667".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionalPriceTier {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Region code (postal code, network area, etc.) (Regionscode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,

    /// Price tiers for this region (Preisstaffeln)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiers: Vec<PriceTier>,
}

impl Bo4eObject for RegionalPriceTier {
    fn type_name_german() -> &'static str {
        "RegionalePreisstaffel"
    }

    fn type_name_english() -> &'static str {
        "RegionalPriceTier"
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
    fn test_regional_tier() {
        let tier = RegionalPriceTier {
            region_code: Some("50667".to_string()),
            tiers: vec![
                PriceTier {
                    lower_limit: Some(0.0),
                    upper_limit: Some(1000.0),
                    unit_price: Some(0.28),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(tier.region_code, Some("50667".to_string()));
        assert_eq!(tier.tiers.len(), 1);
    }

    #[test]
    fn test_default() {
        let tier = RegionalPriceTier::default();
        assert!(tier.region_code.is_none());
        assert!(tier.tiers.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let tier = RegionalPriceTier {
            region_code: Some("NRW-001".to_string()),
            tiers: vec![PriceTier {
                unit_price: Some(0.30),
                tier_number: Some(1),
                ..Default::default()
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&tier).unwrap();
        let parsed: RegionalPriceTier = serde_json::from_str(&json).unwrap();
        assert_eq!(tier, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(RegionalPriceTier::type_name_german(), "RegionalePreisstaffel");
        assert_eq!(RegionalPriceTier::type_name_english(), "RegionalPriceTier");
    }
}
