//! Bundle contract (Buendelvertrag) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::TimePeriod;
use crate::enums::{ContractStatus, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A bundle contract that combines multiple individual contracts.
///
/// German: Buendelvertrag
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::BundleContract;
/// use bo4e_core::enums::{ContractStatus, Division};
///
/// let bundle = BundleContract {
///     bundle_contract_number: Some("BV-2024-001".to_string()),
///     status: Some(ContractStatus::Active),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Buendelvertrag"))]
#[serde(rename_all = "camelCase")]
pub struct BundleContract {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Bundle contract number (Buendelvertragsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "buendelvertragsnummer"))]
    pub bundle_contract_number: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Status of bundle contract (Vertragsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "vertragsstatus"))]
    pub status: Option<ContractStatus>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Bundle contract start date (Vertragsbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "vertragsbeginn"))]
    pub contract_start: Option<DateTime<Utc>>,

    /// Bundle contract end date (Vertragsende)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "vertragsende"))]
    pub contract_end: Option<DateTime<Utc>>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitszeitraum"))]
    pub validity_period: Option<TimePeriod>,

    /// Individual contracts in this bundle (Einzelvertraege)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "einzelvertraege"))]
    pub individual_contracts: Vec<Box<super::Contract>>,

    /// Contracting party (Vertragspartner)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "vertragspartner"))]
    pub contract_partner: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for BundleContract {
    fn type_name_german() -> &'static str {
        "Buendelvertrag"
    }

    fn type_name_english() -> &'static str {
        "BundleContract"
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
    use crate::bo::Contract;
    use crate::enums::ContractType;

    #[test]
    fn test_bundle_contract_creation() {
        let bundle = BundleContract {
            bundle_contract_number: Some("BV-2024-001".to_string()),
            status: Some(ContractStatus::Active),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(bundle.status, Some(ContractStatus::Active));
    }

    #[test]
    fn test_bundle_with_contracts() {
        let contract1 = Box::new(Contract {
            contract_number: Some("V-001".to_string()),
            contract_type: Some(ContractType::EnergySupplyContract),
            ..Default::default()
        });

        let contract2 = Box::new(Contract {
            contract_number: Some("V-002".to_string()),
            contract_type: Some(ContractType::NetworkUsageContract),
            ..Default::default()
        });

        let bundle = BundleContract {
            bundle_contract_number: Some("BV-2024-001".to_string()),
            individual_contracts: vec![contract1, contract2],
            ..Default::default()
        };

        assert_eq!(bundle.individual_contracts.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let bundle = BundleContract {
            meta: Bo4eMeta::with_type("Buendelvertrag"),
            bundle_contract_number: Some("BV-123".to_string()),
            status: Some(ContractStatus::Active),
            ..Default::default()
        };

        let json = serde_json::to_string(&bundle).unwrap();
        assert!(json.contains(r#""bundleContractNumber":"BV-123""#));
    }

    #[test]
    fn test_roundtrip() {
        let bundle = BundleContract {
            meta: Bo4eMeta::with_type("Buendelvertrag"),
            bundle_contract_number: Some("BV-123".to_string()),
            description: Some("Test bundle".to_string()),
            status: Some(ContractStatus::Active),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&bundle).unwrap();
        let parsed: BundleContract = serde_json::from_str(&json).unwrap();
        assert_eq!(bundle, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(BundleContract::type_name_german(), "Buendelvertrag");
        assert_eq!(BundleContract::type_name_english(), "BundleContract");
    }
}
