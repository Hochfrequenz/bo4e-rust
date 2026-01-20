//! External cost block (Fremdkostenblock) component.

use serde::{Deserialize, Serialize};

use crate::enums::CostClass;
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::Amount;

/// A block of external (third-party) costs.
///
/// German: Fremdkostenblock
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{ExternalCostBlock, Amount};
///
/// let cost_block = ExternalCostBlock {
///     designation: Some("Netzkosten Fremd".to_string()),
///     total_amount: Some(Amount::eur(350.0)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Fremdkostenblock"))]
#[serde(rename_all = "camelCase")]
pub struct ExternalCostBlock {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the cost block (Kostenblockbezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kostenblockbezeichnung"))]
    pub designation: Option<String>,

    /// Cost class (Kostenklasse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kostenklasse"))]
    pub cost_class: Option<CostClass>,

    /// Sum of all costs in this block (Summe Kostenblock)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "summeKostenblock"))]
    pub total_amount: Option<Amount>,
}

impl Bo4eObject for ExternalCostBlock {
    fn type_name_german() -> &'static str {
        "Fremdkostenblock"
    }

    fn type_name_english() -> &'static str {
        "ExternalCostBlock"
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
    fn test_external_cost_block() {
        let cost_block = ExternalCostBlock {
            designation: Some("Netzkosten Fremd".to_string()),
            cost_class: Some(CostClass::ExternalCosts),
            total_amount: Some(Amount::eur(350.0)),
            ..Default::default()
        };

        assert_eq!(cost_block.designation, Some("Netzkosten Fremd".to_string()));
    }

    #[test]
    fn test_default() {
        let cost_block = ExternalCostBlock::default();
        assert!(cost_block.designation.is_none());
        assert!(cost_block.total_amount.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let cost_block = ExternalCostBlock {
            designation: Some("Messkosten extern".to_string()),
            cost_class: Some(CostClass::InternalCosts),
            total_amount: Some(Amount::eur(75.50)),
            ..Default::default()
        };

        let json = serde_json::to_string(&cost_block).unwrap();
        let parsed: ExternalCostBlock = serde_json::from_str(&json).unwrap();
        assert_eq!(cost_block, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ExternalCostBlock::type_name_german(), "Fremdkostenblock");
        assert_eq!(ExternalCostBlock::type_name_english(), "ExternalCostBlock");
    }
}
