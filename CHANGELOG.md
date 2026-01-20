# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-20

### Added

- Initial release of BO4E-Rust
- **Business Objects (37 types)**
  - Meter, MarketLocation, MeteringLocation, NetworkLocation
  - BusinessPartner, Person, MarketParticipant
  - Contract, BundleContract, Offer, Tender
  - Invoice, Tariff, Costs, and more
- **Components (64 types)**
  - Address, GeoCoordinates, ContactMethod
  - Price, Amount, PriceTier, Surcharge
  - MeasuredValue, Quantity, TimePeriod, and more
- **Enumerations (97 types)**
  - Division, EnergyDirection, MeterType
  - ContractStatus, CustomerType, PriceType, and more
- **High-performance serialization**
  - SIMD-accelerated JSON parsing via simd-json
  - 2-4x faster than Python for large payloads
- **Bilingual JSON support**
  - German field names (BO4E standard)
  - English field names (optional)
  - Runtime configuration
- **Full compatibility with BO4E-Python**
  - Same data model structure
  - Interoperable JSON format

### Documentation

- Comprehensive API documentation
- Usage examples for all major types
- Benchmark comparisons with Python

[0.1.0]: https://github.com/hochfrequenz/bo4e-rust/releases/tag/v0.1.0
