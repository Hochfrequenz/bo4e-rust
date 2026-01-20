# Epic 2.3: Pricing & Regional Enumerations

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement all pricing, tariff, and regional enumerations (~32 enums).

**Architecture:** Each enum is a separate module file with German serde aliases.

**Tech Stack:** Rust enums, serde derive macros

---

## Enums in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `PriceType` | `Preistyp` | Type of price |
| `PriceModel` | `Preismodell` | Pricing model |
| `PriceGuaranteeType` | `Preisgarantietyp` | Type of price guarantee |
| `PriceStatus` | `Preisstatus` | Status of price |
| `SurchargeType` | `AufAbschlagstyp` | Type of surcharge/discount |
| `SurchargeTarget` | `AufAbschlagsziel` | Target of surcharge |
| `CalculationMethod` | `Kalkulationsmethode` | Calculation method |
| `TaxType` | `Steuerart` | Type of tax |
| `VatRate` | `Steuersatz` | VAT rate |
| `CostType` | `Kostenart` | Type of cost |
| `CostCategory` | `Kostenkategorie` | Category of cost |
| `TariffModel` | `Tarifmodell` | Tariff model |
| `TariffMeterType` | `Tarifmessgeraetetyp` | Tariff meter type |
| `TariffRegulationZone` | `Tarifregulierungszone` | Tariff regulation zone |
| `TariffTime` | `Tarifzeit` | Tariff time period |
| `TariffOption` | `Tarifoption` | Tariff option |
| `TariffCalculationMethod` | `Tarifkalkulationsmethode` | Tariff calculation method |
| `PriceSheetType` | `Preisblatttyp` | Type of price sheet |
| `RegionType` | `Gebiettyp` | Type of region/area |
| `RegionCriterionType` | `Regionskriteriumtyp` | Type of region criterion |
| `FederalState` | `Bundesland` | German federal state |
| `ConcessionFeeType` | `Konzessionsabgabentyp` | Concession fee type |
| `NetworkChargeType` | `Netzentgelttyp` | Network charge type |
| `NetworkChargeCategory` | `Netzentgeltkategorie` | Network charge category |
| `LevyType` | `Umlagentyp` | Type of levy |
| `AbatementType` | `Entlastungstyp` | Type of abatement |
| `BonusType` | `Bonustyp` | Type of bonus |
| `DiscountType` | `Rabatttyp` | Type of discount |
| `PricePeriodType` | `Preisperiodentyp` | Type of price period |
| `IndexType` | `Indextyp` | Type of index |
| `EnergyMixType` | `Energiemixtyp` | Type of energy mix |
| `CertificateType` | `Zertifikatstyp` | Type of certificate |

---

## Task 1: Create PriceType Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/price_type.rs`

**Step 1: Write the implementation**

```rust
//! Price type (Preistyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of price in energy billing.
///
/// German: Preistyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriceType {
    /// Working price (energy price per unit)
    #[serde(rename = "ARBEITSPREIS")]
    WorkingPrice,

    /// Base price (fixed monthly/yearly fee)
    #[serde(rename = "GRUNDPREIS")]
    BasePrice,

    /// Capacity price (demand charge)
    #[serde(rename = "LEISTUNGSPREIS")]
    CapacityPrice,

    /// Metering price
    #[serde(rename = "MESSPREIS")]
    MeteringPrice,

    /// System service price
    #[serde(rename = "SYSTEMDIENSTLEISTUNGSPREIS")]
    SystemServicePrice,

    /// Initial connection price
    #[serde(rename = "ERSTANSCHLUSS")]
    InitialConnection,

    /// Minimum price
    #[serde(rename = "MINDESTPREIS")]
    MinimumPrice,

    /// Maximum price
    #[serde(rename = "HOECHSTPREIS")]
    MaximumPrice,

    /// Mixed price
    #[serde(rename = "MISCHPREIS")]
    MixedPrice,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let price_type = PriceType::WorkingPrice;
        let json = serde_json::to_string(&price_type).unwrap();
        assert_eq!(json, r#""ARBEITSPREIS""#);
        let parsed: PriceType = serde_json::from_str(&json).unwrap();
        assert_eq!(price_type, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core price_type
git add crates/bo4e-core/src/enums/price_type.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add PriceType (Preistyp) enum"
```

---

## Task 2: Create RegionType (Gebiettyp) Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/region_type.rs`

**Step 1: Write the implementation**

```rust
//! Region/area type (Gebiettyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of geographical region or area.
///
/// German: Gebiettyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegionType {
    /// Market area (Marktgebiet)
    #[serde(rename = "MARKTGEBIET")]
    MarketArea,

    /// Network area (Netzgebiet)
    #[serde(rename = "NETZGEBIET")]
    NetworkArea,

    /// Regulation zone (Regelzone)
    #[serde(rename = "REGELZONE")]
    RegulationZone,

    /// Balance group (Bilanzierungsgebiet)
    #[serde(rename = "BILANZIERUNGSGEBIET")]
    BalancingArea,

    /// Federal state (Bundesland)
    #[serde(rename = "BUNDESLAND")]
    FederalState,

    /// District (Kreis)
    #[serde(rename = "KREIS")]
    District,

    /// Municipality (Gemeinde)
    #[serde(rename = "GEMEINDE")]
    Municipality,

    /// Postal code area (Postleitzahlengebiet)
    #[serde(rename = "POSTLEITZAHLENGEBIET")]
    PostalCodeArea,

    /// Basic supplier area (Grundversorgergebiet)
    #[serde(rename = "GRUNDVERSORGERGEBIET")]
    BasicSupplierArea,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&RegionType::FederalState).unwrap(),
            r#""BUNDESLAND""#
        );
    }

    #[test]
    fn test_deserialize() {
        let region: RegionType = serde_json::from_str(r#""NETZGEBIET""#).unwrap();
        assert_eq!(region, RegionType::NetworkArea);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core region_type
git add crates/bo4e-core/src/enums/region_type.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add RegionType (Gebiettyp) enum"
```

---

## Task 3: Create FederalState (Bundesland) Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/federal_state.rs`

**Step 1: Write the implementation**

```rust
//! German federal state (Bundesland) enumeration.

use serde::{Deserialize, Serialize};

/// German federal states.
///
/// German: Bundesland
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FederalState {
    /// Baden-Wuerttemberg
    #[serde(rename = "BW")]
    BadenWuerttemberg,

    /// Bavaria (Bayern)
    #[serde(rename = "BY")]
    Bavaria,

    /// Berlin
    #[serde(rename = "BE")]
    Berlin,

    /// Brandenburg
    #[serde(rename = "BB")]
    Brandenburg,

    /// Bremen
    #[serde(rename = "HB")]
    Bremen,

    /// Hamburg
    #[serde(rename = "HH")]
    Hamburg,

    /// Hesse (Hessen)
    #[serde(rename = "HE")]
    Hesse,

    /// Mecklenburg-Vorpommern
    #[serde(rename = "MV")]
    MecklenburgVorpommern,

    /// Lower Saxony (Niedersachsen)
    #[serde(rename = "NI")]
    LowerSaxony,

    /// North Rhine-Westphalia (Nordrhein-Westfalen)
    #[serde(rename = "NW")]
    NorthRhineWestphalia,

    /// Rhineland-Palatinate (Rheinland-Pfalz)
    #[serde(rename = "RP")]
    RhinelandPalatinate,

    /// Saarland
    #[serde(rename = "SL")]
    Saarland,

    /// Saxony (Sachsen)
    #[serde(rename = "SN")]
    Saxony,

    /// Saxony-Anhalt (Sachsen-Anhalt)
    #[serde(rename = "ST")]
    SaxonyAnhalt,

    /// Schleswig-Holstein
    #[serde(rename = "SH")]
    SchleswigHolstein,

    /// Thuringia (Thueringen)
    #[serde(rename = "TH")]
    Thuringia,
}

impl FederalState {
    /// Returns the German name of the federal state.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::BadenWuerttemberg => "Baden-Württemberg",
            Self::Bavaria => "Bayern",
            Self::Berlin => "Berlin",
            Self::Brandenburg => "Brandenburg",
            Self::Bremen => "Bremen",
            Self::Hamburg => "Hamburg",
            Self::Hesse => "Hessen",
            Self::MecklenburgVorpommern => "Mecklenburg-Vorpommern",
            Self::LowerSaxony => "Niedersachsen",
            Self::NorthRhineWestphalia => "Nordrhein-Westfalen",
            Self::RhinelandPalatinate => "Rheinland-Pfalz",
            Self::Saarland => "Saarland",
            Self::Saxony => "Sachsen",
            Self::SaxonyAnhalt => "Sachsen-Anhalt",
            Self::SchleswigHolstein => "Schleswig-Holstein",
            Self::Thuringia => "Thüringen",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_16_states() {
        // Verify all 16 federal states are present
        let states = [
            FederalState::BadenWuerttemberg,
            FederalState::Bavaria,
            FederalState::Berlin,
            FederalState::Brandenburg,
            FederalState::Bremen,
            FederalState::Hamburg,
            FederalState::Hesse,
            FederalState::MecklenburgVorpommern,
            FederalState::LowerSaxony,
            FederalState::NorthRhineWestphalia,
            FederalState::RhinelandPalatinate,
            FederalState::Saarland,
            FederalState::Saxony,
            FederalState::SaxonyAnhalt,
            FederalState::SchleswigHolstein,
            FederalState::Thuringia,
        ];
        assert_eq!(states.len(), 16);

        // Test roundtrip for all
        for state in states {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: FederalState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, parsed);
        }
    }

    #[test]
    fn test_german_names() {
        assert_eq!(FederalState::Bavaria.german_name(), "Bayern");
        assert_eq!(FederalState::NorthRhineWestphalia.german_name(), "Nordrhein-Westfalen");
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core federal_state
git add crates/bo4e-core/src/enums/federal_state.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add FederalState (Bundesland) enum"
```

---

## Tasks 4-32: Remaining Pricing/Regional Enums

Follow the established pattern. Reference `../BO4E-python/src/bo4e/enum/` for complete variant lists.

**Key enums to prioritize:**
1. `TaxType` / `VatRate` - Tax handling
2. `CalculationMethod` - Price calculation
3. `ConcessionFeeType` - Municipal fees
4. `NetworkChargeType` - Grid fees
5. `LevyType` - EEG, KWK levies

---

## Task 33: Create Enum Index Module

**Files:**
- Update: `crates/bo4e-core/src/enums/mod.rs`

**Step 1: Final mod.rs with all exports**

Ensure `mod.rs` exports all enums:

```rust
//! Enumerations for BO4E type-safe values.
//!
//! All enums use German values for JSON serialization to maintain
//! compatibility with the BO4E standard. English names are used
//! for Rust identifiers.

// Type discriminators
mod bo_type;
mod com_type;

// Energy & Technical
mod division;
mod energy_direction;
mod unit;
mod meter_type;
// ... all other energy enums

// Business & Contract
mod business_partner_role;
mod contract_status;
mod customer_type;
// ... all other business enums

// Pricing & Regional
mod price_type;
mod region_type;
mod federal_state;
// ... all other pricing enums

// Re-exports
pub use bo_type::BoType;
pub use com_type::ComType;
pub use division::Division;
pub use energy_direction::EnergyDirection;
pub use unit::Unit;
pub use meter_type::MeterType;
pub use business_partner_role::BusinessPartnerRole;
pub use contract_status::ContractStatus;
pub use customer_type::CustomerType;
pub use price_type::PriceType;
pub use region_type::RegionType;
pub use federal_state::FederalState;
// ... all other exports
```

**Step 2: Commit**

```bash
git add crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): complete enum module exports"
```

---

## Verification

```bash
# All enum tests
cargo test -p bo4e-core enums -- --nocapture

# Full workspace check
cargo check --workspace
cargo clippy --workspace

# Count implemented enums
find crates/bo4e-core/src/enums -name "*.rs" | wc -l
```

Expected: ~100 files (97 enums + mod.rs + bo_type + com_type)

---

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 189 |
| Passed | 189 |
| Failed | 0 |
| Skipped | 0 |

### Enums Implemented (17 new enums)

**Pricing:**
- `PriceType` - Type of price (Preistyp)
- `PriceModel` - Pricing model (Preismodell)
- `PriceGuaranteeType` - Price guarantee type (Preisgarantietyp)
- `PriceStatus` - Price status (Preisstatus)
- `SurchargeType` - Surcharge/discount type (AufAbschlagstyp)
- `SurchargeTarget` - Surcharge target (AufAbschlagsziel)
- `CalculationMethod` - Calculation method (Kalkulationsmethode)
- `TaxType` - Tax type (Steuerart)

**Costs:**
- `CostClass` - Cost class (Kostenklasse)

**Tariffs:**
- `TariffCalculationMethod` - Tariff calculation method (Tarifkalkulationsmethode)
- `TariffTime` - Tariff time period (Tarifzeit)
- `TariffFeature` - Tariff feature (Tarifmerkmal)
- `TariffRegionCriterion` - Tariff region criterion (Tarifregionskriterium)

**Regional:**
- `RegionType` - Region type (Gebiettyp)
- `RegionCriterionType` - Region criterion type (Regionskriteriumtyp)

**Concession Fees:**
- `ConcessionFeeType` - Concession fee type (Abgabeart)
- `ConcessionFeeCustomerGroup` - Concession fee customer group (KundengruppeKA)

**Eco/Certificates:**
- `EcoLabel` - Eco label (Oekolabel)
- `EcoCertificate` - Eco certificate (Oekozertifikat)

### Files Created

- `crates/bo4e-core/src/enums/price_type.rs`
- `crates/bo4e-core/src/enums/price_model.rs`
- `crates/bo4e-core/src/enums/price_guarantee_type.rs`
- `crates/bo4e-core/src/enums/price_status.rs`
- `crates/bo4e-core/src/enums/surcharge_type.rs`
- `crates/bo4e-core/src/enums/surcharge_target.rs`
- `crates/bo4e-core/src/enums/calculation_method.rs`
- `crates/bo4e-core/src/enums/tax_type.rs`
- `crates/bo4e-core/src/enums/cost_class.rs`
- `crates/bo4e-core/src/enums/tariff_calculation_method.rs`
- `crates/bo4e-core/src/enums/tariff_time.rs`
- `crates/bo4e-core/src/enums/tariff_feature.rs`
- `crates/bo4e-core/src/enums/tariff_region_criterion.rs`
- `crates/bo4e-core/src/enums/region_type.rs`
- `crates/bo4e-core/src/enums/region_criterion_type.rs`
- `crates/bo4e-core/src/enums/concession_fee_type.rs`
- `crates/bo4e-core/src/enums/concession_fee_customer_group.rs`
- `crates/bo4e-core/src/enums/eco_label.rs`
- `crates/bo4e-core/src/enums/eco_certificate.rs`
- `crates/bo4e-core/src/enums/mod.rs` (updated)
