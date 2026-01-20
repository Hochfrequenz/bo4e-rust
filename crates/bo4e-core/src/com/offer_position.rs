//! Offer position (Angebotsposition) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Position within an offer part.
///
/// Offers the individual components being offered. Example:
/// - Position quantity: 4000 kWh
/// - Position price: 24.56 ct/kWh
/// - Position cost: 982.40 EUR
///
/// German: Angebotsposition
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::OfferPosition;
///
/// let position = OfferPosition {
///     position_description: Some("Electricity supply".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferPosition {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description of the offer position (Positionsbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_description: Option<String>,

    // Note: The following fields would typically reference other COM types
    // (Preis, Menge, Betrag) which will be added in a later epic.
    // For now, we use simplified representations.

    /// Position price value (simplified - Positionspreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_price_value: Option<f64>,

    /// Position quantity value (simplified - Positionsmenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_quantity_value: Option<f64>,

    /// Position cost value (simplified - Positionskosten)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_cost_value: Option<f64>,
}

impl Bo4eObject for OfferPosition {
    fn type_name_german() -> &'static str {
        "Angebotsposition"
    }

    fn type_name_english() -> &'static str {
        "OfferPosition"
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
    fn test_offer_position_default() {
        let pos = OfferPosition::default();
        assert!(pos.position_description.is_none());
        assert!(pos.position_price_value.is_none());
    }

    #[test]
    fn test_offer_position_serialize() {
        let pos = OfferPosition {
            position_description: Some("Electricity delivery".to_string()),
            position_price_value: Some(24.56),
            position_quantity_value: Some(4000.0),
            position_cost_value: Some(982.40),
            ..Default::default()
        };

        let json = serde_json::to_string(&pos).unwrap();
        assert!(json.contains(r#""positionDescription":"Electricity delivery""#));
        assert!(json.contains("24.56"));
        assert!(json.contains("4000"));
    }

    #[test]
    fn test_offer_position_roundtrip() {
        let pos = OfferPosition {
            meta: Bo4eMeta::with_type("Angebotsposition"),
            position_description: Some("Gas supply".to_string()),
            position_price_value: Some(0.08),
            position_quantity_value: Some(10000.0),
            position_cost_value: Some(800.0),
        };

        let json = serde_json::to_string(&pos).unwrap();
        let parsed: OfferPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(pos, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(OfferPosition::type_name_german(), "Angebotsposition");
        assert_eq!(OfferPosition::type_name_english(), "OfferPosition");
    }
}
