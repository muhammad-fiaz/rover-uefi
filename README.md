<div align="center">

# Rover UEFI

<a href="https://github.com/muhammad-fiaz/rover-uefi"><img src="https://img.shields.io/badge/Rust-2024%20Edition-orange.svg?logo=rust" alt="Rust Edition"></a>
<a href="https://github.com/muhammad-fiaz/rover-uefi"><img src="https://img.shields.io/github/stars/muhammad-fiaz/rover-uefi" alt="GitHub stars"></a>
<a href="https://github.com/muhammad-fiaz/rover-uefi/issues"><img src="https://img.shields.io/github/issues/muhammad-fiaz/rover-uefi" alt="GitHub issues"></a>
<a href="https://github.com/muhammad-fiaz/rover-uefi/pulls"><img src="https://img.shields.io/github/issues-pr/muhammad-fiaz/rover-uefi" alt="GitHub pull requests"></a>
<a href="https://github.com/muhammad-fiaz/rover-uefi"><img src="https://img.shields.io/github/last-commit/muhammad-fiaz/rover-uefi" alt="GitHub last commit"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache--2.0-blue.svg" alt="License"></a>
<a href="https://github.com/muhammad-fiaz/rover-uefi/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/muhammad-fiaz/rover-uefi/ci.yml?branch=main&label=CI" alt="CI"></a>
<a href="https://github.com/muhammad-fiaz/rover-uefi/actions/workflows/github-code-scanning/codeql"><img src="https://github.com/muhammad-fiaz/rover-uefi/actions/workflows/github-code-scanning/codeql/badge.svg" alt="CodeQL"></a>
<img src="https://img.shields.io/badge/platforms-x86__64--uefi%20%7C%20aarch64--uefi%20%7C%20i686--uefi-blue" alt="Supported Platforms">
<a href="https://pay.muhammadfiaz.com"><img src="https://img.shields.io/badge/Sponsor-pay.muhammadfiaz.com-ff69b4?style=flat&logo=heart" alt="Sponsor"></a>
<a href="https://github.com/sponsors/muhammad-fiaz"><img src="https://img.shields.io/badge/Sponsor-GitHub-pink?style=social&logo=github" alt="GitHub Sponsors"></a>
<a href="https://hits.sh/github.com/muhammad-fiaz/rover-uefi/"><img alt="Hits" src="https://hits.sh/github.com/muhammad-fiaz/rover-uefi.svg"/></a>

<p><em>Modern, feature-rich, cross-platform UEFI and PI firmware development in Rust.</em></p>

</div>

`rover-uefi` is a modern, feature-rich, cross-platform firmware development library for the UEFI and Platform Initialization (PI) specifications in Rust. It provides spec-accurate ABIs, system tables, Boot and Runtime Services, 60+ protocols, native cryptography, and bounded parsers for cross-platform pre-boot environments.

> [!TIP]
> If you build with `rover-uefi`, make sure give it a star ⭐ on Github!

---

<details>
<summary><strong>Features</strong> (click to expand)</summary>

| Feature Category | Description |
|---|---|
| **Core ABI & Tables** | `extern "efiapi"` calling convention, primitive types (`Char16`, `UIntN`, `EfiGuid`, `EfiHandle`, `EfiStatus`, `EfiTime`), `EfiSystemTable`, `EfiBootServices`, `EfiRuntimeServices`, `EfiConfigurationTable`, standard table header validation, EFI System Resource Table (ESRT), and DXE Services Table (`EfiDxeServices`) with Global Coherency Domain (GCD) memory and I/O management. |
| **Boot & Load Management** | Spec-accurate UEFI Load Options (`Boot####`, `Driver####`, `SysPrep####`, `PlatformRecovery####`), zero-copy binary serialization and parsing, `BootOrder` and `DriverOrder` list manipulation, hotkey descriptors (`EfiKeyOptionHeader`, `EfiBootKeyData`), CRC32 target verification, removable media default paths (`BOOTX64.EFI`, `BOOTAA64.EFI`, `BOOTIA32.EFI`, etc.), and capsule update triggers. |
| **PEI & DXE Foundation** | PI PEI Services Table (`EfiPeiServices`) with PPI installation, notification, boot modes, and security dispatch; DXE Dispatcher architectures, architectural protocols, Hand-Off Blocks (HOBs), and Guided Section Extraction (`EFI_SECTION_GUID_DEFINED`) with native LZH decompression (`EFI_DECOMPRESS_PROTOCOL`). |
| **Runtime Services** | Variable services (`get_variable`, `set_variable`, `get_next_variable_name`), time and wakeup alarms, virtual memory mapping (`set_virtual_address_map`), hardware reset systems (`ResetSystem2`), and capsule update queries. |
| **Device Paths** | Device path nodes and instances (Hardware, ACPI, Messaging, Media, BBS, End), node iteration, validation, and Device Path Utilities (`EFI_DEVICE_PATH_UTILITIES_PROTOCOL`). |
| **Hardware Architecture** | x86/x86_64 Local APIC, I/O APIC, and structured CPUID feature trees; IA-32 exception contexts; ARM64 GICv2/v3 distributor/CPU interfaces, system registers, and exception vectors; ARM32 (ARMv7-A) exception contexts; RISC-V contexts and SBI extensions (Base, Timer, IPI, HSM, SRST, PMU, DBCN); LoongArch64 exception contexts and CSR parsing; common MMIO, DMA descriptors, and system timers. |
| **Firmware Synchronization** | `#![no_std]` high-performance concurrency primitives: ticket locks, spinlocks, CPU barriers, and interrupt-safe locks (`IntrLock`) tailored for multi-core MP Services and runtime environments. |
| **PCI Subsystem** | `EfiPciIoProtocol`, `EfiPciRootBridgeIoProtocol`, `EfiPciHostBridgeResourceAllocationProtocol`, `EfiPciHotPlugInitProtocol`, CXL 2.0/3.0 protocol, Type 0/1 header decoding, BAR resource allocation, capability list traversal, and PCIe configuration space access. |
| **USB Subsystem** | USB Host Controller interfaces (`EFI_USB2_HC_PROTOCOL`, `EFI_USB3_HC_PROTOCOL`, EHCI, XHCI TRBs/rings), USB I/O Protocol, standard descriptors, HID report parser, and USB Mass Storage (Bulk-Only Transport & SCSI). |
| **Storage & Filesystems** | Block I/O & Block I/O 2, Disk I/O & Disk I/O 2, Simple File System (`EFI_FILE_PROTOCOL`), Storage Security Command Protocol (TCG Opal / IEEE 1667), Erase Block Protocol, Disk Info Protocol, RAM Disk Protocol, SD/MMC Pass-Thru, Partition discovery (MBR, GPT, protective MBR, EBR, Partition Info), `#![no_std]` FAT12/16/32 reader with LFN, and ATA/NVMe/SCSI pass-thru protocols. |
| **Cryptography & Security** | Constant-time AES-128/256, SHA-1/224/256/384/512, SHA-3, RSA PKCS#1 v1.5 verification, X.509 ASN.1 certificate decoding, Authenticode PE signature verification, Image Execution Information Table (`EFI_IMAGE_EXECUTION_INFO_TABLE_GUID`), Secure Boot key databases (`PK`, `KEK`, `db`, `dbx`), RNG Protocol, PKCS#7 Verification Protocol, Confidential Computing (CC) Measurement Protocol, and Device Security Protocols. |
| **TCG / TPM** | TPM 1.2 and TPM 2.0 protocols (`EfiTcg2Protocol`), TCG event log parsing, SHA-1/SHA-256 PCR bank operations, startup commands, and measurements. |
| **ACPI & SMBIOS** | Full ACPI table generators and bounded parsers (RSDP, RSDT, XSDT, FADT, MADT, MCFG, HPET, DMAR, SRAT, SLIT, SPCR, BGRT, FACS, DSDT, SSDT), ACPI SDT protocol, SMBIOS Protocol (`EFI_SMBIOS_PROTOCOL`), and SMBIOS 2.x/3.x entry points and structured record parsers. |
| **Network Protocols** | SNP (Simple Network), MNP, ARP, VLAN Config, IPv4, IPv6, IPv4/v6 Config (`EFI_IP4_CONFIG2_PROTOCOL`, `EFI_IP6_CONFIG_PROTOCOL`), ICMP, IGMP, NDP, UDPv4/v6, TCPv4/v6, TLS, DHCPv4/v6 (`EFI_DHCP4_PROTOCOL`, `EFI_DHCP6_PROTOCOL`), MTFTPv4/v6 (`EFI_MTFTP4_PROTOCOL`, `EFI_MTFTP6_PROTOCOL`), DNS, HTTP, REST EX, Redfish Discover, PXE Base Code, NII / UNDI (`EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL`), IEEE 802.11 WiFi (`EFI_WIRELESS_MAC_CONNECTION_PROTOCOL`, `EFI_WIFI2_PROTOCOL`), EAP & Supplicant, IPsec / IKEv2 VPN (`EFI_IPSEC_PROTOCOL`, `EFI_IPSEC2_PROTOCOL`, `EFI_IPSEC_CONFIG_PROTOCOL`), and HTTP/SOCKS enterprise proxy & WPAD Option 252 routing. |
| **Firmware & PI Formats** | Universal Payload (`UNIVERSAL_PAYLOAD_IDENTIFIER`, `'PLDH'`), Intel FSP 2.x headers (`FspInfoHeader`, `'FSPH'`), Platform Runtime Mechanism (PRM), Hand-Off Blocks (HOBs), Firmware Volume/FMP, HII/IFR, DEPEX, PE32/PE32+/TE headers, and Standalone/Traditional MM communication (`EFI_MM_COMMUNICATION2_PROTOCOL`, `EFI_MM_SYSTEM_TABLE`). |
| **Manageability** | IPMI Protocol & KCS register interface, MCTP packet/message dispatch, and PLDM Protocol with base and platform monitoring commands. |
| **Drivers & Binding** | UEFI Driver Binding Protocol (`EFI_DRIVER_BINDING_PROTOCOL`), Component Name Protocols (1 & 2), Loaded Image Protocol, Driver Health Protocol, Driver Diagnostics Protocols (1 & 2), Platform/Bus Driver Override Protocols, Driver Supported EFI Version, ARM SCMI Protocol, VirtIO Device Protocol, and native driver lifecycle registries. |
| **Graphics & Display** | Graphics Output Protocol (GOP) with high-performance software BLT engine, Universal Graphics Adapter (UGA) Draw Protocol (`EFI_UGA_DRAW_PROTOCOL_GUID`), EDID Discovered & Active protocols, and Pixel formats (RGB, BGR, Bitmask, BltOnly). |
| **Console & Shell** | Simple Text Input/Output, Simple Pointer & Absolute Pointer protocols, Console Control Protocol (`EFI_CONSOLE_CONTROL_PROTOCOL`), Serial I/O Protocol (`EFI_SERIAL_IO_PROTOCOL`), Unicode Collation Protocols (`EFI_UNICODE_COLLATION_PROTOCOL`, `EFI_UNICODE_COLLATION2_PROTOCOL`), UEFI Shell / Shell Parameters protocols, and Built-in Shell Command Framework with registry & execution status codes (`ShellStatus`). |
| **Mobile & Peripheral** | Android Boot & Fastboot protocols, Bluetooth Host Controller, Bluetooth Configuration & GATT protocols, Audio I/O protocol, and I2C/SPI/SMBus host and device protocols. |

</details>

---

<details>
<summary><strong>Prerequisites and Supported Platforms</strong> (click to expand)</summary>

<br>

## Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| **Rust Toolchain** | Nightly or Stable 2024 Edition (1.85+) | Compatible with standard Rust installations |
| **UEFI Targets** | Target support via `rustup target add` | Bare-metal firmware cross-compilation |

---

## Supported Targets

| Architecture | Target Triple | Supported |
|---|---|:---:|
| **x86_64 (64-bit)** | `x86_64-unknown-uefi` | ✅ |
| **AArch64 (ARM64)** | `aarch64-unknown-uefi` | ✅ |
| **x86 (32-bit)** | `i686-unknown-uefi` | ✅ |
| **Host Test Runner** | `x86_64-pc-windows-msvc` / `x86_64-unknown-linux-gnu` | ✅ |

### Cross-Compilation

```bash
# Add bare-metal UEFI compilation targets
rustup target add x86_64-unknown-uefi
rustup target add aarch64-unknown-uefi
rustup target add i686-unknown-uefi

# Check compilation for x86_64 UEFI
cargo check --target x86_64-unknown-uefi

# Check compilation for ARM64 UEFI
cargo check --target aarch64-unknown-uefi

# Check compilation for 32-bit x86 UEFI
cargo check --target i686-unknown-uefi
```

</details>

---

## Installation

Install using `cargo add`:

```bash
# From the official repository
cargo add --git https://github.com/muhammad-fiaz/rover-uefi.git

# Or from the Rover OS monorepo
cargo add --git https://github.com/muhammad-fiaz/rover.git rover-uefi
```

Or add `rover-uefi` directly to your `Cargo.toml`:

```toml
[dependencies]
# Standalone repository
rover-uefi = { git = "https://github.com/muhammad-fiaz/rover-uefi.git" }

# Or monorepo package
rover-uefi = { git = "https://github.com/muhammad-fiaz/rover.git", package = "rover-uefi" }

# Or local path (within monorepo)
rover-uefi = { path = "boot/rover-uefi" }
```

---

## Quick Start

### 1. Minimal UEFI Application Entry Point

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::tables::{EfiSystemTable, SYSTEM_TABLE_SIGNATURE};

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(
    _image_handle: EfiHandle,
    system_table: *const EfiSystemTable,
) -> EfiStatus {
    if system_table.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }

    let st = unsafe { &*system_table };

    // Validate table header signature ("IBI SYST")
    if !st.hdr.has_signature(SYSTEM_TABLE_SIGNATURE) {
        return EfiStatus::UNSUPPORTED;
    }

    EfiStatus::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

### 2. Boot Services Protocol Discovery

```rust
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::graphics::{EfiGraphicsOutputProtocol, GRAPHICS_OUTPUT_GUID};
use rover_uefi::tables::EfiBootServices;

pub fn locate_gop(bs: &EfiBootServices) -> Option<*mut EfiGraphicsOutputProtocol> {
    let mut interface = core::ptr::null_mut();
    let mut guid = GRAPHICS_OUTPUT_GUID;

    let status = unsafe {
        (bs.locate_protocol)(
            &mut guid,
            core::ptr::null_mut(),
            &mut interface,
        )
    };

    if status.is_success() && !interface.is_null() {
        Some(interface.cast())
    } else {
        None
    }
}
```

### 3. Native Bounded ACPI Parsing

```rust
use rover_uefi::protocols::acpi::{RsdpInfo, parse_rsdp};

pub fn inspect_rsdp(rsdp_bytes: &[u8]) -> Option<RsdpInfo> {
    // Safe bounded zero-copy parsing of ACPI RSDP structure
    let rsdp = parse_rsdp(rsdp_bytes)?;
    let _xsdt_addr = rsdp.xsdt_address;
    Some(rsdp)
}
```

---

<details>
<summary><strong>Validation Matrix</strong> (click to expand)</summary>

<br>

Run the comprehensive test suite and linters locally:

```bash
# Run unit and integration tests (268 suites, 1,000+ tests)
cargo test --all-features

# Strict lint verification
cargo clippy --all-targets --all-features -- -D warnings

# Code style checking
cargo fmt --all -- --check

# Cross-compilation target checks
cargo check --target x86_64-unknown-uefi
cargo check --target aarch64-unknown-uefi
cargo check --target i686-unknown-uefi
```

</details>

---

<details>
<summary><strong>Project Structure</strong> (click to expand)</summary>

```text
rover-uefi/
├── .github/                    # CI workflows, Dependabot & CodeQL security scans
├── .gitignore                  # Git exclusion rules
├── Cargo.lock                  # Dependency lockfile
├── Cargo.toml                  # Package manifest, crates.io categories & strict lints
├── CODE_OF_CONDUCT.md          # Contributor Covenant Code of Conduct
├── CONTRIBUTING.md             # Contribution guidelines & development workflow
├── LICENSE                     # Apache-2.0 license text
├── README.md                   # Project documentation
├── SECURITY.md                 # Security & vulnerability reporting policy
├── src/
│   ├── lib.rs                  # Crate root, no_std configuration & top-level exports
│   ├── abi/                    # Primitive types, calling conventions & status codes
│   ├── arch/                   # Hardware architecture support (x86, ARM64, RISC-V, ARM32, IA-32, LoongArch64)
│   │   ├── arm32/              # ARMv7-A 32-bit exception contexts
│   │   ├── arm64/              # ARM64 exception contexts & GICv2/v3 support
│   │   ├── common/             # MMIO, DMA descriptors, and system timers
│   │   ├── ia32/               # IA-32 32-bit x86 exception contexts
│   │   ├── loongarch64/        # LoongArch64 exception contexts & CSR parsing
│   │   ├── riscv/              # RISC-V contexts & SBI interfaces
│   │   └── x86/                # APIC, I/O APIC, CPUID, and context handling
│   ├── base/                   # GUIDs, handles, statuses, events, CRC32, drivers & bounded byte buffers
│   ├── services/               # Boot Services and Runtime Services wrappers
│   ├── sync/                   # Firmware synchronization primitives (spinlocks, ticket locks, barriers, interrupt locks)
│   ├── tables/                 # System, Boot, Runtime, Configuration, ESRT, and Load Option tables
│   └── protocols/              # UEFI & PI protocol interfaces
│       ├── acpi/               # ACPI table discovery, parsing, validation & SDT protocol
│       ├── android/            # Android boot & Fastboot protocol interfaces
│       ├── audio/              # Audio I/O protocol
│       ├── bluetooth/          # Bluetooth Host Controller, configuration & GATT protocols
│       ├── console/            # Text I/O, absolute pointer, console control, shell & Unicode collation
│       ├── core/               # Handle database, CPU arch, CPU I/O 2, MP Services & memory attributes
│       ├── debug/              # Debug support & debug port communication
│       ├── device_path/        # Device path nodes, instances & utilities
│       ├── diagnostic/         # Status codes, metronome, timestamp & timer architectures
│       ├── driver/             # Driver binding, health, diagnostics, overrides & supported versions
│       ├── firmware/           # Universal Payload, Intel FSP 2.x, PRM, MM/SMM, HOBs, PCD, PEI & DXE services, sections & decompression
│       ├── graphics/           # Graphics Output Protocol (GOP), UGA Draw Protocol, EDID & software BLT engine
│       ├── i2c/                # I2C host, master, and device protocols
│       ├── manageability/      # IPMI, MCTP, and PLDM protocols
│       ├── network/            # SNP, MNP, ARP, VLAN, IPv4/v6, TCP, UDP, DHCP, MTFTP, DNS, HTTP, Redfish, NII, WiFi, IPsec, Proxy
│       ├── pci/                # PCI I/O, root bridge, resource allocation, hot plug, CXL & config spaces
│       ├── security/           # AES, SHA, RSA, X.509, TPM 1.2/2.0, Secure Boot, Image Execution Info & CC measurement
│       ├── serial/             # Serial I/O protocols
│       ├── smbios/             # SMBIOS 2.x/3.x table discovery, structure parsers & SMBIOS protocol
│       ├── smbus/              # SMBus Host Controller protocol
│       ├── spi/                # SPI host, master, and device protocols
│       ├── storage/            # Block I/O, Disk I/O, security, erase, disk info, RAM disk, SD/MMC, FAT
│       └── usb/                # USB I/O, XHCI, EHCI, HID descriptors & mass storage
└── tests/                      # 268 native integration test suites
```

</details>

---

## Acknowledgements

`rover-uefi` is independently developed from the ground up in native Rust, following industry standards:

- [UEFI Specification](https://uefi.org/specifications)
- [UEFI PI Specification](https://uefi.org/specifications)
- [ACPI Specification](https://uefi.org/specifications)
- [SMBIOS Specification](https://www.dmtf.org/standards/smbios)
- [TPM 2.0 Specifications](https://trustedcomputinggroup.org/resource/tpm-library-specification/)
- [PCI Specifications](https://pcisig.com/specifications)

We acknowledge [TianoCore EDK II](https://github.com/tianocore/edk2) as a technical reference for firmware architecture, protocols, and interfaces.

---

## Contributing

Contributions are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) guide for details on our development workflow, coding conventions, `#![no_std]` constraints, and pull request process.

---

## License

This project is licensed under the **Apache License 2.0** (`Apache-2.0`) - see the [LICENSE](LICENSE) file for details.
