//! Offer part (Angebotsteil) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Part of an offer variant.
///
/// Aggregates offer positions. Offer parts are typically created for a market location
/// or delivery address. Contains the quantities and total costs of all offer positions.
/// A variant consists of at least one offer part.
///
/// German: Angebotsteil
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::OfferPart;
///
/// let part = OfferPart {
///     request_sub_reference: Some("Lot 1".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferPart {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Sub-reference identifying a sub-chapter of a request, e.g., tender lot (AnfrageSubreferenz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_sub_reference: Option<String>,

    // Note: The following fields would typically reference other COM types
    // (Angebotsposition, Marktlokation, Menge, Betrag, Zeitraum) which will be added later.
    // For now, we use simplified representations.
    /// Number of positions in this offer part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_count: Option<i32>,

    /// Total quantity value for this offer part (simplified - Gesamtmengeangebotsteil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity_value: Option<f64>,

    /// Total cost value for this offer part (simplified - Gesamtkostenangebotsteil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_value: Option<f64>,

    /// Delivery period start (simplified - Lieferzeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_period_start: Option<String>,

    /// Delivery period end (simplified - Lieferzeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_period_end: Option<String>,
}

impl Bo4eObject for OfferPart {
    fn type_name_german() -> &'static str {
        "Angebotsteil"
    }

    fn type_name_english() -> &'static str {
        "OfferPart"
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
    fn test_offer_part_default() {
        let part = OfferPart::default();
        assert!(part.request_sub_reference.is_none());
        assert!(part.total_cost_value.is_none());
    }

    #[test]
    fn test_offer_part_serialize() {
        let part = OfferPart {
            request_sub_reference: Some("LOT-001".to_string()),
            position_count: Some(5),
            total_quantity_value: Some(50000.0),
            total_cost_value: Some(12500.0),
            ..Default::default()
        };

        let json = serde_json::to_string(&part).unwrap();
        assert!(json.contains(r#""requestSubReference":"LOT-001""#));
        assert!(json.contains(r#""positionCount":5"#));
    }

    #[test]
    fn test_offer_part_roundtrip() {
        let part = OfferPart {
            meta: Bo4eMeta::with_type("Angebotsteil"),
            request_sub_reference: Some("LOT-002".to_string()),
            position_count: Some(3),
            total_quantity_value: Some(30000.0),
            total_cost_value: Some(7500.0),
            delivery_period_start: Some("2024-01-01T00:00:00+01:00".to_string()),
            delivery_period_end: Some("2024-12-31T23:59:59+01:00".to_string()),
        };

        let json = serde_json::to_string(&part).unwrap();
        let parsed: OfferPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(OfferPart::type_name_german(), "Angebotsteil");
        assert_eq!(OfferPart::type_name_english(), "OfferPart");
    }
}
