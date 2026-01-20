---
status: in_progress
---
# Epic 6.2: Benchmarks

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create comprehensive benchmarks comparing Rust implementation to Python.

**Architecture:** Criterion benchmarks, hyperfine for CLI comparison.

**Tech Stack:** criterion, hyperfine, Python (for comparison)

---

## Task 1: Create Comprehensive Benchmark Suite

**Files:**
- Modify: `crates/bo4e-serde/benches/json_parsing.rs`
- Create: `crates/bo4e-serde/benches/serialization.rs`

**Step 1: Expand json_parsing.rs**

```rust
//! Comprehensive JSON parsing benchmarks.

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use bo4e::prelude::*;

fn create_meter() -> Meter {
    Meter {
        meta: Bo4eMeta::with_type("Zaehler"),
        meter_number: Some("1EMH0012345678".to_string()),
        division: Some(Division::Electricity),
        meter_type: Some(MeterType::ModernMeasuringDevice),
        registers: vec![
            MeterRegister {
                obis_code: Some("1-0:1.8.0".to_string()),
                energy_direction: Some(EnergyDirection::Consumption),
                unit: Some(Unit::KilowattHour),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}

fn create_market_location() -> MarketLocation {
    MarketLocation {
        meta: Bo4eMeta::with_type("Marktlokation"),
        market_location_id: Some("12345678901".to_string()),
        division: Some(Division::Electricity),
        energy_direction: Some(EnergyDirection::Consumption),
        address: Some(Address {
            street: Some("Musterstraße".to_string()),
            house_number: Some("42".to_string()),
            postal_code: Some("50667".to_string()),
            city: Some("Köln".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn bench_deserialization(c: &mut Criterion) {
    let meter_json = bo4e::to_json_german(&create_meter()).unwrap();
    let malo_json = bo4e::to_json_german(&create_market_location()).unwrap();

    let mut group = c.benchmark_group("deserialization");

    // Single objects
    group.throughput(Throughput::Bytes(meter_json.len() as u64));
    group.bench_function("meter/simd_json", |b| {
        b.iter(|| {
            let mut bytes = meter_json.as_bytes().to_vec();
            let _: Meter = bo4e::from_json(black_box(&mut bytes)).unwrap();
        })
    });

    group.bench_function("meter/serde_json", |b| {
        b.iter(|| {
            let _: Meter = serde_json::from_str(black_box(&meter_json)).unwrap();
        })
    });

    group.throughput(Throughput::Bytes(malo_json.len() as u64));
    group.bench_function("market_location/simd_json", |b| {
        b.iter(|| {
            let mut bytes = malo_json.as_bytes().to_vec();
            let _: MarketLocation = bo4e::from_json(black_box(&mut bytes)).unwrap();
        })
    });

    group.finish();
}

fn bench_batch_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_parsing");

    for size in [10, 100, 1000] {
        let meters: Vec<Meter> = (0..size).map(|i| {
            Meter {
                meter_number: Some(format!("1EMH{:010}", i)),
                division: Some(Division::Electricity),
                ..Default::default()
            }
        }).collect();

        let json = serde_json::to_string(&meters).unwrap();
        let bytes = json.as_bytes().to_vec();

        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("simd_json", size),
            &bytes,
            |b, bytes| {
                b.iter(|| {
                    let mut bytes = bytes.clone();
                    let _: Vec<Meter> = bo4e::from_json(black_box(&mut bytes)).unwrap();
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("serde_json", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let _: Vec<Meter> = serde_json::from_str(black_box(json)).unwrap();
                })
            },
        );
    }

    group.finish();
}

fn bench_serialization(c: &mut Criterion) {
    let meter = create_meter();
    let malo = create_market_location();

    let mut group = c.benchmark_group("serialization");

    group.bench_function("meter", |b| {
        b.iter(|| {
            let _ = bo4e::to_json_german(black_box(&meter)).unwrap();
        })
    });

    group.bench_function("market_location", |b| {
        b.iter(|| {
            let _ = bo4e::to_json_german(black_box(&malo)).unwrap();
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_deserialization,
    bench_batch_parsing,
    bench_serialization
);
criterion_main!(benches);
```

**Step 2: Run benchmarks**

```bash
cargo bench -p bo4e-serde
```

**Step 3: Commit**

```bash
git add crates/bo4e-serde/benches/
git commit -m "perf: expand benchmark suite"
```

---

## Task 2: Create Python Comparison Script

**Files:**
- Create: `benches/python_comparison/benchmark.py`
- Create: `benches/python_comparison/requirements.txt`

**Step 1: Create requirements.txt**

```
bo4e>=202401.0.0
orjson
```

**Step 2: Create benchmark.py**

```python
#!/usr/bin/env python3
"""Benchmark BO4E-Python for comparison with Rust."""

import json
import time
import statistics

from bo4e import Zaehler
from bo4e.enum import Sparte

try:
    import orjson
    HAS_ORJSON = True
except ImportError:
    HAS_ORJSON = False


def create_meter(i: int) -> Zaehler:
    return Zaehler(
        zaehlernummer=f"1EMH{i:010d}",
        sparte=Sparte.STROM,
    )


def benchmark(name: str, func, iterations: int = 1000):
    """Run a benchmark and report results."""
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        times.append((end - start) * 1_000_000)  # microseconds

    mean = statistics.mean(times)
    stddev = statistics.stdev(times) if len(times) > 1 else 0
    print(f"{name}: {mean:.2f} ± {stddev:.2f} µs")


def main():
    print("BO4E-Python Benchmarks")
    print("=" * 40)

    # Single object serialization
    meter = create_meter(0)

    def serialize_json():
        return meter.model_dump_json()

    benchmark("Single meter (model_dump_json)", serialize_json)

    if HAS_ORJSON:
        def serialize_orjson():
            return orjson.dumps(meter.model_dump())

        benchmark("Single meter (orjson)", serialize_orjson)

    # Single object deserialization
    json_str = meter.model_dump_json()

    def deserialize():
        return Zaehler.model_validate_json(json_str)

    benchmark("Single meter parse", deserialize)

    # Batch operations
    print("\nBatch operations:")
    for size in [10, 100, 1000]:
        meters = [create_meter(i) for i in range(size)]

        def batch_serialize():
            return json.dumps([m.model_dump() for m in meters])

        benchmark(f"Serialize {size} meters", batch_serialize, iterations=100)


if __name__ == "__main__":
    main()
```

**Step 3: Commit**

```bash
git add benches/python_comparison/
git commit -m "perf: add Python comparison benchmarks"
```

---

## Task 3: Document Benchmark Results

**Files:**
- Create: `docs/BENCHMARKS.md`

**Step 1: Create benchmarks documentation**

```markdown
# BO4E-Rust Benchmarks

Performance comparison between BO4E-Rust and BO4E-Python.

## Test Environment

- CPU: [Your CPU]
- Memory: [Your RAM]
- Rust: 1.75.0
- Python: 3.12

## Results

### Single Object Deserialization

| Operation | Rust (simd-json) | Rust (serde_json) | Python |
|-----------|------------------|-------------------|--------|
| Meter parse | X µs | Y µs | Z µs |
| MarketLocation parse | X µs | Y µs | Z µs |

### Batch Processing (1000 objects)

| Operation | Rust (simd-json) | Python |
|-----------|------------------|--------|
| Parse 1000 meters | X ms | Y ms |
| Serialize 1000 meters | X ms | Y ms |

## Running Benchmarks

```bash
# Rust benchmarks
cargo bench -p bo4e-serde

# Python comparison
cd benches/python_comparison
pip install -r requirements.txt
python benchmark.py
```

## Conclusions

[Summary of findings]
```

**Step 2: Commit**

```bash
git add docs/BENCHMARKS.md
git commit -m "docs: add benchmark documentation"
```

---

## Verification

```bash
cargo bench -p bo4e-serde
cd benches/python_comparison && python benchmark.py
```

---

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 297 |
| Passed | 297 |
| Failed | 0 |
| Skipped | 0 |
| Benchmarks | Verified |

Files created/modified:
- `crates/bo4e-serde/benches/json_parsing.rs` (expanded benchmark suite)
- `benches/python_comparison/benchmark.py` (Python comparison script)
- `benches/python_comparison/requirements.txt` (Python dependencies)
- `docs/BENCHMARKS.md` (benchmark documentation)

Benchmark verification output (sample):
```
deserialization/meter/simd_json  time: [770.94 ns 776.31 ns 783.10 ns]
deserialization/meter/serde_json time: [555.37 ns 558.26 ns 561.61 ns]
batch_parsing/simd_json/10       time: [2.54 µs 2.69 µs 2.86 µs]
```
