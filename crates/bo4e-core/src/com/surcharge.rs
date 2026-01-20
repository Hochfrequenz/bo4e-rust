//! Surcharge/discount (AufAbschlag) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, SurchargeTarget, SurchargeType};
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::PriceTier;

/// A surcharge or discount applied to a price.
///
/// German: AufAbschlag
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Surcharge;
///
/// let surcharge = Surcharge {
///     description: Some("Ökosteuer".to_string()),
///     value: Some(0.02),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Surcharge {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the surcharge (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of surcharge (AufAbschlagstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharge_type: Option<SurchargeType>,

    /// Value of the surcharge (Wert)
    /// Positive = surcharge, negative = discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency unit for absolute surcharges (Waehrungseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Target price/cost category (AufAbschlagsziel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<SurchargeTarget>,

    /// Tiered surcharge values (Staffeln)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiers: Vec<PriceTier>,

    /// Additional description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// Website for published information (Website)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl Bo4eObject for Surcharge {
    fn type_name_german() -> &'static str {
        "AufAbschlag"
    }

    fn type_name_english() -> &'static str {
        "Surcharge"
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
    fn test_surcharge() {
        let surcharge = Surcharge {
            description: Some("Ökosteuer".to_string()),
            value: Some(0.02),
            surcharge_type: Some(SurchargeType::Absolute),
            currency: Some(Currency::Cent),
            ..Default::default()
        };

        let json = serde_json::to_string(&surcharge).unwrap();
        let parsed: Surcharge = serde_json::from_str(&json).unwrap();
        assert_eq!(surcharge, parsed);
    }

    #[test]
    fn test_discount() {
        let discount = Surcharge {
            description: Some("Neukunden-Rabatt".to_string()),
            value: Some(-50.0),
            surcharge_type: Some(SurchargeType::Absolute),
            currency: Some(Currency::Euro),
            ..Default::default()
        };

        assert!(discount.value.unwrap() < 0.0);
    }

    #[test]
    fn test_percentage_surcharge() {
        let surcharge = Surcharge {
            description: Some("MwSt".to_string()),
            value: Some(19.0),
            surcharge_type: Some(SurchargeType::Percentage),
            ..Default::default()
        };

        assert_eq!(surcharge.surcharge_type, Some(SurchargeType::Percentage));
    }

    #[test]
    fn test_tiered_surcharge() {
        let surcharge = Surcharge {
            description: Some("Staffelrabatt".to_string()),
            tiers: vec![
                PriceTier {
                    lower_limit: Some(0.0),
                    upper_limit: Some(1000.0),
                    unit_price: Some(0.05),
                    ..Default::default()
                },
                PriceTier {
                    lower_limit: Some(1000.0),
                    upper_limit: Some(5000.0),
                    unit_price: Some(0.03),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(surcharge.tiers.len(), 2);
    }

    #[test]
    fn test_default() {
        let surcharge = Surcharge::default();
        assert!(surcharge.description.is_none());
        assert!(surcharge.value.is_none());
        assert!(surcharge.tiers.is_empty());
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Surcharge::type_name_german(), "AufAbschlag");
        assert_eq!(Surcharge::type_name_english(), "Surcharge");
    }
}
