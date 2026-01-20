# Phase 3: Components (COMs)

This phase implements all 64 BO4E Component types - composite data structures used within Business Objects.

## Epics

| Epic | Description | Component Count |
|------|-------------|-----------------|
| [3.1 Address & Contact](epic-3.1-address-contact-components.md) | Addresses, contacts, signatures | ~15 |
| [3.2 Pricing & Cost](epic-3.2-pricing-cost-components.md) | Prices, costs, surcharges, tariffs | ~30 |
| [3.3 Measurement & Time](epic-3.3-measurement-time-components.md) | Measurements, quantities, time periods | ~19 |

## Component Implementation Pattern

All components follow this consistent pattern:

```rust
use serde::{Deserialize, Serialize};
use crate::{Bo4eMeta, Bo4eObject};

/// English documentation for the component.
///
/// German: GermanComponentName
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentName {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Field description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<FieldType>,
}

impl Bo4eObject for ComponentName {
    fn type_name_german() -> &'static str { "GermanComponentName" }
    fn type_name_english() -> &'static str { "ComponentName" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}
```

## Key Design Decisions

1. **All fields `Option<T>`** - Matches Python, validation is application-specific
2. **`#[serde(flatten)]` for meta** - Embeds `_typ`, `_version`, `_id` at root level
3. **`#[serde(skip_serializing_if = "Option::is_none")]`** - Clean JSON output
4. **`#[serde(rename_all = "camelCase")]`** - German JSON field names via camelCase

## Completion Criteria

- [ ] All 64 components implemented in `bo4e-core/src/com/`
- [ ] Each component has:
  - `Bo4eObject` trait implementation
  - Full field definitions with Option<T>
  - Documentation
  - Unit tests
- [ ] All components exported from `bo4e_core::com`
- [ ] Golden file tests against Python fixtures

## Dependencies

- Phase 1: Foundation (Bo4eMeta, Bo4eObject trait)
- Phase 2: Enums (referenced by component fields)

## Reference

Python COM source: `../BO4E-python/src/bo4e/com/`
