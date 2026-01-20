//! Invoice position (Rechnungsposition) component.

use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Position within an invoice.
///
/// Invoices are structured through invoice positions. Each invoice part
/// bills a self-contained service.
///
/// German: Rechnungsposition
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::InvoicePosition;
///
/// let position = InvoicePosition {
///     position_number: Some(1),
///     position_text: Some("Electricity delivery January 2024".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Rechnungsposition"))]
#[serde(rename_all = "camelCase")]
pub struct InvoicePosition {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Sequential number for the invoice position (Positionsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "positionsnummer"))]
    pub position_number: Option<i32>,

    /// Description of the billed position (Positionstext)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "positionstext"))]
    pub position_text: Option<String>,

    /// Delivery period start (simplified - Lieferungszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "lieferungszeitraumVon"))]
    pub delivery_period_start: Option<String>,

    /// Delivery period end (simplified - Lieferungszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "lieferungszeitraumBis"))]
    pub delivery_period_end: Option<String>,

    // Note: The following fields would typically reference other COM types
    // (Menge, Preis, Betrag, Steuerbetrag). Using simplified representations.
    /// Billed quantity value (simplified - Positionsmenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "positionsmenge"))]
    pub quantity_value: Option<f64>,

    /// Unit price value (simplified - Einzelpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einzelpreis"))]
    pub unit_price_value: Option<f64>,

    /// Total price value (simplified - Gesamtpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gesamtpreis"))]
    pub total_price_value: Option<f64>,

    /// BDEW article number (Artikelnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "artikelnummer"))]
    pub article_number: Option<String>,

    /// Article ID replacing BDEW article number (ArtikelId)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "artikelId"))]
    pub article_id: Option<String>,

    /// Tax amount value (simplified - Steuerbetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "steuerbetrag"))]
    pub tax_amount_value: Option<f64>,

    /// Time unit if price is time-based (Zeiteinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeiteinheit"))]
    pub time_unit: Option<Unit>,

    /// Time-based quantity value (simplified - Zeitbezogene Menge)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitbezogeneMenge"))]
    pub time_based_quantity_value: Option<f64>,
}

impl Bo4eObject for InvoicePosition {
    fn type_name_german() -> &'static str {
        "Rechnungsposition"
    }

    fn type_name_english() -> &'static str {
        "InvoicePosition"
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
    fn test_invoice_position_default() {
        let pos = InvoicePosition::default();
        assert!(pos.position_number.is_none());
        assert!(pos.position_text.is_none());
    }

    #[test]
    fn test_invoice_position_serialize() {
        let pos = InvoicePosition {
            position_number: Some(1),
            position_text: Some("Electricity consumption".to_string()),
            quantity_value: Some(1500.0),
            unit_price_value: Some(0.32),
            total_price_value: Some(480.0),
            ..Default::default()
        };

        let json = serde_json::to_string(&pos).unwrap();
        assert!(json.contains(r#""positionNumber":1"#));
        assert!(json.contains(r#""positionText":"Electricity consumption""#));
        assert!(json.contains(r#""quantityValue":1500"#));
    }

    #[test]
    fn test_invoice_position_roundtrip() {
        let pos = InvoicePosition {
            meta: Bo4eMeta::with_type("Rechnungsposition"),
            position_number: Some(2),
            position_text: Some("Network usage fee".to_string()),
            delivery_period_start: Some("2024-01-01T00:00:00+01:00".to_string()),
            delivery_period_end: Some("2024-01-31T23:59:59+01:00".to_string()),
            quantity_value: Some(1500.0),
            unit_price_value: Some(0.08),
            total_price_value: Some(120.0),
            article_number: Some("9900001000013".to_string()),
            article_id: None,
            tax_amount_value: Some(22.80),
            time_unit: None,
            time_based_quantity_value: None,
        };

        let json = serde_json::to_string(&pos).unwrap();
        let parsed: InvoicePosition = serde_json::from_str(&json).unwrap();
        assert_eq!(pos, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(InvoicePosition::type_name_german(), "Rechnungsposition");
        assert_eq!(InvoicePosition::type_name_english(), "InvoicePosition");
    }
}
