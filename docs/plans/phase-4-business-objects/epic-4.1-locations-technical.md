---
status: in_progress
---
# Epic 4.1: Locations & Technical Business Objects

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement location and technical resource business objects (~12 BOs).

**Architecture:** BOs with references to other BOs via `Box<T>` and embedded COMs.

**Tech Stack:** Rust structs, serde, Box<T> for circular references

---

## Business Objects in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `Meter` | `Zaehler` | A metering device |
| `MarketLocation` | `Marktlokation` | Market location (MaLo) |
| `MeteringLocation` | `Messlokation` | Metering location (MeLo) |
| `NetworkLocation` | `Netzlokation` | Network location |
| `LocationAssignment` | `Lokationszuordnung` | Assignment between locations |
| `Device` | `Geraet` | A technical device |
| `TechnicalResource` | `TechnischeRessource` | Technical resource |
| `ControllableResource` | `SteuerbareRessource` | Controllable resource |
| `EnergyAmount` | `Energiemenge` | Amount of energy |
| `LoadProfile` | `Lastgang` | Load profile data |
| `TimeSeries` | `Zeitreihe` | Time series data |
| `LocationProperties` | `Standorteigenschaften` | Location properties |

---

## Task 1: Create Meter Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/meter.rs`
- Modify: `crates/bo4e-core/src/bo/mod.rs`

**Step 1: Write the implementation**

```rust
//! Meter (Zaehler) business object.

use serde::{Deserialize, Serialize};
use crate::com::{Address, MeterRegister};
use crate::enums::{Division, MeterType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A metering device for energy measurement.
///
/// German: Zaehler
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Meter;
/// use bo4e_core::enums::{Division, MeterType};
///
/// let meter = Meter {
///     meter_number: Some("1EMH0012345678".to_string()),
///     division: Some(Division::Electricity),
///     meter_type: Some(MeterType::ModernMeasuringDevice),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Unique meter identification number (Zaehlernummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_number: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Type of meter (Zaehlertyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<MeterType>,

    /// Meter size classification (Zaehlergroesse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_size: Option<String>,

    /// Installation location address (Standort)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Address>,

    /// Registers on this meter (Zaehlwerke)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub registers: Vec<MeterRegister>,

    /// Reference to associated market location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location: Option<Box<MarketLocation>>,

    /// Reference to associated metering location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_location: Option<Box<MeteringLocation>>,
}

impl Bo4eObject for Meter {
    fn type_name_german() -> &'static str { "Zaehler" }
    fn type_name_english() -> &'static str { "Meter" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

// Forward declarations for references
use super::{MarketLocation, MeteringLocation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meter_creation() {
        let meter = Meter {
            meter_number: Some("1EMH0012345678".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(meter.meter_number, Some("1EMH0012345678".to_string()));
        assert_eq!(meter.division, Some(Division::Electricity));
    }

    #[test]
    fn test_meter_with_registers() {
        let register = MeterRegister {
            obis_code: Some("1-0:1.8.0".to_string()),
            ..Default::default()
        };

        let meter = Meter {
            meter_number: Some("TEST123".to_string()),
            registers: vec![register],
            ..Default::default()
        };

        assert_eq!(meter.registers.len(), 1);
    }

    #[test]
    fn test_serialize() {
        let meter = Meter {
            meta: Bo4eMeta::with_type("Zaehler"),
            meter_number: Some("1EMH0012345678".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&meter).unwrap();
        assert!(json.contains(r#""_typ":"Zaehler""#));
        assert!(json.contains(r#""meterNumber":"1EMH0012345678""#));
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Meter::type_name_german(), "Zaehler");
        assert_eq!(Meter::type_name_english(), "Meter");
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core meter -- --nocapture
git add crates/bo4e-core/src/bo/
git commit -m "feat(core): add Meter (Zaehler) business object"
```

---

## Task 2: Create MarketLocation Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/market_location.rs`

**Step 1: Write the implementation**

```rust
//! Market location (Marktlokation) business object.

use serde::{Deserialize, Serialize};
use crate::com::{Address, TimePeriod};
use crate::enums::{Division, EnergyDirection, CustomerType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A market location (MaLo) - the point of energy delivery/receipt.
///
/// German: Marktlokation
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketLocation {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Market location ID (Marktlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Energy direction (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_direction: Option<EnergyDirection>,

    /// Customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,

    /// Location address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Supply start date (Lieferbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_start: Option<chrono::DateTime<chrono::Utc>>,

    /// Supply end date (Lieferende)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_end: Option<chrono::DateTime<chrono::Utc>>,

    /// Annual consumption in kWh (Jahresverbrauchsprognose)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_consumption: Option<f64>,

    /// Network operator code (Netzbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_operator_code: Option<String>,

    /// Associated metering locations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metering_locations: Vec<Box<MeteringLocation>>,
}

impl Bo4eObject for MarketLocation {
    fn type_name_german() -> &'static str { "Marktlokation" }
    fn type_name_english() -> &'static str { "MarketLocation" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

use super::MeteringLocation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malo_id_format() {
        // MaLo IDs are 11 digits
        let malo = MarketLocation {
            market_location_id: Some("12345678901".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::Consumption),
            ..Default::default()
        };

        assert_eq!(malo.market_location_id.as_ref().unwrap().len(), 11);
    }

    #[test]
    fn test_serialize() {
        let malo = MarketLocation {
            meta: Bo4eMeta::with_type("Marktlokation"),
            market_location_id: Some("12345678901".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&malo).unwrap();
        assert!(json.contains(r#""marketLocationId":"12345678901""#));
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core market_location
git add crates/bo4e-core/src/bo/market_location.rs crates/bo4e-core/src/bo/mod.rs
git commit -m "feat(core): add MarketLocation (Marktlokation) business object"
```

---

## Task 3: Create MeteringLocation Business Object

**Files:**
- Create: `crates/bo4e-core/src/bo/metering_location.rs`

**Step 1: Write the implementation**

```rust
//! Metering location (Messlokation) business object.

use serde::{Deserialize, Serialize};
use crate::com::Address;
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A metering location (MeLo) - where measurement takes place.
///
/// German: Messlokation
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeteringLocation {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Metering location ID (Messlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_location_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Location address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Metering point operator code (Messstellenbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_operator_code: Option<String>,

    /// Associated meters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<Box<super::Meter>>,
}

impl Bo4eObject for MeteringLocation {
    fn type_name_german() -> &'static str { "Messlokation" }
    fn type_name_english() -> &'static str { "MeteringLocation" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_melo_creation() {
        let melo = MeteringLocation {
            metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert!(melo.metering_location_id.is_some());
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core metering_location
git add crates/bo4e-core/src/bo/metering_location.rs crates/bo4e-core/src/bo/mod.rs
git commit -m "feat(core): add MeteringLocation (Messlokation) business object"
```

---

## Tasks 4-12: Remaining Location/Technical BOs

Follow the pattern for:
- `NetworkLocation`
- `LocationAssignment`
- `Device`
- `TechnicalResource`
- `ControllableResource`
- `EnergyAmount`
- `LoadProfile`
- `TimeSeries`
- `LocationProperties`

Reference `../BO4E-python/src/bo4e/bo/` for complete field definitions.

---

## Verification

```bash
cargo test -p bo4e-core bo -- --nocapture
cargo clippy -p bo4e-core
```
