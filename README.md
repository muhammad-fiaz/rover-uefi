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
| **Core ABI & Tables** | `extern "efiapi"` calling convention, primitive types (`Char16`, `UIntN`, `EfiGuid`, `EfiHandle`, `EfiStatus`, `EfiTime`), `EfiSystemTable`, `EfiBootServices`, `EfiRuntimeServices`, `EfiConfigurationTable`, and table headers. |
| **Boot Services** | Allocation (`allocate_pages`, `allocate_pool`), handle databases, protocol locate/open/install, image load/start/unload, event/timer subsystems, watchdog, memory map parsing, and `exit_boot_services`. |
| **Runtime Services** | Variable services (`get_variable`, `set_variable`, `get_next_variable_name`), time/wakeup alarms, virtual memory mapping (`set_virtual_address_map`), hardware reset systems, capsule update queries. |
| **Management Mode (MM/SMM)** | Traditional and Standalone MM communication protocols (`EfiMmCommunication2Protocol`, `EFI_MM_COMMUNICATE2_HEADER`), MM CPU drivers, and secure buffer passing. |
| **Cryptography & Security** | Constant-time AES-128/256, SHA-1/224/256/384/512, RSA PKCS#1 v1.5 verification, X.509 certificate decoding, Authenticode signature verification, Secure Boot key databases (`PK`, `KEK`, `db`, `dbx`), and Device Security Protocol (`PCI`, `USB` identifiers). |
| **TCG / TPM** | TPM 1.2 and TPM 2.0 protocols (`EfiTcg2Protocol`), event log parsing, SHA-1/SHA-256 PCR bank operations, startup commands, and measurements. |
| **ACPI & SMBIOS** | Full ACPI table generators and bounded parsers (RSDP, RSDT, XSDT, FADT, MADT, MCFG, HPET, DMAR, SRAT, SLIT, SPCR, BGRT, FACS, DSDT, SSDT) and SMBIOS 2.x/3.x table parsers. |
| **Network Protocols** | SNP (Simple Network), MNP, ARP, VLAN Config, IPv4, IPv6, UDPv4/v6, TCPv4/v6, DHCPv4/v6, DNS, HTTP, REST EX Protocol, and Redfish Discover Protocol. |
| **Storage & Filesystems** | Block I/O, Disk I/O, Simple File System (`EFI_FILE_PROTOCOL`), Partition discovery (MBR, GPT, protective MBR), and pure `#![no_std]` FAT12/16/32 reader with LFN support. |
| **Firmware & Payload** | Universal Payload (`UNIVERSAL_PAYLOAD_IDENTIFIER`, `'PLDH'`, payload entry points), Intel FSP 2.x headers (`FspInfoHeader`, `'FSPH'`), and Platform Runtime Mechanism (PRM). |
| **Manageability** | IPMI Protocol & KCS registers, MCTP message dispatch, and PLDM Protocol with base and platform monitoring commands. |
| **Hardware & Architecture** | x86/x86_64 Local APIC, I/O APIC, 8259 PIC, 8254 PIT, RTC, CPUID, MSR, CR registers; ARM64 GICv2/v3, system registers, exception vector tables, and ARM SCMI (Base, Clock, Performance). |
| **Virtualization & Drivers** | VirtIO Device Protocol, VirtIO subsystem devices (Net, Block, GPU, Console, Entropy, FS), and Flattened Device Tree (FDT) parsing. |
| **Console & Shell** | Simple Text Input/Output, Graphics Output Protocol (GOP) & BLT engine, UEFI Shell Protocol (`EfiShellProtocol`), and Shell Parameters Protocol (`EfiShellParametersProtocol`). |

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
boot/rover-uefi/
├── Cargo.toml                  # Package manifest, crates.io categories & strict lints
├── README.md                   # Project documentation
├── LICENSE                     # GPL-3.0-only license text
├── src/
│   ├── lib.rs                  # Crate root, no_std configuration & top-level exports
│   ├── abi/                    # Primitive types, calling conventions & status codes
│   ├── arch/                   # Hardware architectures (x86/x86_64, ARM64, RISC-V)
│   │   ├── common/             # MMIO, DMA descriptors, timers
│   │   ├── x86/                # APIC, I/O APIC, PIC, PIT, RTC, CPUID, MSRs
│   │   └── arm/                # GICv2/v3/v5, system registers, exception vectors
│   ├── base/                   # Bounded byte readers/writers, CRC32, GUIDs, handles
│   ├── tables/                 # System, Boot, Runtime, and Configuration tables
│   ├── services/               # Boot Services and Runtime Services wrappers
│   ├── fs/                     # Filesystem parsers (FAT12/16/32, BPB, directory clusters)
│   ├── format/                 # Binary formats (PE32/PE32+, TE terse executables)
│   ├── fdt/                    # Flattened Device Tree (FDT) parsing & token scanner
│   └── protocols/              # UEFI & PI protocol interfaces
│       ├── console/            # Text I/O, Shell, Shell Parameters
│       ├── driver/             # Driver binding, SCMI, VirtIO
│       ├── firmware/           # Universal Payload, Intel FSP 2.x, PRM, MM/SMM
│       ├── graphics/           # Graphics Output Protocol (GOP), BLT operations
│       ├── manageability/      # IPMI, MCTP, PLDM protocols
│       ├── network/            # SNP, MNP, ARP, VLAN, IP, TCP, UDP, DHCP, DNS, HTTP, REST EX, Redfish
│       ├── pci/                # PCI I/O, root bridge, configuration space
│       ├── security/           # AES, SHA, RSA, X.509, TPM 1.2/2.0, Secure Boot, Device Security
│       ├── storage/            # Block I/O, Disk I/O, MBR/GPT partitions, Simple File System
│       └── usb/                # USB I/O, XHCI, EHCI, HID descriptors
└── tests/                      # 114 native integration test suites
```

</details>

---

## Acknowledgements

`rover-uefi` was written from scratch in native Rust to provide a secure, modern, memory-safe firmware foundation for operating systems and bare-metal environments.

During the architecture, protocol definition, and hardware interface design phases, the open-source **[TianoCore EDK II](https://github.com/tianocore/edk2/)** project served as a foundational engineering reference for UEFI and Platform Initialization (PI) specifications, table layouts, GUIDs, and industry standards. We acknowledge and thank the TianoCore EDK II maintainers and contributors for their long-standing contributions to the UEFI firmware ecosystem.

---

## Contributing

Contributions are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) guide for details on our development workflow, coding conventions, `#![no_std]` constraints, and pull request process.

---

## License

This project is licensed under the **Apache License 2.0** (`Apache-2.0`) - see the [LICENSE](LICENSE) file for details.
