//! Offer (Angebot) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{OfferVariant, TimePeriod};
use crate::enums::{Division, OfferStatus};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An offer/quote for energy supply or services.
///
/// German: Angebot
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Offer;
/// use bo4e_core::enums::{Division, OfferStatus};
///
/// let offer = Offer {
///     offer_number: Some("A-2024-001".to_string()),
///     status: Some(OfferStatus::Binding),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Angebot"))]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Offer number (Angebotsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "angebotsnummer"))]
    pub offer_number: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Status of offer (Angebotsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "angebotsstatus"))]
    pub status: Option<OfferStatus>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Date the offer was created (Angebotsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "angebotsdatum"))]
    pub offer_date: Option<DateTime<Utc>>,

    /// Date until which the offer is valid (Gueltig bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigBis"))]
    pub valid_until: Option<DateTime<Utc>>,

    /// Delivery period (Lieferzeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "lieferzeitraum"))]
    pub delivery_period: Option<TimePeriod>,

    /// Offer variants (Angebotsvarianten)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "angebotsvarianten"))]
    pub variants: Vec<OfferVariant>,

    /// Reference to the bidder/supplier (Anbieter)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "anbieter"))]
    pub bidder: Option<Box<super::BusinessPartner>>,

    /// Reference to the customer (Kunde)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kunde"))]
    pub customer: Option<Box<super::BusinessPartner>>,

    /// Reference to associated tender (Ausschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ausschreibungId"))]
    pub tender_id: Option<String>,
}

impl Bo4eObject for Offer {
    fn type_name_german() -> &'static str {
        "Angebot"
    }

    fn type_name_english() -> &'static str {
        "Offer"
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
    fn test_offer_creation() {
        let offer = Offer {
            offer_number: Some("A-2024-001".to_string()),
            status: Some(OfferStatus::Binding),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(offer.status, Some(OfferStatus::Binding));
    }

    #[test]
    fn test_offer_with_variants() {
        let variant = OfferVariant {
            offer_status: Some(OfferStatus::NonBinding),
            parts_count: Some(3),
            ..Default::default()
        };

        let offer = Offer {
            offer_number: Some("A-2024-001".to_string()),
            variants: vec![variant],
            ..Default::default()
        };

        assert_eq!(offer.variants.len(), 1);
    }

    #[test]
    fn test_serialize() {
        let offer = Offer {
            meta: Bo4eMeta::with_type("Angebot"),
            offer_number: Some("A-123".to_string()),
            status: Some(OfferStatus::Binding),
            ..Default::default()
        };

        let json = serde_json::to_string(&offer).unwrap();
        assert!(json.contains(r#""offerNumber":"A-123""#));
    }

    #[test]
    fn test_roundtrip() {
        let offer = Offer {
            meta: Bo4eMeta::with_type("Angebot"),
            offer_number: Some("A-123".to_string()),
            description: Some("Test offer".to_string()),
            status: Some(OfferStatus::Binding),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&offer).unwrap();
        let parsed: Offer = serde_json::from_str(&json).unwrap();
        assert_eq!(offer, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Offer::type_name_german(), "Angebot");
        assert_eq!(Offer::type_name_english(), "Offer");
    }
}
