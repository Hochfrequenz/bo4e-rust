# Epic 3: Add Schemars to Components

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `JsonSchema` derive with German name renames to all 64 component types.

**Architecture:** Each component gets `#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]`, `#[cfg_attr(feature = "json-schema", schemars(rename = "GermanName"))]` on the struct, and `#[cfg_attr(feature = "json-schema", schemars(rename = "germanFieldName"))]` on each field.

**Tech Stack:** schemars 0.8

---

## Pattern Reference

For each component file, apply this pattern:

**Before:**
```rust
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    #[serde(skip_serializing_if = "Option::is_none", alias = "strasse")]
    pub street: Option<String>,
}
```

**After:**
```rust
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Adresse"))]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    #[serde(skip_serializing_if = "Option::is_none", alias = "strasse")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "strasse"))]
    pub street: Option<String>,
}
```

**Important:** Each field needs its own `schemars(rename = "...")` with the German name from the `alias` attribute.

---

## Task 1: Core Address & Contact Components (6 files)

**Files:**
- `address.rs` → `Adresse`
- `cadastral_address.rs` → `Katasteradresse`
- `geo_coordinates.rs` → `Geokoordinaten`
- `contact_method.rs` → `Kontaktweg`
- `signature.rs` → `Unterschrift`
- `external_reference.rs` → `ExterneReferenz`

**Step 1: Add schemars to Address**

Read the file to identify all fields and their German aliases, then add:
- Struct-level derive and rename
- Field-level renames matching each `alias = "..."` value

**Step 2: Apply same pattern to remaining files**

For each file, read it first to get the German type name (from doc comment) and field aliases.

**Step 3: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to address and contact components"
```

---

## Task 2: Pricing Components (12 files)

**Files:**
- `price.rs` → `Preis`
- `price_tier.rs` → `Preisstufe`
- `price_position.rs` → `Preisposition`
- `price_guarantee.rs` → `Preisgarantie`
- `regional_price_tier.rs` → `RegionalePreisstufe`
- `margin_price.rs` → `Margenpreis`
- `surcharge.rs` → `Aufschlag`
- `position_surcharge.rs` → `PositionsAufschlag`
- `regional_surcharge.rs` → `RegionalerAufschlag`
- `surcharge_per_location.rs` → `AufschlagProOrt`
- `levy.rs` → `Abgabe`
- `network_charge.rs` → `Netzentgelt`

**Step 1: Read and update each file**

For each file:
1. Read to get German name and field aliases
2. Add struct-level schemars derive and rename
3. Add field-level schemars renames

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to pricing components"
```

---

## Task 3: Tariff Components (8 files)

**Files:**
- `tariff_price.rs` → `Tarifpreis`
- `tariff_price_position.rs` → `Tarifpreisposition`
- `tariff_calculation_parameter.rs` → `Tarifberechnungsparameter`
- `tariff_restriction.rs` → `Tarifeinschraenkung`
- `seasonal_tariff.rs` → `SaisonTarif`
- `time_of_use_register.rs` → `ZeitverwendungsRegister`
- `energy_mix.rs` → `Energiemix`
- `energy_source.rs` → `Energiequelle`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to tariff components"
```

---

## Task 4: Cost & Invoice Components (10 files)

**Files:**
- `cost_block.rs` → `Kostenblock`
- `cost_position.rs` → `Kostenposition`
- `external_cost_block.rs` → `ExternerKostenblock`
- `external_cost_position.rs` → `ExterneKostenposition`
- `tax_amount.rs` → `Steuerbetrag`
- `concession_fee.rs` → `Konzessionsabgabe`
- `invoice_position.rs` → `Rechnungsposition`
- `billing_period_data.rs` → `Abrechnungszeitraumdaten`
- `bonus.rs` → `Bonus`
- `discount.rs` → `Rabatt`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to cost and invoice components"
```

---

## Task 5: Measurement Components (10 files)

**Files:**
- `quantity.rs` → `Menge`
- `amount.rs` → `Betrag`
- `measured_value.rs` → `Messwert`
- `meter_reading.rs` → `Zaehlwerstand`
- `meter_register.rs` → `Zaehlwerk`
- `metering_point_status.rs` → `Zahlpunktstatus`
- `consumption.rs` → `Verbrauch`
- `consumed_quantity.rs` → `VerbrauchteMenge`
- `aggregated_value.rs` → `AggregierterWert`
- `substitution_value.rs` → `Ersatzwert`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to measurement components"
```

---

## Task 6: Time & Profile Components (8 files)

**Files:**
- `time_period.rs` → `Zeitraum`
- `date_range.rs` → `Datumsbereich`
- `interval.rs` → `Intervall`
- `load_curve_data.rs` → `Lastgangdaten`
- `load_profile_value.rs` → `Lastprofilwert`
- `profile_data.rs` → `Profildaten`
- `time_series_value.rs` → `Zeitreihenwert`
- `quality_indicator.rs` → `Qualitaetsindikator`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to time and profile components"
```

---

## Task 7: Contract & Offer Components (10 files)

**Files:**
- `contract_conditions.rs` → `Vertragsbedingungen`
- `contract_part.rs` → `Vertragsteil`
- `offer_part.rs` → `Angebotsteil`
- `offer_position.rs` → `Angebotsposition`
- `offer_variant.rs` → `Angebotsvariante`
- `region_criterion.rs` → `Regionskriterium`
- `responsibility.rs` → `Zustaendigkeit`
- `service_price.rs` → `Dienstleistungspreis`
- `hardware.rs` → `Hardware`
- `validation_result.rs` → `Validierungsergebnis`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(drift): add schemars to contract and offer components"
```

---

## Task 8: Verify All Components and Update Generator

**Step 1: Verify full build**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 2: Run tests**

Run: `cargo test -p bo4e-core`
Expected: All tests pass

**Step 3: Update generator to include components**

Update `crates/bo4e-core/src/bin/generate_schema.rs` to include component schemas:

```rust
use bo4e_core::com::*;
use bo4e_core::enums::*;
use schemars::schema_for;
use serde_json::json;

fn main() {
    let schemas = json!({
        "bo": {},
        "com": {
            "Adresse": schema_for!(Address),
            "Preis": schema_for!(Price),
            "Menge": schema_for!(Quantity),
            // ... add all components
        },
        "enum": {
            // ... existing enums
        }
    });

    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
}
```

**Step 4: Verify generator outputs component schemas**

Run: `cargo run --bin generate_schema --features json-schema | head -100`
Expected: JSON with component schemas

**Step 5: Commit**

```bash
git add crates/bo4e-core/src/bin/generate_schema.rs
git commit -m "feat(drift): update generator with component schemas"
```

---

## Epic 3 Completion Checklist

- [ ] All 64 components have JsonSchema derive
- [ ] All components have German name via schemars(rename)
- [ ] All fields have German names via schemars(rename)
- [ ] Build succeeds with --features json-schema
- [ ] All tests pass
- [ ] Generator outputs component schemas
