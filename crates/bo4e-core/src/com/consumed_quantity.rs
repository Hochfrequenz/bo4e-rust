//! Consumed quantity (VerbrauchteQuantitaet) component.

use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A consumed quantity with value and unit.
///
/// German: VerbrauchteQuantitaet
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ConsumedQuantity;
/// use bo4e_core::enums::Unit;
///
/// let quantity = ConsumedQuantity {
///     value: Some(3500.0),
///     unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "VerbrauchteQuantitaet"))]
#[serde(rename_all = "camelCase")]
pub struct ConsumedQuantity {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// The quantity value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,
}

impl Bo4eObject for ConsumedQuantity {
    fn type_name_german() -> &'static str {
        "VerbrauchteQuantitaet"
    }

    fn type_name_english() -> &'static str {
        "ConsumedQuantity"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl ConsumedQuantity {
    /// Create a quantity in kWh.
    pub fn kwh(value: f64) -> Self {
        Self {
            value: Some(value),
            unit: Some(Unit::KilowattHour),
            ..Default::default()
        }
    }

    /// Create a quantity in cubic meters.
    pub fn cubic_meters(value: f64) -> Self {
        Self {
            value: Some(value),
            unit: Some(Unit::CubicMeter),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kwh_quantity() {
        let quantity = ConsumedQuantity::kwh(3500.0);
        assert_eq!(quantity.value, Some(3500.0));
        assert_eq!(quantity.unit, Some(Unit::KilowattHour));
    }

    #[test]
    fn test_cubic_meters() {
        let quantity = ConsumedQuantity::cubic_meters(1250.0);
        assert_eq!(quantity.value, Some(1250.0));
        assert_eq!(quantity.unit, Some(Unit::CubicMeter));
    }

    #[test]
    fn test_default() {
        let quantity = ConsumedQuantity::default();
        assert!(quantity.value.is_none());
        assert!(quantity.unit.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let quantity = ConsumedQuantity {
            value: Some(5678.9),
            unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        let json = serde_json::to_string(&quantity).unwrap();
        let parsed: ConsumedQuantity = serde_json::from_str(&json).unwrap();
        assert_eq!(quantity, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            ConsumedQuantity::type_name_german(),
            "VerbrauchteQuantitaet"
        );
        assert_eq!(ConsumedQuantity::type_name_english(), "ConsumedQuantity");
    }
}
