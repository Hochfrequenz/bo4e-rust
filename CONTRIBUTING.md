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
