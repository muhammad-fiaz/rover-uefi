# Architectural Support and Validation

`rover-uefi` targets `#![no_std]` bare-metal firmware across all major architectures supported by the UEFI and PI specifications.

## Architecture Status Matrix

| Architecture | Target Triple / ABI | Context & Regs | Platform Peripherals / Timers | Status |
|---|---|---|---|---|
| **x86_64** | `x86_64-unknown-uefi` | Full registers, CPUID trees | Local APIC, I/O APIC, TSC deadline | Primary Tier 1 |
| **AArch64** | `aarch64-unknown-uefi` | Exception context & vector table | GICv2, GICv3, Arch Generic Timer | Primary Tier 1 |
| **IA-32 (x86)** | `i686-unknown-uefi` | 32-bit registers, CPUID | Local APIC, legacy PIC | Supported |
| **ARM32 (ARMv7-A)** | `armv7a-none-eabi` | Core CPSR & banking context | GICv2 distributor & interface | Supported |
| **RISC-V (RV64)** | `riscv64gc-unknown-none-elf` | Trap context, `scause`, CSRs | SBI Base, Timer, IPI, HSM, DBCN | Supported |
| **LoongArch64** | `loongarch64-unknown-none` | Exception context, ESTAT/ECFG | CSR timer & interrupts | Supported |

All architectural exception context layouts match the exact byte layout required by `EFI_SYSTEM_CONTEXT_*` unions defined in UEFI Specification Section 35.
