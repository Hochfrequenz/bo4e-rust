//! Tariff (Tarif) business object.

use serde::{Deserialize, Serialize};

use crate::com::{EnergyMix, Price, PriceTier, TariffCalculationParameter, TimePeriod};
use crate::enums::{CustomerType, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A tariff definition.
///
/// German: Tarif
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Tariff;
/// use bo4e_core::com::Price;
/// use bo4e_core::enums::{CustomerType, Division};
///
/// let tariff = Tariff {
///     tariff_name: Some("Haushaltstarif 2024".to_string()),
///     division: Some(Division::Electricity),
///     customer_type: Some(CustomerType::Private),
///     base_price: Some(Price::eur_per_month(9.95)),
///     working_price: Some(Price::eur_per_kwh(0.32)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Tarif"))]
#[serde(rename_all = "camelCase")]
pub struct Tariff {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Tariff name (Tarifname)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifname"))]
    pub tariff_name: Option<String>,

    /// Tariff description (Tarifbeschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifbeschreibung"))]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Target customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kundentyp"))]
    pub customer_type: Option<CustomerType>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitszeitraum"))]
    pub validity_period: Option<TimePeriod>,

    /// Base price (Grundpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "grundpreis"))]
    pub base_price: Option<Price>,

    /// Working price (Arbeitspreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "arbeitspreis"))]
    pub working_price: Option<Price>,

    /// Price tiers (Preisstaffeln)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisstaffeln"))]
    pub price_tiers: Vec<PriceTier>,

    /// Calculation parameters (Tarifberechnungsparameter)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "json-schema",
        schemars(rename = "tarifberechnungsparameter")
    )]
    pub calculation_parameters: Option<TariffCalculationParameter>,

    /// Energy mix composition (Energiemix)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energiemix"))]
    pub energy_mix: Option<EnergyMix>,

    /// Provider/supplier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "anbieter"))]
    pub supplier: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for Tariff {
    fn type_name_german() -> &'static str {
        "Tarif"
    }

    fn type_name_english() -> &'static str {
        "Tariff"
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
    fn test_household_tariff() {
        let tariff = Tariff {
            tariff_name: Some("Haushaltstarif 2024".to_string()),
            division: Some(Division::Electricity),
            customer_type: Some(CustomerType::Private),
            base_price: Some(Price::eur_per_month(9.95)),
            working_price: Some(Price::eur_per_kwh(0.32)),
            ..Default::default()
        };

        assert_eq!(tariff.customer_type, Some(CustomerType::Private));
        assert_eq!(tariff.division, Some(Division::Electricity));
    }

    #[test]
    fn test_tiered_tariff() {
        let tariff = Tariff {
            tariff_name: Some("Staffeltarif".to_string()),
            price_tiers: vec![
                PriceTier {
                    lower_limit: Some(0.0),
                    upper_limit: Some(1000.0),
                    unit_price: Some(0.35),
                    tier_number: Some(1),
                    ..Default::default()
                },
                PriceTier {
                    lower_limit: Some(1000.0),
                    upper_limit: Some(5000.0),
                    unit_price: Some(0.30),
                    tier_number: Some(2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(tariff.price_tiers.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let tariff = Tariff {
            meta: Bo4eMeta::with_type("Tarif"),
            tariff_name: Some("Basic Tariff".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff).unwrap();
        assert!(json.contains(r#""tariffName":"Basic Tariff""#));
        assert!(json.contains(r#""_typ":"Tarif""#));
    }

    #[test]
    fn test_roundtrip() {
        let tariff = Tariff {
            meta: Bo4eMeta::with_type("Tarif"),
            tariff_name: Some("Test Tariff".to_string()),
            description: Some("A test tariff".to_string()),
            division: Some(Division::Gas),
            customer_type: Some(CustomerType::Commercial),
            base_price: Some(Price::eur_per_month(15.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&tariff).unwrap();
        let parsed: Tariff = serde_json::from_str(&json).unwrap();
        assert_eq!(tariff, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Tariff::type_name_german(), "Tarif");
        assert_eq!(Tariff::type_name_english(), "Tariff");
    }
}
