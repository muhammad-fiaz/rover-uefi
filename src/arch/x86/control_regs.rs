//! # x86 Processor Control Registers (CR0, CR2, CR3, CR4, XCR0)
//!
//! Provides architectural definitions, bitflags, and read/write accessors for x86
//! control registers governing processor modes, paging, protection, and extended states.

/// Bitflags for Control Register 0 (`CR0`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cr0Flags(pub u64);

impl Cr0Flags {
    /// Protection Enable (`PE`): enables protected mode.
    pub const PROTECTION_ENABLE: Self = Self(1 << 0);
    /// Monitor Coprocessor (`MP`).
    pub const MONITOR_COPROCESSOR: Self = Self(1 << 1);
    /// Emulation (`EM`): when set, no x87 FPU is present.
    pub const EMULATION: Self = Self(1 << 2);
    /// Task Switched (`TS`).
    pub const TASK_SWITCHED: Self = Self(1 << 3);
    /// Extension Type (`ET`).
    pub const EXTENSION_TYPE: Self = Self(1 << 4);
    /// Numeric Error (`NE`): enables internal x87 error reporting.
    pub const NUMERIC_ERROR: Self = Self(1 << 5);
    /// Write Protect (`WP`): prevents supervisor-mode writes to read-only pages.
    pub const WRITE_PROTECT: Self = Self(1 << 16);
    /// Alignment Mask (`AM`).
    pub const ALIGNMENT_MASK: Self = Self(1 << 18);
    /// Not Write-Through (`NW`).
    pub const NOT_WRITE_THROUGH: Self = Self(1 << 29);
    /// Cache Disable (`CD`).
    pub const CACHE_DISABLE: Self = Self(1 << 30);
    /// Paging (`PG`): enables memory paging translation.
    pub const PAGING: Self = Self(1 << 31);

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

/// Bitflags for Control Register 4 (`CR4`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cr4Flags(pub u64);

impl Cr4Flags {
    /// Virtual-8086 Mode Extensions (`VME`).
    pub const VIRTUAL_8086_EXTENSIONS: Self = Self(1 << 0);
    /// Protected-Mode Virtual Interrupts (`PVI`).
    pub const PROTECTED_VIRTUAL_INTERRUPTS: Self = Self(1 << 1);
    /// Time Stamp Disable (`TSD`).
    pub const TIME_STAMP_DISABLE: Self = Self(1 << 2);
    /// Debugging Extensions (`DE`).
    pub const DEBUGGING_EXTENSIONS: Self = Self(1 << 3);
    /// Page Size Extension (`PSE`).
    pub const PAGE_SIZE_EXTENSION: Self = Self(1 << 4);
    /// Physical Address Extension (`PAE`).
    pub const PHYSICAL_ADDRESS_EXTENSION: Self = Self(1 << 5);
    /// Machine Check Enable (`MCE`).
    pub const MACHINE_CHECK_ENABLE: Self = Self(1 << 6);
    /// Page Global Enable (`PGE`).
    pub const PAGE_GLOBAL_ENABLE: Self = Self(1 << 7);
    /// Performance-Monitoring Counter Enable (`PCE`).
    pub const PERFORMANCE_COUNTER_ENABLE: Self = Self(1 << 8);
    /// Operating System FXSAVE/FXRSTOR Support (`OSFXSR`).
    pub const OS_FXSR: Self = Self(1 << 9);
    /// Operating System Unmasked SIMD Exceptions (`OSXMMEXCPT`).
    pub const OS_XMMEXCPT: Self = Self(1 << 10);
    /// User-Mode Instruction Prevention (`UMIP`).
    pub const USER_MODE_INSTRUCTION_PREVENTION: Self = Self(1 << 11);
    /// 57-Bit Linear Addresses / 5-Level Paging (`LA57`).
    pub const LINEAR_ADDRESSES_57_BIT: Self = Self(1 << 12);
    /// VMX Enable (`VMXE`).
    pub const VMX_ENABLE: Self = Self(1 << 13);
    /// SMX Enable (`SMXE`).
    pub const SMX_ENABLE: Self = Self(1 << 14);
    /// FSGSBASE Instructions Enable (`FSGSBASE`).
    pub const FSGSBASE_ENABLE: Self = Self(1 << 16);
    /// Process-Context Identifiers Enable (`PCIDE`).
    pub const PCID_ENABLE: Self = Self(1 << 17);
    /// XSAVE and Extended States Enable (`OSXSAVE`).
    pub const OS_XSAVE: Self = Self(1 << 18);
    /// Supervisor Mode Execution Prevention (`SMEP`).
    pub const SUPERVISOR_EXECUTION_PREVENTION: Self = Self(1 << 20);
    /// Supervisor Mode Access Prevention (`SMAP`).
    pub const SUPERVISOR_ACCESS_PREVENTION: Self = Self(1 << 21);
    /// Protection Key Enable (`PKE`).
    pub const PROTECTION_KEY_ENABLE: Self = Self(1 << 22);

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

/// Bitflags for Extended Control Register 0 (`XCR0`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Xcr0Flags(pub u64);

impl Xcr0Flags {
    /// x87 FPU / MMX State (`X87`).
    pub const X87: Self = Self(1 << 0);
    /// 128-bit SSE State (`SSE`).
    pub const SSE: Self = Self(1 << 1);
    /// 256-bit AVX State (`AVX`).
    pub const AVX: Self = Self(1 << 2);
    /// MPX Bound Registers (`BNDREG`).
    pub const BNDREG: Self = Self(1 << 3);
    /// MPX Bound Configuration State (`BNDCSR`).
    pub const BNDCSR: Self = Self(1 << 4);
    /// AVX-512 Opmask Registers (`OPMASK`).
    pub const OPMASK: Self = Self(1 << 5);
    /// AVX-512 Upper 256 bits of ZMM0-ZMM15 (`ZMM_HI256`).
    pub const ZMM_HI256: Self = Self(1 << 6);
    /// AVX-512 Full 512-bit ZMM16-ZMM31 (`HI16_ZMM`).
    pub const HI16_ZMM: Self = Self(1 << 7);

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

/// Control Register 3 (`CR3`) structure holding the physical base address of the root page table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cr3(pub u64);

impl Cr3 {
    const ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;
    const PWT_MASK: u64 = 1 << 3;
    const PCD_MASK: u64 = 1 << 4;
    const PCID_MASK: u64 = 0x0FFF;

    /// Creates a CR3 value with a 4 KiB aligned physical address and cache flags.
    #[must_use]
    pub const fn new(address: u64, page_write_through: bool, page_cache_disable: bool) -> Self {
        let mut val = address & Self::ADDRESS_MASK;
        if page_write_through {
            val |= Self::PWT_MASK;
        }
        if page_cache_disable {
            val |= Self::PCD_MASK;
        }
        Self(val)
    }

    /// Creates a CR3 value with a 4 KiB aligned physical address and PCID.
    #[must_use]
    #[allow(clippy::cast_lossless)]
    pub const fn new_with_pcid(address: u64, pcid: u16) -> Self {
        let val = (address & Self::ADDRESS_MASK) | ((pcid as u64) & Self::PCID_MASK);
        Self(val)
    }

    /// Returns the 4 KiB aligned physical base address of the level-4/level-5 root page table.
    #[must_use]
    pub const fn address(self) -> u64 {
        self.0 & Self::ADDRESS_MASK
    }

    /// Returns the Process-Context Identifier (PCID) bits when PCIDE is active.
    #[must_use]
    pub const fn pcid(self) -> u16 {
        (self.0 & Self::PCID_MASK) as u16
    }

    /// Returns whether Page-level Write-Through is asserted.
    #[must_use]
    pub const fn is_page_write_through(self) -> bool {
        (self.0 & Self::PWT_MASK) != 0
    }

    /// Returns whether Page-level Cache Disable is asserted.
    #[must_use]
    pub const fn is_page_cache_disabled(self) -> bool {
        (self.0 & Self::PCD_MASK) != 0
    }
}

/// Reads the current value of `CR0`.
#[must_use]
pub fn read_cr0() -> Cr0Flags {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let val: u64;
        core::arch::asm!("mov {}, cr0", out(reg) val, options(nomem, nostack, preserves_flags));
        Cr0Flags(val)
    }
    #[cfg(not(target_arch = "x86_64"))]
    Cr0Flags(0)
}

/// Writes a new value into `CR0`.
///
/// # Safety
///
/// Modifying `CR0` changes processor modes, memory write protections, and paging.
pub unsafe fn write_cr0(flags: Cr0Flags) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mov cr0, {}", in(reg) flags.0, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = flags;
}

/// Reads `CR2`, which stores the linear virtual address that triggered the most recent page fault.
#[must_use]
pub fn read_cr2() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let val: u64;
        core::arch::asm!("mov {}, cr2", out(reg) val, options(nomem, nostack, preserves_flags));
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    0
}

/// Reads the current value of `CR3`.
#[must_use]
pub fn read_cr3() -> Cr3 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let val: u64;
        core::arch::asm!("mov {}, cr3", out(reg) val, options(nomem, nostack, preserves_flags));
        Cr3(val)
    }
    #[cfg(not(target_arch = "x86_64"))]
    Cr3(0)
}

/// Writes a new root page table address into `CR3`. Flushes all non-global TLB entries.
///
/// # Safety
///
/// Writing an invalid page table address into `CR3` causes immediate triple-fault or page-fault crashes.
pub unsafe fn write_cr3(cr3: Cr3) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) cr3.0, options(nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = cr3;
}

/// Reads the current value of `CR4`.
#[must_use]
pub fn read_cr4() -> Cr4Flags {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let val: u64;
        core::arch::asm!("mov {}, cr4", out(reg) val, options(nomem, nostack, preserves_flags));
        Cr4Flags(val)
    }
    #[cfg(not(target_arch = "x86_64"))]
    Cr4Flags(0)
}

/// Writes a new value into `CR4`.
///
/// # Safety
///
/// Changing `CR4` configuration can enable or disable architectural processor features.
pub unsafe fn write_cr4(flags: Cr4Flags) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mov cr4, {}", in(reg) flags.0, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = flags;
}

/// Reads the Extended Control Register 0 (`XCR0`).
#[must_use]
pub fn read_xcr0() -> Xcr0Flags {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let eax: u32;
        let edx: u32;
        core::arch::asm!(
            "xgetbv",
            in("ecx") 0u32,
            out("eax") eax,
            out("edx") edx,
            options(nomem, nostack, preserves_flags)
        );
        Xcr0Flags((u64::from(edx) << 32) | u64::from(eax))
    }
    #[cfg(not(target_arch = "x86_64"))]
    Xcr0Flags(0)
}

/// Writes a new value into Extended Control Register 0 (`XCR0`).
///
/// # Safety
///
/// Writing invalid feature masks into `XCR0` triggers general protection faults (#GP).
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn write_xcr0(flags: Xcr0Flags) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let eax = flags.0 as u32;
        let edx = (flags.0 >> 32) as u32;
        core::arch::asm!(
            "xsetbv",
            in("ecx") 0u32,
            in("eax") eax,
            in("edx") edx,
            options(nomem, nostack, preserves_flags)
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = flags;
}
