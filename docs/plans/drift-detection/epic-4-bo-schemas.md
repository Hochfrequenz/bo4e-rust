---
status: in_progress
---
# Epic 4: Add Schemars to Business Objects

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `JsonSchema` derive with German name renames to all 35 business object types.

**Architecture:** Each BO gets `#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]`, `#[cfg_attr(feature = "json-schema", schemars(rename = "GermanName"))]` on the struct, and `#[cfg_attr(feature = "json-schema", schemars(rename = "germanFieldName"))]` on each field.

**Tech Stack:** schemars 0.8

---

## Pattern Reference

Same pattern as components. For each BO file:

**After:**
```rust
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehler"))]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    #[serde(skip_serializing_if = "Option::is_none", alias = "zaehlernummer")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehlernummer"))]
    pub meter_number: Option<String>,
    // ...
}
```

---

## Task 1: Core Location BOs (6 files)

**Files:**
- `meter.rs` → `Zaehler`
- `market_location.rs` → `Marktlokation`
- `metering_location.rs` → `Messlokation`
- `network_location.rs` → `Netzlokation`
- `location_assignment.rs` → `Lokationszuordnung`
- `location_properties.rs` → `Lokationseigenschaften`

**Step 1: Add schemars to Meter**

Read the file to identify all fields and their German aliases, then add:
- Struct-level derive and rename
- Field-level renames matching each `alias = "..."` value

**Step 2: Apply same pattern to remaining files**

For each file, read it first to get the German type name and field aliases.

**Step 3: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/bo/
git commit -m "feat(drift): add schemars to location business objects"
```

---

## Task 2: Technical Resource BOs (6 files)

**Files:**
- `device.rs` → `Geraet`
- `technical_resource.rs` → `TechnischeRessource`
- `controllable_resource.rs` → `SteijerbareRessource`
- `energy_amount.rs` → `Energiemenge`
- `load_profile.rs` → `Lastprofil`
- `time_series.rs` → `Zeitreihe`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/bo/
git commit -m "feat(drift): add schemars to technical resource business objects"
```

---

## Task 3: Party & Contract BOs (8 files)

**Files:**
- `business_partner.rs` → `Geschaeftspartner`
- `person.rs` → `Person`
- `market_participant.rs` → `Marktteilnehmer`
- `contract.rs` → `Vertrag`
- `bundle_contract.rs` → `Buendelvertrag`
- `offer.rs` → `Angebot`
- `tender.rs` → `Ausschreibung`
- `balancing.rs` → `Bilanzierung`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/bo/
git commit -m "feat(drift): add schemars to party and contract business objects"
```

---

## Task 4: Pricing & Tariff BOs (9 files)

**Files:**
- `tariff.rs` → `Tarif`
- `tariff_info.rs` → `Tarifinfo`
- `tariff_costs.rs` → `Tarifkosten`
- `tariff_price_sheet.rs` → `Tarifpreisblatt`
- `price_sheet.rs` → `Preisblatt`
- `service_price_sheet.rs` → `DienstleistungsPreisblatt`
- `hardware_price_sheet.rs` → `HardwarePreisblatt`
- `metering_price_sheet.rs` → `MessPreisblatt`
- `network_usage_price_sheet.rs` → `NetznutzungsPreisblatt`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/bo/
git commit -m "feat(drift): add schemars to pricing and tariff business objects"
```

---

## Task 5: Remaining BOs (6 files)

**Files:**
- `invoice.rs` → `Rechnung`
- `costs.rs` → `Kosten`
- `tariff_costs.rs` → `Tarifkosten`
- `external_costs.rs` → `ExterneKosten`
- `concession_fee_price_sheet.rs` → `KonzessionsabgabePreisblatt`
- `region.rs` → `Region`
- `regional_tariff.rs` → `RegionalerTarif`

**Step 1: Read and update each file**

Apply the pattern to each file.

**Step 2: Verify compilation**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/bo/
git commit -m "feat(drift): add schemars to remaining business objects"
```

---

## Task 6: Verify All BOs and Update Generator

**Step 1: Verify full build**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 2: Run tests**

Run: `cargo test -p bo4e-core`
Expected: All tests pass

**Step 3: Update generator to include all BOs**

Update `crates/bo4e-core/src/bin/generate_schema.rs` to include all schemas:

```rust
use bo4e_core::bo::*;
use bo4e_core::com::*;
use bo4e_core::enums::*;
use schemars::schema_for;
use serde_json::json;

fn main() {
    let schemas = json!({
        "bo": {
            "Zaehler": schema_for!(Meter),
            "Marktlokation": schema_for!(MarketLocation),
            "Messlokation": schema_for!(MeteringLocation),
            "Geschaeftspartner": schema_for!(BusinessPartner),
            // ... add all BOs
        },
        "com": {
            // ... all components
        },
        "enum": {
            // ... all enums
        }
    });

    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
}
```

**Step 4: Verify generator outputs all schemas**

Run: `cargo run --bin generate_schema --features json-schema | wc -l`
Expected: Large JSON output (thousands of lines)

Run: `cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json`
Expected: File created successfully

**Step 5: Commit**

```bash
git add crates/bo4e-core/src/bin/generate_schema.rs
git commit -m "feat(drift): complete schema generator with all types"
```

---

## Epic 4 Completion Checklist

- [ ] All 35 business objects have JsonSchema derive
- [ ] All BOs have German name via schemars(rename)
- [ ] All fields have German names via schemars(rename)
- [ ] Build succeeds with --features json-schema
- [ ] All tests pass
- [ ] Generator outputs all BO schemas
- [ ] Generator can output to schemas/rust_schema.json
