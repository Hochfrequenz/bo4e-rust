# BO4E-Rust

Rust implementation of [BO4E (Business Objects for Energy)](https://www.bo4e.de/) - a standard for data exchange in the German energy industry.

## Features

- **Type-safe**: Leverage Rust's type system for compile-time guarantees
- **High-performance**: SIMD-accelerated JSON parsing via simd-json
- **Bilingual**: Runtime configuration for German or English JSON field names
- **Compatible**: Interoperable with BO4E-Python and other implementations

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
    meter_number: Some("1234567890".to_string()),
    ..Default::default()
};

// Serialize to German JSON (BO4E standard)
let json = to_json_german(&meter).unwrap();

// Deserialize
let mut bytes = json.into_bytes();
let parsed: Meter = from_json(&mut bytes).unwrap();
```

## Crate Structure

| Crate | Description |
|-------|-------------|
| `bo4e` | Facade crate - use this |
| `bo4e-core` | Pure data types (BOs, COMs, Enums) |
| `bo4e-serde` | Serialization with simd-json |

## Related Projects

- [BO4E-Python](https://github.com/Hochfrequenz/BO4E-python) - Python implementation
- [BO4E-Schemas](https://github.com/bo4e/BO4E-Schemas) - JSON schemas
- [bo4e.de](https://www.bo4e.de/) - Official BO4E website

## License

MIT License - see [LICENSE](LICENSE)
