//! # RISC-V Control and Status Registers (CSRs)
//!
//! Provides architectural Machine-mode and Supervisor-mode CSR numbers, bitflag
//! decoders for `sstatus`/`mstatus`, `satp` address translation register encoding,
//! and atomic CSR manipulation instructions.

/// Supervisor Status register (`sstatus`).
pub const CSR_SSTATUS: u32 = 0x100;
/// Supervisor Interrupt-Enable register (`sie`).
pub const CSR_SIE: u32 = 0x104;
/// Supervisor Trap-Vector Base Address register (`stvec`).
pub const CSR_STVEC: u32 = 0x105;
/// Supervisor Scratch register (`sscratch`).
pub const CSR_SSCRATCH: u32 = 0x140;
/// Supervisor Exception Program Counter (`sepc`).
pub const CSR_SEPC: u32 = 0x141;
/// Supervisor Cause register (`scause`).
pub const CSR_SCAUSE: u32 = 0x142;
/// Supervisor Trap Value register (`stval`).
pub const CSR_STVAL: u32 = 0x143;
/// Supervisor Interrupt-Pending register (`sip`).
pub const CSR_SIP: u32 = 0x144;
/// Supervisor Address Translation and Protection register (`satp`).
pub const CSR_SATP: u32 = 0x180;

/// Machine Status register (`mstatus`).
pub const CSR_MSTATUS: u32 = 0x300;
/// Machine ISA and Extensions register (`misa`).
pub const CSR_MISA: u32 = 0x301;
/// Machine Interrupt-Enable register (`mie`).
pub const CSR_MIE: u32 = 0x304;
/// Machine Trap-Vector Base Address register (`mtvec`).
pub const CSR_MTVEC: u32 = 0x305;
/// Machine Scratch register (`mscratch`).
pub const CSR_MSCRATCH: u32 = 0x340;
/// Machine Exception Program Counter (`mepc`).
pub const CSR_MEPC: u32 = 0x341;
/// Machine Cause register (`mcause`).
pub const CSR_MCAUSE: u32 = 0x342;
/// Machine Trap Value register (`mtval`).
pub const CSR_MTVAL: u32 = 0x343;
/// Machine Interrupt-Pending register (`mip`).
pub const CSR_MIP: u32 = 0x344;

/// Cycle counter (`cycle`).
pub const CSR_CYCLE: u32 = 0xC00;
/// Real-time timer counter (`time`).
pub const CSR_TIME: u32 = 0xC01;

/// Supervisor Status (`sstatus`) bitflags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Sstatus(pub usize);

impl Sstatus {
    /// Supervisor Interrupt Enable (`SIE`, bit 1).
    pub const SIE: Self = Self(1 << 1);
    /// Supervisor Previous Interrupt Enable (`SPIE`, bit 5).
    pub const SPIE: Self = Self(1 << 5);
    /// Supervisor Previous Privilege mode (`SPP`, bit 8): 0 = User, 1 = Supervisor.
    pub const SPP: Self = Self(1 << 8);
    /// Floating-point status mask (`FS`, bits 13..14).
    pub const FS_MASK: Self = Self(0b11 << 13);
    /// Permit Supervisor User Memory access (`SUM`, bit 18).
    pub const SUM: Self = Self(1 << 18);
    /// Make Executable Readable (`MXR`, bit 19).
    pub const MXR: Self = Self(1 << 19);

    /// Returns `true` if Supervisor interrupts are enabled.
    #[must_use]
    pub const fn is_supervisor_interrupt_enabled(self) -> bool {
        (self.0 & Self::SIE.0) != 0
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

impl core::ops::BitOr for Sstatus {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for Sstatus {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for Sstatus {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign for Sstatus {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Address translation modes for `satp.MODE` on 64-bit RISC-V.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SatpMode {
    /// Bare mode: no translation or protection.
    Bare = 0,
    /// Page-based 39-bit virtual addressing (3-level).
    Sv39 = 8,
    /// Page-based 48-bit virtual addressing (4-level).
    Sv48 = 9,
    /// Page-based 57-bit virtual addressing (5-level).
    Sv57 = 10,
}

impl SatpMode {
    /// Decodes a 4-bit integer into a 64-bit SATP mode.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Bare),
            8 => Some(Self::Sv39),
            9 => Some(Self::Sv48),
            10 => Some(Self::Sv57),
            _ => None,
        }
    }
}

/// Decoded representation of the Supervisor Address Translation and Protection (`satp`) register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Satp {
    /// The active virtual memory translation mode.
    pub mode: SatpMode,
    /// Address Space Identifier (`ASID`, 16-bit).
    pub asid: u16,
    /// Physical Page Number of the root translation table (44-bit).
    pub ppn: u64,
}

impl Satp {
    /// Creates a new `satp` descriptor.
    #[must_use]
    pub const fn new(mode: SatpMode, asid: u16, ppn: u64) -> Self {
        Self { mode, asid, ppn }
    }

    /// Returns the active translation mode.
    #[must_use]
    pub const fn mode(&self) -> SatpMode {
        self.mode
    }

    /// Returns the Address Space Identifier (ASID).
    #[must_use]
    pub const fn asid(&self) -> u16 {
        self.asid
    }

    /// Returns the root page table Physical Page Number (PPN).
    #[must_use]
    pub const fn ppn(&self) -> u64 {
        self.ppn
    }

    /// Decodes a raw 64-bit `satp` register value.
    #[must_use]
    pub fn from_raw(raw: u64) -> Option<Self> {
        let mode_raw = ((raw >> 60) & 0x0F) as u8;
        let mode = SatpMode::from_raw(mode_raw)?;
        let asid = ((raw >> 44) & 0xFFFF) as u16;
        let ppn = raw & 0x0FFF_FFFF_FFFF;
        Some(Self { mode, asid, ppn })
    }

    /// Encodes this `satp` structure into a raw 64-bit value.
    #[must_use]
    pub fn to_raw(self) -> u64 {
        ((self.mode as u64) << 60)
            | ((u64::from(self.asid) & 0xFFFF) << 44)
            | (self.ppn & 0x0FFF_FFFF_FFFF)
    }
}

/// Reads a RISC-V Control and Status Register (`csrr`).
#[must_use]
pub fn csrr(csr: u32) -> usize {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        let val: usize;
        core::arch::asm!("csrr {0}, {1}", out(reg) val, in(reg) csr, options(nomem, nostack, preserves_flags));
        val
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = csr;
        0
    }
}

/// Writes a RISC-V Control and Status Register (`csrw`).
///
/// # Safety
///
/// Writing invalid values into CSRs can cause hardware faults or disable address translation.
pub unsafe fn csrw(csr: u32, val: usize) {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("csrw {0}, {1}", in(reg) csr, in(reg) val, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (csr, val);
    }
}

/// Atomically sets bits in a RISC-V Control and Status Register (`csrs`).
///
/// # Safety
///
/// Modifying CSR bits alters CPU execution state and interrupt enable flags.
pub unsafe fn csrs(csr: u32, mask: usize) {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("csrs {0}, {1}", in(reg) csr, in(reg) mask, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (csr, mask);
    }
}

/// Atomically clears bits in a RISC-V Control and Status Register (`csrc`).
///
/// # Safety
///
/// Modifying CSR bits alters CPU execution state and interrupt enable flags.
pub unsafe fn csrc(csr: u32, mask: usize) {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("csrc {0}, {1}", in(reg) csr, in(reg) mask, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        let _ = (csr, mask);
    }
}

/// Atomic Memory Fence and TLB invalidation (`sfence.vma`).
pub fn sfence_vma(vaddr: usize, asid: usize) {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("sfence.vma {0}, {1}", in(reg) vaddr, in(reg) asid, options(nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "riscv64"))]
    let _ = (vaddr, asid);
}
