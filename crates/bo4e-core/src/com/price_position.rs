//! Price position (Preisposition) component.

use serde::{Deserialize, Serialize};

use crate::enums::{CalculationMethod, PriceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::PriceTier;

/// A position in a price sheet with its associated price tiers.
///
/// German: Preisposition
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::PricePosition;
/// use bo4e_core::enums::PriceType;
///
/// let position = PricePosition {
///     description: Some("Arbeitspreis HT".to_string()),
///     price_type: Some(PriceType::WorkingPriceSingleTariff),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Preisposition"))]
#[serde(rename_all = "camelCase")]
pub struct PricePosition {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description/name of the price position (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub description: Option<String>,

    /// Type of price (Preistyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preistyp"))]
    pub price_type: Option<PriceType>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezugseinheit"))]
    pub reference_unit: Option<Unit>,

    /// Calculation method (Berechnungsmethode)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "berechnungsmethode"))]
    pub calculation_method: Option<CalculationMethod>,

    /// Price tiers (Preisstaffeln)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisstaffeln"))]
    pub tiers: Vec<PriceTier>,

    /// Article ID (Artikel-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "artikelId"))]
    pub article_id: Option<String>,

    /// BDEW article number (BDEW Artikelnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bdewArtikelnummer"))]
    pub bdew_article_number: Option<String>,
}

impl Bo4eObject for PricePosition {
    fn type_name_german() -> &'static str {
        "Preisposition"
    }

    fn type_name_english() -> &'static str {
        "PricePosition"
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
    fn test_work_price_position() {
        let position = PricePosition {
            description: Some("Arbeitspreis HT".to_string()),
            price_type: Some(PriceType::WorkingPriceSingleTariff),
            reference_unit: Some(Unit::KilowattHour),
            tiers: vec![PriceTier {
                unit_price: Some(0.30),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(
            position.price_type,
            Some(PriceType::WorkingPriceSingleTariff)
        );
        assert_eq!(position.tiers.len(), 1);
    }

    #[test]
    fn test_default() {
        let position = PricePosition::default();
        assert!(position.description.is_none());
        assert!(position.tiers.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let position = PricePosition {
            description: Some("Grundpreis".to_string()),
            price_type: Some(PriceType::BasePrice),
            reference_unit: Some(Unit::Month),
            bdew_article_number: Some("9990001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&position).unwrap();
        let parsed: PricePosition = serde_json::from_str(&json).unwrap();
        assert_eq!(position, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(PricePosition::type_name_german(), "Preisposition");
        assert_eq!(PricePosition::type_name_english(), "PricePosition");
    }
}
