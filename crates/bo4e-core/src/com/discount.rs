//! Discount (Rabatt) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, SurchargeType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A discount applied to a price.
///
/// German: Rabatt
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Discount;
///
/// let discount = Discount {
///     description: Some("Stammkundenrabatt".to_string()),
///     value: Some(10.0),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description/name of the discount (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of discount (Rabatttyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<SurchargeType>,

    /// Discount value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Conditions for the discount (Bedingungen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
}

impl Bo4eObject for Discount {
    fn type_name_german() -> &'static str {
        "Rabatt"
    }

    fn type_name_english() -> &'static str {
        "Discount"
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
    fn test_percentage_discount() {
        let discount = Discount {
            description: Some("10% Rabatt".to_string()),
            discount_type: Some(SurchargeType::Relative),
            value: Some(10.0),
            ..Default::default()
        };

        assert_eq!(discount.discount_type, Some(SurchargeType::Relative));
        assert_eq!(discount.value, Some(10.0));
    }

    #[test]
    fn test_absolute_discount() {
        let discount = Discount {
            description: Some("50 EUR Rabatt".to_string()),
            discount_type: Some(SurchargeType::Absolute),
            value: Some(50.0),
            currency: Some(Currency::Eur),
            ..Default::default()
        };

        assert_eq!(discount.discount_type, Some(SurchargeType::Absolute));
    }

    #[test]
    fn test_default() {
        let discount = Discount::default();
        assert!(discount.description.is_none());
        assert!(discount.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let discount = Discount {
            description: Some("Frühbucherrabatt".to_string()),
            discount_type: Some(SurchargeType::Relative),
            value: Some(15.0),
            conditions: Some("Bei Buchung bis 31.12.".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&discount).unwrap();
        let parsed: Discount = serde_json::from_str(&json).unwrap();
        assert_eq!(discount, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Discount::type_name_german(), "Rabatt");
        assert_eq!(Discount::type_name_english(), "Discount");
    }
}
