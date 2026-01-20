//! Tariff price (Tarifpreis) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, PriceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A tariff price with its type and value.
///
/// German: Tarifpreis
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TariffPrice;
/// use bo4e_core::enums::{Currency, PriceType, Unit};
///
/// let tariff_price = TariffPrice {
///     price_type: Some(PriceType::WorkPrice),
///     value: Some(0.30),
///     currency: Some(Currency::Cent),
///     reference_unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffPrice {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of tariff price (Preistyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<PriceType>,

    /// Price value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<Unit>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Bo4eObject for TariffPrice {
    fn type_name_german() -> &'static str {
        "Tarifpreis"
    }

    fn type_name_english() -> &'static str {
        "TariffPrice"
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
    fn test_work_price() {
        let price = TariffPrice {
            price_type: Some(PriceType::WorkPrice),
            value: Some(30.5),
            currency: Some(Currency::Cent),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        assert_eq!(price.price_type, Some(PriceType::WorkPrice));
        assert_eq!(price.value, Some(30.5));
    }

    #[test]
    fn test_base_price() {
        let price = TariffPrice {
            price_type: Some(PriceType::BasePrice),
            value: Some(12.50),
            currency: Some(Currency::Euro),
            reference_unit: Some(Unit::Month),
            description: Some("Monthly base fee".to_string()),
            ..Default::default()
        };

        assert_eq!(price.price_type, Some(PriceType::BasePrice));
    }

    #[test]
    fn test_default() {
        let price = TariffPrice::default();
        assert!(price.price_type.is_none());
        assert!(price.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let price = TariffPrice {
            price_type: Some(PriceType::WorkPrice),
            value: Some(25.75),
            currency: Some(Currency::Cent),
            reference_unit: Some(Unit::KilowattHour),
            description: Some("Peak rate".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price).unwrap();
        let parsed: TariffPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(price, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TariffPrice::type_name_german(), "Tarifpreis");
        assert_eq!(TariffPrice::type_name_english(), "TariffPrice");
    }
}
