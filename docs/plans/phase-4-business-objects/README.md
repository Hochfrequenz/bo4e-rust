# Phase 4: Business Objects (BOs)

This phase implements all 37 BO4E Business Object types - the top-level entities in the BO4E model.

## Epics

| Epic | Description | BO Count |
|------|-------------|----------|
| [4.1 Locations & Technical](epic-4.1-locations-technical.md) | Meters, locations, resources | ~12 |
| [4.2 Parties & Contracts](epic-4.2-parties-contracts.md) | Partners, contracts, offers | ~13 |
| [4.3 Pricing & Billing](epic-4.3-pricing-billing.md) | Tariffs, invoices, costs | ~12 |

## Business Object Implementation Pattern

All BOs follow this consistent pattern:

```rust
use serde::{Deserialize, Serialize};
use crate::com::{Address, TimePeriod};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// English documentation for the business object.
///
/// German: GermanBoName
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessObjectName {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<FieldType>,

    /// Reference to another BO (boxed to prevent cycles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_bo: Option<Box<OtherBusinessObject>>,
}
```

## Key Design Decisions

1. **All fields `Option<T>`** - Matches Python, validation is application-specific
2. **`Box<T>` for BO references** - Prevents infinite size from circular references
3. **`#[serde(flatten)]` for meta** - Embeds metadata at root level
4. **Components owned directly** - COMs can be embedded directly (not boxed)

## Completion Criteria

- [ ] All 37 BOs implemented in `bo4e-core/src/bo/`
- [ ] Each BO has:
  - `Bo4eObject` trait implementation
  - Full field definitions matching Python
  - References to other BOs via `Box<T>`
  - Documentation with examples
  - Unit tests
- [ ] All BOs exported from `bo4e_core::bo`
- [ ] Golden file tests against Python fixtures

## Dependencies

- Phase 1: Foundation (Bo4eMeta, Bo4eObject trait)
- Phase 2: Enums
- Phase 3: Components

## Reference

Python BO source: `../BO4E-python/src/bo4e/bo/`
