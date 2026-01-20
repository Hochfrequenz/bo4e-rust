//! Costs (Kosten) business object.

use serde::{Deserialize, Serialize};

use crate::com::{Amount, CostBlock, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A cost breakdown/summary.
///
/// German: Kosten
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Costs;
/// use bo4e_core::com::Amount;
/// use bo4e_core::enums::Division;
///
/// let costs = Costs {
///     designation: Some("Jahreskosten 2024".to_string()),
///     division: Some(Division::Electricity),
///     total_amount: Some(Amount::eur(2500.00)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kosten"))]
#[serde(rename_all = "camelCase")]
pub struct Costs {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the cost summary (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezeichnung"))]
    pub designation: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Period the costs apply to (Abrechnungszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "abrechnungszeitraum"))]
    pub period: Option<TimePeriod>,

    /// Total amount (Gesamtbetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gesamtbetrag"))]
    pub total_amount: Option<Amount>,

    /// Cost blocks (Kostenbloecke)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kostenbloecke"))]
    pub cost_blocks: Vec<CostBlock>,

    /// Related market location (Marktlokation)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktlokation"))]
    pub market_location: Option<Box<super::MarketLocation>>,
}

impl Bo4eObject for Costs {
    fn type_name_german() -> &'static str {
        "Kosten"
    }

    fn type_name_english() -> &'static str {
        "Costs"
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
    use crate::com::CostBlock;

    #[test]
    fn test_costs_creation() {
        let costs = Costs {
            designation: Some("Jahreskosten 2024".to_string()),
            division: Some(Division::Electricity),
            total_amount: Some(Amount::eur(2500.00)),
            ..Default::default()
        };

        assert_eq!(costs.total_amount, Some(Amount::eur(2500.00)));
    }

    #[test]
    fn test_costs_with_blocks() {
        let costs = Costs {
            designation: Some("Annual Costs".to_string()),
            cost_blocks: vec![
                CostBlock {
                    designation: Some("Energy".to_string()),
                    total_amount: Some(Amount::eur(1500.0)),
                    ..Default::default()
                },
                CostBlock {
                    designation: Some("Network".to_string()),
                    total_amount: Some(Amount::eur(800.0)),
                    ..Default::default()
                },
            ],
            total_amount: Some(Amount::eur(2300.0)),
            ..Default::default()
        };

        assert_eq!(costs.cost_blocks.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let costs = Costs {
            meta: Bo4eMeta::with_type("Kosten"),
            designation: Some("Test Costs".to_string()),
            total_amount: Some(Amount::eur(1000.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&costs).unwrap();
        assert!(json.contains(r#""designation":"Test Costs""#));
        assert!(json.contains(r#""_typ":"Kosten""#));
    }

    #[test]
    fn test_roundtrip() {
        let costs = Costs {
            meta: Bo4eMeta::with_type("Kosten"),
            designation: Some("Gas Costs".to_string()),
            description: Some("Annual gas costs".to_string()),
            division: Some(Division::Gas),
            total_amount: Some(Amount::eur(1234.56)),
            ..Default::default()
        };

        let json = serde_json::to_string(&costs).unwrap();
        let parsed: Costs = serde_json::from_str(&json).unwrap();
        assert_eq!(costs, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Costs::type_name_german(), "Kosten");
        assert_eq!(Costs::type_name_english(), "Costs");
    }
}
