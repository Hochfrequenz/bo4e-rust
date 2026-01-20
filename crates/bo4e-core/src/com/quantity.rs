//! Quantity (Menge) component.

use serde::{Deserialize, Serialize};

use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A quantity with value and unit.
///
/// German: Menge
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Quantity;
/// use bo4e_core::enums::Unit;
///
/// let qty = Quantity {
///     value: Some(3500.0),
///     unit: Some(Unit::KilowattHour),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Menge"))]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Numeric value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einheit"))]
    pub unit: Option<Unit>,
}

impl Bo4eObject for Quantity {
    fn type_name_german() -> &'static str {
        "Menge"
    }

    fn type_name_english() -> &'static str {
        "Quantity"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl Quantity {
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
        let qty = Quantity::kwh(3500.0);
        assert_eq!(qty.value, Some(3500.0));
        assert_eq!(qty.unit, Some(Unit::KilowattHour));
    }

    #[test]
    fn test_gas_quantity() {
        let qty = Quantity::cubic_meters(1500.0);
        assert_eq!(qty.unit, Some(Unit::CubicMeter));
    }

    #[test]
    fn test_roundtrip() {
        let qty = Quantity::kwh(12345.67);
        let json = serde_json::to_string(&qty).unwrap();
        let parsed: Quantity = serde_json::from_str(&json).unwrap();
        assert_eq!(qty, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Quantity::type_name_german(), "Menge");
        assert_eq!(Quantity::type_name_english(), "Quantity");
    }
}
