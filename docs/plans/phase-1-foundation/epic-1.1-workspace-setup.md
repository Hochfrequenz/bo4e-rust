---
status: in_progress
---
# Epic 1.1: Workspace Setup

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Initialize the Cargo workspace with three crates: bo4e-core, bo4e-serde, and bo4e facade.

**Architecture:** Multi-crate workspace where bo4e-core contains pure types, bo4e-serde handles serialization, and bo4e re-exports everything for convenience.

**Tech Stack:** Rust 1.75+, Cargo workspaces

---

## Task 1: Initialize Workspace Root

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`

**Step 1: Create workspace Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/bo4e-core",
    "crates/bo4e-serde",
    "crates/bo4e",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
license = "MIT"
repository = "https://github.com/hochfrequenz/bo4e-rust"
authors = ["Hochfrequenz Unternehmensberatung GmbH"]
keywords = ["bo4e", "energy", "edi", "bdew"]
categories = ["data-structures", "encoding"]

[workspace.dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
simd-json = "0.14"

# Testing
proptest = "1.4"
serde_json = "1.0"

# Internal crates
bo4e-core = { path = "crates/bo4e-core", version = "0.1.0" }
bo4e-serde = { path = "crates/bo4e-serde", version = "0.1.0" }
```

**Step 2: Create .gitignore**

```
/target
Cargo.lock
*.swp
*.swo
.DS_Store
.idea/
.vscode/
```

**Step 3: Commit**

```bash
git add Cargo.toml .gitignore
git commit -m "chore: initialize cargo workspace"
```

---

## Task 2: Create bo4e-core Crate

**Files:**
- Create: `crates/bo4e-core/Cargo.toml`
- Create: `crates/bo4e-core/src/lib.rs`

**Step 1: Create crate directory**

```bash
mkdir -p crates/bo4e-core/src
```

**Step 2: Create Cargo.toml**

```toml
[package]
name = "bo4e-core"
description = "Core types for BO4E (Business Objects for Energy)"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
keywords.workspace = true
categories.workspace = true

[dependencies]
serde = { workspace = true }

[dev-dependencies]
serde_json = { workspace = true }
```

**Step 3: Create lib.rs**

```rust
//! # bo4e-core
//!
//! Core types for BO4E (Business Objects for Energy) - a standard for
//! data exchange in the German energy industry.
//!
//! This crate provides:
//! - Business Objects (BOs): Top-level entities like `Meter`, `MarketLocation`
//! - Components (COMs): Composite types like `Address`, `Price`
//! - Enumerations: Type-safe enums for all BO4E enum values
//!
//! ## Example
//!
//! ```rust
//! use bo4e_core::bo::Meter;
//!
//! let meter = Meter::default();
//! ```

pub mod bo;
pub mod com;
pub mod enums;
pub mod traits;

pub use traits::{Bo4eObject, Bo4eMeta};
```

**Step 4: Create placeholder modules**

Create `crates/bo4e-core/src/traits.rs`:
```rust
//! Core traits for BO4E types.

/// Metadata common to all BO4E objects.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Bo4eMeta {
    /// BO4E schema version
    pub version: Option<String>,
    /// External system ID
    pub id: Option<String>,
}

/// Trait implemented by all BO4E types.
pub trait Bo4eObject {
    /// Returns the type name as used in the `_typ` field.
    fn type_name() -> &'static str;
}
```

Create `crates/bo4e-core/src/bo/mod.rs`:
```rust
//! Business Objects (BOs) - top-level entities in BO4E.
```

Create `crates/bo4e-core/src/com/mod.rs`:
```rust
//! Components (COMs) - composite types used within Business Objects.
```

Create `crates/bo4e-core/src/enums/mod.rs`:
```rust
//! Enumerations for BO4E type-safe values.
```

**Step 5: Verify it compiles**

```bash
cargo check -p bo4e-core
```
Expected: Compiles successfully

**Step 6: Commit**

```bash
git add crates/bo4e-core/
git commit -m "feat(core): initialize bo4e-core crate structure"
```

---

## Task 3: Create bo4e-serde Crate

**Files:**
- Create: `crates/bo4e-serde/Cargo.toml`
- Create: `crates/bo4e-serde/src/lib.rs`
- Create: `crates/bo4e-serde/src/config.rs`

**Step 1: Create crate directory**

```bash
mkdir -p crates/bo4e-serde/src
```

**Step 2: Create Cargo.toml**

```toml
[package]
name = "bo4e-serde"
description = "High-performance serialization for BO4E types"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
keywords.workspace = true
categories.workspace = true

[dependencies]
bo4e-core = { workspace = true }
serde = { workspace = true }
simd-json = { workspace = true }

[dev-dependencies]
serde_json = { workspace = true }
```

**Step 3: Create lib.rs**

```rust
//! # bo4e-serde
//!
//! High-performance serialization for BO4E types using SIMD-accelerated JSON parsing.
//!
//! ## Features
//!
//! - SIMD-accelerated JSON parsing via `simd-json`
//! - Runtime configuration for German or English field names
//! - Compatible with standard `serde` traits
//!
//! ## Example
//!
//! ```rust,ignore
//! use bo4e_serde::{to_json_german, from_json};
//! use bo4e_core::bo::Meter;
//!
//! let meter = Meter::default();
//! let json = to_json_german(&meter).unwrap();
//! ```

mod config;

pub use config::{JsonLanguage, SerializeConfig};

use bo4e_core::Bo4eObject;
use serde::{de::DeserializeOwned, Serialize};

/// Error type for serialization operations.
#[derive(Debug)]
pub enum Error {
    /// JSON serialization error
    Serialize(String),
    /// JSON deserialization error
    Deserialize(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Serialize(msg) => write!(f, "serialization error: {}", msg),
            Error::Deserialize(msg) => write!(f, "deserialization error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Serialize a BO4E object to JSON with German field names (default).
pub fn to_json_german<T: Bo4eObject + Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Serialize a BO4E object to JSON with English field names.
pub fn to_json_english<T: Bo4eObject + Serialize>(value: &T) -> Result<String, Error> {
    // TODO: Implement English field name mapping
    serde_json::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Deserialize a BO4E object from JSON.
///
/// Note: simd-json requires a mutable slice for in-place parsing.
pub fn from_json<T: Bo4eObject + DeserializeOwned>(json: &mut [u8]) -> Result<T, Error> {
    simd_json::from_slice(json).map_err(|e| Error::Deserialize(e.to_string()))
}
```

**Step 4: Create config.rs**

```rust
//! Runtime configuration for JSON serialization.

/// Controls JSON field naming language.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JsonLanguage {
    /// German field names (e.g., "zaehlernummer", "marktlokationsId")
    #[default]
    German,
    /// English field names (e.g., "meterNumber", "marketLocationId")
    English,
}

/// Configuration for JSON serialization.
#[derive(Debug, Clone)]
pub struct SerializeConfig {
    /// Language for field names
    pub language: JsonLanguage,
    /// Pretty-print JSON output
    pub pretty: bool,
}

impl Default for SerializeConfig {
    fn default() -> Self {
        Self {
            language: JsonLanguage::German,
            pretty: false,
        }
    }
}

impl SerializeConfig {
    /// Create config for German JSON output.
    pub fn german() -> Self {
        Self::default()
    }

    /// Create config for English JSON output.
    pub fn english() -> Self {
        Self {
            language: JsonLanguage::English,
            pretty: false,
        }
    }

    /// Enable pretty-printing.
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }
}
```

**Step 5: Verify it compiles**

```bash
cargo check -p bo4e-serde
```
Expected: Compiles successfully

**Step 6: Commit**

```bash
git add crates/bo4e-serde/
git commit -m "feat(serde): initialize bo4e-serde crate with simd-json"
```

---

## Task 4: Create bo4e Facade Crate

**Files:**
- Create: `crates/bo4e/Cargo.toml`
- Create: `crates/bo4e/src/lib.rs`

**Step 1: Create crate directory**

```bash
mkdir -p crates/bo4e/src
```

**Step 2: Create Cargo.toml**

```toml
[package]
name = "bo4e"
description = "BO4E (Business Objects for Energy) - Rust implementation"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
keywords.workspace = true
categories.workspace = true

[dependencies]
bo4e-core = { workspace = true }
bo4e-serde = { workspace = true }

[dev-dependencies]
serde_json = { workspace = true }
```

**Step 3: Create lib.rs**

```rust
//! # bo4e
//!
//! Rust implementation of BO4E (Business Objects for Energy) - a standard for
//! data exchange in the German energy industry.
//!
//! This crate re-exports types from `bo4e-core` and serialization from `bo4e-serde`
//! for convenient single-crate usage.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use bo4e::prelude::*;
//!
//! // Create a meter
//! let meter = Meter::default();
//!
//! // Serialize to German JSON (BO4E standard)
//! let json = to_json_german(&meter).unwrap();
//!
//! // Deserialize
//! let mut bytes = json.into_bytes();
//! let parsed: Meter = from_json(&mut bytes).unwrap();
//! ```
//!
//! ## Crate Structure
//!
//! - `bo4e::bo` - Business Objects (Meter, MarketLocation, Contract, etc.)
//! - `bo4e::com` - Components (Address, Price, MeterReading, etc.)
//! - `bo4e::enums` - Enumerations (MeterType, Division, EnergyDirection, etc.)
//!
//! ## Serialization
//!
//! BO4E uses German field names in JSON by default. Use `to_json_english` for
//! English field names.

// Re-export core types
pub use bo4e_core::bo;
pub use bo4e_core::com;
pub use bo4e_core::enums;
pub use bo4e_core::traits;
pub use bo4e_core::{Bo4eMeta, Bo4eObject};

// Re-export serialization
pub use bo4e_serde::{from_json, to_json_english, to_json_german};
pub use bo4e_serde::{Error, JsonLanguage, SerializeConfig};

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::bo::*;
    pub use crate::com::*;
    pub use crate::enums::*;
    pub use crate::{from_json, to_json_english, to_json_german};
    pub use crate::{Bo4eMeta, Bo4eObject};
}
```

**Step 4: Verify workspace compiles**

```bash
cargo check --workspace
```
Expected: Compiles successfully

**Step 5: Commit**

```bash
git add crates/bo4e/
git commit -m "feat: add bo4e facade crate with re-exports"
```

---

## Task 5: Add LICENSE and README

**Files:**
- Create: `LICENSE`
- Create: `README.md`

**Step 1: Create MIT LICENSE**

```
MIT License

Copyright (c) 2026 Hochfrequenz Unternehmensberatung GmbH

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

**Step 2: Create README.md**

```markdown
# BO4E-Rust

Rust implementation of [BO4E (Business Objects for Energy)](https://www.bo4e.de/) - a standard for data exchange in the German energy industry.

## Features

- **Type-safe**: Leverage Rust's type system for compile-time guarantees
- **High-performance**: SIMD-accelerated JSON parsing via simd-json
- **Bilingual**: Runtime configuration for German or English JSON field names
- **Compatible**: Interoperable with BO4E-Python and other implementations

## Installation

```toml
[dependencies]
bo4e = "0.1"
```

## Quick Start

```rust
use bo4e::prelude::*;

// Create a meter
let meter = Meter {
    meter_number: Some("1234567890".to_string()),
    ..Default::default()
};

// Serialize to German JSON (BO4E standard)
let json = to_json_german(&meter).unwrap();

// Deserialize
let mut bytes = json.into_bytes();
let parsed: Meter = from_json(&mut bytes).unwrap();
```

## Crate Structure

| Crate | Description |
|-------|-------------|
| `bo4e` | Facade crate - use this |
| `bo4e-core` | Pure data types (BOs, COMs, Enums) |
| `bo4e-serde` | Serialization with simd-json |

## Related Projects

- [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python) - Python implementation
- [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) - JSON schemas
- [bo4e.de](https://www.bo4e.de/) - Official BO4E website

## License

MIT License - see [LICENSE](LICENSE)
```

**Step 3: Commit**

```bash
git add LICENSE README.md
git commit -m "docs: add LICENSE and README"
```

---

## Verification

Run full workspace check:

```bash
cargo check --workspace
cargo test --workspace
```

Expected: All checks pass, no tests yet but compiles clean.

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 1 |
| Passed | 1 |
| Failed | 0 |
| Skipped | 2 |
| Coverage | N/A |

Files created:
- Cargo.toml (workspace root)
- .gitignore
- crates/bo4e-core/Cargo.toml
- crates/bo4e-core/src/lib.rs
- crates/bo4e-core/src/traits.rs
- crates/bo4e-core/src/bo/mod.rs
- crates/bo4e-core/src/bo/meter.rs
- crates/bo4e-core/src/com/mod.rs
- crates/bo4e-core/src/enums/mod.rs
- crates/bo4e-serde/Cargo.toml
- crates/bo4e-serde/src/lib.rs
- crates/bo4e-serde/src/config.rs
- crates/bo4e/Cargo.toml
- crates/bo4e/src/lib.rs
- LICENSE
- README.md

Verification results:
- `cargo build --workspace`: SUCCESS (no errors)
- `cargo test --workspace`: SUCCESS (1 doc-test passed, 2 ignored)
- `cargo clippy --workspace --all-targets -- -D warnings`: SUCCESS (no warnings)
- `cargo fmt --all --check`: SUCCESS (correctly formatted)
