# Epic 4.3: Pricing & Billing Business Objects

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement tariff, invoice, and cost-related business objects (~12 BOs).

**Architecture:** BOs with pricing structures and billing relationships.

**Tech Stack:** Rust structs, serde, decimal handling

---

## Business Objects in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `Tariff` | `Tarif` | A tariff definition |
| `TariffInfo` | `Tarifinfo` | Tariff information |
| `TariffPriceSheet` | `Tarifpreisblatt` | Tariff price sheet |
| `PriceSheet` | `Preisblatt` | Generic price sheet |
| `ServicePriceSheet` | `PreisblattDienstleistung` | Service price sheet |
| `HardwarePriceSheet` | `PreisblattHardware` | Hardware price sheet |
| `MeteringPriceSheet` | `PreisblattMessung` | Metering price sheet |
| `NetworkUsagePriceSheet` | `PreisblattNetznutzung` | Network usage price sheet |
| `ConcessionFeePriceSheet` | `PreisblattKonzessionsabgabe` | Concession fee price sheet |
| `Invoice` | `Rechnung` | An invoice |
| `Costs` | `Kosten` | Cost breakdown |
| `TariffCosts` | `Tarifkosten` | Tariff-related costs |
| `ExternalCosts` | `Fremdkosten` | External/third-party costs |

---

## Task 1: Create Invoice Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/invoice.rs`

**Step 1: Write the implementation**

```rust
//! Invoice (Rechnung) business object.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use crate::com::{Amount, InvoicePosition, TimePeriod};
use crate::enums::{Division, InvoiceStatus, InvoiceType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An invoice for energy services.
///
/// German: Rechnung
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Invoice number (Rechnungsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,

    /// Invoice type (Rechnungstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<InvoiceType>,

    /// Invoice status (Rechnungsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Invoice date (Rechnungsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<NaiveDate>,

    /// Due date (Faelligkeitsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,

    /// Billing period (Abrechnungszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<TimePeriod>,

    /// Net amount (Nettobetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Amount>,

    /// Tax amount (Steuerbetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,

    /// Gross amount (Bruttobetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<Amount>,

    /// Invoice line items (Rechnungspositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<InvoicePosition>,

    /// Invoice recipient (Rechnungsempfaenger)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for Invoice {
    fn type_name_german() -> &'static str { "Rechnung" }
    fn type_name_english() -> &'static str { "Invoice" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::com::Amount;

    #[test]
    fn test_invoice_creation() {
        let invoice = Invoice {
            invoice_number: Some("RE-2024-001234".to_string()),
            invoice_type: Some(InvoiceType::AnnualBill),
            net_amount: Some(Amount::eur(1000.00)),
            tax_amount: Some(Amount::eur(190.00)),
            gross_amount: Some(Amount::eur(1190.00)),
            ..Default::default()
        };

        assert_eq!(invoice.invoice_number, Some("RE-2024-001234".to_string()));
    }

    #[test]
    fn test_serialize() {
        let invoice = Invoice {
            meta: Bo4eMeta::with_type("Rechnung"),
            invoice_number: Some("RE-001".to_string()),
            gross_amount: Some(Amount::eur(119.00)),
            ..Default::default()
        };

        let json = serde_json::to_string(&invoice).unwrap();
        assert!(json.contains(r#""invoiceNumber":"RE-001""#));
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core invoice
git add crates/bo4e-core/src/bo/invoice.rs crates/bo4e-core/src/bo/mod.rs
git commit -m "feat(core): add Invoice (Rechnung) business object"
```

---

## Task 2: Create Tariff Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/tariff.rs`

**Step 1: Write the implementation**

```rust
//! Tariff (Tarif) business object.

use serde::{Deserialize, Serialize};
use crate::com::{Price, PriceTier, TariffCalculationParameter, TimePeriod};
use crate::enums::{CustomerType, Division, TariffModel};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A tariff definition.
///
/// German: Tarif
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tariff {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Tariff name (Tarifname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_name: Option<String>,

    /// Tariff description (Tarifbeschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Tariff model (Tarifmodell)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_model: Option<TariffModel>,

    /// Target customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Base price (Grundpreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_price: Option<Price>,

    /// Working price (Arbeitspreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_price: Option<Price>,

    /// Price tiers (Preisstaffeln)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_tiers: Vec<PriceTier>,

    /// Calculation parameters (Tarifberechnungsparameter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_parameters: Option<TariffCalculationParameter>,

    /// Provider/supplier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for Tariff {
    fn type_name_german() -> &'static str { "Tarif" }
    fn type_name_english() -> &'static str { "Tariff" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
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
    }

    #[test]
    fn test_tiered_tariff() {
        let tariff = Tariff {
            tariff_name: Some("Staffeltarif".to_string()),
            price_tiers: vec![
                PriceTier {
                    lower_limit: Some(0.0),
                    upper_limit: Some(1000.0),
                    price: Some(0.35),
                    tier_number: Some(1),
                    ..Default::default()
                },
                PriceTier {
                    lower_limit: Some(1000.0),
                    upper_limit: Some(5000.0),
                    price: Some(0.30),
                    tier_number: Some(2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(tariff.price_tiers.len(), 2);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core tariff
git add crates/bo4e-core/src/bo/tariff.rs crates/bo4e-core/src/bo/mod.rs
git commit -m "feat(core): add Tariff (Tarif) business object"
```

---

## Tasks 3-12: Remaining Pricing/Billing BOs

Follow the pattern for:
- `TariffInfo`
- `TariffPriceSheet`
- `PriceSheet`
- `ServicePriceSheet`
- `HardwarePriceSheet`
- `MeteringPriceSheet`
- `NetworkUsagePriceSheet`
- `ConcessionFeePriceSheet`
- `Costs`
- `TariffCosts`
- `ExternalCosts`

---

## Verification

```bash
cargo test -p bo4e-core bo -- --nocapture
cargo clippy -p bo4e-core

# Count implemented BOs
find crates/bo4e-core/src/bo -name "*.rs" | wc -l
```

Expected: ~40 files (37 BOs + mod.rs + 2 shared)
