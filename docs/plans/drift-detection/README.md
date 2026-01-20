# Drift Detection Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement schema drift detection between BO4E-Rust and BO4E-Python using JSON Schema comparison.

**Architecture:** Add `schemars` crate with optional `json-schema` feature. Derive `JsonSchema` on all types with German name renames. Generate and compare schemas via Python scripts.

**Tech Stack:** schemars 0.8, Python 3.11+, bo4e Python package, GitHub Actions

---

## Epic Overview

| Epic | Description | Files | Estimated Tasks |
|------|-------------|-------|-----------------|
| [Epic 1](epic-1-infrastructure.md) | Rust infrastructure setup | 3 | 5 |
| [Epic 2](epic-2-enum-schemas.md) | Add schemars to 73 enums | 73 | 8 |
| [Epic 3](epic-3-component-schemas.md) | Add schemars to 64 components | 64 | 8 |
| [Epic 4](epic-4-bo-schemas.md) | Add schemars to 35 business objects | 35 | 6 |
| [Epic 5](epic-5-scripts-ci.md) | Python scripts and CI | 6 | 7 |

## Dependency Order

```
Epic 1 (infrastructure)
    ↓
Epic 2, 3, 4 (can run in parallel after Epic 1)
    ↓
Epic 5 (scripts and CI - requires all types to have schemas)
```

## Design Reference

See [drift-detection-design.md](../2026-01-20-drift-detection-design.md) for full design rationale.

## Quick Verification

After completing all epics:

```bash
# Generate schemas and compare
pip install -r scripts/requirements.txt
python scripts/extract_python_schema.py > schemas/python_schema.json
cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json
python scripts/compare_schemas.py
```
