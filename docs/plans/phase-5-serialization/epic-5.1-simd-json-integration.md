---
status: complete
---
# Epic 5.1: SIMD-JSON Integration

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Integrate simd-json for high-performance JSON parsing.

**Architecture:** Wrapper functions around simd-json with fallback to serde_json.

**Tech Stack:** simd-json, serde, criterion for benchmarks

---

## Task 1: Update Dependencies

**Files:**
- Modify: `Cargo.toml` (workspace)
- Modify: `crates/bo4e-serde/Cargo.toml`

**Step 1: Update workspace Cargo.toml**

```toml
[workspace.dependencies]
simd-json = "0.14"
criterion = { version = "0.5", features = ["html_reports"] }
```

**Step 2: Update bo4e-serde Cargo.toml**

```toml
[dependencies]
simd-json = { workspace = true }

[dev-dependencies]
criterion = { workspace = true }
serde_json = { workspace = true }

[[bench]]
name = "json_parsing"
harness = false
```

**Step 3: Commit**

```bash
git add Cargo.toml crates/bo4e-serde/Cargo.toml
git commit -m "chore(serde): add simd-json and criterion dependencies"
```

---

## Task 2: Implement SIMD Deserializer

**Files:**
- Create: `crates/bo4e-serde/src/simd.rs`
- Modify: `crates/bo4e-serde/src/lib.rs`

**Step 1: Write the implementation**

Create `crates/bo4e-serde/src/simd.rs`:

```rust
//! SIMD-accelerated JSON parsing.

use serde::de::DeserializeOwned;

/// Deserialize from a mutable byte slice using simd-json.
///
/// simd-json performs in-place parsing, which requires a mutable slice.
/// The input will be modified during parsing.
///
/// # Example
///
/// ```rust,ignore
/// use bo4e_serde::simd::from_slice;
/// use bo4e_core::bo::Meter;
///
/// let mut json = br#"{"meterNumber":"123"}"#.to_vec();
/// let meter: Meter = from_slice(&mut json)?;
/// ```
pub fn from_slice<T: DeserializeOwned>(json: &mut [u8]) -> Result<T, simd_json::Error> {
    simd_json::from_slice(json)
}

/// Deserialize from a string, converting to mutable bytes internally.
///
/// This is slightly less efficient than `from_slice` as it requires
/// copying the string to a mutable buffer.
pub fn from_str<T: DeserializeOwned>(json: &str) -> Result<T, simd_json::Error> {
    let mut bytes = json.as_bytes().to_vec();
    from_slice(&mut bytes)
}

/// Deserialize from owned bytes (Vec<u8>).
///
/// Takes ownership of the vector, avoiding copies.
pub fn from_vec<T: DeserializeOwned>(mut json: Vec<u8>) -> Result<T, simd_json::Error> {
    from_slice(&mut json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bo4e_core::bo::Meter;

    #[test]
    fn test_from_slice() {
        let mut json = br#"{"meterNumber":"TEST123"}"#.to_vec();
        let meter: Meter = from_slice(&mut json).unwrap();
        assert_eq!(meter.meter_number, Some("TEST123".to_string()));
    }

    #[test]
    fn test_from_str() {
        let json = r#"{"meterNumber":"TEST456"}"#;
        let meter: Meter = from_str(json).unwrap();
        assert_eq!(meter.meter_number, Some("TEST456".to_string()));
    }

    #[test]
    fn test_from_vec() {
        let json = br#"{"meterNumber":"TEST789"}"#.to_vec();
        let meter: Meter = from_vec(json).unwrap();
        assert_eq!(meter.meter_number, Some("TEST789".to_string()));
    }
}
```

**Step 2: Export from lib.rs**

Add to `crates/bo4e-serde/src/lib.rs`:

```rust
pub mod simd;

pub use simd::{from_slice, from_str, from_vec};
```

**Step 3: Run tests and commit**

```bash
cargo test -p bo4e-serde simd -- --nocapture
git add crates/bo4e-serde/src/simd.rs crates/bo4e-serde/src/lib.rs
git commit -m "feat(serde): add simd-json deserialization functions"
```

---

## Task 3: Create Benchmark Suite

**Files:**
- Create: `crates/bo4e-serde/benches/json_parsing.rs`

**Step 1: Write benchmarks**

```rust
//! JSON parsing benchmarks comparing simd-json vs serde_json.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use bo4e_core::bo::Meter;
use bo4e_core::enums::Division;

fn create_test_json() -> String {
    // Create a realistic meter JSON
    serde_json::to_string(&Meter {
        meter_number: Some("1EMH0012345678".to_string()),
        division: Some(Division::Electricity),
        ..Default::default()
    }).unwrap()
}

fn create_large_json() -> String {
    // Array of 1000 meters
    let meters: Vec<Meter> = (0..1000)
        .map(|i| Meter {
            meter_number: Some(format!("1EMH{:010}", i)),
            division: Some(Division::Electricity),
            ..Default::default()
        })
        .collect();
    serde_json::to_string(&meters).unwrap()
}

fn bench_single_object(c: &mut Criterion) {
    let json = create_test_json();
    let json_bytes = json.as_bytes().to_vec();

    let mut group = c.benchmark_group("single_object");
    group.throughput(Throughput::Bytes(json.len() as u64));

    group.bench_function("serde_json", |b| {
        b.iter(|| {
            let meter: Meter = serde_json::from_str(black_box(&json)).unwrap();
            black_box(meter)
        })
    });

    group.bench_function("simd_json", |b| {
        b.iter(|| {
            let mut bytes = json_bytes.clone();
            let meter: Meter = bo4e_serde::from_slice(black_box(&mut bytes)).unwrap();
            black_box(meter)
        })
    });

    group.finish();
}

fn bench_array_1000(c: &mut Criterion) {
    let json = create_large_json();
    let json_bytes = json.as_bytes().to_vec();

    let mut group = c.benchmark_group("array_1000");
    group.throughput(Throughput::Bytes(json.len() as u64));

    group.bench_function("serde_json", |b| {
        b.iter(|| {
            let meters: Vec<Meter> = serde_json::from_str(black_box(&json)).unwrap();
            black_box(meters)
        })
    });

    group.bench_function("simd_json", |b| {
        b.iter(|| {
            let mut bytes = json_bytes.clone();
            let meters: Vec<Meter> = bo4e_serde::from_slice(black_box(&mut bytes)).unwrap();
            black_box(meters)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_single_object, bench_array_1000);
criterion_main!(benches);
```

**Step 2: Run benchmarks**

```bash
cargo bench -p bo4e-serde
```

**Step 3: Commit**

```bash
git add crates/bo4e-serde/benches/
git commit -m "feat(serde): add JSON parsing benchmarks"
```

---

## Task 4: Add Serialization Functions

**Files:**
- Create: `crates/bo4e-serde/src/serialize.rs`

**Step 1: Write implementation**

```rust
//! JSON serialization functions.

use serde::Serialize;

/// Serialize to a compact JSON string.
pub fn to_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serialize to a pretty-printed JSON string.
pub fn to_string_pretty<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

/// Serialize to a byte vector.
pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bo4e_core::bo::Meter;

    #[test]
    fn test_to_string() {
        let meter = Meter {
            meter_number: Some("TEST".to_string()),
            ..Default::default()
        };

        let json = to_string(&meter).unwrap();
        assert!(json.contains(r#""meterNumber":"TEST""#));
    }

    #[test]
    fn test_to_string_pretty() {
        let meter = Meter::default();
        let json = to_string_pretty(&meter).unwrap();
        assert!(json.contains('\n'));
    }
}
```

**Step 2: Export and commit**

```bash
git add crates/bo4e-serde/src/serialize.rs crates/bo4e-serde/src/lib.rs
git commit -m "feat(serde): add JSON serialization functions"
```

---

## Verification

```bash
cargo test -p bo4e-serde
cargo bench -p bo4e-serde
```

Expected: simd-json should be 2-4x faster than serde_json for large payloads.

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 738 |
| Passed | 738 |
| Failed | 0 |
| Skipped | 3 |

Files tested:
- crates/bo4e-serde/src/simd.rs
- crates/bo4e-serde/src/serialize.rs
- crates/bo4e-serde/benches/json_parsing.rs
