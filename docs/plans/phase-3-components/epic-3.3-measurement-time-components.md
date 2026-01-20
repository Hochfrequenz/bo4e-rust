# Epic 3.3: Measurement & Time Components

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement measurement, quantity, and time-related components (~19 components).

**Architecture:** Each component is a struct implementing `Bo4eObject` with optional fields.

**Tech Stack:** Rust structs, serde, chrono for datetime handling

---

## Components in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `MeasuredValue` | `Messwert` | A measured value with timestamp |
| `Quantity` | `Menge` | Quantity with unit |
| `TimePeriod` | `Zeitraum` | A time period (start/end) |
| `MeterRegister` | `Zaehlwerk` | Meter register |
| `TimeOfUseRegister` | `Zaehlzeitregister` | Time-of-use register |
| `MeterReading` | `Zaehlwerksstand` | Meter reading/state |
| `LoadProfileValue` | `Lastgangwert` | Load profile value |
| `TimeSeriesValue` | `Zeitreihenwert` | Time series value |
| `Interval` | `Intervall` | Time interval |
| `DateRange` | `Datumsbereich` | Date range |
| `SeasonalTariff` | `Saisontarif` | Seasonal tariff period |
| `BillingPeriodData` | `Abrechnungsperiodendaten` | Billing period data |
| `MeteringPointStatus` | `Messstellenstatus` | Metering point status |
| `ValidationResult` | `Validierungsergebnis` | Validation result |
| `QualityIndicator` | `Qualitaetsindikator` | Quality indicator |
| `SubstitutionValue` | `Ersatzwert` | Substituted value |
| `AggregatedValue` | `Aggregiertwert` | Aggregated value |
| `ProfileData` | `Profildaten` | Profile data |
| `LoadCurveData` | `Lastkurvendaten` | Load curve data |

---

## Task 1: Add chrono Dependency

**Files:**
- Modify: `crates/bo4e-core/Cargo.toml`

**Step 1: Update Cargo.toml**

Add to workspace Cargo.toml:
```toml
[workspace.dependencies]
chrono = { version = "0.4", features = ["serde"] }
```

Add to `crates/bo4e-core/Cargo.toml`:
```toml
[dependencies]
chrono = { workspace = true }
```

**Step 2: Commit**

```bash
git add Cargo.toml crates/bo4e-core/Cargo.toml
git commit -m "chore(core): add chrono dependency for datetime handling"
```

---

## Task 2: Create TimePeriod Component

**Files:**
- Create: `crates/bo4e-core/src/com/time_period.rs`

**Step 1: Write the implementation**

```rust
//! Time period (Zeitraum) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A time period with start and end timestamps.
///
/// German: Zeitraum
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TimePeriod;
/// use chrono::Utc;
///
/// let period = TimePeriod {
///     start: Some(Utc::now()),
///     end: None, // Open-ended
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriod {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Start of the period (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,

    /// End of the period (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
}

impl Bo4eObject for TimePeriod {
    fn type_name_german() -> &'static str { "Zeitraum" }
    fn type_name_english() -> &'static str { "TimePeriod" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

impl TimePeriod {
    /// Create a time period from start to end.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
            ..Default::default()
        }
    }

    /// Create an open-ended period starting from a given time.
    pub fn starting_from(start: DateTime<Utc>) -> Self {
        Self {
            start: Some(start),
            end: None,
            ..Default::default()
        }
    }

    /// Check if this period contains a given timestamp.
    pub fn contains(&self, timestamp: DateTime<Utc>) -> bool {
        let after_start = self.start.map_or(true, |s| timestamp >= s);
        let before_end = self.end.map_or(true, |e| timestamp < e);
        after_start && before_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_time_period_creation() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();

        let period = TimePeriod::new(start, end);
        assert_eq!(period.start, Some(start));
        assert_eq!(period.end, Some(end));
    }

    #[test]
    fn test_contains() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let period = TimePeriod::new(start, end);

        let mid = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        assert!(period.contains(mid));

        let before = Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
        assert!(!period.contains(before));
    }

    #[test]
    fn test_serialize_iso8601() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let period = TimePeriod::starting_from(start);

        let json = serde_json::to_string(&period).unwrap();
        assert!(json.contains("2024-01-01"));
    }

    #[test]
    fn test_roundtrip() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let period = TimePeriod::new(start, end);

        let json = serde_json::to_string(&period).unwrap();
        let parsed: TimePeriod = serde_json::from_str(&json).unwrap();
        assert_eq!(period, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core time_period -- --nocapture
git add crates/bo4e-core/src/com/time_period.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add TimePeriod (Zeitraum) component"
```

---

## Task 3: Create MeasuredValue Component

**Files:**
- Create: `crates/bo4e-core/src/com/measured_value.rs`

**Step 1: Write the implementation**

```rust
//! Measured value (Messwert) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::enums::{MeasuredValueStatus, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A measured value at a specific timestamp.
///
/// German: Messwert
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasuredValue {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Timestamp of measurement (Zeitpunkt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Measured value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Status/quality of the value (Status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MeasuredValueStatus>,

    /// OBIS code identifying the measurement (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,
}

impl Bo4eObject for MeasuredValue {
    fn type_name_german() -> &'static str { "Messwert" }
    fn type_name_english() -> &'static str { "MeasuredValue" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_measured_value() {
        let value = MeasuredValue {
            timestamp: Some(Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap()),
            value: Some(12345.67),
            unit: Some(Unit::KilowattHour),
            obis_code: Some("1-0:1.8.0".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("12345.67"));
        assert!(json.contains("1-0:1.8.0"));
    }

    #[test]
    fn test_roundtrip() {
        let value = MeasuredValue {
            timestamp: Some(Utc::now()),
            value: Some(999.99),
            unit: Some(Unit::CubicMeter),
            ..Default::default()
        };

        let json = serde_json::to_string(&value).unwrap();
        let parsed: MeasuredValue = serde_json::from_str(&json).unwrap();
        assert_eq!(value.value, parsed.value);
        assert_eq!(value.unit, parsed.unit);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core measured_value
git add crates/bo4e-core/src/com/measured_value.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add MeasuredValue (Messwert) component"
```

---

## Task 4: Create Quantity Component

**Files:**
- Create: `crates/bo4e-core/src/com/quantity.rs`

**Step 1: Write the implementation**

```rust
//! Quantity (Menge) component.

use serde::{Deserialize, Serialize};
use crate::enums::Unit;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A quantity with value and unit.
///
/// German: Menge
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Numeric value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
}

impl Bo4eObject for Quantity {
    fn type_name_german() -> &'static str { "Menge" }
    fn type_name_english() -> &'static str { "Quantity" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

impl Quantity {
    /// Create a quantity in kWh.
    pub fn kwh(value: f64) -> Self {
        Self {
            value: Some(value),
            unit: Some(Unit::KilowattHour),
            ..Default::default()
        }
    }

    /// Create a quantity in cubic meters.
    pub fn cubic_meters(value: f64) -> Self {
        Self {
            value: Some(value),
            unit: Some(Unit::CubicMeter),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kwh_quantity() {
        let qty = Quantity::kwh(3500.0);
        assert_eq!(qty.value, Some(3500.0));
        assert_eq!(qty.unit, Some(Unit::KilowattHour));
    }

    #[test]
    fn test_gas_quantity() {
        let qty = Quantity::cubic_meters(1500.0);
        assert_eq!(qty.unit, Some(Unit::CubicMeter));
    }

    #[test]
    fn test_roundtrip() {
        let qty = Quantity::kwh(12345.67);
        let json = serde_json::to_string(&qty).unwrap();
        let parsed: Quantity = serde_json::from_str(&json).unwrap();
        assert_eq!(qty, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core quantity
git add crates/bo4e-core/src/com/quantity.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add Quantity (Menge) component"
```

---

## Task 5: Create MeterRegister Component

**Files:**
- Create: `crates/bo4e-core/src/com/meter_register.rs`

**Step 1: Write the implementation**

```rust
//! Meter register (Zaehlwerk) component.

use serde::{Deserialize, Serialize};
use crate::enums::{EnergyDirection, RegisterType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A register on a meter that records consumption.
///
/// German: Zaehlwerk
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterRegister {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Register number/ID (Zaehlwerkskennung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_id: Option<String>,

    /// OBIS code (OBIS-Kennzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obis_code: Option<String>,

    /// Type of register (Registerart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_type: Option<RegisterType>,

    /// Direction of energy flow (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_direction: Option<EnergyDirection>,

    /// Unit of measurement (Einheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,

    /// Number of decimal places (Nachkommastellen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,

    /// Multiplier/transformer ratio (Wandlerfaktor)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformer_ratio: Option<f64>,

    /// Description (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Bo4eObject for MeterRegister {
    fn type_name_german() -> &'static str { "Zaehlwerk" }
    fn type_name_english() -> &'static str { "MeterRegister" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electricity_register() {
        let register = MeterRegister {
            obis_code: Some("1-0:1.8.0".to_string()),
            energy_direction: Some(EnergyDirection::Consumption),
            unit: Some(Unit::KilowattHour),
            description: Some("Bezug".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&register).unwrap();
        assert!(json.contains("1-0:1.8.0"));
    }

    #[test]
    fn test_roundtrip() {
        let register = MeterRegister {
            obis_code: Some("1-0:2.8.0".to_string()),
            energy_direction: Some(EnergyDirection::FeedIn),
            unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        let json = serde_json::to_string(&register).unwrap();
        let parsed: MeterRegister = serde_json::from_str(&json).unwrap();
        assert_eq!(register, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core meter_register
git add crates/bo4e-core/src/com/meter_register.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add MeterRegister (Zaehlwerk) component"
```

---

## Tasks 6-19: Remaining Measurement/Time Components

Follow the established pattern for:
- `TimeOfUseRegister`
- `MeterReading`
- `LoadProfileValue`
- `TimeSeriesValue`
- `Interval`
- `DateRange`
- `SeasonalTariff`
- `BillingPeriodData`
- `MeteringPointStatus`
- `ValidationResult`
- `QualityIndicator`
- `SubstitutionValue`
- `AggregatedValue`
- `ProfileData`

---

## Verification

```bash
cargo test -p bo4e-core com -- --nocapture
cargo clippy -p bo4e-core
```
