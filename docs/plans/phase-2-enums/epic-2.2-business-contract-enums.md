# Epic 2.2: Business & Contract Enumerations

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement all business, contract, and organization-related enumerations (~35 enums).

**Architecture:** Each enum is a separate module file with German serde aliases.

**Tech Stack:** Rust enums, serde derive macros

---

## Enums in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `BusinessPartnerRole` | `Geschaeftspartnerrolle` | Role of business partner |
| `MarketRole` | `Marktrolle` | Market participant role |
| `OrganizationType` | `Organisationstyp` | Type of organization |
| `ContactType` | `Kontaktart` | Type of contact method |
| `Title` | `Anrede` | Salutation/title |
| `Gender` | `Geschlecht` | Gender |
| `ContractType` | `Vertragsart` | Type of contract |
| `ContractStatus` | `Vertragsstatus` | Status of contract |
| `ContractForm` | `Vertragsform` | Form of contract |
| `CustomerType` | `Kundentyp` | Type of customer |
| `CustomerGroup` | `Kundengruppe` | Customer group |
| `InvoiceType` | `Rechnungstyp` | Type of invoice |
| `InvoiceStatus` | `Rechnungsstatus` | Status of invoice |
| `PaymentMethod` | `Zahlungsmethode` | Payment method |
| `PaymentStatus` | `Zahlungsstatus` | Payment status |
| `DocumentType` | `Dokumenttyp` | Type of document |
| `ProcessStatus` | `Prozessstatus` | Status of process |
| `TransactionReason` | `Transaktionsgrund` | Reason for transaction |
| `CancellationReason` | `Kuendigungsgrund` | Reason for cancellation |
| `TerminationType` | `Kuendigungsart` | Type of termination |
| `BillingPeriod` | `Abrechnungsperiode` | Billing period |
| `BillingType` | `Abrechnungsart` | Type of billing |
| `TenderType` | `Ausschreibungstyp` | Type of tender |
| `TenderStatus` | `Ausschreibungsstatus` | Status of tender |
| `OfferStatus` | `Angebotsstatus` | Status of offer |
| `LegalForm` | `Rechtsform` | Legal form of organization |
| `Language` | `Sprache` | Language |
| `Country` | `Landescode` | Country code |
| `AddressType` | `Adresstyp` | Type of address |
| `CommunicationType` | `Kommunikationstyp` | Type of communication |
| `SeriesType` | `Serientyp` | Type of series |
| `SupplyStatus` | `Versorgungsstatus` | Supply status |
| `BalanceGroupType` | `Bilanzkreistyp` | Balance group type |
| `PortfolioType` | `Portfoliotyp` | Portfolio type |
| `ServiceType` | `Dienstleistungstyp` | Type of service |

---

## Task 1: Create BusinessPartnerRole Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/business_partner_role.rs`

**Step 1: Write the implementation**

```rust
//! Business partner role (Geschaeftspartnerrolle) enumeration.

use serde::{Deserialize, Serialize};

/// Role of a business partner in the energy market.
///
/// German: Geschaeftspartnerrolle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessPartnerRole {
    /// Customer (Kunde)
    #[serde(rename = "KUNDE")]
    Customer,

    /// Supplier (Lieferant)
    #[serde(rename = "LIEFERANT")]
    Supplier,

    /// Network operator (Netzbetreiber)
    #[serde(rename = "NETZBETREIBER")]
    NetworkOperator,

    /// Metering point operator (Messstellenbetreiber)
    #[serde(rename = "MESSSTELLENBETREIBER")]
    MeteringPointOperator,

    /// Metering service provider (Messdienstleister)
    #[serde(rename = "MESSDIENSTLEISTER")]
    MeteringServiceProvider,

    /// Transmission system operator (Uebertragungsnetzbetreiber)
    #[serde(rename = "UEBERTRAGUNGSNETZBETREIBER")]
    TransmissionSystemOperator,

    /// Distribution system operator (Verteilnetzbetreiber)
    #[serde(rename = "VERTEILNETZBETREIBER")]
    DistributionSystemOperator,

    /// Balance responsible party (Bilanzkreisverantwortlicher)
    #[serde(rename = "BILANZKREISVERANTWORTLICHER")]
    BalanceResponsibleParty,

    /// Balance coordinator (Bilanzkoordinator)
    #[serde(rename = "BILANZKOORDINATOR")]
    BalanceCoordinator,

    /// Energy service provider (Energiedienstleister)
    #[serde(rename = "ENERGIEDIENSTLEISTER")]
    EnergyServiceProvider,

    /// Basic supplier (Grundversorger)
    #[serde(rename = "GRUNDVERSORGER")]
    BasicSupplier,

    /// Interested party (Interessent)
    #[serde(rename = "INTERESSENT")]
    InterestedParty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let role = BusinessPartnerRole::NetworkOperator;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, r#""NETZBETREIBER""#);
        let parsed: BusinessPartnerRole = serde_json::from_str(&json).unwrap();
        assert_eq!(role, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core business_partner_role
git add crates/bo4e-core/src/enums/business_partner_role.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add BusinessPartnerRole (Geschaeftspartnerrolle) enum"
```

---

## Task 2: Create ContractStatus Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/contract_status.rs`

**Step 1: Write the implementation**

```rust
//! Contract status (Vertragsstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status of a contract in its lifecycle.
///
/// German: Vertragsstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractStatus {
    /// In negotiation (In Verhandlung)
    #[serde(rename = "IN_VERHANDLUNG")]
    InNegotiation,

    /// Active (Aktiv)
    #[serde(rename = "AKTIV")]
    Active,

    /// Terminated (Gekuendigt)
    #[serde(rename = "GEKUENDIGT")]
    Terminated,

    /// Ended (Beendet)
    #[serde(rename = "BEENDET")]
    Ended,

    /// Rejected (Abgelehnt)
    #[serde(rename = "ABGELEHNT")]
    Rejected,

    /// Withdrawn (Zurueckgezogen)
    #[serde(rename = "ZURUECKGEZOGEN")]
    Withdrawn,

    /// Draft (Entwurf)
    #[serde(rename = "ENTWURF")]
    Draft,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        for status in [
            ContractStatus::InNegotiation,
            ContractStatus::Active,
            ContractStatus::Terminated,
            ContractStatus::Ended,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: ContractStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core contract_status
git add crates/bo4e-core/src/enums/contract_status.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add ContractStatus (Vertragsstatus) enum"
```

---

## Task 3: Create CustomerType Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/customer_type.rs`

**Step 1: Write the implementation**

```rust
//! Customer type (Kundentyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of customer based on consumption characteristics.
///
/// German: Kundentyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomerType {
    /// Private household (Privat)
    #[serde(rename = "PRIVAT")]
    Private,

    /// Commercial/business (Gewerbe)
    #[serde(rename = "GEWERBE")]
    Commercial,

    /// Industrial (Industrie)
    #[serde(rename = "INDUSTRIE")]
    Industrial,

    /// Agriculture (Landwirtschaft)
    #[serde(rename = "LANDWIRTSCHAFT")]
    Agriculture,

    /// Public institutions (Oeffentlich)
    #[serde(rename = "OEFFENTLICH")]
    Public,

    /// Mixed (Gemischt)
    #[serde(rename = "GEMISCHT")]
    Mixed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&CustomerType::Private).unwrap(), r#""PRIVAT""#);
        assert_eq!(serde_json::to_string(&CustomerType::Commercial).unwrap(), r#""GEWERBE""#);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core customer_type
git add crates/bo4e-core/src/enums/customer_type.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add CustomerType (Kundentyp) enum"
```

---

## Tasks 4-35: Remaining Business/Contract Enums

Follow the established pattern for all remaining enums. Reference the Python source at `../BO4E-python/src/bo4e/enum/` for complete variant lists.

**Key enums to prioritize:**
1. `MarketRole` - Critical for market communication
2. `ContractType` - Contract classification
3. `InvoiceType` / `InvoiceStatus` - Billing
4. `Title` / `Gender` - Person data
5. `Country` - ISO 3166 codes

---

## Verification

```bash
cargo test -p bo4e-core -- --nocapture
cargo clippy -p bo4e-core
```

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 145 |
| Passed | 145 |
| Failed | 0 |
| Skipped | 0 |

Files created:
- `crates/bo4e-core/src/enums/business_partner_role.rs`
- `crates/bo4e-core/src/enums/market_role.rs`
- `crates/bo4e-core/src/enums/organization_type.rs`
- `crates/bo4e-core/src/enums/contact_type.rs`
- `crates/bo4e-core/src/enums/salutation.rs`
- `crates/bo4e-core/src/enums/title.rs`
- `crates/bo4e-core/src/enums/contract_type.rs`
- `crates/bo4e-core/src/enums/contract_status.rs`
- `crates/bo4e-core/src/enums/contract_form.rs`
- `crates/bo4e-core/src/enums/customer_type.rs`
- `crates/bo4e-core/src/enums/customer_group.rs`
- `crates/bo4e-core/src/enums/invoice_type.rs`
- `crates/bo4e-core/src/enums/invoice_status.rs`
- `crates/bo4e-core/src/enums/payment_method.rs`
- `crates/bo4e-core/src/enums/offer_status.rs`
- `crates/bo4e-core/src/enums/tender_type.rs`
- `crates/bo4e-core/src/enums/tender_status.rs`
- `crates/bo4e-core/src/enums/service_type.rs`
- `crates/bo4e-core/src/enums/area_type.rs`
- `crates/bo4e-core/src/enums/country.rs`

Files modified:
- `crates/bo4e-core/src/enums/mod.rs`
