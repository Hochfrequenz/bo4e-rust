//! Bonus (Bonus) component.

use serde::{Deserialize, Serialize};

use crate::enums::Currency;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A bonus or incentive payment.
///
/// German: Bonus
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Bonus;
///
/// let bonus = Bonus {
///     description: Some("Neukundenbonus".to_string()),
///     value: Some(100.0),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Bonus"))]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description/name of the bonus (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub description: Option<String>,

    /// Bonus value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "waehrung"))]
    pub currency: Option<Currency>,

    /// Conditions for receiving the bonus (Bedingungen)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bedingungen"))]
    pub conditions: Option<String>,

    /// Whether the bonus is a one-time payment (Einmalig)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einmalig"))]
    pub is_one_time: Option<bool>,
}

impl Bo4eObject for Bonus {
    fn type_name_german() -> &'static str {
        "Bonus"
    }

    fn type_name_english() -> &'static str {
        "Bonus"
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
    fn test_new_customer_bonus() {
        let bonus = Bonus {
            description: Some("Neukundenbonus".to_string()),
            value: Some(100.0),
            currency: Some(Currency::Eur),
            is_one_time: Some(true),
            conditions: Some("Bei Vertragsabschluss".to_string()),
            ..Default::default()
        };

        assert_eq!(bonus.value, Some(100.0));
        assert_eq!(bonus.is_one_time, Some(true));
    }

    #[test]
    fn test_loyalty_bonus() {
        let bonus = Bonus {
            description: Some("Treuebonus".to_string()),
            value: Some(25.0),
            currency: Some(Currency::Eur),
            is_one_time: Some(false),
            ..Default::default()
        };

        assert_eq!(bonus.is_one_time, Some(false));
    }

    #[test]
    fn test_default() {
        let bonus = Bonus::default();
        assert!(bonus.description.is_none());
        assert!(bonus.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let bonus = Bonus {
            description: Some("Online-Bonus".to_string()),
            value: Some(50.0),
            currency: Some(Currency::Eur),
            conditions: Some("Bei Online-Abschluss".to_string()),
            is_one_time: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&bonus).unwrap();
        let parsed: Bonus = serde_json::from_str(&json).unwrap();
        assert_eq!(bonus, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Bonus::type_name_german(), "Bonus");
        assert_eq!(Bonus::type_name_english(), "Bonus");
    }
}
