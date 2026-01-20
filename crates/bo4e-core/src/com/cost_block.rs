//! Cost block (Kostenblock) component.

use serde::{Deserialize, Serialize};

use crate::enums::CostClass;
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::{Amount, CostPosition};

/// A block of costs containing multiple cost positions.
///
/// German: Kostenblock
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{CostBlock, Amount};
///
/// let cost_block = CostBlock {
///     designation: Some("Netzkosten".to_string()),
///     total_amount: Some(Amount::eur(500.0)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kostenblock"))]
#[serde(rename_all = "camelCase")]
pub struct CostBlock {
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

    /// Sum of all cost positions in this block (Summe Kostenblock)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "summeKostenblock"))]
    pub total_amount: Option<Amount>,

    /// Individual cost positions (Kostenpositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kostenpositionen"))]
    pub positions: Vec<CostPosition>,
}

impl Bo4eObject for CostBlock {
    fn type_name_german() -> &'static str {
        "Kostenblock"
    }

    fn type_name_english() -> &'static str {
        "CostBlock"
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
    use crate::com::Price;

    #[test]
    fn test_cost_block() {
        let cost_block = CostBlock {
            designation: Some("Netzkosten".to_string()),
            cost_class: Some(CostClass::ExternalCosts),
            total_amount: Some(Amount::eur(500.0)),
            positions: vec![
                CostPosition {
                    title: Some("Arbeitspreis Netz".to_string()),
                    amount: Some(Amount::eur(300.0)),
                    ..Default::default()
                },
                CostPosition {
                    title: Some("Leistungspreis Netz".to_string()),
                    amount: Some(Amount::eur(200.0)),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(cost_block.designation, Some("Netzkosten".to_string()));
        assert_eq!(cost_block.positions.len(), 2);
    }

    #[test]
    fn test_default() {
        let cost_block = CostBlock::default();
        assert!(cost_block.designation.is_none());
        assert!(cost_block.total_amount.is_none());
        assert!(cost_block.positions.is_empty());
    }

    #[test]
    fn test_serialize() {
        let cost_block = CostBlock {
            designation: Some("Messkosten".to_string()),
            total_amount: Some(Amount::eur(100.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&cost_block).unwrap();
        assert!(json.contains(r#""designation":"Messkosten""#));
    }

    #[test]
    fn test_roundtrip() {
        let cost_block = CostBlock {
            designation: Some("Energiekosten".to_string()),
            cost_class: Some(CostClass::Procurement),
            total_amount: Some(Amount::eur(1234.56)),
            positions: vec![CostPosition {
                title: Some("Arbeitspreis Energie".to_string()),
                unit_price: Some(Price::eur_per_kwh(0.25)),
                ..Default::default()
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&cost_block).unwrap();
        let parsed: CostBlock = serde_json::from_str(&json).unwrap();
        assert_eq!(cost_block, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(CostBlock::type_name_german(), "Kostenblock");
        assert_eq!(CostBlock::type_name_english(), "CostBlock");
    }
}
