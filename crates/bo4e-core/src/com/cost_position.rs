//! Cost position (Kostenposition) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::{Amount, Price};

/// A cost position representing a single line item in a cost breakdown.
///
/// German: Kostenposition
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{CostPosition, Price, Amount};
///
/// let position = CostPosition {
///     title: Some("Netznutzungsentgelt".to_string()),
///     article_description: Some("Arbeitspreis Netz".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostPosition {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Title of the position (Positionstitel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Total amount for this position (Betrag Kostenposition)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,

    /// Description of the article (Artikelbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_description: Option<String>,

    /// Price per unit (Einzelpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Price>,

    /// Start date of the cost period inclusive (Von)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,

    /// End date of the cost period exclusive (Bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    /// Quantity value (Menge - Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_value: Option<f64>,

    /// Quantity unit (Menge - Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<Unit>,

    /// Time-based quantity value (Zeitmenge - Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_quantity_value: Option<f64>,

    /// Time-based quantity unit (Zeitmenge - Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_quantity_unit: Option<Unit>,

    /// Optional article details (Artikeldetail)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_detail: Option<String>,
}

impl Bo4eObject for CostPosition {
    fn type_name_german() -> &'static str {
        "Kostenposition"
    }

    fn type_name_english() -> &'static str {
        "CostPosition"
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
    use crate::enums::Currency;

    #[test]
    fn test_cost_position() {
        let position = CostPosition {
            title: Some("Netznutzung".to_string()),
            article_description: Some("Arbeitspreis".to_string()),
            unit_price: Some(Price::eur_per_kwh(0.0582)),
            quantity_value: Some(3660.0),
            quantity_unit: Some(Unit::KilowattHour),
            amount: Some(Amount::eur(213.01)),
            ..Default::default()
        };

        assert_eq!(position.title, Some("Netznutzung".to_string()));
        assert_eq!(position.quantity_value, Some(3660.0));
    }

    #[test]
    fn test_default() {
        let position = CostPosition::default();
        assert!(position.title.is_none());
        assert!(position.amount.is_none());
    }

    #[test]
    fn test_serialize() {
        let position = CostPosition {
            title: Some("Stromsteuer".to_string()),
            amount: Some(Amount {
                value: Some(100.0),
                currency: Some(Currency::Eur),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&position).unwrap();
        assert!(json.contains(r#""title":"Stromsteuer""#));
    }

    #[test]
    fn test_roundtrip() {
        let position = CostPosition {
            title: Some("Netzentgelt".to_string()),
            article_description: Some("Leistungspreis".to_string()),
            unit_price: Some(Price::eur_per_month(55.0)),
            amount: Some(Amount::eur(660.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&position).unwrap();
        let parsed: CostPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(position, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(CostPosition::type_name_german(), "Kostenposition");
        assert_eq!(CostPosition::type_name_english(), "CostPosition");
    }
}
