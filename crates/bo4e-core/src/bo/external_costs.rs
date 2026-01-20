//! External costs (Fremdkosten) business object.

use serde::{Deserialize, Serialize};

use crate::com::{Amount, ExternalCostBlock, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// External/third-party costs.
///
/// Represents costs from external parties like network operators,
/// metering service providers, etc.
///
/// German: Fremdkosten
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::ExternalCosts;
/// use bo4e_core::com::Amount;
/// use bo4e_core::enums::Division;
///
/// let external_costs = ExternalCosts {
///     designation: Some("Netzkosten 2024".to_string()),
///     division: Some(Division::Electricity),
///     total_amount: Some(Amount::eur(350.00)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalCosts {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the external costs (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designation: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Period the costs apply to (Abrechnungszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<TimePeriod>,

    /// Total amount (Gesamtbetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<Amount>,

    /// External cost blocks (Fremdkostenbloecke)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost_blocks: Vec<ExternalCostBlock>,

    /// External provider/party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_party: Option<Box<super::BusinessPartner>>,

    /// Related market location (Marktlokation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location: Option<Box<super::MarketLocation>>,
}

impl Bo4eObject for ExternalCosts {
    fn type_name_german() -> &'static str {
        "Fremdkosten"
    }

    fn type_name_english() -> &'static str {
        "ExternalCosts"
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
    use crate::com::ExternalCostBlock;

    #[test]
    fn test_external_costs_creation() {
        let external_costs = ExternalCosts {
            designation: Some("Netzkosten 2024".to_string()),
            division: Some(Division::Electricity),
            total_amount: Some(Amount::eur(350.00)),
            ..Default::default()
        };

        assert_eq!(external_costs.total_amount, Some(Amount::eur(350.00)));
    }

    #[test]
    fn test_external_costs_with_blocks() {
        let external_costs = ExternalCosts {
            designation: Some("Network Costs".to_string()),
            cost_blocks: vec![
                ExternalCostBlock {
                    designation: Some("Network usage".to_string()),
                    total_amount: Some(Amount::eur(200.0)),
                    ..Default::default()
                },
                ExternalCostBlock {
                    designation: Some("Metering".to_string()),
                    total_amount: Some(Amount::eur(50.0)),
                    ..Default::default()
                },
            ],
            total_amount: Some(Amount::eur(250.0)),
            ..Default::default()
        };

        assert_eq!(external_costs.cost_blocks.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let external_costs = ExternalCosts {
            meta: Bo4eMeta::with_type("Fremdkosten"),
            designation: Some("Test External Costs".to_string()),
            total_amount: Some(Amount::eur(100.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&external_costs).unwrap();
        assert!(json.contains(r#""designation":"Test External Costs""#));
        assert!(json.contains(r#""_typ":"Fremdkosten""#));
    }

    #[test]
    fn test_roundtrip() {
        let external_costs = ExternalCosts {
            meta: Bo4eMeta::with_type("Fremdkosten"),
            designation: Some("Gas Network Costs".to_string()),
            description: Some("External gas network costs".to_string()),
            division: Some(Division::Gas),
            total_amount: Some(Amount::eur(180.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&external_costs).unwrap();
        let parsed: ExternalCosts = serde_json::from_str(&json).unwrap();
        assert_eq!(external_costs, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ExternalCosts::type_name_german(), "Fremdkosten");
        assert_eq!(ExternalCosts::type_name_english(), "ExternalCosts");
    }
}
