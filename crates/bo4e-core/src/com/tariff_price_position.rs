//! Tariff price position (Tarifpreisposition) component.

use serde::{Deserialize, Serialize};

use crate::enums::{CalculationMethod, PriceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::TariffPrice;

/// A tariff price position with its associated tariff prices.
///
/// German: Tarifpreisposition
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{TariffPricePosition, TariffPrice};
/// use bo4e_core::enums::PriceType;
///
/// let position = TariffPricePosition {
///     description: Some("Arbeitspreis".to_string()),
///     price_type: Some(PriceType::WorkingPriceSingleTariff),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffPricePosition {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description of the position (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of price (Preistyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<PriceType>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<Unit>,

    /// Calculation method (Berechnungsmethode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_method: Option<CalculationMethod>,

    /// Tariff prices (Tarifpreise)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prices: Vec<TariffPrice>,

    /// Article ID (Artikel-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
}

impl Bo4eObject for TariffPricePosition {
    fn type_name_german() -> &'static str {
        "Tarifpreisposition"
    }

    fn type_name_english() -> &'static str {
        "TariffPricePosition"
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
    fn test_tariff_price_position() {
        let position = TariffPricePosition {
            description: Some("Arbeitspreis HT/NT".to_string()),
            price_type: Some(PriceType::WorkingPriceSingleTariff),
            reference_unit: Some(Unit::KilowattHour),
            prices: vec![
                TariffPrice {
                    description: Some("HT".to_string()),
                    value: Some(30.5),
                    currency: Some(Currency::Eur),
                    ..Default::default()
                },
                TariffPrice {
                    description: Some("NT".to_string()),
                    value: Some(22.0),
                    currency: Some(Currency::Eur),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(position.prices.len(), 2);
    }

    #[test]
    fn test_default() {
        let position = TariffPricePosition::default();
        assert!(position.description.is_none());
        assert!(position.prices.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let position = TariffPricePosition {
            description: Some("Grundpreis".to_string()),
            price_type: Some(PriceType::BasePrice),
            article_id: Some("GP-001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&position).unwrap();
        let parsed: TariffPricePosition = serde_json::from_str(&json).unwrap();
        assert_eq!(position, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            TariffPricePosition::type_name_german(),
            "Tarifpreisposition"
        );
        assert_eq!(
            TariffPricePosition::type_name_english(),
            "TariffPricePosition"
        );
    }
}
