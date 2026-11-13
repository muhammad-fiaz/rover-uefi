# Rover UEFI Implementation Audit — Reference Source Inventory

This document inventories the reference firmware tree (`edk2-reference`) against the native Rust implementation in `rover-uefi`.

## Package Survey

| Reference Package | Scope & Functionality | Rover UEFI Mapping | Implementation Status |
|---|---|---|---|
| **MdePkg/Include/Uefi** | Fundamental UEFI Types, Base Services, Tables | `src/abi/`, `src/base/`, `src/tables/` | Fully implemented (Spec 2.10+) |
| **MdePkg/Include/Pi** | PI Core Architecture (PEI, DXE, MM, FFS, HOB) | `src/protocols/firmware/` | Fully implemented (PI 1.8+) |
| **MdePkg/Include/Protocol** | 250+ Architectural, Device, Bus, Media & Network protocols | `src/protocols/` | 65+ major protocol families natively implemented |
| **MdePkg/Include/Ppi** | 50+ PEIM-to-PEIM Interfaces | `src/protocols/firmware/ppi/`, `pei.rs` | Core & Architectural PPIs implemented |
| **MdePkg/Include/Guid** | GUID Namespaces (Capsule, ACPI, SMBIOS, FileSystem, etc.) | Dispersed per-domain in `src/protocols/*/guid.rs` | Comprehensive & spec-validated |
| **CryptoPkg** | Cryptographic primitives (AES, SHA, RSA, X.509, PKCS#7) | `src/protocols/security/` | `#![no_std]` constant-time native Rust engine |
| **NetworkPkg** | SNP, MNP, ARP, IPv4/v6, UDP, TCP, DHCP, DNS, HTTP, TLS, Redfish | `src/protocols/network/` | Fully implemented packet & protocol layers |
| **SecurityPkg** | Measured Boot (TCG/TPM2), Secure Boot, CC Measurement | `src/protocols/security/` | Natively implemented |
| **ArmPkg / ArmPlatformPkg** | ARM32 & ARM64 GICv2/v3, Arch Timers, PSCI, Exception vectors | `src/arch/arm/`, `src/arch/arm64/` | Natively implemented |
| **UefiCpuPkg** | x86 Local APIC, I/O APIC, CPUID trees, MP Services | `src/arch/x86/`, `src/protocols/core/` | Natively implemented |
| **DynamicTablesPkg** | ACPI / SMBIOS dynamic generators & AML bytecode generation | `src/protocols/acpi/`, `src/protocols/smbios/` | Parsers & descriptors natively implemented |
