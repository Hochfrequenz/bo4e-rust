# Epic 1: Rust Infrastructure Setup

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Set up schemars dependency, feature flag, and schema generator binary.

**Architecture:** Add schemars as optional dependency, create `json-schema` feature, implement generator binary that outputs all schemas as JSON.

**Tech Stack:** schemars 0.8, serde_json

---

## Task 1: Add schemars Dependency

**Files:**
- Modify: `crates/bo4e-core/Cargo.toml`

**Step 1: Add schemars dependency**

Add to `crates/bo4e-core/Cargo.toml`:

```toml
[dependencies]
serde = { workspace = true }
chrono = { workspace = true }
schemars = { version = "0.8", optional = true }

[features]
default = []
json-schema = ["schemars"]
```

**Step 2: Verify it compiles**

Run: `cargo build -p bo4e-core`
Expected: Success

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 3: Commit**

```bash
git add crates/bo4e-core/Cargo.toml
git commit -m "feat(drift): add schemars dependency with json-schema feature"
```

---

## Task 2: Add JsonSchema derive to Bo4eMeta

**Files:**
- Modify: `crates/bo4e-core/src/traits.rs`

**Step 1: Read current traits.rs**

Run: `head -50 crates/bo4e-core/src/traits.rs`

**Step 2: Add schemars derive to Bo4eMeta**

The `Bo4eMeta` struct needs the derive. Add after the existing derives:

```rust
/// Metadata common to all BO4E objects.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Bo4eMeta {
    // ... existing fields
}
```

Also add to `AdditionalAttribute` if it exists:

```rust
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct AdditionalAttribute {
    // ... existing fields
}
```

**Step 3: Verify it compiles**

Run: `cargo build -p bo4e-core --features json-schema`
Expected: Success

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/traits.rs
git commit -m "feat(drift): add JsonSchema derive to Bo4eMeta"
```

---

## Task 3: Create Schema Generator Binary Skeleton

**Files:**
- Create: `crates/bo4e-core/src/bin/generate_schema.rs`

**Step 1: Create the binary file**

Create `crates/bo4e-core/src/bin/generate_schema.rs`:

```rust
//! Schema generator for drift detection.
//!
//! Outputs JSON schemas for all BO4E types.
//!
//! Run with: cargo run --bin generate_schema --features json-schema

use serde_json::json;

fn main() {
    let schemas = json!({
        "bo": {},
        "com": {},
        "enum": {}
    });

    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
}
```

**Step 2: Add serde_json as dev-dependency**

Add to `crates/bo4e-core/Cargo.toml` under `[dev-dependencies]`:

```toml
[dev-dependencies]
serde_json = { workspace = true }
```

**Step 3: Verify it runs**

Run: `cargo run --bin generate_schema --features json-schema`
Expected: Outputs `{"bo":{},"com":{},"enum":{}}`

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/bin/generate_schema.rs crates/bo4e-core/Cargo.toml
git commit -m "feat(drift): add schema generator binary skeleton"
```

---

## Task 4: Create schemas Directory

**Files:**
- Create: `schemas/.gitignore`

**Step 1: Create directory and gitignore**

```bash
mkdir -p schemas
```

Create `schemas/.gitignore`:

```
# Generated schema files
python_schema.json
rust_schema.json
```

**Step 2: Commit**

```bash
git add schemas/.gitignore
git commit -m "feat(drift): add schemas directory with gitignore"
```

---

## Task 5: Verify Full Infrastructure

**Step 1: Run full build with feature**

Run: `cargo build --workspace --features json-schema`
Expected: Success (may warn about unused schemars, that's OK)

**Step 2: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 3: Verify generator runs**

Run: `cargo run --bin generate_schema --features json-schema`
Expected: Outputs JSON structure

---

## Epic 1 Completion Checklist

- [ ] schemars dependency added with optional feature
- [ ] Bo4eMeta has JsonSchema derive
- [ ] Schema generator binary exists and runs
- [ ] schemas/ directory created with .gitignore
- [ ] All tests pass
