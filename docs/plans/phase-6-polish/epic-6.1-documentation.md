---
status: in_progress
---
# Epic 6.1: Documentation

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create comprehensive documentation with examples for all public APIs.

**Architecture:** Rustdoc with examples, mdBook for guide (optional).

**Tech Stack:** rustdoc, doc-tests

---

## Task 1: Add Module-Level Documentation

**Files:**
- Modify: All `mod.rs` and `lib.rs` files

**Step 1: Add crate-level docs to bo4e/src/lib.rs**

```rust
//! # BO4E - Business Objects for Energy
//!
//! Rust implementation of the BO4E standard for data exchange in the
//! German energy industry.
//!
//! ## Quick Start
//!
//! ```rust
//! use bo4e::prelude::*;
//!
//! // Create a meter
//! let meter = Meter {
//!     meter_number: Some("1EMH0012345678".to_string()),
//!     division: Some(Division::Electricity),
//!     meter_type: Some(MeterType::ModernMeasuringDevice),
//!     ..Default::default()
//! };
//!
//! // Serialize to German JSON (BO4E standard)
//! let json = to_json_german(&meter).unwrap();
//! println!("{}", json);
//!
//! // Deserialize
//! let mut bytes = json.into_bytes();
//! let parsed: Meter = from_json(&mut bytes).unwrap();
//! assert_eq!(parsed.meter_number, Some("1EMH0012345678".to_string()));
//! ```
//!
//! ## Crate Structure
//!
//! - [`bo`] - Business Objects (Meter, MarketLocation, Contract, etc.)
//! - [`com`] - Components (Address, Price, MeterReading, etc.)
//! - [`enums`] - Enumerations (Division, MeterType, ContractStatus, etc.)
//!
//! ## Serialization
//!
//! BO4E uses German field names in JSON by default to maintain compatibility
//! with the standard. Use [`to_json_english`] for English field names.
//!
//! ```rust
//! use bo4e::prelude::*;
//!
//! let meter = Meter::default();
//!
//! // German (standard)
//! let german = to_json_german(&meter).unwrap();
//!
//! // English
//! let english = to_json_english(&meter).unwrap();
//! ```
//!
//! ## Performance
//!
//! This crate uses [simd-json](https://github.com/simd-lite/simd-json) for
//! SIMD-accelerated JSON parsing. For best performance, use [`from_json`]
//! with a mutable byte slice.
//!
//! ## Related Projects
//!
//! - [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python) - Python implementation
//! - [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) - JSON schemas
//! - [bo4e.de](https://www.bo4e.de/) - Official website
```

**Step 2: Run doc tests**

```bash
cargo test --doc -p bo4e
```

**Step 3: Commit**

```bash
git add crates/bo4e/src/lib.rs
git commit -m "docs: add comprehensive crate-level documentation"
```

---

## Task 2: Add Examples for Major Types

**Files:**
- Create: `crates/bo4e/examples/meter_roundtrip.rs`
- Create: `crates/bo4e/examples/market_location.rs`
- Create: `crates/bo4e/examples/invoice.rs`

**Step 1: Create meter_roundtrip.rs**

```rust
//! Example: Creating and serializing a meter.
//!
//! Run with: `cargo run --example meter_roundtrip`

use bo4e::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a smart meter
    let meter = Meter {
        meta: Bo4eMeta::with_type("Zaehler").version("202401.0.1"),
        meter_number: Some("1EMH0012345678".to_string()),
        division: Some(Division::Electricity),
        meter_type: Some(MeterType::IntelligentMeasuringSystem),
        registers: vec![
            MeterRegister {
                obis_code: Some("1-0:1.8.0".to_string()),
                energy_direction: Some(EnergyDirection::Consumption),
                unit: Some(Unit::KilowattHour),
                description: Some("Bezug".to_string()),
                ..Default::default()
            },
            MeterRegister {
                obis_code: Some("1-0:2.8.0".to_string()),
                energy_direction: Some(EnergyDirection::FeedIn),
                unit: Some(Unit::KilowattHour),
                description: Some("Einspeisung".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Serialize to German JSON
    let json = to_json_german(&meter)?;
    println!("German JSON:\n{}\n", json);

    // Serialize to English JSON
    let english = to_json_english(&meter)?;
    println!("English JSON:\n{}\n", english);

    // Parse back
    let mut bytes = json.into_bytes();
    let parsed: Meter = from_json(&mut bytes)?;

    assert_eq!(parsed.meter_number, Some("1EMH0012345678".to_string()));
    println!("Roundtrip successful!");

    Ok(())
}
```

**Step 2: Create market_location.rs**

```rust
//! Example: Working with market locations and associated metering locations.
//!
//! Run with: `cargo run --example market_location`

use bo4e::prelude::*;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a market location for electricity consumption
    let malo = MarketLocation {
        meta: Bo4eMeta::with_type("Marktlokation"),
        market_location_id: Some("12345678901".to_string()),
        division: Some(Division::Electricity),
        energy_direction: Some(EnergyDirection::Consumption),
        customer_type: Some(CustomerType::Private),
        address: Some(Address {
            street: Some("Musterstraße".to_string()),
            house_number: Some("42".to_string()),
            postal_code: Some("50667".to_string()),
            city: Some("Köln".to_string()),
            ..Default::default()
        }),
        annual_consumption: Some(3500.0), // kWh
        supply_start: Some(Utc::now()),
        ..Default::default()
    };

    let json = to_json_german(&malo)?;
    println!("Market Location JSON:\n{}", json);

    Ok(())
}
```

**Step 3: Run examples**

```bash
cargo run --example meter_roundtrip
cargo run --example market_location
```

**Step 4: Commit**

```bash
git add crates/bo4e/examples/
git commit -m "docs: add usage examples"
```

---

## Task 3: Generate and Review Documentation

**Step 1: Generate documentation**

```bash
cargo doc --workspace --no-deps --open
```

**Step 2: Check for missing docs**

```bash
# Add to lib.rs of each crate
#![warn(missing_docs)]
```

**Step 3: Fix any missing documentation**

Review the generated docs and add missing documentation for public items.

**Step 4: Commit**

```bash
git add -A
git commit -m "docs: add missing documentation"
```

---

## Verification

```bash
# All doc tests pass
cargo test --doc --workspace

# Examples run
cargo run --example meter_roundtrip

# Docs generate without warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```
