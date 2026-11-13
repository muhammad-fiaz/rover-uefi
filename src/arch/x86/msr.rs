//! # x86 Model-Specific Registers (MSRs)
//!
//! Provides architectural MSR addresses, typed bitflag structures for extended features
//! (such as `EFER`, `PAT`, `APIC_BASE`), and safe reading/writing helpers.

/// Extended Feature Enable Register (`IA32_EFER`).
pub const IA32_EFER: u32 = 0xC000_0080;
/// Legacy SYSCALL target address / segments (`IA32_STAR`).
pub const IA32_STAR: u32 = 0xC000_0081;
/// Long Mode SYSCALL target instruction pointer (`IA32_LSTAR`).
pub const IA32_LSTAR: u32 = 0xC000_0082;
/// Compatibility mode SYSCALL target instruction pointer (`IA32_CSTAR`).
pub const IA32_CSTAR: u32 = 0xC000_0083;
/// SYSCALL RFLAGS mask (`IA32_FMASK`).
pub const IA32_FMASK: u32 = 0xC000_0084;
/// FS segment base address (`IA32_FS_BASE`).
pub const IA32_FS_BASE: u32 = 0xC000_0100;
/// GS segment base address (`IA32_GS_BASE`).
pub const IA32_GS_BASE: u32 = 0xC000_0101;
/// Kernel GS segment base address for `SWAPGS` (`IA32_KERNEL_GS_BASE`).
pub const IA32_KERNEL_GS_BASE: u32 = 0xC000_0102;
/// TSC Auxiliary register for `RDTSCP` (`IA32_TSC_AUX`).
pub const IA32_TSC_AUX: u32 = 0xC000_0103;

/// Time Stamp Counter (`IA32_TIME_STAMP_COUNTER`).
pub const IA32_TSC: u32 = 0x0000_0010;
/// APIC Base address and enable flags (`IA32_APIC_BASE`).
pub const IA32_APIC_BASE: u32 = 0x0000_001B;
/// TSC Adjust register (`IA32_TSC_ADJUST`).
pub const IA32_TSC_ADJUST: u32 = 0x0000_003B;
/// MTRR Capabilities (`IA32_MTRR_CAP`).
pub const IA32_MTRR_CAP: u32 = 0x0000_00FE;
/// SYSENTER CS target (`IA32_SYSENTER_CS`).
pub const IA32_SYSENTER_CS: u32 = 0x0000_0174;
/// SYSENTER ESP target (`IA32_SYSENTER_ESP`).
pub const IA32_SYSENTER_ESP: u32 = 0x0000_0175;
/// SYSENTER EIP target (`IA32_SYSENTER_EIP`).
pub const IA32_SYSENTER_EIP: u32 = 0x0000_0176;
/// Page Attribute Table register (`IA32_PAT`).
pub const IA32_PAT: u32 = 0x0000_0277;
/// MTRR Default Type register (`IA32_MTRR_DEF_TYPE`).
pub const IA32_MTRR_DEF_TYPE: u32 = 0x0000_02FF;
/// TSC Deadline counter (`IA32_TSC_DEADLINE`).
pub const IA32_TSC_DEADLINE: u32 = 0x0000_06E0;

/// Bitflags for the Extended Feature Enable Register (`IA32_EFER`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EferFlags(pub u64);

impl EferFlags {
    /// System Call Extensions (`SCE`): enables `SYSCALL` and `SYSRET`.
    pub const SYSTEM_CALL_EXTENSIONS: Self = Self(1 << 0);
    /// Long Mode Enable (`LME`).
    pub const LONG_MODE_ENABLE: Self = Self(1 << 8);
    /// Long Mode Active (`LMA`): read-only, indicates processor is running in 64-bit mode.
    pub const LONG_MODE_ACTIVE: Self = Self(1 << 10);
    /// No-Execute Enable (`NXE`): enables page table entry bit 63 (`XD`/`NX`).
    pub const NO_EXECUTE_ENABLE: Self = Self(1 << 11);
    /// Secure Virtual Machine Enable (`SVME`).
    pub const SECURE_VIRTUAL_MACHINE_ENABLE: Self = Self(1 << 12);
    /// Long Mode Segment Limit Enable (`LMSLE`).
    pub const LONG_MODE_SEGMENT_LIMIT_ENABLE: Self = Self(1 << 13);
    /// Fast FXSAVE/FXRSTOR (`FFXSR`).
    pub const FAST_FXSAVE_FXRSTOR: Self = Self(1 << 14);
    /// Translation Cache Extension (`TCE`).
    pub const TRANSLATION_CACHE_EXTENSION: Self = Self(1 << 15);

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

/// Decoded contents of `IA32_APIC_BASE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApicBase {
    /// Whether this core is the Bootstrap Processor (`BSP`).
    pub is_bsp: bool,
    /// Global APIC enable bit.
    pub is_global_enabled: bool,
    /// x2APIC mode enable bit.
    pub is_x2apic_enabled: bool,
    /// 4 KiB aligned physical base address of the Local APIC MMIO registers.
    pub base_address: u64,
}

impl ApicBase {
    const BSP_MASK: u64 = 1 << 8;
    const X2APIC_MASK: u64 = 1 << 10;
    const GLOBAL_ENABLE_MASK: u64 = 1 << 11;
    const ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;

    /// Decodes a raw 64-bit `IA32_APIC_BASE` value.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self {
            is_bsp: (raw & Self::BSP_MASK) != 0,
            is_x2apic_enabled: (raw & Self::X2APIC_MASK) != 0,
            is_global_enabled: (raw & Self::GLOBAL_ENABLE_MASK) != 0,
            base_address: raw & Self::ADDRESS_MASK,
        }
    }

    /// Encodes this descriptor into a raw 64-bit value suitable for `IA32_APIC_BASE`.
    #[must_use]
    pub const fn to_raw(self) -> u64 {
        let mut val = self.base_address & Self::ADDRESS_MASK;
        if self.is_bsp {
            val |= Self::BSP_MASK;
        }
        if self.is_x2apic_enabled {
            val |= Self::X2APIC_MASK;
        }
        if self.is_global_enabled {
            val |= Self::GLOBAL_ENABLE_MASK;
        }
        val
    }
}

/// Page Attribute Table (PAT) memory types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PatMemoryType {
    /// Uncacheable (`UC`).
    Uncacheable = 0x00,
    /// Write Combining (`WC`).
    WriteCombining = 0x01,
    /// Write Through (`WT`).
    WriteThrough = 0x04,
    /// Write Protected (`WP`).
    WriteProtected = 0x05,
    /// Write Back (`WB`).
    WriteBack = 0x06,
    /// Uncached (`UC-`).
    Uncached = 0x07,
}

impl PatMemoryType {
    /// Decodes a 3-bit value into a PAT memory type.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw & 0x07 {
            0x00 => Some(Self::Uncacheable),
            0x01 => Some(Self::WriteCombining),
            0x04 => Some(Self::WriteThrough),
            0x05 => Some(Self::WriteProtected),
            0x06 => Some(Self::WriteBack),
            0x07 => Some(Self::Uncached),
            _ => None,
        }
    }
}

/// Page Attribute Table register (`IA32_PAT`) containing 8 memory type entries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatRegister(pub [PatMemoryType; 8]);

impl PatRegister {
    /// Standard reset default PAT layout.
    pub const RESET_DEFAULT: Self = Self([
        PatMemoryType::WriteBack,
        PatMemoryType::WriteThrough,
        PatMemoryType::Uncached,
        PatMemoryType::Uncacheable,
        PatMemoryType::WriteBack,
        PatMemoryType::WriteThrough,
        PatMemoryType::Uncached,
        PatMemoryType::Uncacheable,
    ]);

    /// Decodes a raw 64-bit `IA32_PAT` MSR value.
    #[must_use]
    pub fn from_raw(raw: u64) -> Option<Self> {
        let mut entries = [PatMemoryType::Uncacheable; 8];
        let mut i = 0;
        while i < 8 {
            let byte = ((raw >> (i * 8)) & 0xFF) as u8;
            entries[i] = PatMemoryType::from_raw(byte)?;
            i += 1;
        }
        Some(Self(entries))
    }

    /// Encodes this PAT structure into a 64-bit value for `IA32_PAT`.
    #[must_use]
    pub fn to_raw(self) -> u64 {
        let mut val = 0u64;
        let mut i = 0;
        while i < 8 {
            val |= u64::from(self.0[i] as u8) << (i * 8);
            i += 1;
        }
        val
    }
}

/// Reads the 64-bit value of a Model-Specific Register (`rdmsr`).
///
/// # Safety
///
/// Reading unsupported or reserved MSR indices causes a general protection fault (#GP).
#[must_use]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let eax: u32;
        let edx: u32;
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") eax,
            out("edx") edx,
            options(nomem, nostack, preserves_flags)
        );
        (u64::from(edx) << 32) | u64::from(eax)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = msr;
        0
    }
}

/// Writes a 64-bit value into a Model-Specific Register (`wrmsr`).
///
/// # Safety
///
/// Writing to MSRs alters fundamental processor execution parameters and can cause faults.
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn wrmsr(msr: u32, value: u64) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let eax = value as u32;
        let edx = (value >> 32) as u32;
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") eax,
            in("edx") edx,
            options(nomem, nostack, preserves_flags)
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (msr, value);
    }
}

/// Reads the Time Stamp Counter (`rdtsc`).
#[must_use]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let eax: u32;
        let edx: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") eax,
            out("edx") edx,
            options(nomem, nostack, preserves_flags)
        );
        (u64::from(edx) << 32) | u64::from(eax)
    }
    #[cfg(not(target_arch = "x86_64"))]
    0
}

/// Reads the Time Stamp Counter and processor ID (`rdtscp`).
///
/// Returns `(tsc_cycles, processor_id)`.
#[must_use]
pub fn rdtscp() -> (u64, u32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let eax: u32;
        let edx: u32;
        let ecx: u32;
        core::arch::asm!(
            "rdtscp",
            out("eax") eax,
            out("edx") edx,
            out("ecx") ecx,
            options(nomem, nostack, preserves_flags)
        );
        ((u64::from(edx) << 32) | u64::from(eax), ecx)
    }
    #[cfg(not(target_arch = "x86_64"))]
    (0, 0)
}
