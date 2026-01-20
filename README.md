# BO4E-Rust

[![CI](https://github.com/Hochfrequenz/bo4e-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/Hochfrequenz/bo4e-rust/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/bo4e.svg)](https://crates.io/crates/bo4e)
[![Documentation](https://docs.rs/bo4e/badge.svg)](https://docs.rs/bo4e)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust implementation of [BO4E (Business Objects for Energy)](https://www.bo4e.de/) - a standardized data model for the German energy industry.

## Features

- **Complete BO4E Coverage**: 35 Business Objects, 64 Components, 73 Enumerations
- **Type-safe**: Leverage Rust's type system for compile-time guarantees
- **High-performance**: SIMD-accelerated JSON parsing via [simd-json](https://github.com/simd-lite/simd-json)
- **Bilingual JSON**: Runtime configuration for German (default) or English field names
- **Compatible**: Interoperable with [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python) and other implementations

## Installation

```toml
[dependencies]
bo4e = "0.1"
```

## Quick Start

```rust
use bo4e::prelude::*;

// Create a meter
let meter = Meter {
    meter_number: Some("1EMH0012345678".to_string()),
    division: Some(Division::Electricity),
    meter_type: Some(MeterType::ModernMeasuringDevice),
    ..Default::default()
};

// Serialize to German JSON (BO4E standard)
let json = to_json_german(&meter).unwrap();
println!("{}", json);

// Deserialize (requires mutable slice for simd-json)
let mut bytes = json.into_bytes();
let parsed: Meter = from_json(&mut bytes).unwrap();
```

## Bilingual JSON Support

BO4E-Rust supports both German (standard) and English field names at runtime:

```rust
use bo4e::prelude::*;

let meter = Meter {
    meter_number: Some("123".to_string()),
    ..Default::default()
};

// German field names (BO4E standard)
let german = to_json_german(&meter).unwrap();
// {"zaehlernummer":"123",...}

// English field names
let english = to_json_english(&meter).unwrap();
// {"meterNumber":"123",...}
```

## Type Overview

### Business Objects (35 types)

| Category | Types |
|----------|-------|
| **Locations & Technical** | `Meter`, `MarketLocation`, `MeteringLocation`, `NetworkLocation`, `Device`, `TechnicalResource`, `ControllableResource`, `EnergyAmount`, `LoadProfile`, `TimeSeries`, `LocationAssignment`, `LocationProperties` |
| **Parties & Contracts** | `BusinessPartner`, `Person`, `MarketParticipant`, `Contract`, `BundleContract`, `Offer`, `Tender`, `Balancing`, `Region`, `RegionalTariff` |
| **Pricing & Billing** | `Invoice`, `Tariff`, `TariffInfo`, `TariffPriceSheet`, `PriceSheet`, `ServicePriceSheet`, `HardwarePriceSheet`, `MeteringPriceSheet`, `NetworkUsagePriceSheet`, `ConcessionFeePriceSheet`, `Costs`, `TariffCosts`, `ExternalCosts` |

### Components (64 types)

Address, contact, pricing, measurement, and time-related composite types including:
`Address`, `GeoCoordinates`, `ContactMethod`, `Price`, `Amount`, `PriceTier`, `Surcharge`, `MeasuredValue`, `Quantity`, `TimePeriod`, `MeterReading`, `MeterRegister`, and many more.

### Enumerations (73 types)

Type-safe enums for all BO4E values including:
`Division`, `EnergyDirection`, `MeterType`, `ContractStatus`, `CustomerType`, `PriceType`, `Unit`, `Currency`, and more.

## Crate Structure

| Crate | Description |
|-------|-------------|
| [`bo4e`](https://crates.io/crates/bo4e) | Facade crate - **use this** |
| [`bo4e-core`](https://crates.io/crates/bo4e-core) | Pure data types (BOs, COMs, Enums) |
| [`bo4e-serde`](https://crates.io/crates/bo4e-serde) | SIMD-accelerated serialization |

## MSRV

Minimum Supported Rust Version: **1.75**

## Related Projects

- [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python) - Python implementation
- [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) - JSON schemas
- [bo4e.de](https://www.bo4e.de/) - Official BO4E website

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Format & lint
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

## License

MIT License - see [LICENSE](LICENSE)
