## Description
<!-- Provide a brief description of the changes introduced by this pull request. -->

## Motivation & Context
<!-- Why is this change required? What problem does it solve? -->
<!-- If it fixes an open issue, link to it here: e.g. Fixes #123 -->

## Type of Change
<!-- Mark the relevant options with an 'x': -->
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality or UEFI/PI protocol)
- [ ] Breaking change (fix or feature causing existing code not to work as expected)
- [ ] Documentation update
- [ ] Architecture / hardware backend enhancement

## Subsystems Affected
- [ ] Core ABI / System Tables
- [ ] Boot / Runtime Services
- [ ] PI Phases (PEI / DXE / MM / SMM)
- [ ] Hardware / Architecture (x86_64, IA-32, ARM64, ARM32, RISC-V, LoongArch64)
- [ ] Protocols (Storage, Network, Security, PCI, USB, Graphics, Console, etc.)
- [ ] Cryptography / TPM / Secure Boot

## Checklist
- [ ] My code adheres to the project's `#![no_std]` and zero-dependency requirements.
- [ ] I have run `cargo fmt --all -- --check` and confirmed formatting passes.
- [ ] I have run `cargo check --all-targets` with 0 errors.
- [ ] I have run `cargo clippy --all-targets --all-features -- -D warnings` with 0 warnings.
- [ ] I have run `cargo test --all-targets` and all unit and integration tests pass.
- [ ] I have added appropriate unit/integration tests for any new functionality.
- [ ] I have verified that documentation builds cleanly with `cargo doc --no-deps`.
- [ ] I have updated relevant documentation / `README.md` where appropriate.
