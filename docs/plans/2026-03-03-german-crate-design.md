# German Crate Design Document

**Date:** 2026-03-03
**Status:** Draft

## Overview

Add a `bo4e-german` crate that provides a full German developer experience: German struct field names, German enum variant names, German type names, and natural German JSON serialization. Generated via a code generation script from `bo4e-core` sources.

### Goals

1. German field names in Rust code (`zaehler.zaehlernummer` instead of `meter.meter_number`)
2. German type names (`Zaehler`, `MarktLokation`, `Sparte`)
3. German enum variant names (`Sparte::Strom` instead of `Division::Electricity`)
4. German JSON output by default (`"zaehlernummer"` not `"meterNumber"`)
5. `From`/`Into` conversions between English and German types
6. Zero changes to `bo4e-core` — the English crate stays untouched

### Non-Goals

- Runtime language switching (that's `bo4e-serde`'s domain)
- Replacing the English crate — both coexist
- Changing the BO4E wire format — both versions produce BO4E-compliant JSON

## Architecture

```
bo4e-core (English types, unchanged)
    ↑
bo4e-german (German types, generated)
    ↑
bo4e-serde (serialization, unchanged)
    ↑
bo4e (facade, re-exports both)

scripts/
└── generate_german/     Code generator (Rust binary using syn/quote)
```

### Why Code Generation (Not Proc Macros)

The generator needs to see ALL types at once to resolve cross-references (e.g., knowing that a `MarketLocation` field becomes `MarktLokation`). Proc macros process one type at a time and can't do this without a separate mapping file. A code generator is simpler, more debuggable, and the generated code can be reviewed in PRs.

## Generator Design

### Input Sources

The generator reads two things:

1. **bo4e-core source files** — Parses all `.rs` files in `src/bo/`, `src/com/`, `src/enums/` using the `syn` crate
2. **Type mapping table** — A TOML file mapping English names to German names for types, fields, and enum variants

### Type Mapping File

`scripts/german_type_mapping.toml`:

```toml
# Business Objects
[bo]
Meter = "Zaehler"
MarketLocation = "MarktLokation"
MeteringLocation = "MessLokation"
Contract = "Vertrag"
BusinessPartner = "Geschaeftspartner"
# ... all 35 BOs

# Components
[com]
Address = "Adresse"
Price = "Preis"
MeterRegister = "Zaehlwerk"
MeterReading = "Zaehlerstand"
Hardware = "Hardware"
# ... all 64 COMs

# Enums
[enum]
Division = "Sparte"
MeterType = "Zaehlertyp"
MeterSize = "Zaehlergroesse"
EnergyDirection = "Energierichtung"
CustomerType = "Kundentyp"
# ... all 73 enums
```

Field-level and variant-level mappings are extracted from existing `#[serde(alias = "...")]` attributes and doc comments in bo4e-core. No separate field mapping file needed.

### Generator Algorithm

For each struct in bo4e-core:

1. Parse the struct definition with `syn`
2. Look up the German type name from the TOML mapping
3. For each field:
   - Extract the German name from `#[serde(alias = "...")]`
   - Convert from camelCase to snake_case for the Rust field name
   - Look up the field's type in the type mapping; replace BO4E types with German equivalents
   - Preserve wrapper types: `Option<Box<MarketLocation>>` → `Option<Box<MarktLokation>>`
4. Generate the German struct with appropriate serde attributes
5. Generate `From`/`Into` impls between English and German types

For each enum:

1. Parse the enum definition
2. Look up the German enum name
3. For each variant, extract the German name from doc comments (e.g., `/// Electricity (Strom)` → `Strom`)
4. Preserve `#[serde(rename = "STROM")]` — the wire format is identical
5. Generate `From`/`Into` between English and German enums

### Generated Output Structure

```
crates/bo4e-german/src/
├── lib.rs              # Crate root, module declarations
├── prelude.rs          # use bo4e_german::prelude::*
├── bo/
│   ├── mod.rs
│   ├── zaehler.rs
│   ├── markt_lokation.rs
│   └── ...             # 35 files
├── com/
│   ├── mod.rs
│   ├── adresse.rs
│   ├── preis.rs
│   └── ...             # 64 files
└── enums/
    ├── mod.rs
    ├── sparte.rs
    ├── zaehlertyp.rs
    └── ...              # 73 files
```

## Generated Code Examples

### Struct (Zaehler)

```rust
// AUTO-GENERATED — do not edit manually
// Source: bo4e-core/src/bo/meter.rs

use serde::{Deserialize, Serialize};

use crate::com::Adresse;
use crate::com::Hardware as HardwareDe;
use crate::com::Zaehlwerk;
use crate::enums::{Sparte, Zaehlergroesse, Zaehlertyp};
use bo4e_core::Bo4eMeta;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zaehler {
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    #[serde(skip_serializing_if = "Option::is_none", alias = "meterNumber")]
    pub zaehlernummer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<Sparte>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "meterType")]
    pub zaehlertyp: Option<Zaehlertyp>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "meterSize")]
    pub zaehlergroesse: Option<Zaehlergroesse>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "location")]
    pub standort: Option<Adresse>,

    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "registers")]
    pub zaehlwerke: Vec<Zaehlwerk>,

    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "hardware")]
    pub geraeteeigenschaften: Vec<HardwareDe>,

    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,

    // ... remaining fields
}

impl From<bo4e_core::bo::Meter> for Zaehler {
    fn from(m: bo4e_core::bo::Meter) -> Self {
        Self {
            meta: m.meta,
            zaehlernummer: m.meter_number,
            sparte: m.division.map(Into::into),
            zaehlertyp: m.meter_type.map(Into::into),
            zaehlergroesse: m.meter_size.map(Into::into),
            standort: m.location.map(Into::into),
            zaehlwerke: m.registers.into_iter().map(Into::into).collect(),
            geraeteeigenschaften: m.hardware.into_iter().map(Into::into).collect(),
            marktlokations_id: m.market_location_id,
            // ...
        }
    }
}

impl From<Zaehler> for bo4e_core::bo::Meter {
    fn from(z: Zaehler) -> Self {
        Self {
            meta: z.meta,
            meter_number: z.zaehlernummer,
            division: z.sparte.map(Into::into),
            meter_type: z.zaehlertyp.map(Into::into),
            meter_size: z.zaehlergroesse.map(Into::into),
            location: z.standort.map(Into::into),
            registers: z.zaehlwerke.into_iter().map(Into::into).collect(),
            hardware: z.geraeteeigenschaften.into_iter().map(Into::into).collect(),
            market_location_id: z.marktlokations_id,
            // ...
        }
    }
}
```

### Enum (Sparte)

```rust
// AUTO-GENERATED — do not edit manually
// Source: bo4e-core/src/enums/division.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Sparte {
    #[serde(rename = "STROM")]
    Strom,

    #[serde(rename = "GAS")]
    Gas,

    #[serde(rename = "FERNWAERME")]
    Fernwaerme,

    #[serde(rename = "NAHWAERME")]
    Nahwaerme,

    #[serde(rename = "WASSER")]
    Wasser,

    #[serde(rename = "ABWASSER")]
    Abwasser,

    #[serde(rename = "STROM_UND_GAS")]
    StromUndGas,
}

impl From<bo4e_core::enums::Division> for Sparte {
    fn from(d: bo4e_core::enums::Division) -> Self {
        match d {
            bo4e_core::enums::Division::Electricity => Sparte::Strom,
            bo4e_core::enums::Division::Gas => Sparte::Gas,
            bo4e_core::enums::Division::DistrictHeating => Sparte::Fernwaerme,
            bo4e_core::enums::Division::LocalHeating => Sparte::Nahwaerme,
            bo4e_core::enums::Division::Water => Sparte::Wasser,
            bo4e_core::enums::Division::Wastewater => Sparte::Abwasser,
            bo4e_core::enums::Division::ElectricityAndGas => Sparte::StromUndGas,
        }
    }
}

impl From<Sparte> for bo4e_core::enums::Division {
    fn from(s: Sparte) -> Self {
        match s {
            Sparte::Strom => bo4e_core::enums::Division::Electricity,
            Sparte::Gas => bo4e_core::enums::Division::Gas,
            Sparte::Fernwaerme => bo4e_core::enums::Division::DistrictHeating,
            Sparte::Nahwaerme => bo4e_core::enums::Division::LocalHeating,
            Sparte::Wasser => bo4e_core::enums::Division::Water,
            Sparte::Abwasser => bo4e_core::enums::Division::Wastewater,
            Sparte::StromUndGas => bo4e_core::enums::Division::ElectricityAndGas,
        }
    }
}
```

## Shared Types

`Bo4eMeta` is NOT duplicated — German structs use `bo4e_core::Bo4eMeta` directly. The `_typ`, `_version`, `_id` fields are protocol-level and language-independent.

Standard library types (`String`, `i32`, `f64`, `chrono::DateTime<Utc>`) are used as-is.

## Serde Behavior

### Serialization

German structs use `#[serde(rename_all = "camelCase")]`. Since German field names are already German, the camelCase conversion produces correct BO4E German JSON:

- `zaehlernummer` → `"zaehlernummer"` (no change, already lowercase)
- `marktlokations_id` → `"marktlokationsId"` (correct German camelCase)

### Deserialization

German structs accept both German and English JSON via `alias` attributes:

- Primary: German camelCase (from `rename_all`)
- Alias: English camelCase (added as `alias`)

### Wire Compatibility

English and German types produce different JSON field names but identical structure. Both are valid BO4E JSON — the standard supports both languages.

## Cargo.toml

```toml
[package]
name = "bo4e-german"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
description = "German API for BO4E (Business Objects for Energy) - Deutsche Feldnamen"
keywords = ["bo4e", "energy", "edi", "bdew", "deutsch"]
categories = ["data-structures", "encoding"]

[dependencies]
serde = { workspace = true }
chrono = { workspace = true }
bo4e-core = { workspace = true }
```

## Testing Strategy

1. **Roundtrip tests:** German struct → JSON → German struct
2. **Cross-language roundtrip:** English struct → German struct → JSON → German struct → English struct
3. **Wire compatibility:** Verify German JSON matches BO4E-Python German output
4. **From/Into correctness:** Property-based tests with proptest for all conversions
5. **Generator freshness check:** CI verifies generated code matches what the generator would produce

## CI Integration

```yaml
# In .github/workflows/ci.yml or a separate workflow
- name: Check German crate freshness
  run: |
    cargo run -p generate-german
    git diff --exit-code crates/bo4e-german/src/
```

## Implementation Phases

### Phase 1: Type Mapping Table
- Create `scripts/german_type_mapping.toml` with all 172 type mappings
- Extract German names from existing `Bo4eObject` impls and serde aliases

### Phase 2: Generator Script
- Create `scripts/generate-german/` as a Rust binary (workspace member)
- Uses `syn` to parse bo4e-core sources
- Uses `quote` to generate Rust code
- Uses `prettyplease` for formatting output

### Phase 3: bo4e-german Crate Skeleton
- Add `crates/bo4e-german/` to workspace
- Generate initial code for all 172 types
- Verify it compiles

### Phase 4: From/Into Conversions
- Generate all bidirectional conversions
- Handle Option, Vec, Box wrapper types

### Phase 5: Testing
- Roundtrip serialization tests
- Cross-language conversion tests
- CI freshness check

### Phase 6: Integration
- Add bo4e-german to workspace dependencies
- Optionally re-export from the `bo4e` facade crate
- Update documentation
