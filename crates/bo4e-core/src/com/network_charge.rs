//! Network charge (Netzentgelt) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, PriceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A network charge/fee component.
///
/// German: Netzentgelt
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::NetworkCharge;
/// use bo4e_core::enums::PriceType;
///
/// let charge = NetworkCharge {
///     price_type: Some(PriceType::WorkingPriceSingleTariff),
///     value: Some(5.82),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Netzentgelt"))]
#[serde(rename_all = "camelCase")]
pub struct NetworkCharge {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of price (Preistyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preistyp"))]
    pub price_type: Option<PriceType>,

    /// Charge value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "waehrung"))]
    pub currency: Option<Currency>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezugseinheit"))]
    pub reference_unit: Option<Unit>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Network operator code (Netzbetreiber)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzbetreiber"))]
    pub network_operator_code: Option<String>,
}

impl Bo4eObject for NetworkCharge {
    fn type_name_german() -> &'static str {
        "Netzentgelt"
    }

    fn type_name_english() -> &'static str {
        "NetworkCharge"
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
    fn test_work_price_charge() {
        let charge = NetworkCharge {
            price_type: Some(PriceType::WorkingPriceSingleTariff),
            value: Some(5.82),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        assert_eq!(charge.price_type, Some(PriceType::WorkingPriceSingleTariff));
        assert_eq!(charge.value, Some(5.82));
    }

    #[test]
    fn test_base_price_charge() {
        let charge = NetworkCharge {
            price_type: Some(PriceType::BasePrice),
            value: Some(55.0),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::Year),
            network_operator_code: Some("9900001".to_string()),
            ..Default::default()
        };

        assert_eq!(charge.price_type, Some(PriceType::BasePrice));
    }

    #[test]
    fn test_default() {
        let charge = NetworkCharge::default();
        assert!(charge.price_type.is_none());
        assert!(charge.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let charge = NetworkCharge {
            price_type: Some(PriceType::WorkingPriceSingleTariff),
            value: Some(6.25),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            description: Some("Arbeitspreis Netznutzung".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&charge).unwrap();
        let parsed: NetworkCharge = serde_json::from_str(&json).unwrap();
        assert_eq!(charge, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(NetworkCharge::type_name_german(), "Netzentgelt");
        assert_eq!(NetworkCharge::type_name_english(), "NetworkCharge");
    }
}
