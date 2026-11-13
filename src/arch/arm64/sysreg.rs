//! # ARM64 Architecture System Registers
//!
//! Provides structures, bitmasks, and accessors for core ARMv8-A system registers
//! governing execution level, interrupt masks, processor configuration, identity,
//! floating point / SIMD enablement, and vector bases.

/// Current Exception Level (`CurrentEL`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurrentEl {
    /// Exception Level 0 (User mode).
    El0,
    /// Exception Level 1 (Firmware / Operating System supervisor mode).
    El1,
    /// Exception Level 2 (Hypervisor mode).
    El2,
    /// Exception Level 3 (Secure Monitor mode).
    El3,
    /// Unknown or reserved exception level encoding.
    Unknown(u8),
}

impl CurrentEl {
    /// Decodes the raw `CurrentEL` system register value (bits 2..3).
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        match ((raw >> 2) & 0x03) as u8 {
            0 => Self::El0,
            1 => Self::El1,
            2 => Self::El2,
            3 => Self::El3,
            other => Self::Unknown(other),
        }
    }

    /// Returns the integer exception level (0, 1, 2, or 3).
    #[must_use]
    pub const fn level(self) -> u8 {
        match self {
            Self::El0 => 0,
            Self::El1 => 1,
            Self::El2 => 2,
            Self::El3 => 3,
            Self::Unknown(x) => x,
        }
    }
}

/// Bitflags representing the processor interrupt and exception mask state (`DAIF`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Daif(pub u64);

impl Daif {
    /// Debug exception mask bit (`D`, bit 9).
    pub const DEBUG_MASK: Self = Self(1 << 9);
    /// `SError` interrupt mask bit (`A`, bit 8).
    pub const SERROR_MASK: Self = Self(1 << 8);
    /// IRQ interrupt mask bit (`I`, bit 7).
    pub const IRQ_MASK: Self = Self(1 << 7);
    /// FIQ interrupt mask bit (`F`, bit 6).
    pub const FIQ_MASK: Self = Self(1 << 6);

    /// Combines two sets of DAIF mask flags.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Checks if a specific mask flag is asserted.
    #[must_use]
    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Returns `true` if IRQs are unmasked (enabled).
    #[must_use]
    pub const fn are_irqs_enabled(self) -> bool {
        (self.0 & Self::IRQ_MASK.0) == 0
    }

    /// Returns `true` if FIQs are unmasked (enabled).
    #[must_use]
    pub const fn are_fiqs_enabled(self) -> bool {
        (self.0 & Self::FIQ_MASK.0) == 0
    }

    /// Returns `true` if `SError` aborts are unmasked (enabled).
    #[must_use]
    pub const fn are_serrors_enabled(self) -> bool {
        (self.0 & Self::SERROR_MASK.0) == 0
    }
}

/// System Control Register EL1 (`SCTLR_EL1`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SctlrEl1(pub u64);

impl SctlrEl1 {
    /// MMU enable for EL1 and EL0 stage 1 address translation (`M`, bit 0).
    pub const MMU_ENABLE: Self = Self(1 << 0);
    /// Strict data alignment check enable (`A`, bit 1).
    pub const ALIGNMENT_CHECK: Self = Self(1 << 1);
    /// Stage 1 data and unified cacheability enable (`C`, bit 2).
    pub const DATA_CACHE_ENABLE: Self = Self(1 << 2);
    /// Stack Pointer Alignment check enable for EL1 (`SA`, bit 3).
    pub const SP_ALIGNMENT_CHECK: Self = Self(1 << 3);
    /// Stack Pointer Alignment check enable for EL0 (`SA0`, bit 4).
    pub const SP0_ALIGNMENT_CHECK: Self = Self(1 << 4);
    /// Stage 1 instruction cacheability enable (`I`, bit 12).
    pub const INSTRUCTION_CACHE_ENABLE: Self = Self(1 << 12);
    /// Write permission implies Execute-Never (`WXN`, bit 19).
    pub const WRITE_EXECUTE_NEVER: Self = Self(1 << 19);

    /// Checks if a specific flag is set.
    #[must_use]
    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Combines two sets of flags.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Multiprocessor Affinity Register (`MPIDR_EL1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MpidrEl1 {
    /// Affinity Level 0 (typically core ID within a cluster).
    pub aff0: u8,
    /// Affinity Level 1 (typically cluster ID).
    pub aff1: u8,
    /// Affinity Level 2 (typically socket ID).
    pub aff2: u8,
    /// Affinity Level 3 (system level grouping).
    pub aff3: u8,
    /// Multi-threading implementation flag (`MT`, bit 24).
    pub is_multithreaded: bool,
    /// Uniprocessor system flag (`U`, bit 30).
    pub is_uniprocessor: bool,
}

impl MpidrEl1 {
    /// Decodes a raw `MPIDR_EL1` register value.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self {
            aff0: (raw & 0xFF) as u8,
            aff1: ((raw >> 8) & 0xFF) as u8,
            aff2: ((raw >> 16) & 0xFF) as u8,
            aff3: ((raw >> 32) & 0xFF) as u8,
            is_multithreaded: (raw & (1 << 24)) != 0,
            is_uniprocessor: (raw & (1 << 30)) != 0,
        }
    }

    /// Returns the linear composite core identifier `(aff3 << 24) | (aff2 << 16) | (aff1 << 8) | aff0`.
    #[must_use]
    pub const fn linear_core_id(&self) -> u64 {
        ((self.aff3 as u64) << 24)
            | ((self.aff2 as u64) << 16)
            | ((self.aff1 as u64) << 8)
            | (self.aff0 as u64)
    }
}

/// Main ID Register (`MIDR_EL1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MidrEl1 {
    /// Processor revision.
    pub revision: u8,
    /// Primary part number.
    pub part_num: u16,
    /// Architecture code.
    pub architecture: u8,
    /// Variant number.
    pub variant: u8,
    /// ASCII implementer code (e.g. 0x41 for 'A' / ARM).
    pub implementer: u8,
}

impl MidrEl1 {
    /// Decodes a raw `MIDR_EL1` register value.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self {
            revision: (raw & 0x0F) as u8,
            part_num: ((raw >> 4) & 0x0FFF) as u16,
            architecture: ((raw >> 16) & 0x0F) as u8,
            variant: ((raw >> 20) & 0x0F) as u8,
            implementer: ((raw >> 24) & 0xFF) as u8,
        }
    }
}

/// Architectural Feature Access Control Register (`CPACR_EL1`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CpacrEl1(pub u64);

impl CpacrEl1 {
    /// Enables non-trapping Floating Point and Advanced SIMD instructions at EL0 and EL1 (bits 20..21 = `0b11`).
    pub const FP_SIMD_ENABLE: Self = Self(0b11 << 20);

    /// Checks if a specific flag is set.
    #[must_use]
    pub const fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }
}

/// Reads the current exception level from `CurrentEL`.
#[must_use]
pub fn read_current_el() -> CurrentEl {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, CurrentEL", out(reg) raw, options(nomem, nostack, preserves_flags));
        CurrentEl::from_raw(raw)
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    CurrentEl::El1
}

/// Reads the current processor interrupt mask flags (`DAIF`).
#[must_use]
pub fn read_daif() -> Daif {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, daif", out(reg) raw, options(nomem, nostack, preserves_flags));
        Daif(raw)
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    Daif(0)
}

/// Reads `SCTLR_EL1`.
#[must_use]
pub fn read_sctlr_el1() -> SctlrEl1 {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, sctlr_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        SctlrEl1(raw)
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    SctlrEl1(0)
}

/// Writes `SCTLR_EL1`.
///
/// # Safety
///
/// Modifying `SCTLR_EL1` changes MMU and caching state.
pub unsafe fn write_sctlr_el1(val: SctlrEl1) {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("msr sctlr_el1, {}", in(reg) val.0, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    let _ = val;
}

/// Reads `MPIDR_EL1`.
#[must_use]
pub fn read_mpidr_el1() -> MpidrEl1 {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, mpidr_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        MpidrEl1::from_raw(raw)
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    MpidrEl1 {
        aff0: 0,
        aff1: 0,
        aff2: 0,
        aff3: 0,
        is_multithreaded: false,
        is_uniprocessor: true,
    }
}

/// Reads `MIDR_EL1`.
#[must_use]
pub fn read_midr_el1() -> MidrEl1 {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, midr_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        MidrEl1::from_raw(raw)
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    MidrEl1 {
        revision: 0,
        part_num: 0,
        architecture: 0,
        variant: 0,
        implementer: 0x41,
    }
}

/// Reads `CPACR_EL1`.
#[must_use]
pub fn read_cpacr_el1() -> CpacrEl1 {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, cpacr_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        CpacrEl1(raw)
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    CpacrEl1(0)
}

/// Writes `CPACR_EL1`.
///
/// # Safety
///
/// Modifying `CPACR_EL1` alters architectural feature access traps.
pub unsafe fn write_cpacr_el1(val: CpacrEl1) {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("msr cpacr_el1, {}", in(reg) val.0, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    let _ = val;
}

/// Enables Floating Point and Advanced SIMD instructions by configuring `CPACR_EL1.FPEN = 0b11`.
pub fn enable_fp_simd() {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let mut raw: u64;
        core::arch::asm!("mrs {}, cpacr_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        raw |= CpacrEl1::FP_SIMD_ENABLE.0;
        core::arch::asm!("msr cpacr_el1, {}", in(reg) raw, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
}

/// Reads the Exception Vector Base Address Register (`VBAR_EL1`).
#[must_use]
pub fn read_vbar_el1() -> u64 {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, vbar_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        raw
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    0
}

/// Writes the Exception Vector Base Address Register (`VBAR_EL1`).
///
/// # Safety
///
/// Modifying the vector base address alters the processor exception dispatch table.
pub unsafe fn write_vbar_el1(base: u64) {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("msr vbar_el1, {}", in(reg) base, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
    let _ = base;
}
