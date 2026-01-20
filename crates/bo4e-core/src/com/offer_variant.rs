//! Offer variant (Angebotsvariante) component.

use serde::{Deserialize, Serialize};

use crate::enums::OfferStatus;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Variant of an offer with different calculation options.
///
/// German: Angebotsvariante
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::OfferVariant;
/// use bo4e_core::enums::OfferStatus;
///
/// let variant = OfferVariant {
///     offer_status: Some(OfferStatus::NonBinding),
///     creation_date: Some("2024-01-15T10:00:00+01:00".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Angebotsvariante"))]
#[serde(rename_all = "camelCase")]
pub struct OfferVariant {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Status of the offer (Angebotsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "angebotsstatus"))]
    pub offer_status: Option<OfferStatus>,

    /// Creation date of the offer variant (Erstellungsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "erstellungsdatum"))]
    pub creation_date: Option<String>,

    /// Binding deadline - until this time the offer variant is valid (Bindefrist)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bindefrist"))]
    pub binding_deadline: Option<String>,

    // Note: The following fields would typically reference other COM types
    // (Angebotsteil, Menge, Betrag) which will be added later.
    /// Number of offer parts in this variant
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "anzahlTeile"))]
    pub parts_count: Option<i32>,

    /// Total quantity value across all offer parts (simplified - Gesamtmenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gesamtmenge"))]
    pub total_quantity_value: Option<f64>,

    /// Total cost value across all offer parts (simplified - Gesamtkosten)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gesamtkosten"))]
    pub total_cost_value: Option<f64>,
}

impl Bo4eObject for OfferVariant {
    fn type_name_german() -> &'static str {
        "Angebotsvariante"
    }

    fn type_name_english() -> &'static str {
        "OfferVariant"
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
    fn test_offer_variant_default() {
        let variant = OfferVariant::default();
        assert!(variant.offer_status.is_none());
        assert!(variant.creation_date.is_none());
    }

    #[test]
    fn test_offer_variant_serialize() {
        let variant = OfferVariant {
            offer_status: Some(OfferStatus::Binding),
            creation_date: Some("2024-03-01T09:00:00+01:00".to_string()),
            binding_deadline: Some("2024-04-01T23:59:59+02:00".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&variant).unwrap();
        assert!(json.contains(r#""offerStatus":"VERBINDLICH""#));
        assert!(json.contains(r#""creationDate":"#));
        assert!(json.contains(r#""bindingDeadline":"#));
    }

    #[test]
    fn test_offer_variant_roundtrip() {
        let variant = OfferVariant {
            meta: Bo4eMeta::with_type("Angebotsvariante"),
            offer_status: Some(OfferStatus::NonBinding),
            creation_date: Some("2024-02-15T10:30:00+01:00".to_string()),
            binding_deadline: Some("2024-03-15T23:59:59+01:00".to_string()),
            parts_count: Some(2),
            total_quantity_value: Some(100000.0),
            total_cost_value: Some(25000.0),
        };

        let json = serde_json::to_string(&variant).unwrap();
        let parsed: OfferVariant = serde_json::from_str(&json).unwrap();
        assert_eq!(variant, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(OfferVariant::type_name_german(), "Angebotsvariante");
        assert_eq!(OfferVariant::type_name_english(), "OfferVariant");
    }
}
