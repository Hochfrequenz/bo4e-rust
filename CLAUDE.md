# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

BO4E-Rust is a Rust implementation of the BO4E (Business Objects for Energy) standard - a standardized data model for the German energy industry. This is a **new project** being ported from [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python).

**Implementation plans:** See `docs/plans/` for detailed phase-by-phase implementation plans.

## Before Committing

**Always run before committing code changes:**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

CI will reject commits that fail formatting checks.

## Releasing

All crate versions are managed centrally in the workspace `Cargo.toml`:

```toml
[workspace.package]
version = "0.1.1"  # Change this for all crates

[workspace.dependencies]
bo4e-core = { path = "crates/bo4e-core", version = "0.1.1" }  # Update these too
bo4e-serde = { path = "crates/bo4e-serde", version = "0.1.1" }
```

**To release a new version:**

1. Update version in `Cargo.toml` (3 places: `workspace.package.version` and the two internal crate dependencies)
2. Commit: `git commit -am "chore: bump version to X.Y.Z"`
3. Push: `git push origin main`
4. Create and push tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z" && git push origin vX.Y.Z`

The release workflow will automatically publish to crates.io using OIDC trusted publishing.

## Build Commands

```bash
# Build workspace
cargo build --workspace

# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p bo4e-core
cargo test -p bo4e-serde
cargo test -p bo4e

# Run single test
cargo test -p bo4e-core test_name

# Check formatting (CI uses this)
cargo fmt --all --check

# Linting
cargo clippy --workspace --all-targets -- -D warnings

# Generate docs
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

# Run benchmarks (bo4e-serde)
cargo bench -p bo4e-serde
```

## Architecture

### Workspace Structure

Three crates with clear separation of concerns:

```
crates/
├── bo4e-core/     # Pure data types (BOs, COMs, Enums) - no serialization logic
├── bo4e-serde/    # SIMD-accelerated serialization (simd-json), runtime language config
└── bo4e/          # Facade crate - re-exports everything for `use bo4e::prelude::*`
```

**Dependency flow:** `bo4e-core` ← `bo4e-serde` ← `bo4e`

### Type Categories

- **Business Objects (BOs):** 37 top-level entities (`Meter`, `MarketLocation`, `Contract`, etc.)
- **Components (COMs):** 64 composite types (`Address`, `Price`, `MeterReading`, etc.)
- **Enumerations:** 97 type-safe enums (`Division`, `MeterType`, `EnergyDirection`, etc.)

### Key Design Decisions

1. **All fields `Option<T>`:** Mirrors Python implementation; validation is application-specific per BO4E philosophy
2. **`Box<T>` for cross-references:** Breaks cycles between BOs (e.g., `Meter` → `MarketLocation`)
3. **Bilingual JSON:** Runtime configurable German (default) or English field names
4. **SIMD parsing:** Uses `simd-json` for high-performance deserialization

### Serialization API

```rust
// German field names (BO4E standard)
let json = bo4e::to_json_german(&meter)?;

// English field names
let json = bo4e::to_json_english(&meter)?;

// Deserialize (accepts both languages, requires mutable slice for simd-json)
let mut bytes = json.into_bytes();
let meter: Meter = bo4e::from_json(&mut bytes)?;
```

### Bo4eObject Trait

All types implement `Bo4eObject` for consistent metadata handling (`_typ`, `_version`, `_id` fields).

## Naming Conventions

- **Rust code:** English names, snake_case (`meter_number`, `MarketLocation`)
- **JSON output:** Configurable - German camelCase default (`zaehlernummer`, `marktlokationsId`)

## Testing Strategy

- **Unit tests:** Per-type serialization roundtrips
- **Golden file tests:** JSON fixtures generated from Python for compatibility verification (`tests/golden/`)
- **Property-based tests:** `proptest` for randomized roundtrip testing

## MSRV

Minimum Supported Rust Version: **1.75**
