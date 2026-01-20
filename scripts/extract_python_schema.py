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
