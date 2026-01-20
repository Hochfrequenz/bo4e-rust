# Epic 4.2: Parties & Contracts Business Objects

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement business partner, contract, and offer business objects (~13 BOs).

**Architecture:** BOs with party relationships and contract lifecycle management.

**Tech Stack:** Rust structs, serde, Box<T> for entity references

---

## Business Objects in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `BusinessPartner` | `Geschaeftspartner` | A business partner entity |
| `Person` | `Person` | A natural person |
| `MarketParticipant` | `Marktteilnehmer` | Market participant |
| `Contract` | `Vertrag` | A contract |
| `BundleContract` | `Buendelvertrag` | Bundle contract |
| `Offer` | `Angebot` | An offer/quote |
| `Tender` | `Ausschreibung` | A tender/RFP |
| `Balancing` | `Bilanzierung` | Balance group data |
| `Region` | `Region` | Geographical region |
| `RegionalTariff` | `Regionaltarif` | Regional tariff definition |

---

## Task 1: Create BusinessPartner Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/business_partner.rs`

**Step 1: Write the implementation**

```rust
//! Business partner (Geschaeftspartner) business object.

use serde::{Deserialize, Serialize};
use crate::com::{Address, ContactMethod};
use crate::enums::BusinessPartnerRole;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A business partner (company or organization).
///
/// German: Geschaeftspartner
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessPartner {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Partner ID (Geschaeftspartner-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    /// Company/organization name (Name1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name1: Option<String>,

    /// Additional name line (Name2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name2: Option<String>,

    /// Additional name line (Name3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name3: Option<String>,

    /// Roles this partner has (Rollen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<BusinessPartnerRole>,

    /// Primary address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Contact methods (Kontaktwege)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact_methods: Vec<ContactMethod>,

    /// Commercial register number (Handelsregisternummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commercial_register_number: Option<String>,

    /// Tax ID (Steuernummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// VAT ID (Umsatzsteuer-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
}

impl Bo4eObject for BusinessPartner {
    fn type_name_german() -> &'static str { "Geschaeftspartner" }
    fn type_name_english() -> &'static str { "BusinessPartner" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supplier_partner() {
        let partner = BusinessPartner {
            name1: Some("Stadtwerke Musterstadt GmbH".to_string()),
            roles: vec![BusinessPartnerRole::Supplier],
            ..Default::default()
        };

        assert!(partner.roles.contains(&BusinessPartnerRole::Supplier));
    }

    #[test]
    fn test_multiple_roles() {
        let partner = BusinessPartner {
            name1: Some("Multi-Utility AG".to_string()),
            roles: vec![
                BusinessPartnerRole::Supplier,
                BusinessPartnerRole::NetworkOperator,
            ],
            ..Default::default()
        };

        assert_eq!(partner.roles.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let partner = BusinessPartner {
            meta: Bo4eMeta::with_type("Geschaeftspartner"),
            name1: Some("Test GmbH".to_string()),
            vat_id: Some("DE123456789".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&partner).unwrap();
        assert!(json.contains(r#""name1":"Test GmbH""#));
        assert!(json.contains(r#""vatId":"DE123456789""#));
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core business_partner
git add crates/bo4e-core/src/bo/business_partner.rs crates/bo4e-core/src/bo/mod.rs
git commit -m "feat(core): add BusinessPartner (Geschaeftspartner) business object"
```

---

## Task 2: Create Contract Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/contract.rs`

**Step 1: Write the implementation**

```rust
//! Contract (Vertrag) business object.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::com::{ContractConditions, ContractPart, TimePeriod};
use crate::enums::{ContractStatus, ContractType, Division};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A contract between parties.
///
/// German: Vertrag
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
    fn type_name_german() -> &'static str { "Vertrag" }
    fn type_name_english() -> &'static str { "Contract" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_contract() {
        let contract = Contract {
            contract_number: Some("V-2024-001".to_string()),
            contract_type: Some(ContractType::SupplyContract),
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
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core contract
git add crates/bo4e-core/src/bo/contract.rs crates/bo4e-core/src/bo/mod.rs
git commit -m "feat(core): add Contract (Vertrag) business object"
```

---

## Tasks 3-13: Remaining Party/Contract BOs

Follow the pattern for:
- `Person`
- `MarketParticipant`
- `BundleContract`
- `Offer`
- `Tender`
- `Balancing`
- `Region`
- `RegionalTariff`

---

## Verification

```bash
cargo test -p bo4e-core bo -- --nocapture
cargo clippy -p bo4e-core
```
