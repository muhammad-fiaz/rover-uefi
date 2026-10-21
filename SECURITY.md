# Security Policy

The `rover-uefi` project takes security, memory safety, and firmware integrity seriously. This document describes our vulnerability disclosure process and security policies.

---

## Supported Versions

Security updates and critical vulnerability patches are actively maintained for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

---

## Reporting a Vulnerability

If you discover a security vulnerability or potential exploit in `rover-uefi`, please report it responsibly so that we can resolve the issue before public disclosure.

**Please DO NOT report security issues via public GitHub issues, discussions, or pull requests.**

### How to Report

You can report vulnerabilities securely via:

1. **Email:** Send your report directly to **[contact@muhammadfiaz.com](mailto:contact@muhammadfiaz.com)** with the subject line `[SECURITY] rover-uefi: <Brief Summary>`.
2. **GitHub Security Advisory:** Submit a private vulnerability report via [GitHub Private Vulnerability Reporting](https://github.com/muhammad-fiaz/rover/security/advisories/new).

### What to Include in Your Report

To help us triage and resolve the issue quickly, please provide:

- **Component:** The affected module, protocol, or parser (e.g., `protocols::security::x509`, `protocols::file_system::fat`, `protocols::acpi`).
- **Severity & Impact:** Your assessment of the vulnerability's impact (e.g., memory exhaustion, out-of-bounds read in bare-metal UEFI, denial of service).
- **Reproduction Steps:** Minimal proof-of-concept (PoC) code or malformed input payload that reproduces the issue.
- **Environment:** Target architecture (`x86_64-unknown-uefi`, `aarch64-unknown-uefi`, `i686-unknown-uefi`), Rust toolchain version, and compiler flags.
- **Mitigation:** Any proposed fix or workaround, if available.

---

## Response Process & Timelines

1. **Acknowledgment:** We will acknowledge receipt of your report within **48 hours**.
2. **Investigation & Triage:** We will assess and confirm the vulnerability within **5 business days**, providing an initial severity rating and timeline for a patch.
3. **Fix & Verification:** A patch will be engineered in a private security fork, verified against target platforms, and reviewed.
4. **Coordinated Disclosure:** We will coordinate with the reporter on a public disclosure date and release a security advisory alongside a patched crate release.
5. **Credit:** Reporters will be acknowledged and credited in the release notes and security advisories (unless you prefer to remain anonymous).

---

## Security Architecture & Best Practices

`rover-uefi` is designed from the ground up for high-assurance firmware environments:

- **Strict `#![no_std]`:** No reliance on host operating system libraries or runtime allocators.
- **Zero Panics:** Parsers use bounded slice access and checked arithmetic; non-test code enforces `#![deny(clippy::unwrap_used)]` and `#![deny(clippy::expect_used)]`.
- **Constant-Time Primitives:** Cryptographic operations (AES, SHA, HMAC) avoid secret-dependent timing variations.
- **Specification-Accurate ABIs:** All function pointers use `extern "efiapi"` with explicit `#[repr(C)]` layouts.

