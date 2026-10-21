# Contributing to rover-uefi

Thank you for your interest in contributing to `rover-uefi`! This document outlines our engineering standards, architecture constraints, development workflows, and pull request guidelines.

---

## Table of Contents

- [Core Principles](#core-principles)
- [Prerequisites](#prerequisites)
- [Development Workflow](#development-workflow)
- [Coding & Architectural Standards](#coding--architectural-standards)
- [Testing & Quality Assurance](#testing--quality-assurance)
- [Pull Request Process](#pull-request-process)
- [License](#license)

---

## Core Principles

`rover-uefi` is engineered as a modern, high-assurance UEFI and Platform Initialization (PI) firmware ecosystem. Every contribution must adhere to these non-negotiable fundamentals:

1. **Pure `#![no_std]` Core:** The library operates in pre-boot bare-metal environments. It must never depend on the Rust standard library (`std`) or external production runtime dependencies in `Cargo.toml`.
2. **Zero External Runtime Dependencies:** To maintain auditability, supply-chain security, and firmware determinism, `[dependencies]` in `Cargo.toml` must remain strictly empty.
3. **Specification-Accurate ABI:** All firmware function pointers, protocols, and table definitions must precisely match UEFI 2.10/2.11 and PI 1.8 specifications with `#[repr(C)]` layout and `extern "efiapi"` calling conventions.
4. **Defensive Bounded Parsing:** Firmware parsing logic must never panic. All slice indexes, buffer reads, and dynamic tables must use checked bounds (`unwrap_used = "deny"` and `expect_used = "deny"` are enforced compiler lints).
5. **Independently Engineered:** All implementations are crafted natively in pure Rust from official specification standards without incorporating third-party C codebase files or raw header copies.

---

## Prerequisites

Ensure you have a modern Rust toolchain installed (Rust 1.85+ recommended for Rust 2024 edition support) along with the bare-metal UEFI target triples:

```bash
# Add supported UEFI compilation targets
rustup target add x86_64-unknown-uefi
rustup target add aarch64-unknown-uefi
rustup target add i686-unknown-uefi

# Ensure clippy and rustfmt are up to date
rustup component add clippy rustfmt
```

---

## Development Workflow

### 1. Build Verification

Verify that your changes compile cleanly across host and target platforms:

```bash
# Host check
cargo check

# Target cross-compilation checks
cargo check --target x86_64-unknown-uefi
cargo check --target aarch64-unknown-uefi
cargo check --target i686-unknown-uefi
```

### 2. Formatting

Code formatting is enforced across the entire repository using `rustfmt`:

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting automatically
cargo fmt --all
```

### 3. Static Analysis & Linting

We enforce strict compiler and Clippy lints with warnings treated as hard errors:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### 4. Running Tests

Run the full integration and unit test suite:

```bash
cargo test --all-features
```

---

## Coding & Architectural Standards

### 1. Protocols & System Tables

- Use `#[repr(C)]` on all UEFI/PI structures, protocol tables, and packet layouts.
- Mark all function pointer types with `pub type Efi<Name> = unsafe extern "efiapi" fn(...) -> EfiStatus;`.
- Include the canonical protocol `EfiGuid` definition alongside the protocol structure.
- Provide safe higher-level wrappers or parsing helpers where raw pointer operations can be encapsulated cleanly.

### 2. Documentation

- Every public type, function, constant, and module must include comprehensive Rust docstrings (`missing_docs = "deny"` is enforced).
- Describe protocol capabilities, parameter semantics, and expected return status codes (`EfiStatus::SUCCESS`, `EfiStatus::BUFFER_TOO_SMALL`, etc.).
- Avoid references to proprietary or legacy C filenames; document concepts in terms of official UEFI/PI specifications.

### 3. Error Handling

- Return descriptive `EfiStatus` values or strongly-typed Rust error enums.
- Never use `.unwrap()` or `.expect()` in non-test library code. Use pattern matching (`if let`, `match`), `ok_or()`, or the `?` operator instead.

---

## Testing & Quality Assurance

- **Unit Tests:** Located in `tests/` or inline inside module `#[cfg(test)]` blocks.
- **Malformed Input Testing:** Parsers (FAT, ACPI, USB descriptors, Device Paths, Cryptographic signatures) must include tests with truncated, boundary-exceeding, or corrupted data to guarantee resilience against invalid firmware tables.
- **Round-Trip Tests:** Serialization and deserialization operations should test round-trip fidelity.

---

## Pull Request Process

1. **Create a Topic Branch:** Branch off `main` with a descriptive name (e.g., `feat/virtio-scsi-protocol`, `fix/acpi-slit-parsing`).
2. **Make Atomic Commits:** Follow [Conventional Commits](https://www.conventionalcommits.org/) format:
   - `feat:` for new protocols, parsers, or architecture support
   - `fix:` for bug fixes
   - `docs:` for documentation updates
   - `refactor:` for code cleanups without functional change
   - `test:` for adding or updating test cases
   - `chore:` for build scripts, CI, or manifest updates
3. **Verify All Gates Pass Locally:**
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo check --target x86_64-unknown-uefi`
   - `cargo check --target aarch64-unknown-uefi`
   - `cargo check --target i686-unknown-uefi`
   - `cargo test --all-features`
4. **Submit Pull Request:** Provide a clear summary of your changes, the rationale, and any specification references.

---

## License

By contributing to `rover-uefi`, you agree that your contributions will be licensed under the **Apache License, Version 2.0** terms as detailed in the [LICENSE](LICENSE) file.

