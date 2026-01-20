---
status: in_progress
---
# Epic 2: Add Schemars to Enums

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `JsonSchema` derive with German name renames to all 73 enum types.

**Architecture:** Each enum gets `#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]` and `#[cfg_attr(feature = "json-schema", schemars(rename = "GermanName"))]`.

**Tech Stack:** schemars 0.8

---

## Pattern Reference

For each enum file, apply this pattern:

**Before:**
```rust
use serde::{Deserialize, Serialize};

/// Energy division/sector.
///
/// German: Sparte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Division {
    #[serde(rename = "STROM")]
    Electricity,
    // ...
}
```

**After:**
```rust
use serde::{Deserialize, Serialize};

/// Energy division/sector.
///
/// German: Sparte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Sparte"))]
#[non_exhaustive]
pub enum Division {
    #[serde(rename = "STROM")]
    Electricity,
    // ...
}
```

**Note:** Enum variants already have `#[serde(rename = "...")]` which schemars will pick up automatically. Only the enum type name needs the schemars rename.

---

## Task 1: Core Type Enums (6 files)

**Files:**
- Modify: `crates/bo4e-core/src/enums/bo_type.rs`
- Modify: `crates/bo4e-core/src/enums/com_type.rs`
- Modify: `crates/bo4e-core/src/enums/division.rs`
- Modify: `crates/bo4e-core/src/enums/country.rs`
- Modify: `crates/bo4e-core/src/enums/currency.rs`
- Modify: `crates/bo4e-core/src/enums/unit.rs`

**Step 1: Add schemars to each file**

| File | German Name |
|------|-------------|
| `bo_type.rs` | `BoTyp` |
| `com_type.rs` | `ComTyp` |
| `division.rs` | `Sparte` |
| `country.rs` | `Landescode` |
| `currency.rs` | `Waehrungscode` |
| `unit.rs` | `Mengeneinheit` |

Add after existing derives on each enum:
```rust
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "GermanName"))]
```

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to core type enums"
```

---

## Task 2: Energy & Meter Enums (12 files)

**Files:**
- `energy_direction.rs` → `Energierichtung`
- `meter_type.rs` → `Zaehlertyp`
- `meter_size.rs` → `Zaehlergroesse`
- `meter_category.rs` → `Zaehlerkategorie`
- `medium.rs` → `Medium`
- `register_type.rs` → `Registertyp`
- `reading_type.rs` → `Ableseart`
- `voltage_level.rs` → `Spannungsebene`
- `network_level.rs` → `Netzebene`
- `phase_type.rs` → `Phasentyp`
- `generation_type.rs` → `Erzeugungsart`
- `eco_certificate.rs` → `Oekozertifikat`

**Step 1: Add schemars to each file**

Apply the pattern to each file with the German name from the table above.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to energy and meter enums"
```

---

## Task 3: Business & Contract Enums (14 files)

**Files:**
- `business_partner_role.rs` → `Geschaeftspartnerrolle`
- `market_role.rs` → `Marktrolle`
- `customer_type.rs` → `Kundentyp`
- `customer_group.rs` → `Kundengruppe`
- `organization_type.rs` → `Organisationstyp`
- `salutation.rs` → `Anrede`
- `title.rs` → `Titel`
- `contact_type.rs` → `Kontaktart`
- `contract_type.rs` → `Vertragsart`
- `contract_status.rs` → `Vertragsstatus`
- `contract_form.rs` → `Vertragsform`
- `payment_method.rs` → `Zahlungsweise`
- `offer_status.rs` → `Angebotsstatus`
- `tender_status.rs` → `Ausschreibungsstatus`

**Step 1: Add schemars to each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to business and contract enums"
```

---

## Task 4: Pricing & Tariff Enums (16 files)

**Files:**
- `price_type.rs` → `Preistyp`
- `price_status.rs` → `Preisstatus`
- `price_model.rs` → `Preismodell`
- `price_guarantee_type.rs` → `Preisgarantietyp`
- `tariff_type.rs` → `Tarifart`
- `tariff_time.rs` → `Tarifzeit`
- `tariff_region_criterion.rs` → `Tarifregionskriterium`
- `tariff_calculation_method.rs` → `Tarifkalkulationsmethode`
- `tariff_feature.rs` → `Tarifmerkmal`
- `surcharge_type.rs` → `Aufschlagstyp`
- `surcharge_target.rs` → `Aufschlagsziel`
- `tax_type.rs` → `Steuerart`
- `cost_class.rs` → `Kostenklasse`
- `concession_fee_type.rs` → `Konzessionsabgabetyp`
- `concession_fee_customer_group.rs` → `KonzsessionsabgabeKundengruppe`
- `rounding_mode.rs` → `Rundungsmodus`

**Step 1: Add schemars to each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to pricing and tariff enums"
```

---

## Task 5: Measurement & Technical Enums (12 files)

**Files:**
- `measurement_type.rs` → `Messart`
- `measurement_price_type.rs` → `Messpreistyp`
- `measured_quantity.rs` → `Messwerterfassung`
- `measured_value_status.rs` → `Messwertstatus`
- `arithmetic_operation.rs` → `ArithmetischeOperation`
- `calculation_method.rs` → `Berechnungsmethode`
- `calculation_formula.rs` → `Berechnungsformel`
- `time_unit.rs` → `Zeiteinheit`
- `unit_prefix.rs` → `Mengeneinheitvorsilbe`
- `validity_type.rs` → `Gueltigkeitstyp`
- `device_type.rs` → `Geraetetyp`
- `device_category.rs` → `Geraetekategorie`

**Step 1: Add schemars to each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to measurement and technical enums"
```

---

## Task 6: Location & Region Enums (8 files)

**Files:**
- `location_type.rs` → `Lokationstyp`
- `area_type.rs` → `Gebiettyp`
- `region_type.rs` → `Regiontyp`
- `region_criterion_type.rs` → `Regionskriteriumtyp`
- `subject_area.rs` → `Themengebiet`
- `controllable_resource_type.rs` → `SteijerbareRessourceTyp`
- `technical_resource_usage.rs` → `TechnischeRessourceNutzung`
- `usage_type.rs` → `Verwendungszweck`

**Step 1: Add schemars to each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to location and region enums"
```

---

## Task 7: Invoice & Service Enums (7 files)

**Files:**
- `invoice_type.rs` → `Rechnungstyp`
- `invoice_status.rs` → `Rechnungsstatus`
- `service_type.rs` → `Dienstleistungstyp`
- `tender_type.rs` → `Ausschreibungstyp`
- `eco_label.rs` → `Oekolabel`

**Step 1: Add schemars to each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(drift): add schemars to invoice and service enums"
```

---

## Task 8: Verify All Enums and Update Generator

**Step 1: Verify full build**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success with no warnings about missing JsonSchema

**Step 2: Run tests**

Run: `cargo test -p bo4e-core`
Expected: All tests pass

**Step 3: Update generator to include enums**

Update `crates/bo4e-core/src/bin/generate_schema.rs` to include enum schemas:

```rust
use bo4e_core::enums::*;
use schemars::schema_for;
use serde_json::json;

fn main() {
    let schemas = json!({
        "bo": {},
        "com": {},
        "enum": {
            "Sparte": schema_for!(Division),
            "Energierichtung": schema_for!(EnergyDirection),
            "Zaehlertyp": schema_for!(MeterType),
            // ... add all enums
        }
    });

    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
}
```

**Step 4: Verify generator outputs enum schemas**

Run: `cargo run --bin generate_schema --features json-schema | head -50`
Expected: JSON with enum schemas

**Step 5: Commit**

```bash
git add crates/bo4e-core/src/bin/generate_schema.rs
git commit -m "feat(drift): update generator with enum schemas"
```

---

## Epic 2 Completion Checklist

- [ ] All 73 enums have JsonSchema derive
- [ ] All enums have German name via schemars(rename)
- [ ] Build succeeds with --features json-schema
- [ ] All tests pass
- [ ] Generator outputs enum schemas
