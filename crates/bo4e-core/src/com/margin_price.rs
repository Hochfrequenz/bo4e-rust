//! Margin price (Margenpreis) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A margin price component.
///
/// German: Margenpreis
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::MarginPrice;
///
/// let margin = MarginPrice {
///     value: Some(0.5),
///     description: Some("Vertriebsmarge".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginPrice {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Margin value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<Unit>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Bo4eObject for MarginPrice {
    fn type_name_german() -> &'static str {
        "Margenpreis"
    }

    fn type_name_english() -> &'static str {
        "MarginPrice"
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
    fn test_margin_price() {
        let margin = MarginPrice {
            value: Some(0.5),
            currency: Some(Currency::Cent),
            reference_unit: Some(Unit::KilowattHour),
            description: Some("Vertriebsmarge".to_string()),
            ..Default::default()
        };

        assert_eq!(margin.value, Some(0.5));
    }

    #[test]
    fn test_default() {
        let margin = MarginPrice::default();
        assert!(margin.value.is_none());
        assert!(margin.description.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let margin = MarginPrice {
            value: Some(1.25),
            currency: Some(Currency::Cent),
            reference_unit: Some(Unit::KilowattHour),
            description: Some("Deckungsbeitrag".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&margin).unwrap();
        let parsed: MarginPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(margin, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MarginPrice::type_name_german(), "Margenpreis");
        assert_eq!(MarginPrice::type_name_english(), "MarginPrice");
    }
}
