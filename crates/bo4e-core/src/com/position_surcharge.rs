//! Position surcharge (PositionsAufAbschlag) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, SurchargeType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A surcharge or discount applied to a specific position.
///
/// German: PositionsAufAbschlag
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::PositionSurcharge;
///
/// let surcharge = PositionSurcharge {
///     description: Some("Sonderrabatt".to_string()),
///     value: Some(-10.0),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "PositionsAufAbschlag"))]
#[serde(rename_all = "camelCase")]
pub struct PositionSurcharge {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub description: Option<String>,

    /// Type of surcharge (AufAbschlagstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aufAbschlagstyp"))]
    pub surcharge_type: Option<SurchargeType>,

    /// Value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "waehrung"))]
    pub currency: Option<Currency>,
}

impl Bo4eObject for PositionSurcharge {
    fn type_name_german() -> &'static str {
        "PositionsAufAbschlag"
    }

    fn type_name_english() -> &'static str {
        "PositionSurcharge"
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
    fn test_position_surcharge() {
        let surcharge = PositionSurcharge {
            description: Some("Position discount".to_string()),
            surcharge_type: Some(SurchargeType::Absolute),
            value: Some(-5.0),
            currency: Some(Currency::Eur),
            ..Default::default()
        };

        assert_eq!(surcharge.value, Some(-5.0));
    }

    #[test]
    fn test_percentage_surcharge() {
        let surcharge = PositionSurcharge {
            description: Some("10% markup".to_string()),
            surcharge_type: Some(SurchargeType::Relative),
            value: Some(10.0),
            ..Default::default()
        };

        assert_eq!(surcharge.surcharge_type, Some(SurchargeType::Relative));
    }

    #[test]
    fn test_default() {
        let surcharge = PositionSurcharge::default();
        assert!(surcharge.description.is_none());
        assert!(surcharge.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let surcharge = PositionSurcharge {
            description: Some("Test surcharge".to_string()),
            surcharge_type: Some(SurchargeType::Absolute),
            value: Some(15.50),
            currency: Some(Currency::Eur),
            ..Default::default()
        };

        let json = serde_json::to_string(&surcharge).unwrap();
        let parsed: PositionSurcharge = serde_json::from_str(&json).unwrap();
        assert_eq!(surcharge, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            PositionSurcharge::type_name_german(),
            "PositionsAufAbschlag"
        );
        assert_eq!(PositionSurcharge::type_name_english(), "PositionSurcharge");
    }
}
