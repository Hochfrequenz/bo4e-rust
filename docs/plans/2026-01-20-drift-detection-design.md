# Drift Detection Design Document

**Date:** 2026-01-20
**Status:** Approved

## Overview

Drift detection system to identify schema differences between BO4E-Rust and BO4E-Python. Detects missing types, missing fields, missing enum variants, and type mismatches.

### Goals

1. Full schema comparison (types, fields, enum variants, type mismatches)
2. Available both locally and in CI
3. Report-only output (warns but doesn't fail CI)

### Approach

Dual JSON Schema comparison:
- Python generates JSON schemas from Pydantic models
- Rust generates JSON schemas using `schemars` crate
- Comparator script diffs the two and generates a report

## Architecture

```
Python (bo4e) ──► extract_python_schema.py ──► python_schema.json ─┐
                                                                    ├──► compare_schemas.py ──► Report
Rust (bo4e-core) ──► generate_schema binary ──► rust_schema.json ──┘
```

### File Structure

```
scripts/
├── extract_python_schema.py  # Extract schemas from BO4E-Python
├── compare_schemas.py        # Compare and generate diff report
└── requirements.txt          # Python dependencies (bo4e)
schemas/
├── python_schema.json        # Generated, gitignored
└── rust_schema.json          # Generated, gitignored
crates/bo4e-core/
└── src/bin/generate_schema.rs  # Rust schema generator binary
.github/workflows/
└── drift.yml                 # CI workflow
```

## Rust Side - schemars Integration

### Cargo.toml Changes

```toml
[dependencies]
schemars = { version = "0.8", optional = true }

[features]
default = []
json-schema = ["schemars"]
```

### Type Annotation Pattern

All BOs, COMs, and Enums follow this pattern:

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

    #[serde(skip_serializing_if = "Option::is_none", alias = "sparte")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    // ... all fields follow same pattern
}
```

Key points:
- `cfg_attr` ensures attributes only apply with `json-schema` feature
- `schemars(rename = "...")` uses German names to match Python
- Keeps serialization behavior unchanged (aliases still work)

### Schema Generator Binary

`crates/bo4e-core/src/bin/generate_schema.rs`:

```rust
use bo4e_core::{bo::*, com::*, enums::*};
use schemars::schema_for;
use serde_json::json;

fn main() {
    let schemas = json!({
        "bo": {
            "Zaehler": schema_for!(Meter),
            "Marktlokation": schema_for!(MarketLocation),
            // ... all BOs
        },
        "com": {
            "Adresse": schema_for!(Address),
            // ... all COMs
        },
        "enum": {
            "Sparte": schema_for!(Division),
            // ... all Enums
        }
    });

    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
}
```

Run with:
```bash
cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json
```

## Python Side - Schema Extraction

### scripts/extract_python_schema.py

```python
#!/usr/bin/env python3
"""Extract JSON schemas from BO4E-Python for drift detection."""

import json
import inspect
from pydantic import BaseModel
from enum import Enum

from bo4e import bo, com
from bo4e import enum as bo4e_enum

def get_all_models(module):
    """Get all Pydantic models from a module."""
    models = {}
    for name, obj in inspect.getmembers(module):
        if inspect.isclass(obj) and issubclass(obj, BaseModel) and obj is not BaseModel:
            models[name] = obj
    return models

def get_all_enums(module):
    """Get all Enum classes from a module."""
    enums = {}
    for name, obj in inspect.getmembers(module):
        if inspect.isclass(obj) and issubclass(obj, Enum) and obj is not Enum:
            enums[name] = obj
    return enums

def extract_schemas():
    schemas = {"bo": {}, "com": {}, "enum": {}}

    # Business Objects
    for name, cls in get_all_models(bo).items():
        schemas["bo"][name] = cls.model_json_schema()

    # Components
    for name, cls in get_all_models(com).items():
        schemas["com"][name] = cls.model_json_schema()

    # Enums (as list of variants)
    for name, cls in get_all_enums(bo4e_enum).items():
        schemas["enum"][name] = [e.value for e in cls]

    return schemas

if __name__ == "__main__":
    print(json.dumps(extract_schemas(), indent=2))
```

### scripts/requirements.txt

```
bo4e>=202401.0.0
```

## Schema Comparator

### scripts/compare_schemas.py

```python
#!/usr/bin/env python3
"""Compare Python and Rust schemas to detect drift."""

import json
import sys
from pathlib import Path
from dataclasses import dataclass

@dataclass
class DriftReport:
    missing_in_rust: dict
    missing_in_python: dict
    type_mismatches: list

    @property
    def has_drift(self):
        return bool(self.missing_in_rust or self.missing_in_python or self.type_mismatches)

def compare_enums(python_enums: dict, rust_enums: dict) -> tuple[dict, dict]:
    """Compare enum variants."""
    missing_in_rust = {}
    missing_in_python = {}

    all_names = set(python_enums.keys()) | set(rust_enums.keys())
    for name in all_names:
        py_variants = set(python_enums.get(name, []))
        rs_variants = set(rust_enums.get(name, []))

        if name not in rust_enums:
            missing_in_rust[name] = list(py_variants)
        elif name not in python_enums:
            missing_in_python[name] = list(rs_variants)
        else:
            missing = py_variants - rs_variants
            extra = rs_variants - py_variants
            if missing:
                missing_in_rust[f"{name} variants"] = list(missing)
            if extra:
                missing_in_python[f"{name} variants"] = list(extra)

    return missing_in_rust, missing_in_python

def compare_models(python_models: dict, rust_models: dict) -> tuple[dict, dict, list]:
    """Compare model fields and types."""
    missing_in_rust = {}
    missing_in_python = {}
    type_mismatches = []

    all_names = set(python_models.keys()) | set(rust_models.keys())
    for name in all_names:
        if name not in rust_models:
            missing_in_rust[name] = "entire type"
            continue
        if name not in python_models:
            missing_in_python[name] = "entire type"
            continue

        py_props = python_models[name].get("properties", {})
        rs_props = rust_models[name].get("properties", {})

        py_fields = set(py_props.keys())
        rs_fields = set(rs_props.keys())

        missing = py_fields - rs_fields
        extra = rs_fields - py_fields

        if missing:
            missing_in_rust[f"{name} fields"] = list(missing)
        if extra:
            missing_in_python[f"{name} fields"] = list(extra)

        # Check type mismatches for common fields
        for field in py_fields & rs_fields:
            py_type = py_props[field].get("type")
            rs_type = rs_props[field].get("type")
            if py_type and rs_type and py_type != rs_type:
                type_mismatches.append(f"{name}.{field}: Python={py_type}, Rust={rs_type}")

    return missing_in_rust, missing_in_python, type_mismatches

def generate_report(report: DriftReport) -> str:
    """Generate human-readable report."""
    lines = ["# BO4E Drift Report", ""]

    if not report.has_drift:
        lines.append("No drift detected - schemas are in sync!")
        return "\n".join(lines)

    lines.append("Drift detected between Python and Rust schemas\n")

    if report.missing_in_rust:
        lines.append("## Missing in Rust (exists in Python)")
        for key, value in report.missing_in_rust.items():
            lines.append(f"- **{key}**: {value}")
        lines.append("")

    if report.missing_in_python:
        lines.append("## Missing in Python (exists in Rust)")
        for key, value in report.missing_in_python.items():
            lines.append(f"- **{key}**: {value}")
        lines.append("")

    if report.type_mismatches:
        lines.append("## Type Mismatches")
        for mismatch in report.type_mismatches:
            lines.append(f"- {mismatch}")
        lines.append("")

    return "\n".join(lines)

def main():
    schemas_dir = Path(__file__).parent.parent / "schemas"

    with open(schemas_dir / "python_schema.json") as f:
        python = json.load(f)
    with open(schemas_dir / "rust_schema.json") as f:
        rust = json.load(f)

    missing_rust, missing_py = {}, {}
    type_mismatches = []

    # Compare each category
    for category in ["bo", "com"]:
        mr, mp, tm = compare_models(python.get(category, {}), rust.get(category, {}))
        missing_rust.update(mr)
        missing_py.update(mp)
        type_mismatches.extend(tm)

    er, ep = compare_enums(python.get("enum", {}), rust.get("enum", {}))
    missing_rust.update(er)
    missing_py.update(ep)

    report = DriftReport(missing_rust, missing_py, type_mismatches)
    print(generate_report(report))

    sys.exit(0)  # Report only, don't fail

if __name__ == "__main__":
    main()
```

## CI Workflow

### .github/workflows/drift.yml

```yaml
name: Drift Detection

on:
  schedule:
    - cron: '0 6 * * 1'  # Weekly on Monday at 6am UTC
  workflow_dispatch:  # Manual trigger
  pull_request:
    paths:
      - 'crates/bo4e-core/src/**'

jobs:
  drift:
    name: Check Schema Drift
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable

      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'

      - name: Install Python dependencies
        run: pip install -r scripts/requirements.txt

      - name: Generate Python schema
        run: python scripts/extract_python_schema.py > schemas/python_schema.json

      - name: Generate Rust schema
        run: cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json

      - name: Compare schemas
        run: python scripts/compare_schemas.py | tee drift_report.md

      - name: Upload drift report
        uses: actions/upload-artifact@v4
        with:
          name: drift-report
          path: drift_report.md
```

## Local Workflow

Add to CLAUDE.md:

```bash
# Check for schema drift with BO4E-Python
pip install -r scripts/requirements.txt
python scripts/extract_python_schema.py > schemas/python_schema.json
cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json
python scripts/compare_schemas.py
```

## Implementation Tasks

1. Add `schemars` dependency to `bo4e-core/Cargo.toml` with `json-schema` feature
2. Add `#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]` to all types
3. Add `#[cfg_attr(feature = "json-schema", schemars(rename = "..."))]` for German names on all types and fields
4. Create `crates/bo4e-core/src/bin/generate_schema.rs`
5. Create `scripts/extract_python_schema.py`
6. Create `scripts/compare_schemas.py`
7. Create `scripts/requirements.txt`
8. Create `schemas/` directory with `.gitignore` for generated files
9. Create `.github/workflows/drift.yml`
10. Update CLAUDE.md with local workflow instructions

## References

- [schemars crate](https://docs.rs/schemars)
- [Pydantic JSON Schema](https://docs.pydantic.dev/latest/concepts/json_schema/)
- [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python)
