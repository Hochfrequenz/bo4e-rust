//! External cost position (Fremdkostenposition) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::{Amount, Price};

/// A cost position for external (third-party) costs.
///
/// German: Fremdkostenposition
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{ExternalCostPosition, Amount};
///
/// let position = ExternalCostPosition {
///     title: Some("Netzbetreiber ABC".to_string()),
///     amount: Some(Amount::eur(120.0)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Fremdkostenposition"))]
#[serde(rename_all = "camelCase")]
pub struct ExternalCostPosition {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Title of the position (Positionstitel)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "positionstitel"))]
    pub title: Option<String>,

    /// Total amount for this position (Betrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "betrag"))]
    pub amount: Option<Amount>,

    /// Description of the article (Artikelbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "artikelbezeichnung"))]
    pub article_description: Option<String>,

    /// Price per unit (Einzelpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einzelpreis"))]
    pub unit_price: Option<Price>,

    /// Start date inclusive (Von)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "von"))]
    pub start_date: Option<DateTime<Utc>>,

    /// End date exclusive (Bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bis"))]
    pub end_date: Option<DateTime<Utc>>,

    /// Quantity value (Menge - Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "mengeWert"))]
    pub quantity_value: Option<f64>,

    /// Quantity unit (Menge - Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "mengeEinheit"))]
    pub quantity_unit: Option<Unit>,

    /// External ID/reference (Link)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "link"))]
    pub external_reference: Option<String>,
}

impl Bo4eObject for ExternalCostPosition {
    fn type_name_german() -> &'static str {
        "Fremdkostenposition"
    }

    fn type_name_english() -> &'static str {
        "ExternalCostPosition"
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
    fn test_external_cost_position() {
        let position = ExternalCostPosition {
            title: Some("Netzbetreiber ABC".to_string()),
            article_description: Some("Netznutzungsentgelt".to_string()),
            amount: Some(Amount::eur(120.0)),
            quantity_value: Some(1500.0),
            quantity_unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        assert_eq!(position.title, Some("Netzbetreiber ABC".to_string()));
        assert_eq!(position.quantity_value, Some(1500.0));
    }

    #[test]
    fn test_default() {
        let position = ExternalCostPosition::default();
        assert!(position.title.is_none());
        assert!(position.amount.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let position = ExternalCostPosition {
            title: Some("Messstellenbetreiber".to_string()),
            amount: Some(Amount::eur(80.0)),
            external_reference: Some("MSB-2024-001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&position).unwrap();
        let parsed: ExternalCostPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(position, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            ExternalCostPosition::type_name_german(),
            "Fremdkostenposition"
        );
        assert_eq!(
            ExternalCostPosition::type_name_english(),
            "ExternalCostPosition"
        );
    }
}
