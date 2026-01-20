---
status: complete
---
# Epic 5: Python Scripts and CI

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create Python scripts for schema extraction and comparison, plus CI workflow for automated drift detection.

**Architecture:** Python script extracts schemas from BO4E-Python using Pydantic introspection, comparator script diffs the schemas and generates reports, GitHub Actions workflow runs weekly and on PRs.

**Tech Stack:** Python 3.11+, bo4e package, pydantic, GitHub Actions

---

## Task 1: Create Python Requirements File

**Files:**
- Create: `scripts/requirements.txt`

**Step 1: Create requirements file**

Create `scripts/requirements.txt`:

```
bo4e>=202401.0.0
```

**Step 2: Commit**

```bash
git add scripts/requirements.txt
git commit -m "feat(drift): add Python requirements for drift detection"
```

---

## Task 2: Create Python Schema Extractor

**Files:**
- Create: `scripts/extract_python_schema.py`

**Step 1: Create the extraction script**

Create `scripts/extract_python_schema.py`:

```python
#!/usr/bin/env python3
"""Extract JSON schemas from BO4E-Python for drift detection.

Usage:
    python scripts/extract_python_schema.py > schemas/python_schema.json
"""

import json
import inspect
from enum import Enum
from pydantic import BaseModel

from bo4e import bo, com
from bo4e import enum as bo4e_enum


def get_all_models(module):
    """Get all Pydantic models from a module."""
    models = {}
    for name, obj in inspect.getmembers(module):
        if (
            inspect.isclass(obj)
            and issubclass(obj, BaseModel)
            and obj is not BaseModel
            and obj.__module__.startswith("bo4e")
        ):
            models[name] = obj
    return models


def get_all_enums(module):
    """Get all Enum classes from a module."""
    enums = {}
    for name, obj in inspect.getmembers(module):
        if (
            inspect.isclass(obj)
            and issubclass(obj, Enum)
            and obj is not Enum
            and obj.__module__.startswith("bo4e")
        ):
            enums[name] = obj
    return enums


def extract_schemas():
    """Extract schemas from all BO4E types."""
    schemas = {"bo": {}, "com": {}, "enum": {}}

    # Business Objects
    for name, cls in get_all_models(bo).items():
        try:
            schemas["bo"][name] = cls.model_json_schema()
        except Exception as e:
            print(f"Warning: Could not extract schema for BO {name}: {e}", file=__import__('sys').stderr)

    # Components
    for name, cls in get_all_models(com).items():
        try:
            schemas["com"][name] = cls.model_json_schema()
        except Exception as e:
            print(f"Warning: Could not extract schema for COM {name}: {e}", file=__import__('sys').stderr)

    # Enums (as list of variants)
    for name, cls in get_all_enums(bo4e_enum).items():
        try:
            schemas["enum"][name] = [e.value for e in cls]
        except Exception as e:
            print(f"Warning: Could not extract variants for enum {name}: {e}", file=__import__('sys').stderr)

    return schemas


def main():
    schemas = extract_schemas()
    print(json.dumps(schemas, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
```

**Step 2: Make executable**

```bash
chmod +x scripts/extract_python_schema.py
```

**Step 3: Test locally (if Python environment available)**

```bash
pip install -r scripts/requirements.txt
python scripts/extract_python_schema.py | head -50
```

Expected: JSON output with BO4E types

**Step 4: Commit**

```bash
git add scripts/extract_python_schema.py
git commit -m "feat(drift): add Python schema extraction script"
```

---

## Task 3: Create Schema Comparator Script

**Files:**
- Create: `scripts/compare_schemas.py`

**Step 1: Create the comparison script**

Create `scripts/compare_schemas.py`:

```python
#!/usr/bin/env python3
"""Compare Python and Rust schemas to detect drift.

Usage:
    python scripts/compare_schemas.py

Expects:
    schemas/python_schema.json
    schemas/rust_schema.json
"""

import json
import sys
from pathlib import Path
from dataclasses import dataclass, field


@dataclass
class DriftReport:
    """Report of schema differences."""
    missing_in_rust: dict = field(default_factory=dict)
    missing_in_python: dict = field(default_factory=dict)
    type_mismatches: list = field(default_factory=list)

    @property
    def has_drift(self) -> bool:
        return bool(self.missing_in_rust or self.missing_in_python or self.type_mismatches)


def compare_enums(python_enums: dict, rust_enums: dict, report: DriftReport):
    """Compare enum variants."""
    all_names = set(python_enums.keys()) | set(rust_enums.keys())

    for name in sorted(all_names):
        py_variants = set(python_enums.get(name, []))
        rs_variants = set(rust_enums.get(name, []))

        if name not in rust_enums:
            report.missing_in_rust[f"enum/{name}"] = "entire type"
        elif name not in python_enums:
            report.missing_in_python[f"enum/{name}"] = "entire type"
        else:
            missing = py_variants - rs_variants
            extra = rs_variants - py_variants
            if missing:
                report.missing_in_rust[f"enum/{name}"] = f"variants: {sorted(missing)}"
            if extra:
                report.missing_in_python[f"enum/{name}"] = f"variants: {sorted(extra)}"


def compare_models(category: str, python_models: dict, rust_models: dict, report: DriftReport):
    """Compare model fields and types."""
    all_names = set(python_models.keys()) | set(rust_models.keys())

    for name in sorted(all_names):
        if name not in rust_models:
            report.missing_in_rust[f"{category}/{name}"] = "entire type"
            continue
        if name not in python_models:
            report.missing_in_python[f"{category}/{name}"] = "entire type"
            continue

        py_props = python_models[name].get("properties", {})
        rs_props = rust_models[name].get("properties", {})

        py_fields = set(py_props.keys())
        rs_fields = set(rs_props.keys())

        # Ignore metadata fields that may differ
        ignore_fields = {"_typ", "_version", "_id", "zusatzAttribute"}
        py_fields -= ignore_fields
        rs_fields -= ignore_fields

        missing = py_fields - rs_fields
        extra = rs_fields - py_fields

        if missing:
            report.missing_in_rust[f"{category}/{name}"] = f"fields: {sorted(missing)}"
        if extra:
            report.missing_in_python[f"{category}/{name}"] = f"fields: {sorted(extra)}"

        # Check type mismatches for common fields
        for fld in sorted(py_fields & rs_fields):
            py_type = py_props[fld].get("type")
            rs_type = rs_props[fld].get("type")
            if py_type and rs_type and py_type != rs_type:
                report.type_mismatches.append(
                    f"{category}/{name}.{fld}: Python={py_type}, Rust={rs_type}"
                )


def generate_report(report: DriftReport) -> str:
    """Generate human-readable report."""
    lines = ["# BO4E Drift Report", ""]

    if not report.has_drift:
        lines.append("No drift detected - schemas are in sync!")
        return "\n".join(lines)

    lines.append("Drift detected between Python and Rust schemas\n")

    if report.missing_in_rust:
        lines.append("## Missing in Rust (exists in Python)\n")
        for key in sorted(report.missing_in_rust.keys()):
            lines.append(f"- **{key}**: {report.missing_in_rust[key]}")
        lines.append("")

    if report.missing_in_python:
        lines.append("## Missing in Python (exists in Rust)\n")
        for key in sorted(report.missing_in_python.keys()):
            lines.append(f"- **{key}**: {report.missing_in_python[key]}")
        lines.append("")

    if report.type_mismatches:
        lines.append("## Type Mismatches\n")
        for mismatch in sorted(report.type_mismatches):
            lines.append(f"- {mismatch}")
        lines.append("")

    # Summary
    lines.append("## Summary\n")
    lines.append(f"- Missing in Rust: {len(report.missing_in_rust)}")
    lines.append(f"- Missing in Python: {len(report.missing_in_python)}")
    lines.append(f"- Type mismatches: {len(report.type_mismatches)}")

    return "\n".join(lines)


def main():
    schemas_dir = Path(__file__).parent.parent / "schemas"

    python_path = schemas_dir / "python_schema.json"
    rust_path = schemas_dir / "rust_schema.json"

    if not python_path.exists():
        print(f"Error: {python_path} not found", file=sys.stderr)
        print("Run: python scripts/extract_python_schema.py > schemas/python_schema.json", file=sys.stderr)
        sys.exit(1)

    if not rust_path.exists():
        print(f"Error: {rust_path} not found", file=sys.stderr)
        print("Run: cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json", file=sys.stderr)
        sys.exit(1)

    with open(python_path) as f:
        python = json.load(f)
    with open(rust_path) as f:
        rust = json.load(f)

    report = DriftReport()

    # Compare each category
    for category in ["bo", "com"]:
        compare_models(category, python.get(category, {}), rust.get(category, {}), report)

    compare_enums(python.get("enum", {}), rust.get("enum", {}), report)

    print(generate_report(report))

    # Exit 0 (report only, don't fail CI)
    sys.exit(0)


if __name__ == "__main__":
    main()
```

**Step 2: Make executable**

```bash
chmod +x scripts/compare_schemas.py
```

**Step 3: Commit**

```bash
git add scripts/compare_schemas.py
git commit -m "feat(drift): add schema comparison script"
```

---

## Task 4: Create CI Workflow

**Files:**
- Create: `.github/workflows/drift.yml`

**Step 1: Create the workflow file**

Create `.github/workflows/drift.yml`:

```yaml
name: Drift Detection

on:
  schedule:
    - cron: '0 6 * * 1'  # Weekly on Monday at 6am UTC
  workflow_dispatch:  # Manual trigger
  pull_request:
    paths:
      - 'crates/bo4e-core/src/**'
      - 'scripts/**'

jobs:
  drift:
    name: Check Schema Drift
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable

      - uses: Swatinem/rust-cache@v2

      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
          cache: 'pip'
          cache-dependency-path: scripts/requirements.txt

      - name: Install Python dependencies
        run: pip install -r scripts/requirements.txt

      - name: Create schemas directory
        run: mkdir -p schemas

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
          retention-days: 30

      - name: Post report to PR (if applicable)
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const report = fs.readFileSync('drift_report.md', 'utf8');

            // Only comment if there's actual drift
            if (!report.includes('No drift detected')) {
              github.rest.issues.createComment({
                issue_number: context.issue.number,
                owner: context.repo.owner,
                repo: context.repo.repo,
                body: '## Drift Detection Results\n\n' + report
              });
            }
```

**Step 2: Commit**

```bash
git add .github/workflows/drift.yml
git commit -m "feat(drift): add CI workflow for drift detection"
```

---

## Task 5: Update CLAUDE.md with Drift Detection Instructions

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Add drift detection section to CLAUDE.md**

Add after the "Releasing" section:

```markdown
## Drift Detection

Check for schema drift between Rust and Python implementations:

```bash
# Install Python dependencies
pip install -r scripts/requirements.txt

# Generate schemas
python scripts/extract_python_schema.py > schemas/python_schema.json
cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json

# Compare and view report
python scripts/compare_schemas.py
```

Drift detection runs automatically:
- Weekly (Monday 6am UTC)
- On PRs that modify `crates/bo4e-core/src/**`

Reports are uploaded as artifacts in GitHub Actions.
```

**Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add drift detection instructions to CLAUDE.md"
```

---

## Task 6: Test Full Drift Detection Pipeline

**Step 1: Install Python dependencies**

```bash
pip install -r scripts/requirements.txt
```

**Step 2: Generate Python schema**

```bash
python scripts/extract_python_schema.py > schemas/python_schema.json
```

Verify: `ls -la schemas/python_schema.json` shows file exists

**Step 3: Generate Rust schema**

```bash
cargo run --bin generate_schema --features json-schema > schemas/rust_schema.json
```

Verify: `ls -la schemas/rust_schema.json` shows file exists

**Step 4: Run comparison**

```bash
python scripts/compare_schemas.py
```

Expected: Report showing drift (since this is first run, there will be differences)

**Step 5: Verify schemas are gitignored**

```bash
git status
```

Expected: `schemas/*.json` files not showing as untracked

---

## Task 7: Final Verification and Push

**Step 1: Run full CI checks**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p bo4e-core --features json-schema
```

Expected: All pass

**Step 2: Push all changes**

```bash
git push origin main
```

**Step 3: Manually trigger CI workflow**

Go to GitHub Actions and manually trigger the "Drift Detection" workflow to verify it works.

---

## Epic 5 Completion Checklist

- [x] `scripts/requirements.txt` created
- [x] `scripts/extract_python_schema.py` created and executable
- [x] `scripts/compare_schemas.py` created and executable
- [x] `.github/workflows/drift.yml` created
- [x] `CLAUDE.md` updated with drift detection instructions
- [x] Full pipeline tested locally
- [ ] CI workflow runs successfully

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 102 |
| Passed | 102 |
| Failed | 0 |
| Skipped | 2 |
| Clippy | Pass |

Files created/modified:
- scripts/requirements.txt (new)
- scripts/extract_python_schema.py (new)
- scripts/compare_schemas.py (new)
- .github/workflows/drift.yml (new)
- CLAUDE.md (updated)
- .gitignore (updated)
