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
