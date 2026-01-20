# Epic 3.2: Pricing & Cost Components

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement pricing, cost, and tariff-related components (~30 components).

**Architecture:** Each component is a struct implementing `Bo4eObject` with optional fields.

**Tech Stack:** Rust structs, serde, references to enums from Phase 2

---

## Components in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `Price` | `Preis` | A price with value and unit |
| `PricePosition` | `Preisposition` | Position in a price sheet |
| `PriceTier` | `Preisstaffel` | Price tier/bracket |
| `TariffPrice` | `Tarifpreis` | Tariff price |
| `TariffPricePosition` | `Tarifpreisposition` | Tariff price position |
| `RegionalPriceTier` | `RegionalePreisstaffel` | Regional price tier |
| `Surcharge` | `AufAbschlag` | Surcharge or discount |
| `SurchargePerLocation` | `AufAbschlagProOrt` | Surcharge per location |
| `RegionalSurcharge` | `AufAbschlagRegional` | Regional surcharge |
| `PositionSurcharge` | `PositionsAufAbschlag` | Position-specific surcharge |
| `Amount` | `Betrag` | Monetary amount |
| `TaxAmount` | `Steuerbetrag` | Tax amount |
| `CostBlock` | `Kostenblock` | Block of costs |
| `CostPosition` | `Kostenposition` | Cost position |
| `ExternalCostBlock` | `Fremdkostenblock` | External cost block |
| `ExternalCostPosition` | `Fremdkostenposition` | External cost position |
| `TariffCalculationParameter` | `Tarifberechnungsparameter` | Tariff calculation parameters |
| `TariffRestriction` | `Tarifeinschraenkung` | Tariff restriction |
| `PriceGuarantee` | `Preisgarantie` | Price guarantee |
| `EnergySource` | `Energieherkunft` | Energy source/origin |
| `EnergyMix` | `Energiemix` | Energy mix composition |
| `Consumption` | `Verbrauch` | Consumption data |
| `ConsumedQuantity` | `VerbrauchteQuantitaet` | Consumed quantity |
| `ConcessionFee` | `Konzessionsabgabe` | Concession fee |
| `NetworkCharge` | `Netzentgelt` | Network charge |
| `Levy` | `Umlage` | Levy (EEG, KWK, etc.) |
| `Bonus` | `Bonus` | Bonus/incentive |
| `Discount` | `Rabatt` | Discount |
| `MarginPrice` | `Margenpreis` | Margin price |
| `ServicePrice` | `Dienstleistungspreis` | Service price |

---

## Task 1: Create Price Component

**Files:**
- Create: `crates/bo4e-core/src/com/price.rs`

**Step 1: Write the implementation**

```rust
//! Price (Preis) component.

use serde::{Deserialize, Serialize};
use crate::enums::{Currency, PriceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price with value, currency, and unit.
///
/// German: Preis
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Price;
///
/// let price = Price {
///     value: Some(0.25),
///     currency: Some(Currency::Euro),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Price value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Unit that the price applies to (Bezugsgroesse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<Unit>,

    /// Type of price (Preistyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<PriceType>,

    /// Status of the price (Preisstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Bo4eObject for Price {
    fn type_name_german() -> &'static str { "Preis" }
    fn type_name_english() -> &'static str { "Price" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

impl Price {
    /// Create a price in EUR per kWh.
    pub fn eur_per_kwh(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: Some(Currency::Euro),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        }
    }

    /// Create a price in EUR per month (base price).
    pub fn eur_per_month(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: Some(Currency::Euro),
            reference_unit: Some(Unit::Month),
            price_type: Some(PriceType::BasePrice),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_price() {
        let price = Price::eur_per_kwh(0.30);
        assert_eq!(price.value, Some(0.30));
        assert_eq!(price.currency, Some(Currency::Euro));
        assert_eq!(price.reference_unit, Some(Unit::KilowattHour));
    }

    #[test]
    fn test_serialize() {
        let price = Price {
            value: Some(12.50),
            currency: Some(Currency::Euro),
            ..Default::default()
        };

        let json = serde_json::to_string(&price).unwrap();
        assert!(json.contains(r#""value":12.5"#));
    }

    #[test]
    fn test_roundtrip() {
        let price = Price::eur_per_kwh(0.2567);
        let json = serde_json::to_string(&price).unwrap();
        let parsed: Price = serde_json::from_str(&json).unwrap();
        assert_eq!(price, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core price -- --nocapture
git add crates/bo4e-core/src/com/price.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add Price (Preis) component"
```

---

## Task 2: Create Amount Component

**Files:**
- Create: `crates/bo4e-core/src/com/amount.rs`

**Step 1: Write the implementation**

```rust
//! Amount (Betrag) component.

use serde::{Deserialize, Serialize};
use crate::enums::Currency;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A monetary amount with currency.
///
/// German: Betrag
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// The amount value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
}

impl Bo4eObject for Amount {
    fn type_name_german() -> &'static str { "Betrag" }
    fn type_name_english() -> &'static str { "Amount" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

impl Amount {
    /// Create an amount in EUR.
    pub fn eur(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: Some(Currency::Euro),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eur_amount() {
        let amount = Amount::eur(100.50);
        assert_eq!(amount.value, Some(100.50));
        assert_eq!(amount.currency, Some(Currency::Euro));
    }

    #[test]
    fn test_roundtrip() {
        let amount = Amount::eur(999.99);
        let json = serde_json::to_string(&amount).unwrap();
        let parsed: Amount = serde_json::from_str(&json).unwrap();
        assert_eq!(amount, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core amount
git add crates/bo4e-core/src/com/amount.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add Amount (Betrag) component"
```

---

## Task 3: Create PriceTier Component

**Files:**
- Create: `crates/bo4e-core/src/com/price_tier.rs`

**Step 1: Write the implementation**

```rust
//! Price tier (Preisstaffel) component.

use serde::{Deserialize, Serialize};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price tier based on consumption brackets.
///
/// German: Preisstaffel
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTier {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Lower consumption limit (Staffelgrenzeunten)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<f64>,

    /// Upper consumption limit (Staffelgrenzeoben)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<f64>,

    /// Price for this tier (Preis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Tier number/sequence (Staffelnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_number: Option<i32>,
}

impl Bo4eObject for PriceTier {
    fn type_name_german() -> &'static str { "Preisstaffel" }
    fn type_name_english() -> &'static str { "PriceTier" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumption_tiers() {
        let tier1 = PriceTier {
            lower_limit: Some(0.0),
            upper_limit: Some(1000.0),
            price: Some(0.30),
            tier_number: Some(1),
            ..Default::default()
        };

        let tier2 = PriceTier {
            lower_limit: Some(1000.0),
            upper_limit: Some(5000.0),
            price: Some(0.25),
            tier_number: Some(2),
            ..Default::default()
        };

        assert!(tier1.price.unwrap() > tier2.price.unwrap());
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core price_tier
git add crates/bo4e-core/src/com/price_tier.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add PriceTier (Preisstaffel) component"
```

---

## Task 4: Create Surcharge Component

**Files:**
- Create: `crates/bo4e-core/src/com/surcharge.rs`

**Step 1: Write the implementation**

```rust
//! Surcharge/discount (AufAbschlag) component.

use serde::{Deserialize, Serialize};
use crate::enums::{SurchargeType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A surcharge or discount applied to a price.
///
/// German: AufAbschlag
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Surcharge {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description of the surcharge (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of surcharge (AufAbschlagstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharge_type: Option<SurchargeType>,

    /// Value of the surcharge (Wert)
    /// Positive = surcharge, negative = discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Unit the surcharge applies to (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Whether this is a discount (Ist Abschlag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_discount: Option<bool>,
}

impl Bo4eObject for Surcharge {
    fn type_name_german() -> &'static str { "AufAbschlag" }
    fn type_name_english() -> &'static str { "Surcharge" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surcharge() {
        let surcharge = Surcharge {
            description: Some("Ökosteuer".to_string()),
            value: Some(0.02),
            unit: Some(Unit::KilowattHour),
            is_discount: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&surcharge).unwrap();
        let parsed: Surcharge = serde_json::from_str(&json).unwrap();
        assert_eq!(surcharge, parsed);
    }

    #[test]
    fn test_discount() {
        let discount = Surcharge {
            description: Some("Neukunden-Rabatt".to_string()),
            value: Some(-50.0),
            is_discount: Some(true),
            ..Default::default()
        };

        assert!(discount.value.unwrap() < 0.0);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core surcharge
git add crates/bo4e-core/src/com/surcharge.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add Surcharge (AufAbschlag) component"
```

---

## Tasks 5-30: Remaining Pricing/Cost Components

Follow the established pattern for all remaining components. Reference `../BO4E-python/src/bo4e/com/` for field definitions.

**Priority order:**
1. `TaxAmount` - Tax calculations
2. `CostBlock` / `CostPosition` - Cost breakdown
3. `PriceGuarantee` - Price guarantees
4. `EnergyMix` / `EnergySource` - Energy origin
5. `TariffCalculationParameter` - Tariff config
6. `Consumption` - Usage data

---

## Verification

```bash
cargo test -p bo4e-core com -- --nocapture
cargo clippy -p bo4e-core
```

---

## Test Summary

**Completed:** 2026-01-20

### Implementation Results

All 30 pricing and cost components were successfully implemented:

| Component | Tests | Status |
|-----------|-------|--------|
| `Price` | 4 tests | ✅ Passing |
| `Amount` | 4 tests | ✅ Passing |
| `PriceTier` | 4 tests | ✅ Passing |
| `Surcharge` | 6 tests | ✅ Passing |
| `TaxAmount` | 6 tests | ✅ Passing |
| `CostBlock` | 5 tests | ✅ Passing |
| `CostPosition` | 5 tests | ✅ Passing |
| `PriceGuarantee` | 6 tests | ✅ Passing |
| `EnergySource` | 5 tests | ✅ Passing |
| `EnergyMix` | 5 tests | ✅ Passing |
| `Consumption` | 6 tests | ✅ Passing |
| `ConsumedQuantity` | 4 tests | ✅ Passing |
| `PricePosition` | 4 tests | ✅ Passing |
| `TariffPrice` | 4 tests | ✅ Passing |
| `TariffPricePosition` | 4 tests | ✅ Passing |
| `RegionalPriceTier` | 4 tests | ✅ Passing |
| `SurchargePerLocation` | 4 tests | ✅ Passing |
| `RegionalSurcharge` | 4 tests | ✅ Passing |
| `PositionSurcharge` | 5 tests | ✅ Passing |
| `ExternalCostBlock` | 4 tests | ✅ Passing |
| `ExternalCostPosition` | 4 tests | ✅ Passing |
| `TariffCalculationParameter` | 5 tests | ✅ Passing |
| `TariffRestriction` | 5 tests | ✅ Passing |
| `ConcessionFee` | 4 tests | ✅ Passing |
| `NetworkCharge` | 5 tests | ✅ Passing |
| `Levy` | 5 tests | ✅ Passing |
| `Bonus` | 5 tests | ✅ Passing |
| `Discount` | 5 tests | ✅ Passing |
| `MarginPrice` | 4 tests | ✅ Passing |
| `ServicePrice` | 5 tests | ✅ Passing |

### Test Execution

```
cargo test --workspace
running 407 tests
test result: ok. 407 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo clippy --workspace --all-targets -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s)
```

### Key Design Decisions

1. **Consistent Pattern**: All components follow the same struct pattern with `Bo4eMeta` flattened and `Bo4eObject` trait implementation.

2. **Helper Methods**: Added convenience constructors where appropriate (e.g., `Amount::eur()`, `Price::eur_per_kwh()`).

3. **Enum Variant Mapping**: Correctly mapped to existing Phase 2 enum variants:
   - `Currency::Eur` (not Euro)
   - `SurchargeType::Relative` (for percentage)
   - `PriceGuaranteeType::AllComponentsGross`, `EnergyPriceOnly`
   - `MeasuredValueStatus::Read` (for measured values)
   - `CostClass::ExternalCosts`, `Procurement`, `InternalCosts`
   - `TariffRegionCriterion::NetworkNumber`, `PostalCode`

4. **Floating Point Precision**: Used approximate comparison for tests involving floating point calculations to avoid precision issues.
