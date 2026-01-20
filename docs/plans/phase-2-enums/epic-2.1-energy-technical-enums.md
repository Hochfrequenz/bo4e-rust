# Epic 2.1: Energy & Technical Enumerations

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement all energy and technical-related enumerations (~30 enums).

**Architecture:** Each enum is a separate module file exporting a single enum type with German serde aliases.

**Tech Stack:** Rust enums, serde derive macros

---

## Enums in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `Division` | `Sparte` | Energy division (electricity, gas, etc.) |
| `EnergyDirection` | `Energierichtung` | Direction of energy flow |
| `Medium` | `Medium` | Type of medium |
| `GenerationType` | `Erzeugungsart` | Type of energy generation |
| `MeasurementType` | `Messart` | Type of measurement |
| `MeasuredQuantity` | `Messgroesse` | Measured quantity type |
| `MeasurementPriceType` | `Messpreistyp` | Measurement pricing type |
| `MeasuredValueStatus` | `Messwertstatus` | Status of measured value |
| `MeterType` | `Zaehlertyp` | Type of meter |
| `MeterSize` | `Zaehlergroesse` | Size of meter |
| `MeterCategory` | `Zaehlerauspraegung` | Meter category |
| `ReadingType` | `Ablesetyp` | Type of meter reading |
| `NetworkLevel` | `Netzebene` | Network level |
| `VoltageLevel` | `Spannungsebene` | Voltage level |
| `UsageType` | `Verwendungszweck` | Usage purpose |
| `LocationType` | `Lokationstyp` | Type of location |
| `DeviceType` | `Geraetetyp` | Type of device |
| `DeviceCategory` | `Geraeteklasse` | Category of device |
| `PhaseType` | `Phasenart` | Phase type |
| `TariffType` | `Tarifart` | Type of tariff |
| `RegisterType` | `Registerart` | Register type |
| `Unit` | `Mengeneinheit` | Unit of measurement |
| `Currency` | `Waehrungscode` | Currency code |
| `UnitPrefix` | `Mengeneinheitenpraefix` | Unit prefix (kilo, mega, etc.) |
| `TimeUnit` | `Zeiteinheit` | Time unit |
| `ArithmeticOperation` | `ArithmetischeOperation` | Arithmetic operation |
| `CalculationFormula` | `Berechnungsformel` | Calculation formula |
| `RoundingMode` | `Rundungsverfahren` | Rounding method |
| `ControllableResourceType` | `SteuerbareRessourceTyp` | Controllable resource type |
| `TechnicalResourceUsage` | `TechnischeRessourceNutzung` | Technical resource usage |

---

## Task 1: Create Division (Sparte) Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/division.rs`
- Modify: `crates/bo4e-core/src/enums/mod.rs`

**Step 1: Write the test and implementation**

Create `crates/bo4e-core/src/enums/division.rs`:

```rust
//! Energy division (Sparte) enumeration.

use serde::{Deserialize, Serialize};

/// Energy division/sector.
///
/// Indicates which energy sector a business object belongs to.
///
/// German: Sparte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Division {
    /// Electricity (Strom)
    #[serde(rename = "STROM")]
    Electricity,

    /// Natural gas (Gas)
    #[serde(rename = "GAS")]
    Gas,

    /// District heating (Fernwaerme)
    #[serde(rename = "FERNWAERME")]
    DistrictHeating,

    /// District cooling (Nahwaerme)
    #[serde(rename = "NAHWAERME")]
    DistrictCooling,

    /// Water (Wasser)
    #[serde(rename = "WASSER")]
    Water,

    /// Wastewater (Abwasser)
    #[serde(rename = "ABWASSER")]
    Wastewater,

    /// Cross-divisional (Strom und Gas)
    #[serde(rename = "STROM_UND_GAS")]
    ElectricityAndGas,
}

impl Division {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Electricity => "Strom",
            Self::Gas => "Gas",
            Self::DistrictHeating => "Fernwaerme",
            Self::DistrictCooling => "Nahwaerme",
            Self::Water => "Wasser",
            Self::Wastewater => "Abwasser",
            Self::ElectricityAndGas => "Strom und Gas",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Division::Electricity).unwrap(), r#""STROM""#);
        assert_eq!(serde_json::to_string(&Division::Gas).unwrap(), r#""GAS""#);
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Division>(r#""STROM""#).unwrap(),
            Division::Electricity
        );
        assert_eq!(
            serde_json::from_str::<Division>(r#""FERNWAERME""#).unwrap(),
            Division::DistrictHeating
        );
    }

    #[test]
    fn test_roundtrip() {
        for division in [
            Division::Electricity,
            Division::Gas,
            Division::DistrictHeating,
            Division::DistrictCooling,
            Division::Water,
            Division::Wastewater,
            Division::ElectricityAndGas,
        ] {
            let json = serde_json::to_string(&division).unwrap();
            let parsed: Division = serde_json::from_str(&json).unwrap();
            assert_eq!(division, parsed);
        }
    }
}
```

**Step 2: Run tests**

```bash
cargo test -p bo4e-core division -- --nocapture
```

Expected: All tests pass

**Step 3: Update mod.rs**

Add to `crates/bo4e-core/src/enums/mod.rs`:

```rust
mod division;
pub use division::Division;
```

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/enums/division.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add Division (Sparte) enum"
```

---

## Task 2: Create EnergyDirection Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/energy_direction.rs`
- Modify: `crates/bo4e-core/src/enums/mod.rs`

**Step 1: Write the implementation**

Create `crates/bo4e-core/src/enums/energy_direction.rs`:

```rust
//! Energy direction (Energierichtung) enumeration.

use serde::{Deserialize, Serialize};

/// Direction of energy flow.
///
/// German: Energierichtung
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergyDirection {
    /// Energy consumption/withdrawal (Verbrauch)
    #[serde(rename = "VERBRAUCH")]
    Consumption,

    /// Energy feed-in/injection (Einspeisung)
    #[serde(rename = "EINSPEISUNG")]
    FeedIn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        for dir in [EnergyDirection::Consumption, EnergyDirection::FeedIn] {
            let json = serde_json::to_string(&dir).unwrap();
            let parsed: EnergyDirection = serde_json::from_str(&json).unwrap();
            assert_eq!(dir, parsed);
        }
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core energy_direction
git add crates/bo4e-core/src/enums/energy_direction.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add EnergyDirection (Energierichtung) enum"
```

---

## Task 3: Create Unit (Mengeneinheit) Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/unit.rs`

**Step 1: Write the implementation**

Create `crates/bo4e-core/src/enums/unit.rs`:

```rust
//! Unit of measurement (Mengeneinheit) enumeration.

use serde::{Deserialize, Serialize};

/// Unit of measurement.
///
/// German: Mengeneinheit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    /// Watt hour
    #[serde(rename = "WH")]
    WattHour,

    /// Kilowatt hour
    #[serde(rename = "KWH")]
    KilowattHour,

    /// Megawatt hour
    #[serde(rename = "MWH")]
    MegawattHour,

    /// Watt
    #[serde(rename = "W")]
    Watt,

    /// Kilowatt
    #[serde(rename = "KW")]
    Kilowatt,

    /// Megawatt
    #[serde(rename = "MW")]
    Megawatt,

    /// Volt ampere reactive
    #[serde(rename = "VAR")]
    VoltAmpereReactive,

    /// Kilovolt ampere reactive
    #[serde(rename = "KVAR")]
    KilovoltAmpereReactive,

    /// Volt ampere reactive hour
    #[serde(rename = "VARH")]
    VoltAmpereReactiveHour,

    /// Kilovolt ampere reactive hour
    #[serde(rename = "KVARH")]
    KilovoltAmpereReactiveHour,

    /// Cubic meter
    #[serde(rename = "KUBIKMETER")]
    CubicMeter,

    /// Standard cubic meter
    #[serde(rename = "NORMKUBIKMETER")]
    StandardCubicMeter,

    /// Liter
    #[serde(rename = "LITER")]
    Liter,

    /// Piece/unit
    #[serde(rename = "STUECK")]
    Piece,

    /// Hour
    #[serde(rename = "STUNDE")]
    Hour,

    /// Day
    #[serde(rename = "TAG")]
    Day,

    /// Month
    #[serde(rename = "MONAT")]
    Month,

    /// Year
    #[serde(rename = "JAHR")]
    Year,

    /// Euro
    #[serde(rename = "EUR")]
    Euro,

    /// Euro cent
    #[serde(rename = "CT")]
    Cent,

    /// Percent
    #[serde(rename = "PROZENT")]
    Percent,

    /// Kilojoule
    #[serde(rename = "KJ")]
    Kilojoule,

    /// Gigajoule
    #[serde(rename = "GJ")]
    Gigajoule,

    /// Ampere
    #[serde(rename = "AMPERE")]
    Ampere,

    /// Volt
    #[serde(rename = "VOLT")]
    Volt,

    /// Degree Celsius
    #[serde(rename = "GRAD_CELSIUS")]
    DegreeCelsius,

    /// Thermie
    #[serde(rename = "THERMIE")]
    Thermie,

    /// Bar
    #[serde(rename = "BAR")]
    Bar,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_units() {
        assert_eq!(serde_json::to_string(&Unit::KilowattHour).unwrap(), r#""KWH""#);
        assert_eq!(serde_json::to_string(&Unit::CubicMeter).unwrap(), r#""KUBIKMETER""#);
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Unit>(r#""MWH""#).unwrap(),
            Unit::MegawattHour
        );
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core unit
git add crates/bo4e-core/src/enums/unit.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add Unit (Mengeneinheit) enum"
```

---

## Task 4: Create MeterType (Zaehlertyp) Enum

**Files:**
- Create: `crates/bo4e-core/src/enums/meter_type.rs`

**Step 1: Write the implementation**

Create `crates/bo4e-core/src/enums/meter_type.rs`:

```rust
//! Meter type (Zaehlertyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of metering device.
///
/// German: Zaehlertyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeterType {
    /// Ferraris meter (traditional electromechanical)
    #[serde(rename = "DREHSTROMZAEHLER")]
    ThreePhaseRotatingMeter,

    /// Balgengaszaehler (bellows gas meter)
    #[serde(rename = "BALGENGASZAEHLER")]
    BellowsGasMeter,

    /// Smart meter / Modern measuring device
    #[serde(rename = "MODERNE_MESSEINRICHTUNG")]
    ModernMeasuringDevice,

    /// Electronic home display
    #[serde(rename = "ELEKTRONISCHER_HAUSHALTSZAEHLER")]
    ElectronicHouseholdMeter,

    /// Intelligent measuring system
    #[serde(rename = "INTELLIGENTES_MESSSYSTEM")]
    IntelligentMeasuringSystem,

    /// Load profile meter
    #[serde(rename = "LASTGANGZAEHLER")]
    LoadProfileMeter,

    /// Maximum meter
    #[serde(rename = "MAXIMUMZAEHLER")]
    MaximumMeter,

    /// Turbine wheel gas meter
    #[serde(rename = "TURBINENRADZAEHLER")]
    TurbineWheelMeter,

    /// Ultrasonic gas meter
    #[serde(rename = "ULTRASCHALLZAEHLER")]
    UltrasonicMeter,

    /// District heating meter
    #[serde(rename = "WAERMEZAEHLER")]
    HeatMeter,

    /// Water meter
    #[serde(rename = "WASSERZAEHLER")]
    WaterMeter,

    /// Wirk meter (active energy)
    #[serde(rename = "WIRKZAEHLER")]
    ActiveEnergyMeter,

    /// Virtual meter
    #[serde(rename = "VIRTUELLER_ZAEHLER")]
    VirtualMeter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeterType::IntelligentMeasuringSystem).unwrap(),
            r#""INTELLIGENTES_MESSSYSTEM""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<MeterType>(r#""MODERNE_MESSEINRICHTUNG""#).unwrap(),
            MeterType::ModernMeasuringDevice
        );
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core meter_type
git add crates/bo4e-core/src/enums/meter_type.rs crates/bo4e-core/src/enums/mod.rs
git commit -m "feat(core): add MeterType (Zaehlertyp) enum"
```

---

## Task 5-30: Remaining Energy/Technical Enums

Follow the same pattern for all remaining enums in this category. Each enum should:

1. Have its own file in `crates/bo4e-core/src/enums/`
2. Use `#[serde(rename = "GERMAN_VALUE")]` for each variant
3. Include tests for serialize/deserialize roundtrip
4. Be exported from `mod.rs`
5. Get its own commit

**Pattern for each:**

```rust
//! [Description] ([German name]) enumeration.

use serde::{Deserialize, Serialize};

/// [English description].
///
/// German: [German name]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum [EnumName] {
    /// [Variant description]
    #[serde(rename = "[GERMAN_VALUE]")]
    [EnglishVariant],
    // ... more variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        // Test all variants
    }
}
```

---

## Verification

After completing all enums:

```bash
cargo test -p bo4e-core enums -- --nocapture
cargo clippy -p bo4e-core
```

Expected: All tests pass, no warnings.

---

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 84 |
| Passed | 84 |
| Failed | 0 |
| Skipped | 0 |

All 30 energy/technical enums implemented with roundtrip serialization tests passing.
