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
/// Machine Exception Delegation register (`medeleg`).
pub const CSR_MEDELEG: u32 = 0x302;
/// Machine Interrupt Delegation register (`mideleg`).
pub const CSR_MIDELEG: u32 = 0x303;
/// Machine Interrupt-Enable register (`mie`).
pub const CSR_MIE: u32 = 0x304;
/// Machine Trap-Vector Base Address register (`mtvec`).
pub const CSR_MTVEC: u32 = 0x305;
/// Machine Counter Enable register (`mcounteren`).
pub const CSR_MCOUNTEREN: u32 = 0x306;
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

/// Physical Memory Protection Configuration Register 0 (`pmpcfg0`).
pub const CSR_PMPCFG0: u32 = 0x3A0;
/// Physical Memory Protection Configuration Register 1 (`pmpcfg1`, RV32 only).
pub const CSR_PMPCFG1: u32 = 0x3A1;
/// Physical Memory Protection Configuration Register 2 (`pmpcfg2`).
pub const CSR_PMPCFG2: u32 = 0x3A2;
/// Physical Memory Protection Configuration Register 3 (`pmpcfg3`, RV32 only).
pub const CSR_PMPCFG3: u32 = 0x3A3;

/// Physical Memory Protection Address Register 0 (`pmpaddr0`).
pub const CSR_PMPADDR0: u32 = 0x3B0;
/// Physical Memory Protection Address Register 1 (`pmpaddr1`).
pub const CSR_PMPADDR1: u32 = 0x3B1;
/// Physical Memory Protection Address Register 2 (`pmpaddr2`).
pub const CSR_PMPADDR2: u32 = 0x3B2;
/// Physical Memory Protection Address Register 3 (`pmpaddr3`).
pub const CSR_PMPADDR3: u32 = 0x3B3;
/// Physical Memory Protection Address Register 4 (`pmpaddr4`).
pub const CSR_PMPADDR4: u32 = 0x3B4;
/// Physical Memory Protection Address Register 5 (`pmpaddr5`).
pub const CSR_PMPADDR5: u32 = 0x3B5;
/// Physical Memory Protection Address Register 6 (`pmpaddr6`).
pub const CSR_PMPADDR6: u32 = 0x3B6;
/// Physical Memory Protection Address Register 7 (`pmpaddr7`).
pub const CSR_PMPADDR7: u32 = 0x3B7;
/// Physical Memory Protection Address Register 8 (`pmpaddr8`).
pub const CSR_PMPADDR8: u32 = 0x3B8;
/// Physical Memory Protection Address Register 9 (`pmpaddr9`).
pub const CSR_PMPADDR9: u32 = 0x3B9;
/// Physical Memory Protection Address Register 10 (`pmpaddr10`).
pub const CSR_PMPADDR10: u32 = 0x3BA;
/// Physical Memory Protection Address Register 11 (`pmpaddr11`).
pub const CSR_PMPADDR11: u32 = 0x3BB;
/// Physical Memory Protection Address Register 12 (`pmpaddr12`).
pub const CSR_PMPADDR12: u32 = 0x3BC;
/// Physical Memory Protection Address Register 13 (`pmpaddr13`).
pub const CSR_PMPADDR13: u32 = 0x3BD;
/// Physical Memory Protection Address Register 14 (`pmpaddr14`).
pub const CSR_PMPADDR14: u32 = 0x3BE;
/// Physical Memory Protection Address Register 15 (`pmpaddr15`).
pub const CSR_PMPADDR15: u32 = 0x3BF;

/// Address matching modes for Physical Memory Protection (`pmpcfg.A`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PmpAddressMode {
    /// Null region: disabled.
    Off = 0,
    /// Top of Range.
    Tor = 1,
    /// Naturally aligned 4-byte region.
    Na4 = 2,
    /// Naturally aligned power-of-two region (`>= 8` bytes).
    Napot = 3,
}

impl PmpAddressMode {
    /// Decodes a 2-bit raw integer into a PMP address matching mode.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        match raw & 0x03 {
            0 => Self::Off,
            1 => Self::Tor,
            2 => Self::Na4,
            3 => Self::Napot,
            _ => unreachable!(),
        }
    }
}

/// Physical Memory Protection configuration entry (1 byte per entry).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PmpConfig(pub u8);

impl PmpConfig {
    /// Read permission (`R`, bit 0).
    pub const R: u8 = 1 << 0;
    /// Write permission (`W`, bit 1).
    pub const W: u8 = 1 << 1;
    /// Execute permission (`X`, bit 2).
    pub const X: u8 = 1 << 2;
    /// Address-matching mode mask (`A`, bits 3..4).
    pub const A_MASK: u8 = 0b11 << 3;
    /// Lock bit (`L`, bit 7).
    pub const L: u8 = 1 << 7;

    /// Creates a new PMP configuration descriptor.
    #[allow(clippy::fn_params_excessive_bools)]
    #[must_use]
    pub const fn new(
        read: bool,
        write: bool,
        exec: bool,
        mode: PmpAddressMode,
        lock: bool,
    ) -> Self {
        let mut val = 0u8;
        if read {
            val |= Self::R;
        }
        if write {
            val |= Self::W;
        }
        if exec {
            val |= Self::X;
        }
        val |= (mode as u8) << 3;
        if lock {
            val |= Self::L;
        }
        Self(val)
    }

    /// Returns `true` if read access is permitted.
    #[must_use]
    pub const fn is_readable(&self) -> bool {
        (self.0 & Self::R) != 0
    }

    /// Returns `true` if write access is permitted.
    #[must_use]
    pub const fn is_writable(&self) -> bool {
        (self.0 & Self::W) != 0
    }

    /// Returns `true` if execute access is permitted.
    #[must_use]
    pub const fn is_executable(&self) -> bool {
        (self.0 & Self::X) != 0
    }

    /// Returns `true` if the entry is locked.
    #[must_use]
    pub const fn is_locked(&self) -> bool {
        (self.0 & Self::L) != 0
    }

    /// Returns the address-matching mode.
    #[must_use]
    pub const fn address_mode(&self) -> PmpAddressMode {
        PmpAddressMode::from_raw((self.0 & Self::A_MASK) >> 3)
    }

    /// Returns the raw 8-bit configuration value.
    #[must_use]
    pub const fn to_raw(&self) -> u8 {
        self.0
    }
}

/// Decodes a Naturally Aligned Power-of-Two (NAPOT) encoded `pmpaddr` value into `(base_address, size_bytes)`.
#[must_use]
pub fn decode_pmp_napot(pmpaddr: u64) -> (u64, u64) {
    let trailing_ones = (!pmpaddr).trailing_zeros();
    let size = 1u64 << (trailing_ones + 3);
    let mask = (1u64 << (trailing_ones + 1)) - 1;
    let base = (pmpaddr & !mask) << 2;
    (base, size)
}

/// Encodes a base address and power-of-two size (`>= 8` bytes) into a NAPOT `pmpaddr` register value.
#[must_use]
pub fn encode_pmp_napot(base: u64, size: u64) -> Option<u64> {
    if size < 8 || !size.is_power_of_two() || (base % size) != 0 {
        return None;
    }
    let k = size.trailing_zeros().checked_sub(3)?;
    let mask = (1u64 << k) - 1;
    let pmpaddr = (base >> 2) | mask;
    Some(pmpaddr)
}

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
