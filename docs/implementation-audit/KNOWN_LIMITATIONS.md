# Known Limitations & Hardware Firmware Boundaries

`rover-uefi` is designed as a `#![no_std]` native Rust library providing full binary compatibility with UEFI and PI specifications.

The following elements reside outside the scope of a static Rust firmware library and require external platform firmware or execution context:

1. **Host-Specific Platform Silicon Initialization**:
   - Board-specific DDR memory training (MRC / FSP-M) is proprietary to silicon vendors (Intel FSP, AMD AGESA) and is consumed via standard FSP / UPL / PEI handover interfaces.
2. **Microcode Updates**:
   - Binary processor microcode patch applications are platform/silicon-dependent blobs dispatched via FSP or vendor-specific PEI/SEC code.
3. **Hardware Execution Context**:
   - Certain CPU instructions (`CPUID`, `wrmsr`, `rdmsr`, `in`/`out`) require appropriate CPU privilege levels (Ring 0 / EL1 / M-mode) and target-specific processor architectures.
