//! Price (Preis) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, PriceStatus, PriceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price with value, currency, and unit.
///
/// German: Preis
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Price;
/// use bo4e_core::enums::{Currency, Unit};
///
/// let price = Price {
///     value: Some(0.25),
///     currency: Some(Currency::Eur),
///     reference_unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Preis"))]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Price value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "waehrung"))]
    pub currency: Option<Currency>,

    /// Unit that the price applies to (Bezugswert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezugswert"))]
    pub reference_unit: Option<Unit>,

    /// Type of price (Preistyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preistyp"))]
    pub price_type: Option<PriceType>,

    /// Status of the price (Preisstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisstatus"))]
    pub status: Option<PriceStatus>,
}

impl Bo4eObject for Price {
    fn type_name_german() -> &'static str {
        "Preis"
    }

    fn type_name_english() -> &'static str {
        "Price"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl Price {
    /// Create a price in EUR per kWh.
    pub fn eur_per_kwh(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        }
    }

    /// Create a price in EUR per month (base price).
    pub fn eur_per_month(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::Month),
            price_type: Some(PriceType::BasePrice),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_price() {
        let price = Price::eur_per_kwh(0.30);
        assert_eq!(price.value, Some(0.30));
        assert_eq!(price.currency, Some(Currency::Eur));
        assert_eq!(price.reference_unit, Some(Unit::KilowattHour));
    }

    #[test]
    fn test_base_price() {
        let price = Price::eur_per_month(12.50);
        assert_eq!(price.value, Some(12.50));
        assert_eq!(price.currency, Some(Currency::Eur));
        assert_eq!(price.reference_unit, Some(Unit::Month));
        assert_eq!(price.price_type, Some(PriceType::BasePrice));
    }

    #[test]
    fn test_serialize() {
        let price = Price {
            value: Some(12.50),
            currency: Some(Currency::Eur),
            ..Default::default()
        };

        let json = serde_json::to_string(&price).unwrap();
        assert!(json.contains(r#""value":12.5"#));
        assert!(json.contains(r#""currency":"EUR""#));
    }

    #[test]
    fn test_roundtrip() {
        let price = Price::eur_per_kwh(0.2567);
        let json = serde_json::to_string(&price).unwrap();
        let parsed: Price = serde_json::from_str(&json).unwrap();
        assert_eq!(price, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Price::type_name_german(), "Preis");
        assert_eq!(Price::type_name_english(), "Price");
    }
}
