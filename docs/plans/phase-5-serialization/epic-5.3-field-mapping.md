---
status: complete
---
# Epic 5.3: Field Name Mapping

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement compile-time field name mappings for English <-> German translation.

**Architecture:** Build script generates mapping tables, custom serde attributes use mappings.

**Tech Stack:** Build scripts, proc macros (optional), serde field attributes

---

## Task 1: Create Field Mapping Module

**Files:**
- Create: `crates/bo4e-serde/src/mapping.rs`

**Step 1: Write mapping implementation**

```rust
//! Field name mappings between English (Rust) and German (BO4E JSON).

use std::collections::HashMap;
use std::sync::LazyLock;

/// Mapping from English field names to German field names.
static ENGLISH_TO_GERMAN: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Meter fields
    map.insert("meter_number", "zaehlernummer");
    map.insert("meter_type", "zaehlertyp");
    map.insert("meter_size", "zaehlergroesse");
    map.insert("registers", "zaehlwerke");

    // MarketLocation fields
    map.insert("market_location_id", "marktlokationsId");
    map.insert("energy_direction", "energierichtung");
    map.insert("annual_consumption", "jahresverbrauchsprognose");
    map.insert("supply_start", "lieferbeginn");
    map.insert("supply_end", "lieferende");
    map.insert("network_operator_code", "netzbetreiberCodenummer");

    // MeteringLocation fields
    map.insert("metering_location_id", "messlokationsId");
    map.insert("metering_operator_code", "messstellenbetreiberCodenummer");

    // BusinessPartner fields
    map.insert("partner_id", "geschaeftspartnerId");
    map.insert("commercial_register_number", "handelsregisternummer");
    map.insert("tax_id", "steuernummer");
    map.insert("vat_id", "umsatzsteuerId");

    // Contract fields
    map.insert("contract_number", "vertragsnummer");
    map.insert("contract_type", "vertragsart");
    map.insert("contract_start", "vertragsbeginn");
    map.insert("contract_end", "vertragsende");
    map.insert("signing_date", "unterzeichnungsdatum");
    map.insert("validity_period", "gueltigkeitszeitraum");
    map.insert("contract_partner", "vertragspartner");

    // Invoice fields
    map.insert("invoice_number", "rechnungsnummer");
    map.insert("invoice_type", "rechnungstyp");
    map.insert("invoice_date", "rechnungsdatum");
    map.insert("due_date", "faelligkeitsdatum");
    map.insert("billing_period", "abrechnungszeitraum");
    map.insert("net_amount", "nettobetrag");
    map.insert("tax_amount", "steuerbetrag");
    map.insert("gross_amount", "bruttobetrag");
    map.insert("positions", "rechnungspositionen");

    // Address fields
    map.insert("street", "strasse");
    map.insert("house_number", "hausnummer");
    map.insert("house_number_addition", "hausnummernzusatz");
    map.insert("postal_code", "postleitzahl");
    map.insert("city", "ort");
    map.insert("district", "ortsteil");
    map.insert("country_code", "landescode");
    map.insert("po_box", "postfach");

    // Common fields
    map.insert("division", "sparte");
    map.insert("description", "beschreibung");
    map.insert("status", "status");
    map.insert("value", "wert");
    map.insert("unit", "einheit");
    map.insert("currency", "waehrung");
    map.insert("timestamp", "zeitpunkt");
    map.insert("start", "startdatum");
    map.insert("end", "enddatum");

    // Price fields
    map.insert("price_type", "preistyp");
    map.insert("reference_unit", "bezugsgroesse");
    map.insert("base_price", "grundpreis");
    map.insert("working_price", "arbeitspreis");
    map.insert("price_tiers", "preisstaffeln");

    // Additional attribute fields
    map.insert("additional_attributes", "zusatzAttribute");

    map
});

/// Mapping from German field names to English field names.
static GERMAN_TO_ENGLISH: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    ENGLISH_TO_GERMAN
        .iter()
        .map(|(en, de)| (*de, *en))
        .collect()
});

/// Get the German field name for an English field name.
pub fn to_german(english: &str) -> Option<&'static str> {
    ENGLISH_TO_GERMAN.get(english).copied()
}

/// Get the English field name for a German field name.
pub fn to_english(german: &str) -> Option<&'static str> {
    GERMAN_TO_ENGLISH.get(german).copied()
}

/// Check if a field name is a known English field.
pub fn is_english_field(name: &str) -> bool {
    ENGLISH_TO_GERMAN.contains_key(name)
}

/// Check if a field name is a known German field.
pub fn is_german_field(name: &str) -> bool {
    GERMAN_TO_ENGLISH.contains_key(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_german() {
        assert_eq!(to_german("meter_number"), Some("zaehlernummer"));
        assert_eq!(to_german("market_location_id"), Some("marktlokationsId"));
        assert_eq!(to_german("unknown_field"), None);
    }

    #[test]
    fn test_to_english() {
        assert_eq!(to_english("zaehlernummer"), Some("meter_number"));
        assert_eq!(to_english("marktlokationsId"), Some("market_location_id"));
        assert_eq!(to_english("unknown_field"), None);
    }

    #[test]
    fn test_bidirectional() {
        // Every English -> German should have German -> English
        for (en, de) in ENGLISH_TO_GERMAN.iter() {
            assert_eq!(
                to_english(de),
                Some(*en),
                "Missing reverse mapping for {} -> {}",
                en,
                de
            );
        }
    }

    #[test]
    fn test_field_detection() {
        assert!(is_english_field("meter_number"));
        assert!(!is_english_field("zaehlernummer"));

        assert!(is_german_field("zaehlernummer"));
        assert!(!is_german_field("meter_number"));
    }
}
```

**Step 2: Export and commit**

```bash
git add crates/bo4e-serde/src/mapping.rs crates/bo4e-serde/src/lib.rs
git commit -m "feat(serde): add field name mapping module"
```

---

## Task 2: Create Golden File Test Infrastructure

**Files:**
- Create: `tests/golden/mod.rs`
- Create: `tests/golden/fixtures/` directory
- Create: `tests/golden/generate.py`

**Step 1: Create generate.py**

```python
#!/usr/bin/env python3
"""Generate JSON fixtures from BO4E-Python for golden file testing."""

import json
from pathlib import Path

from bo4e import Zaehler, Marktlokation, Geschaeftspartner
from bo4e.enum import Sparte, Energierichtung, Geschaeftspartnerrolle

FIXTURES_DIR = Path(__file__).parent / "fixtures"
FIXTURES_DIR.mkdir(exist_ok=True)


def write_fixture(name: str, obj):
    """Write a BO4E object as a JSON fixture."""
    path = FIXTURES_DIR / f"{name}.json"
    with open(path, "w") as f:
        json.dump(obj.model_dump(by_alias=True, exclude_none=True), f, indent=2)
    print(f"Wrote {path}")


def main():
    # Meter fixture
    meter = Zaehler(
        zaehlernummer="1EMH0012345678",
        sparte=Sparte.STROM,
    )
    write_fixture("meter", meter)

    # MarketLocation fixture
    malo = Marktlokation(
        marktlokationsId="12345678901",
        sparte=Sparte.STROM,
        energierichtung=Energierichtung.VERBRAUCH,
    )
    write_fixture("market_location", malo)

    # BusinessPartner fixture
    partner = Geschaeftspartner(
        name1="Stadtwerke Musterstadt GmbH",
        geschaeftspartnerrollen=[Geschaeftspartnerrolle.LIEFERANT],
    )
    write_fixture("business_partner", partner)

    print("Done generating fixtures")


if __name__ == "__main__":
    main()
```

**Step 2: Create Rust golden tests**

Create `tests/golden/mod.rs`:

```rust
//! Golden file tests comparing Rust output to Python fixtures.

use bo4e::prelude::*;
use std::fs;

fn load_fixture(name: &str) -> String {
    let path = format!("tests/golden/fixtures/{}.json", name);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to load fixture: {}", path))
}

fn normalize_json(json: &str) -> serde_json::Value {
    serde_json::from_str(json).expect("Invalid JSON")
}

#[test]
fn test_meter_golden() {
    let fixture = load_fixture("meter");
    let mut bytes = fixture.as_bytes().to_vec();

    // Parse Python JSON
    let meter: Meter = bo4e::from_json(&mut bytes).expect("Failed to parse");

    // Verify fields
    assert_eq!(meter.meter_number, Some("1EMH0012345678".to_string()));
    assert_eq!(meter.division, Some(Division::Electricity));

    // Re-serialize and compare
    let rust_json = bo4e::to_json_german(&meter).expect("Failed to serialize");
    let python_value = normalize_json(&fixture);
    let rust_value = normalize_json(&rust_json);

    assert_eq!(rust_value, python_value, "JSON mismatch for meter");
}

#[test]
fn test_market_location_golden() {
    let fixture = load_fixture("market_location");
    let mut bytes = fixture.as_bytes().to_vec();

    let malo: MarketLocation = bo4e::from_json(&mut bytes).expect("Failed to parse");

    assert_eq!(malo.market_location_id, Some("12345678901".to_string()));
    assert_eq!(malo.division, Some(Division::Electricity));
    assert_eq!(malo.energy_direction, Some(EnergyDirection::Consumption));
}

#[test]
fn test_business_partner_golden() {
    let fixture = load_fixture("business_partner");
    let mut bytes = fixture.as_bytes().to_vec();

    let partner: BusinessPartner = bo4e::from_json(&mut bytes).expect("Failed to parse");

    assert_eq!(partner.name1, Some("Stadtwerke Musterstadt GmbH".to_string()));
    assert!(partner.roles.contains(&BusinessPartnerRole::Supplier));
}
```

**Step 3: Commit**

```bash
git add tests/golden/
git commit -m "test: add golden file test infrastructure"
```

---

## Task 3: Add serde Aliases for German Deserialization

For types that need to accept German input, add aliases:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    #[serde(alias = "zaehlernummer")]
    pub meter_number: Option<String>,

    #[serde(alias = "sparte")]
    pub division: Option<Division>,
    // ...
}
```

This allows deserializing from both German and English JSON.

---

## Verification

```bash
# Generate fixtures
cd tests/golden && python generate.py

# Run golden tests
cargo test golden --

# Full test suite
cargo test --workspace
```

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 660 |
| Passed | 660 |
| Failed | 0 |
| Skipped | 3 (doc-tests with `ignore` attribute) |

### New Tests Added

**Golden File Tests (`crates/bo4e/tests/golden.rs`):**
- `test_meter_from_german_json` - Verifies Meter deserialization from German JSON
- `test_market_location_from_german_json` - Verifies MarketLocation deserialization from German JSON
- `test_business_partner_from_german_json` - Verifies BusinessPartner deserialization from German JSON
- `test_roundtrip_meter` - Tests Rust serialize/deserialize roundtrip
- `test_german_alias_deserialization` - Tests German field name aliases work
- `test_english_deserialization` - Tests English camelCase field names work
- `test_address_german_aliases` - Tests Address component with German aliases
- `test_market_location_german_aliases` - Tests MarketLocation with German aliases
- `test_business_partner_german_aliases` - Tests BusinessPartner with German aliases

**Mapping Module Tests (`crates/bo4e-serde/src/mapping.rs`):**
- `test_to_german` - Tests English to German field name translation
- `test_to_english` - Tests German to English field name translation
- `test_bidirectional` - Verifies all mappings are bidirectional
- `test_field_detection` - Tests field type detection functions

### Files Modified/Added

**New Files:**
- `crates/bo4e-serde/src/mapping.rs` - Field name mapping module
- `crates/bo4e/tests/golden.rs` - Golden file tests
- `crates/bo4e/tests/golden/fixtures/meter.json` - Meter fixture
- `crates/bo4e/tests/golden/fixtures/market_location.json` - MarketLocation fixture
- `crates/bo4e/tests/golden/fixtures/business_partner.json` - BusinessPartner fixture
- `crates/bo4e/tests/golden/generate.py` - Python script to regenerate fixtures

**Modified Files:**
- `crates/bo4e-serde/src/lib.rs` - Export mapping module
- `crates/bo4e-serde/Cargo.toml` - Add once_cell dependency
- `crates/bo4e-core/src/bo/meter.rs` - Add German serde aliases
- `crates/bo4e-core/src/bo/market_location.rs` - Add German serde aliases
- `crates/bo4e-core/src/bo/business_partner.rs` - Add German serde aliases
- `crates/bo4e-core/src/com/address.rs` - Add German serde aliases
