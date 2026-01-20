---
status: in_progress
---
# Epic 1.2: CI/CD Pipeline

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Set up GitHub Actions for continuous integration, testing, and automated releases to crates.io.

**Architecture:** Two workflows - CI runs on every PR (fmt, clippy, test, coverage), Release runs on version tags.

**Tech Stack:** GitHub Actions, cargo-llvm-cov, codecov

---

## Task 1: Create CI Workflow

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: Create workflow directory**

```bash
mkdir -p .github/workflows
```

**Step 2: Create ci.yml**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings

jobs:
  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace --all-targets --all-features -- -D warnings

  test:
    name: Test (${{ matrix.os }}, ${{ matrix.rust }})
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
        include:
          - os: ubuntu-latest
            rust: "1.75"  # MSRV
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace --all-features

  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - uses: taiki-e/install-action@cargo-llvm-cov
      - name: Generate coverage
        run: cargo llvm-cov --workspace --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          fail_ci_if_error: false

  docs:
    name: Docs
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --workspace --no-deps
        env:
          RUSTDOCFLAGS: -Dwarnings
```

**Step 3: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add CI workflow with fmt, clippy, test, coverage"
```

---

## Task 2: Create Release Workflow

**Files:**
- Create: `.github/workflows/release.yml`

**Step 1: Create release.yml**

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

env:
  CARGO_TERM_COLOR: always

jobs:
  # Run full CI first
  ci:
    uses: ./.github/workflows/ci.yml

  publish:
    name: Publish to crates.io
    needs: ci
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable

      - name: Publish bo4e-core
        run: cargo publish -p bo4e-core --token ${{ secrets.CARGO_REGISTRY_TOKEN }}

      - name: Wait for crates.io
        run: sleep 30

      - name: Publish bo4e-serde
        run: cargo publish -p bo4e-serde --token ${{ secrets.CARGO_REGISTRY_TOKEN }}

      - name: Wait for crates.io
        run: sleep 30

      - name: Publish bo4e
        run: cargo publish -p bo4e --token ${{ secrets.CARGO_REGISTRY_TOKEN }}

  github-release:
    name: Create GitHub Release
    needs: publish
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          generate_release_notes: true
```

**Step 2: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add release workflow for crates.io publishing"
```

---

## Task 3: Add Rust Toolchain Config

**Files:**
- Create: `rust-toolchain.toml`
- Create: `rustfmt.toml`
- Create: `clippy.toml`

**Step 1: Create rust-toolchain.toml**

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

**Step 2: Create rustfmt.toml**

```toml
# Stable rustfmt options
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
```

**Step 3: Create clippy.toml**

```toml
# Clippy configuration
msrv = "1.75"
```

**Step 4: Commit**

```bash
git add rust-toolchain.toml rustfmt.toml clippy.toml
git commit -m "chore: add rust toolchain and linter configs"
```

---

## Task 4: Add Dependabot Configuration

**Files:**
- Create: `.github/dependabot.yml`

**Step 1: Create dependabot.yml**

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    commit-message:
      prefix: "deps"
    groups:
      rust-dependencies:
        patterns:
          - "*"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    commit-message:
      prefix: "ci"
    groups:
      github-actions:
        patterns:
          - "*"
```

**Step 2: Commit**

```bash
git add .github/dependabot.yml
git commit -m "ci: add dependabot for automated dependency updates"
```

---

## Task 5: Create CONTRIBUTING Guide

**Files:**
- Create: `CONTRIBUTING.md`

**Step 1: Create CONTRIBUTING.md**

```markdown
# Contributing to BO4E-Rust

Thank you for your interest in contributing to BO4E-Rust!

## Development Setup

1. Install Rust 1.75 or later: https://rustup.rs/
2. Clone the repository:
   ```bash
   git clone https://github.com/hochfrequenz/bo4e-rust.git
   cd bo4e-rust
   ```
3. Build and test:
   ```bash
   cargo build --workspace
   cargo test --workspace
   ```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Write tests for new functionality

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with clear commit messages
3. Ensure CI passes (fmt, clippy, tests)
4. Open a PR with a description of changes

## Commit Messages

Follow conventional commits:
- `feat:` new features
- `fix:` bug fixes
- `docs:` documentation
- `chore:` maintenance
- `ci:` CI/CD changes
- `test:` test additions/changes

## Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p bo4e-core

# With output
cargo test --workspace -- --nocapture
```

## Questions?

Open an issue or discussion on GitHub.
```

**Step 2: Commit**

```bash
git add CONTRIBUTING.md
git commit -m "docs: add CONTRIBUTING guide"
```

---

## Verification

Check that all files are in place:

```bash
ls -la .github/workflows/
ls -la *.toml
```

Expected files:
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/dependabot.yml`
- `rust-toolchain.toml`
- `rustfmt.toml`
- `clippy.toml`
- `CONTRIBUTING.md`

---

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 1 |
| Passed | 1 |
| Failed | 0 |
| Skipped | 2 |
| Coverage | N/A (CI/CD config only) |

Files created:
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/dependabot.yml`
- `rust-toolchain.toml`
- `rustfmt.toml`
- `clippy.toml`
- `CONTRIBUTING.md`

Verification:
- `cargo test --workspace` - passed
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` - passed
- `cargo fmt --all --check` - passed
