# BO4E-Rust Design Document

**Date:** 2026-01-20
**Status:** Approved
**Author:** Brainstorming session

## Overview

This document describes the design for BO4E-Rust, a Rust implementation of the BO4E (Business Objects for Energy) standard. BO4E is a standardized data model for the German energy industry, enabling communication between energy companies, network operators, and service providers.

### Goals

1. **Strict port** - Mirror the Python API structure for consistency across implementations
2. **Idiomatic Rust** - Leverage Rust's type system fully (newtypes, enums, compile-time guarantees)
3. **Performance-focused** - Optimized for high-throughput parsing/serialization using SIMD acceleration

### Non-Goals

- FFI bindings (PyO3, C, WASM) - out of scope for initial implementation
- Runtime validation beyond type system - validation is application-specific per BO4E philosophy

## Architecture

### Workspace Structure

```
bo4e-rust/
├── Cargo.toml                 # Workspace manifest
├── crates/
│   ├── bo4e-core/            # Core types (BOs, COMs, Enums)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── bo/           # 37 Business Objects
│   │   │   ├── com/          # 64 Components
│   │   │   ├── enums/        # 97 Enumerations
│   │   │   └── traits.rs     # Bo4eObject trait
│   │   └── Cargo.toml
│   │
│   ├── bo4e-serde/           # Serialization layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── config.rs     # Runtime German/English config
│   │   │   └── simd.rs       # simd-json integration
│   │   └── Cargo.toml
│   │
│   └── bo4e/                 # Facade crate (re-exports everything)
│       └── Cargo.toml
│
├── tests/                    # Integration tests
│   ├── golden/              # Python-generated JSON fixtures
│   └── compatibility.rs
│
└── .github/workflows/       # CI/CD
```

### Crate Responsibilities

- **bo4e-core**: Pure data types, no serialization dependencies beyond serde traits. Zero-cost abstractions.
- **bo4e-serde**: Handles simd-json integration, runtime language configuration, custom serializers for German/English field names.
- **bo4e**: Convenience crate that re-exports everything. Users can `use bo4e::prelude::*` for common types.

### Dependency Flow

```
bo4e-core  <-  bo4e-serde  <-  bo4e (facade)
    ^              ^
  serde        simd-json
```

## Core Trait System

### The `Bo4eObject` Trait

All BO4E types implement this trait for consistent metadata handling:

```rust
// bo4e-core/src/traits.rs

use crate::enums::{BoType, ComType};

/// Marker for the type discriminator
pub enum TypeDiscriminator {
    Bo(BoType),
    Com(ComType),
}

/// Core trait implemented by all BO4E types
pub trait Bo4eObject {
    /// Returns the type discriminator (_typ field)
    fn typ(&self) -> TypeDiscriminator;

    /// Returns the BO4E version
    fn version(&self) -> Option<&str>;

    /// Returns the external ID (_id field)
    fn id(&self) -> Option<&str>;

    /// Returns additional attributes for extensibility
    fn additional_attributes(&self) -> &[AdditionalAttribute];
}

/// Additional attribute for external system IDs and metadata
#[derive(Debug, Clone, PartialEq)]
pub struct AdditionalAttribute {
    pub name: String,
    pub value: Option<AttributeValue>,
}

pub enum AttributeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(Vec<AdditionalAttribute>),
}
```

### Derive Macro

A derive macro generates trait implementations:

```rust
#[derive(Debug, Clone, Bo4eObject)]
#[bo4e(typ = "Meter")]
pub struct Meter {
    #[bo4e(meta)]
    pub meta: Bo4eMeta,

    pub meter_number: Option<String>,
    pub meter_type: Option<MeterType>,
}
```

## Serialization & Language Configuration

### Runtime Configuration

```rust
// bo4e-serde/src/config.rs

/// Controls JSON field naming at runtime
#[derive(Debug, Clone, Default)]
pub enum JsonLanguage {
    #[default]
    German,   // "zaehlernummer", "marktlokationsId"
    English,  // "meterNumber", "marketLocationId"
}

/// Serialization configuration
#[derive(Debug, Clone)]
pub struct SerializeConfig {
    pub language: JsonLanguage,
    pub pretty: bool,
}
```

### Serialization API

```rust
// bo4e-serde/src/lib.rs

/// Serialize with configuration
pub fn to_json<T: Bo4eObject + Serialize>(
    value: &T,
    config: &SerializeConfig
) -> Result<String, Error>;

/// Deserialize (accepts both German and English field names)
pub fn from_json<T: Bo4eObject + DeserializeOwned>(
    json: &mut [u8]
) -> Result<T, Error>;

/// Convenience functions
pub fn to_json_german<T: Bo4eObject + Serialize>(value: &T) -> Result<String, Error>;
pub fn to_json_english<T: Bo4eObject + Serialize>(value: &T) -> Result<String, Error>;
```

### Field Name Mapping

Compile-time generated mapping table for zero-cost lookups:

```rust
const FIELD_NAMES: &[(&str, &str)] = &[
    ("meter_number", "zaehlernummer"),
    ("market_location_id", "marktlokationsId"),
    // ... all fields
];
```

## Type Definitions

### Naming Conventions

- **Rust code**: English names, snake_case (`meter_number`, `MarketLocation`)
- **JSON output**: Configurable German (default) or English, camelCase

### Enums (97 types)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnergyDirection {
    Consumption,  // Verbrauch
    FeedIn,       // Einspeisung
}
```

### Components (64 types)

```rust
#[derive(Debug, Clone, PartialEq, Bo4eObject)]
#[bo4e(typ = Com::Address)]
pub struct Address {
    #[bo4e(meta)]
    pub meta: Bo4eMeta,

    pub street: Option<String>,
    pub house_number: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<CountryCode>,
}
```

### Business Objects (37 types)

```rust
#[derive(Debug, Clone, PartialEq, Bo4eObject)]
#[bo4e(typ = Bo::Meter)]
pub struct Meter {
    #[bo4e(meta)]
    pub meta: Bo4eMeta,

    pub meter_number: Option<String>,
    pub meter_type: Option<MeterType>,
    pub division: Option<Division>,
    pub meter_readings: Option<Vec<MeterReading>>,
    pub market_location: Option<Box<MarketLocation>>,
}
```

### Design Decisions

- **All fields `Option<T>`**: Mirrors Python, validation is application-specific
- **`Box<T>` for cross-references**: Breaks cycles, keeps structs sized
- **Type-system validation**: Invalid states unrepresentable at compile time

## Testing Strategy

### Test Types

1. **Unit tests**: Per-type serialization/deserialization
2. **Golden file tests**: Python-generated JSON fixtures for compatibility
3. **Property-based tests**: `proptest` for randomized roundtrip testing

### Test Organization

```
tests/
├── golden/
│   ├── fixtures/           # JSON from Python
│   └── generate.py         # Regeneration script
├── unit/
│   ├── bo/
│   ├── com/
│   └── enums/
└── property/
```

### Golden File Example

```rust
#[test]
fn meter_roundtrip_german() {
    let fixture = include_str!("fixtures/meter.json");
    let mut json = fixture.as_bytes().to_vec();

    let meter: Meter = bo4e::from_json(&mut json).unwrap();
    let output = bo4e::to_json_german(&meter).unwrap();

    assert_json_eq!(fixture, &output);
}
```

## CI/CD Pipeline

### Continuous Integration

- **Platforms**: Linux, macOS, Windows
- **Rust versions**: stable, beta, MSRV (1.75)
- **Checks**: fmt, clippy, test, coverage
- **Golden tests**: Regenerate Python fixtures, verify compatibility

### Release Process

1. Tag with `v*` pattern
2. CI runs full test suite
3. Publish to crates.io in dependency order:
   - `bo4e-core`
   - `bo4e-serde`
   - `bo4e`

## Implementation Roadmap

### Phase 1: Foundation
- Initialize workspace and crate structure
- Set up CI/CD pipeline
- Implement `Bo4eObject` trait and derive macro
- Implement `Bo4eMeta` struct

### Phase 2: Enums (97 types)
- Port all 97 enumerations from Python
- Add English names with German serde aliases
- Unit tests for each enum

### Phase 3: Components (64 types)
- Port all 64 COM types
- Implement field mappings (English <-> German)
- Golden file tests against Python fixtures

### Phase 4: Business Objects (37 types)
- Port all 37 BO types
- Handle cross-references with `Box<T>`
- Property-based tests

### Phase 5: Serialization Layer
- Integrate simd-json
- Runtime language configuration
- Dual-format serialization (German/English)

### Phase 6: Polish
- Comprehensive documentation with examples
- Benchmark suite (vs Python, vs standard serde_json)
- Publish to crates.io

## Scope Estimates

- ~200 type definitions total
- ~15,000-20,000 lines of Rust
- Derive macro significantly reduces boilerplate

## References

- [BO4E Standard](https://www.bo4e.de/)
- [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python)
- [BO4E JSON Schemas](https://github.com/bo4e/BO4E-Schemas)
- [simd-json](https://github.com/simd-lite/simd-json)
