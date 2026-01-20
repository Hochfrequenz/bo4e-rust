---
status: in_progress
---
# Epic 6.3: Release Preparation

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Prepare and publish v0.1.0 to crates.io.

**Architecture:** Final checks, changelog, version bump, tag, publish.

**Tech Stack:** cargo-release, GitHub releases

---

## Task 1: Final Pre-Release Checks

**Step 1: Run full test suite**

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

**Step 2: Check for security advisories**

```bash
cargo install cargo-audit
cargo audit
```

**Step 3: Check for outdated dependencies**

```bash
cargo install cargo-outdated
cargo outdated
```

**Step 4: Verify minimum supported Rust version**

```bash
rustup run 1.75 cargo test --workspace
```

**Step 5: Commit any fixes**

```bash
git add -A
git commit -m "chore: pre-release fixes"
```

---

## Task 2: Create CHANGELOG

**Files:**
- Create: `CHANGELOG.md`

**Step 1: Write changelog**

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-XX-XX

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
```

**Step 2: Commit**

```bash
git add CHANGELOG.md
git commit -m "docs: add CHANGELOG for v0.1.0"
```

---

## Task 3: Update Version and Metadata

**Files:**
- Modify: `Cargo.toml` (workspace)
- Modify: `crates/*/Cargo.toml`

**Step 1: Verify version is 0.1.0**

Check `[workspace.package]` in root `Cargo.toml`:

```toml
[workspace.package]
version = "0.1.0"
```

**Step 2: Verify crate metadata**

Each crate should have:
- `description`
- `license`
- `repository`
- `keywords`
- `categories`

**Step 3: Run cargo publish dry-run**

```bash
cargo publish -p bo4e-core --dry-run
cargo publish -p bo4e-serde --dry-run
cargo publish -p bo4e --dry-run
```

**Step 4: Commit metadata changes**

```bash
git add Cargo.toml crates/*/Cargo.toml
git commit -m "chore: prepare v0.1.0 metadata"
```

---

## Task 4: Create Release Tag

**Step 1: Create annotated tag**

```bash
git tag -a v0.1.0 -m "Release v0.1.0

Initial release of BO4E-Rust - Rust implementation of the Business Objects
for Energy standard.

Features:
- 37 Business Objects
- 64 Components
- 97 Enumerations
- SIMD-accelerated JSON parsing
- German/English field name support
- Full BO4E-Python compatibility"
```

**Step 2: Push tag**

```bash
git push origin main
git push origin v0.1.0
```

---

## Task 5: Publish to crates.io

**Prerequisites:**
- crates.io account
- API token configured: `cargo login`

**Step 1: Publish bo4e-core**

```bash
cargo publish -p bo4e-core
```

Wait for it to appear on crates.io (~30 seconds).

**Step 2: Publish bo4e-serde**

```bash
cargo publish -p bo4e-serde
```

Wait for it to appear on crates.io.

**Step 3: Publish bo4e**

```bash
cargo publish -p bo4e
```

**Step 4: Verify publication**

Visit:
- https://crates.io/crates/bo4e-core
- https://crates.io/crates/bo4e-serde
- https://crates.io/crates/bo4e

---

## Task 6: Create GitHub Release

**Step 1: Create release on GitHub**

Go to https://github.com/hochfrequenz/bo4e-rust/releases/new

- Tag: `v0.1.0`
- Title: `v0.1.0`
- Description: Copy from CHANGELOG.md
- Check "Create a discussion for this release"

**Step 2: Announce release**

- Post in relevant communities
- Update BO4E website if applicable

---

## Verification Checklist

- [ ] All tests pass on CI
- [ ] Documentation builds without warnings
- [ ] cargo audit reports no vulnerabilities
- [ ] MSRV (1.75) verified
- [ ] CHANGELOG.md complete
- [ ] Version 0.1.0 in all Cargo.toml
- [ ] Tag v0.1.0 created and pushed
- [ ] All three crates published to crates.io
- [ ] GitHub release created
- [ ] Crates appear in search: `cargo search bo4e`
