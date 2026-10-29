<div align="center">

<a href="https://github.com/muhammad-fiaz/rover"><img src="https://img.shields.io/badge/Rust-2024%20Edition-orange.svg?logo=rust" alt="Rust Edition"></a>
<a href="https://github.com/muhammad-fiaz/rover"><img src="https://img.shields.io/github/stars/muhammad-fiaz/rover" alt="GitHub stars"></a>
<a href="https://github.com/muhammad-fiaz/rover/issues"><img src="https://img.shields.io/github/issues/muhammad-fiaz/rover" alt="GitHub issues"></a>
<a href="https://github.com/muhammad-fiaz/rover/pulls"><img src="https://img.shields.io/github/issues-pr/muhammad-fiaz/rover" alt="GitHub pull requests"></a>
<a href="https://github.com/muhammad-fiaz/rover"><img src="https://img.shields.io/github/last-commit/muhammad-fiaz/rover" alt="GitHub last commit"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache--2.0-blue.svg" alt="License"></a>
<a href="https://github.com/muhammad-fiaz/rover/actions"><img src="https://img.shields.io/github/actions/workflow/status/muhammad-fiaz/rover/ci.yml?branch=main&label=CI" alt="CI"></a>
<img src="https://img.shields.io/badge/platforms-x86__64--uefi%20%7C%20aarch64--uefi%20%7C%20i686--uefi-blue" alt="Supported Platforms">
<img src="https://img.shields.io/badge/dependencies-0%20(zero)-brightgreen" alt="Zero Dependencies">
<a href="https://pay.muhammadfiaz.com"><img src="https://img.shields.io/badge/Sponsor-pay.muhammadfiaz.com-ff69b4?style=flat&logo=heart" alt="Sponsor"></a>
<a href="https://github.com/sponsors/muhammad-fiaz"><img src="https://img.shields.io/badge/Sponsor-GitHub-pink?style=social&logo=github" alt="GitHub Sponsors"></a>
<a href="https://hits.sh/muhammad-fiaz/rover/"><img src="https://hits.sh/muhammad-fiaz/rover.svg?label=Visitors&extraCount=0&color=green" alt="Repo Visitors"></a>

<p><em>A modern, feature-rich, cross-platform firmware development library for UEFI and PI specifications in Rust.</em></p>

<b><a href="#features">Features</a> |
<a href="#prerequisites-and-supported-platforms">Supported Platforms</a> |
<a href="#quick-start">Quick Start</a> |
<a href="#project-structure">Project Structure</a> |
<a href="#acknowledgements">Acknowledgements</a> |
<a href="#contributing">Contributing</a> |
<a href="SECURITY.md">Security</a></b>

</div>

`rover-uefi` is a modern, feature-rich, cross-platform firmware development library for the UEFI and Platform Initialization (PI) specifications in Rust. It provides spec-accurate ABIs, system tables, Boot and Runtime Services, 60+ protocols, native cryptography, and bounded parsers for cross-platform pre-boot environments.

> [!TIP]
> If you build with `rover-uefi`, make sure give it a star ⭐ on Github!

---

<details>
<summary><strong>Features</strong> (click to expand)</summary>

| Feature Category | Description |
|---|---|
| **Core ABI & Tables** | `extern "efiapi"` calling convention, primitive types (`Char16`, `UIntN`, `EfiGuid`, `EfiHandle`, `EfiStatus`, `EfiTime`), `EfiSystemTable`, `EfiBootServices`, `EfiRuntimeServices`, `EfiConfigurationTable`, and standard table headers. |
| **Boot Services** | Memory allocation (`allocate_pages`, `allocate_pool`), handle databases, protocol locate/open/install, image load/start/unload, event/timer subsystems, watchdog, memory map parsing, and `exit_boot_services`. |
| **Runtime Services** | Variable services (`get_variable`, `set_variable`, `get_next_variable_name`), time and wakeup alarms, virtual memory mapping (`set_virtual_address_map`), hardware reset systems, and capsule update queries. |
| **Device Paths** | Device path nodes and instances (Hardware, ACPI, Messaging, Media, BBS, End), node iteration, validation, and Device Path Utilities (`EFI_DEVICE_PATH_UTILITIES_PROTOCOL`). |
| **Hardware Architecture** | x86/x86_64 Local APIC, I/O APIC, and structured CPUID feature trees; ARM64 GICv2/v3 distributor/CPU interfaces, system registers, and exception vectors; RISC-V contexts and SBI extensions (Base, Timer, IPI, HSM, SRST, PMU, DBCN); common MMIO, DMA descriptors, and system timers. |
| **PCI Subsystem** | `EfiPciIoProtocol`, `EfiPciRootBridgeIoProtocol`, Type 0/1 header decoding, BAR resource allocation, capability list traversal, and PCIe configuration space access. |
| **USB Subsystem** | USB Host Controller interfaces (`EFI_USB2_HC_PROTOCOL`, `EFI_USB3_HC_PROTOCOL`, EHCI, XHCI TRBs/rings), USB I/O Protocol, standard descriptors, HID report parser, and USB Mass Storage (Bulk-Only Transport & SCSI). |
| **Storage & Filesystems** | Block I/O, Disk I/O, Simple File System (`EFI_FILE_PROTOCOL`), Partition discovery (MBR, GPT, protective MBR, EBR), `#![no_std]` FAT12/16/32 reader with LFN, and ATA/NVMe/SCSI pass-thru protocols. |
| **Cryptography & Security** | Constant-time AES-128/256, SHA-1/224/256/384/512, SHA-3, RSA PKCS#1 v1.5 verification, X.509 ASN.1 certificate decoding, Authenticode PE signature verification, Secure Boot key databases (`PK`, `KEK`, `db`, `dbx`), RNG Protocol, and Device Security Protocols. |
| **TCG / TPM** | TPM 1.2 and TPM 2.0 protocols (`EfiTcg2Protocol`), TCG event log parsing, SHA-1/SHA-256 PCR bank operations, startup commands, and measurements. |
| **ACPI & SMBIOS** | Full ACPI table generators and bounded parsers (RSDP, RSDT, XSDT, FADT, MADT, MCFG, HPET, DMAR, SRAT, SLIT, SPCR, BGRT, FACS, DSDT, SSDT) and SMBIOS 2.x/3.x entry points and structured record parsers. |
| **Network Protocols** | SNP (Simple Network), MNP, ARP, VLAN Config, IPv4, IPv6, ICMP, IGMP, NDP, UDPv4/v6, TCPv4/v6, TLS, DHCPv4/v6, DNS, HTTP, REST EX, Redfish Discover, and PXE Base Code. |
| **Firmware & PI Formats** | Universal Payload (`UNIVERSAL_PAYLOAD_IDENTIFIER`, `'PLDH'`), Intel FSP 2.x headers (`FspInfoHeader`, `'FSPH'`), Platform Runtime Mechanism (PRM), Hand-Off Blocks (HOBs), Firmware Volume/FMP, HII/IFR, DEPEX, PE32/PE32+/TE headers, and Standalone/Traditional MM communication (`EfiMmCommunication2Protocol`). |
| **Manageability** | IPMI Protocol & KCS register interface, MCTP packet/message dispatch, and PLDM Protocol with base and platform monitoring commands. |
| **Drivers & Binding** | UEFI Driver Binding Protocol (`EFI_DRIVER_BINDING_PROTOCOL`), Component Name Protocols, Loaded Image Protocol, ARM SCMI Protocol, VirtIO Device Protocol, and native driver lifecycle registries. |
| **Console & Serial** | Simple Text Input/Output, Simple Pointer & Absolute Pointer protocols, Graphics Output Protocol (GOP) with BLT engine, Serial I/O Protocol (`EFI_SERIAL_IO_PROTOCOL`), and UEFI Shell / Shell Parameters protocols. |

</details>

---

<details>
<summary><strong>Prerequisites and Supported Platforms</strong> (click to expand)</summary>

<br>

## Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| **Rust Toolchain** | **Nightly or Stable 2024 Edition (1.85+)** | Compatible with standard Rust installations |
| **UEFI Targets** | Target support via `rustup target add` | Bare-metal firmware cross-compilation |

---

## Supported Targets

| Architecture | Target Triple | Support Tier |
|---|---|---|
| **x86_64 (64-bit)** | `x86_64-unknown-uefi` | Tier 1 (Fully Verified) |
| **AArch64 (ARM64)** | `aarch64-unknown-uefi` | Tier 1 (Fully Verified) |
| **x86 (32-bit)** | `i686-unknown-uefi` | Tier 1 (Fully Verified) |
| **Host Test Runner** | `x86_64-pc-windows-msvc` / `x86_64-unknown-linux-gnu` | Full Integration Testing |

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

Add `rover-uefi` to your `Cargo.toml`:

```toml
[dependencies]
rover-uefi = { path = "boot/rover-uefi" }
```

Or from git:

```toml
[dependencies]
rover-uefi = { git = "https://github.com/muhammad-fiaz/rover.git", package = "rover-uefi" }
```

---

## Quick Start

### 1. Minimal UEFI Application Entry Point

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rover_uefi::abi::Status;
use rover_uefi::base::Handle;
use rover_uefi::tables::SystemTable;

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(image_handle: Handle, system_table: *const SystemTable) -> Status {
    if system_table.is_null() {
        return Status::INVALID_PARAMETER;
    }

    let st = unsafe { &*system_table };
    
    // Validate table header signature
    if !st.header.is_valid() {
        return Status::UNSUPPORTED;
    }

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

### 2. Boot Services Protocol Discovery

```rust
use rover_uefi::base::{Guid, Handle};
use rover_uefi::protocols::graphics::GopProtocol;
use rover_uefi::tables::BootServices;

pub fn locate_gop(bs: &BootServices) -> Option<*mut GopProtocol> {
    let mut interface = core::ptr::null_mut();
    let status = unsafe {
        (bs.locate_protocol)(
            &raw const GopProtocol::GUID,
            core::ptr::null_mut(),
            &raw mut interface,
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
use rover_uefi::acpi::{Rsdp, Xsdt};

pub fn find_table_in_rsdp<'a>(rsdp_bytes: &'a [u8], signature: &[u8; 4]) -> Option<&'a [u8]> {
    let rsdp = Rsdp::parse(rsdp_bytes).ok()?;
    let xsdt_addr = rsdp.xsdt_address();
    
    // Safe bounded inspection of memory tables
    None
}
```

---

## Validation Matrix

Run the comprehensive test suite and linters locally:

```bash
# Run unit and integration tests (114 suites, 610+ tests)
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

---

<details>
<summary><strong>Project Structure</strong> (click to expand)</summary>

```text
rover-uefi/
├── .gitignore                  # Git exclusion rules
├── Cargo.lock                  # Dependency lockfile
├── Cargo.toml                  # Package manifest, crates.io categories & strict lints
├── CONTRIBUTING.md             # Contribution guidelines & development workflow
├── LICENSE                     # Apache-2.0 license text
├── README.md                   # Project documentation
├── SECURITY.md                 # Security & vulnerability reporting policy
├── src/
│   ├── lib.rs                  # Crate root, no_std configuration & top-level exports
│   ├── abi/                    # Primitive types, calling conventions & status codes
│   ├── arch/                   # Hardware architecture support (x86, ARM64, RISC-V)
│   │   ├── arm64/              # ARM64 exception contexts & GICv2/v3 support
│   │   ├── common/             # MMIO, DMA descriptors, and system timers
│   │   ├── riscv/              # RISC-V contexts & SBI interfaces
│   │   └── x86/                # APIC, I/O APIC, CPUID, and context handling
│   ├── base/                   # GUIDs, handles, statuses, events, CRC32, drivers & bounded byte buffers
│   ├── tables/                 # System, Boot, Runtime, and Configuration tables
│   ├── services/               # Boot Services and Runtime Services wrappers
│   └── protocols/              # UEFI & PI protocol interfaces
│       ├── acpi/               # ACPI table discovery, parsing & validation
│       ├── console/            # Text I/O, absolute pointer, and shell protocols
│       ├── core/               # Handle database & protocol opening/closing
│       ├── debug/              # Debug support & port communication
│       ├── device_path/        # Device path nodes, instances & utilities
│       ├── diagnostic/         # Status codes & timer architectures
│       ├── driver/             # Driver binding, loaded images, SCMI, VirtIO
│       ├── firmware/           # Universal Payload, Intel FSP 2.x, PRM, MM/SMM, HOBs
│       ├── graphics/           # Graphics Output Protocol (GOP), pixel formats, BLT
│       ├── manageability/      # IPMI, MCTP, and PLDM protocols
│       ├── network/            # SNP, MNP, ARP, VLAN, IPv4/v6, TCP, UDP, DHCP, DNS, HTTP, Redfish
│       ├── pci/                # PCI I/O, root bridge & configuration spaces
│       ├── security/           # AES, SHA, RSA, X.509, TPM 1.2/2.0, Secure Boot
│       ├── serial/             # Serial I/O protocols
│       ├── smbios/             # SMBIOS 2.x/3.x table discovery & structure parsers
│       ├── storage/            # Block I/O, Disk I/O, MBR/GPT partitions, FAT filesystem
│       └── usb/                # USB I/O, XHCI, EHCI, HID descriptors & mass storage
└── tests/                      # 114 native integration test suites
```

</details>

---

## Acknowledgements

`rover-uefi` was developed from the ground up in native Rust to provide a modern, memory-safe firmware foundation for operating systems, bootloaders, and bare-metal environments.

We gratefully acknowledge the **[TianoCore EDK II](https://github.com/tianocore/edk2/)** project as a foundational engineering reference for UEFI and PI specifications, table layouts, and GUID definitions.

---

## Contributing

Contributions are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) guide for details on our development workflow, coding conventions, `#![no_std]` constraints, and pull request process.

---

## License

This project is licensed under the **Apache License 2.0** (`Apache-2.0`) - see the [LICENSE](LICENSE) file for details.
