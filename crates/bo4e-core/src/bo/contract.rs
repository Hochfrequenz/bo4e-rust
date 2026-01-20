//! Contract (Vertrag) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{ContractConditions, ContractPart, TimePeriod};
use crate::enums::{ContractStatus, ContractType, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A contract between parties.
///
/// German: Vertrag
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Contract;
/// use bo4e_core::enums::{ContractStatus, ContractType, Division};
///
/// let contract = Contract {
///     contract_number: Some("V-2024-001".to_string()),
///     contract_type: Some(ContractType::EnergySupplyContract),
///     status: Some(ContractStatus::Active),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Contract number (Vertragsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of contract (Vertragsart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<ContractType>,

    /// Status of contract (Vertragsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContractStatus>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Contract start date (Vertragsbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_start: Option<DateTime<Utc>>,

    /// Contract end date (Vertragsende)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_end: Option<DateTime<Utc>>,

    /// Signing date (Unterzeichnungsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_date: Option<DateTime<Utc>>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Contract conditions (Vertragskonditionen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<ContractConditions>,

    /// Contract parts (Vertragsteile)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<ContractPart>,

    /// Contracting party (Vertragspartner)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_partner: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for Contract {
    fn type_name_german() -> &'static str {
        "Vertrag"
    }

    fn type_name_english() -> &'static str {
        "Contract"
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
    fn test_supply_contract() {
        let contract = Contract {
            contract_number: Some("V-2024-001".to_string()),
            contract_type: Some(ContractType::EnergySupplyContract),
            status: Some(ContractStatus::Active),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(contract.status, Some(ContractStatus::Active));
    }

    #[test]
    fn test_serialize() {
        let contract = Contract {
            meta: Bo4eMeta::with_type("Vertrag"),
            contract_number: Some("V-123".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&contract).unwrap();
        assert!(json.contains(r#""contractNumber":"V-123""#));
    }

    #[test]
    fn test_roundtrip() {
        let contract = Contract {
            meta: Bo4eMeta::with_type("Vertrag"),
            contract_number: Some("V-123".to_string()),
            description: Some("Test contract".to_string()),
            contract_type: Some(ContractType::EnergySupplyContract),
            status: Some(ContractStatus::Active),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&contract).unwrap();
        let parsed: Contract = serde_json::from_str(&json).unwrap();
        assert_eq!(contract, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Contract::type_name_german(), "Vertrag");
        assert_eq!(Contract::type_name_english(), "Contract");
    }
}
