//! # `LoongArch` Control Status Registers (CSRs)
//!
//! Provides architectural Control Status Register numbers and bitfield helpers
//! for `LoongArch` 64-bit processor configuration, privilege levels, and address translation.

/// Current Mode Information (`CRMD`).
pub const CSR_CRMD: u32 = 0x00;
/// Previous Mode Information (`PRMD`).
pub const CSR_PRMD: u32 = 0x01;
/// Extended User Enable (`EUEN`).
pub const CSR_EUEN: u32 = 0x02;
/// Exception Configuration (`ECFG`).
pub const CSR_ECFG: u32 = 0x04;
/// Exception Status (`ESTAT`).
pub const CSR_ESTAT: u32 = 0x05;
/// Exception Return Address (`ERA`).
pub const CSR_ERA: u32 = 0x06;
/// Bad Virtual Address (`BADV`).
pub const CSR_BADV: u32 = 0x07;
/// Bad Instruction (`BADI`).
pub const CSR_BADI: u32 = 0x08;
/// Exception Entry Base Address (`EENTRY`).
pub const CSR_EENTRY: u32 = 0x0C;
/// TLB Index (`TLBIDX`).
pub const CSR_TLBIDX: u32 = 0x10;
/// TLB Entry High (`TLBEHI`).
pub const CSR_TLBEHI: u32 = 0x11;
/// TLB Entry Low 0 (`TLBELO0`).
pub const CSR_TLBELO0: u32 = 0x12;
/// TLB Entry Low 1 (`TLBELO1`).
pub const CSR_TLBELO1: u32 = 0x13;
/// Address Space Identifier (`ASID`).
pub const CSR_ASID: u32 = 0x18;
/// Page Global Directory Base Low (`PGDL`).
pub const CSR_PGDL: u32 = 0x19;
/// Page Global Directory Base High (`PGDH`).
pub const CSR_PGDH: u32 = 0x1A;
/// Page Global Directory Base (`PGD`).
pub const CSR_PGD: u32 = 0x1B;

/// Current Mode Information (`CRMD`) bitflags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Crmd(pub u64);

impl Crmd {
    /// Privilege Level mask (`PLV`, bits 0..1): 0 = Highest (Kernel/Firmware), 3 = Lowest (User).
    pub const PLV_MASK: u64 = 0x03;
    /// Globally enables hardware interrupts (`IE`, bit 2).
    pub const INTERRUPT_ENABLE: Self = Self(1 << 2);
    /// Alias for `INTERRUPT_ENABLE`.
    pub const IE: Self = Self::INTERRUPT_ENABLE;
    /// Direct Address Translation mode (`DA`, bit 3): addresses bypass MMU paging when set.
    pub const DIRECT_ADDRESSING: Self = Self(1 << 3);
    /// Alias for `DIRECT_ADDRESSING`.
    pub const DA: Self = Self::DIRECT_ADDRESSING;
    /// Page-based MMU Translation mode (`PG`, bit 4).
    pub const PAGING_ENABLE: Self = Self(1 << 4);
    /// Alias for `PAGING_ENABLE`.
    pub const PG: Self = Self::PAGING_ENABLE;

    /// Returns the current Privilege Level (`PLV`, 0..3).
    #[must_use]
    pub const fn privilege_level(self) -> u8 {
        (self.0 & Self::PLV_MASK) as u8
    }

    /// Returns `true` if interrupts are globally enabled.
    #[must_use]
    pub const fn are_interrupts_enabled(self) -> bool {
        (self.0 & Self::INTERRUPT_ENABLE.0) != 0
    }

    /// Alias for `are_interrupts_enabled`.
    #[must_use]
    pub const fn is_interrupts_enabled(self) -> bool {
        self.are_interrupts_enabled()
    }

    /// Returns `true` if page-based address translation is enabled.
    #[must_use]
    pub const fn is_paging_enabled(self) -> bool {
        self.contains(Self::PAGING_ENABLE)
    }

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

impl core::ops::BitOr for Crmd {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for Crmd {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for Crmd {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign for Crmd {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Reads a `LoongArch` Control Status Register (`csrrd`).
#[must_use]
pub fn csr_read(csr: u32) -> u64 {
    #[cfg(target_arch = "loongarch64")]
    unsafe {
        let val: u64;
        core::arch::asm!("csrrd {0}, {1}", out(reg) val, in(reg) csr, options(nomem, nostack, preserves_flags));
        val
    }
    #[cfg(not(target_arch = "loongarch64"))]
    {
        let _ = csr;
        0
    }
}

/// Writes a `LoongArch` Control Status Register (`csrwr`).
///
/// # Safety
///
/// Writing invalid values into CSRs can cause hardware faults or disable address translation.
pub unsafe fn csr_write(csr: u32, val: u64) {
    #[cfg(target_arch = "loongarch64")]
    unsafe {
        core::arch::asm!("csrwr {0}, {1}", in(reg) val, in(reg) csr, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "loongarch64"))]
    {
        let _ = (csr, val);
    }
}
