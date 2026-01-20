//! Amount (Betrag) component.

use serde::{Deserialize, Serialize};

use crate::enums::Currency;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A monetary amount with currency.
///
/// German: Betrag
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Amount;
/// use bo4e_core::enums::Currency;
///
/// let amount = Amount {
///     value: Some(100.50),
///     currency: Some(Currency::Eur),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// The amount value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
}

impl Bo4eObject for Amount {
    fn type_name_german() -> &'static str {
        "Betrag"
    }

    fn type_name_english() -> &'static str {
        "Amount"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl Amount {
    /// Create an amount in EUR.
    pub fn eur(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: Some(Currency::Eur),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eur_amount() {
        let amount = Amount::eur(100.50);
        assert_eq!(amount.value, Some(100.50));
        assert_eq!(amount.currency, Some(Currency::Eur));
    }

    #[test]
    fn test_default() {
        let amount = Amount::default();
        assert!(amount.value.is_none());
        assert!(amount.currency.is_none());
    }

    #[test]
    fn test_serialize() {
        let amount = Amount::eur(250.75);
        let json = serde_json::to_string(&amount).unwrap();
        assert!(json.contains(r#""value":250.75"#));
        assert!(json.contains(r#""currency":"EUR""#));
    }

    #[test]
    fn test_roundtrip() {
        let amount = Amount::eur(999.99);
        let json = serde_json::to_string(&amount).unwrap();
        let parsed: Amount = serde_json::from_str(&json).unwrap();
        assert_eq!(amount, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Amount::type_name_german(), "Betrag");
        assert_eq!(Amount::type_name_english(), "Amount");
    }
}
