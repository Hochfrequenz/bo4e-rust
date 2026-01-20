# Phase 2: Enumerations

This phase ports all 97 BO4E enumerations from Python to Rust.

## Epics

| Epic | Description | Enum Count |
|------|-------------|------------|
| [2.1 Energy & Technical](epic-2.1-energy-technical-enums.md) | Energy types, directions, measurements | ~30 |
| [2.2 Business & Contract](epic-2.2-business-contract-enums.md) | Roles, contracts, organizations | ~35 |
| [2.3 Pricing & Regional](epic-2.3-pricing-regional-enums.md) | Pricing, tariffs, regions | ~32 |

## Enum Implementation Pattern

All enums follow this consistent pattern:

```rust
use serde::{Deserialize, Serialize};

/// English documentation describing the enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnumName {
    /// English doc for variant
    #[serde(rename = "GERMAN_VALUE")]
    EnglishVariant,
}
```

## Completion Criteria

- [ ] All 97 enums implemented in `bo4e-core/src/enums/`
- [ ] Each enum has:
  - English variant names
  - German serde aliases
  - Documentation
  - Unit tests for serialize/deserialize
- [ ] All enums exported from `bo4e_core::enums`
- [ ] Golden file tests against Python-generated JSON

## Reference

Python enum source: `../BO4E-python/src/bo4e/enum/`
