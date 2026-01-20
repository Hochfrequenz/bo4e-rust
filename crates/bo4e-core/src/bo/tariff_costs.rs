//! Tariff costs (Tarifkosten) business object.

use serde::{Deserialize, Serialize};

use crate::com::{Amount, CostBlock, Price, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Tariff-related costs.
///
/// Represents the costs associated with a specific tariff.
///
/// German: Tarifkosten
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::TariffCosts;
/// use bo4e_core::com::{Amount, Price};
/// use bo4e_core::enums::Division;
///
/// let tariff_costs = TariffCosts {
///     designation: Some("Haushaltstarif Kosten".to_string()),
///     division: Some(Division::Electricity),
///     base_price_cost: Some(Amount::eur(119.40)),
///     working_price_cost: Some(Amount::eur(960.00)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffCosts {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the tariff costs (Bezeichnung)
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

    /// Base price applied (Grundpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price: Option<Price>,

    /// Base price cost (Grundpreiskosten)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price_cost: Option<Amount>,

    /// Working price applied (Arbeitspreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_price: Option<Price>,

    /// Working price cost (Arbeitspreiskosten)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_price_cost: Option<Amount>,

    /// Consumption quantity (Verbrauchsmenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption: Option<f64>,

    /// Cost blocks (Kostenbloecke)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost_blocks: Vec<CostBlock>,

    /// Reference to the tariff
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff: Option<Box<super::Tariff>>,
}

impl Bo4eObject for TariffCosts {
    fn type_name_german() -> &'static str {
        "Tarifkosten"
    }

    fn type_name_english() -> &'static str {
        "TariffCosts"
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
    fn test_tariff_costs_creation() {
        let tariff_costs = TariffCosts {
            designation: Some("Haushaltstarif Kosten".to_string()),
            division: Some(Division::Electricity),
            base_price: Some(Price::eur_per_month(9.95)),
            base_price_cost: Some(Amount::eur(119.40)),
            working_price: Some(Price::eur_per_kwh(0.32)),
            working_price_cost: Some(Amount::eur(960.00)),
            consumption: Some(3000.0),
            total_amount: Some(Amount::eur(1079.40)),
            ..Default::default()
        };

        assert_eq!(tariff_costs.consumption, Some(3000.0));
    }

    #[test]
    fn test_serialize() {
        let tariff_costs = TariffCosts {
            meta: Bo4eMeta::with_type("Tarifkosten"),
            designation: Some("Test Tariff Costs".to_string()),
            total_amount: Some(Amount::eur(500.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff_costs).unwrap();
        assert!(json.contains(r#""designation":"Test Tariff Costs""#));
        assert!(json.contains(r#""_typ":"Tarifkosten""#));
    }

    #[test]
    fn test_roundtrip() {
        let tariff_costs = TariffCosts {
            meta: Bo4eMeta::with_type("Tarifkosten"),
            designation: Some("Gas Tariff Costs".to_string()),
            description: Some("Annual gas tariff costs".to_string()),
            division: Some(Division::Gas),
            consumption: Some(15000.0),
            total_amount: Some(Amount::eur(1500.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff_costs).unwrap();
        let parsed: TariffCosts = serde_json::from_str(&json).unwrap();
        assert_eq!(tariff_costs, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TariffCosts::type_name_german(), "Tarifkosten");
        assert_eq!(TariffCosts::type_name_english(), "TariffCosts");
    }
}
