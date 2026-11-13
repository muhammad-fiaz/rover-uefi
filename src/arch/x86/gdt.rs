//! # x86 Global Descriptor Table (GDT), Task State Segment (TSS), and Segments
//!
//! Provides structures and instructions for managing the x86 processor Global
//! Descriptor Table (`GDT`), Task State Segment (`TSS`), segment selectors,
//! and processor segment registers.

/// A 16-bit x86 Segment Selector identifying an entry in the GDT or LDT.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    /// Standard 64-bit Kernel Code Segment Selector (GDT index 1, RPL 0).
    pub const KERNEL_CODE64: Self = Self(0x08);
    /// Standard 64-bit Kernel Data Segment Selector (GDT index 2, RPL 0).
    pub const KERNEL_DATA: Self = Self(0x10);
    /// Standard 64-bit User Code Segment Selector (GDT index 3, RPL 3).
    pub const USER_CODE64: Self = Self(0x1B);
    /// Standard 64-bit User Data Segment Selector (GDT index 4, RPL 3).
    pub const USER_DATA: Self = Self(0x23);

    /// Requested Privilege Level (RPL, bits 0..1).
    pub const RPL_RING0: u16 = 0;
    /// User ring 3 privilege level.
    pub const RPL_RING3: u16 = 3;
    /// Table Indicator bit (bit 2): 0 = GDT, 1 = LDT.
    pub const TABLE_LDT: u16 = 1 << 2;

    /// Creates a segment selector from an index, table indicator, and privilege level.
    #[must_use]
    pub const fn new(index: u16, is_ldt: bool, rpl: u8) -> Self {
        let ldt_bit = if is_ldt { Self::TABLE_LDT } else { 0 };
        Self((index << 3) | ldt_bit | ((rpl & 0x03) as u16))
    }

    /// Returns the raw 16-bit selector value.
    #[must_use]
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Returns the Requested Privilege Level (`RPL`, bits 0..1).
    #[must_use]
    pub const fn rpl(self) -> u8 {
        (self.0 & 0x03) as u8
    }

    /// Returns `true` if this selector refers to the Local Descriptor Table (`LDT`).
    #[must_use]
    pub const fn is_ldt(self) -> bool {
        (self.0 & Self::TABLE_LDT) != 0
    }

    /// Returns the table index (bits 3..15).
    #[must_use]
    pub const fn index(self) -> u16 {
        self.0 >> 3
    }
}

/// Pointer structure used by `LGDT` and `SGDT` (and `LIDT`/`SIDT`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    /// Size of the descriptor table in bytes minus 1.
    pub limit: u16,
    /// Linear base address of the descriptor table.
    pub base: u64,
}

impl DescriptorTablePointer {
    /// Creates a new descriptor table pointer.
    #[must_use]
    pub const fn new(base: u64, limit: u16) -> Self {
        Self { limit, base }
    }

    /// Returns the limit value safely without creating unaligned references.
    #[must_use]
    pub const fn limit(&self) -> u16 {
        unsafe { core::ptr::addr_of!(self.limit).read_unaligned() }
    }

    /// Returns the base address value safely without creating unaligned references.
    #[must_use]
    pub const fn base(&self) -> u64 {
        unsafe { core::ptr::addr_of!(self.base).read_unaligned() }
    }
}

/// An 8-byte segment descriptor in the Global Descriptor Table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SegmentDescriptor(pub u64);

impl SegmentDescriptor {
    /// Standard 64-bit Kernel Code segment descriptor (read/execute, DPL 0, long mode).
    pub const KERNEL_CODE64: Self = Self(0x0020_9A00_0000_0000);
    /// Standard 64-bit Kernel Data segment descriptor (read/write, DPL 0).
    pub const KERNEL_DATA: Self = Self(0x0000_9200_0000_0000);
    /// Standard 64-bit User Code segment descriptor (read/execute, DPL 3, long mode).
    pub const USER_CODE64: Self = Self(0x0020_FA00_0000_0000);
    /// Standard 64-bit User Data segment descriptor (read/write, DPL 3).
    pub const USER_DATA: Self = Self(0x0000_F200_0000_0000);

    /// Constructs a 16-byte TSS descriptor (split into two 8-byte entries) pointing to `tss_base`.
    #[must_use]
    pub fn new_tss(tss_base: u64, tss_limit: u16) -> (Self, Self) {
        let base_low = tss_base & 0x00FF_FFFF;
        let base_mid = (tss_base >> 24) & 0xFF;
        let base_high = tss_base >> 32;

        let limit_low = u64::from(tss_limit);
        // Type 0x9 = 64-bit TSS (Available), Present bit 47 = 1
        let low = limit_low | (base_low << 16) | (0x89u64 << 40) | (base_mid << 56);
        let high = base_high;

        (Self(low), Self(high))
    }
}

/// 64-bit Task State Segment (`TSS`) structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TaskStateSegment64 {
    _reserved0: u32,
    /// Privilege Level 0 Stack Pointer.
    pub rsp0: u64,
    /// Privilege Level 1 Stack Pointer.
    pub rsp1: u64,
    /// Privilege Level 2 Stack Pointer.
    pub rsp2: u64,
    _reserved1: u64,
    /// Interrupt Stack Table entry 1.
    pub ist1: u64,
    /// Interrupt Stack Table entry 2.
    pub ist2: u64,
    /// Interrupt Stack Table entry 3.
    pub ist3: u64,
    /// Interrupt Stack Table entry 4.
    pub ist4: u64,
    /// Interrupt Stack Table entry 5.
    pub ist5: u64,
    /// Interrupt Stack Table entry 6.
    pub ist6: u64,
    /// Interrupt Stack Table entry 7.
    pub ist7: u64,
    _reserved2: u64,
    _reserved3: u16,
    /// I/O Map Base Address offset relative to the TSS base.
    pub iomap_base: u16,
}

impl Default for TaskStateSegment64 {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskStateSegment64 {
    /// Creates an initialized 64-bit TSS with empty stack pointers and I/O map disabled.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new() -> Self {
        Self {
            _reserved0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            _reserved1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            _reserved2: 0,
            _reserved3: 0,
            iomap_base: size_of::<Self>() as u16,
        }
    }

    /// Safely reads `rsp0` without creating unaligned references.
    #[must_use]
    pub const fn rsp0(&self) -> u64 {
        unsafe { core::ptr::addr_of!(self.rsp0).read_unaligned() }
    }

    /// Safely reads `rsp1` without creating unaligned references.
    #[must_use]
    pub const fn rsp1(&self) -> u64 {
        unsafe { core::ptr::addr_of!(self.rsp1).read_unaligned() }
    }

    /// Safely reads `rsp2` without creating unaligned references.
    #[must_use]
    pub const fn rsp2(&self) -> u64 {
        unsafe { core::ptr::addr_of!(self.rsp2).read_unaligned() }
    }

    /// Safely sets `rsp0` without creating unaligned references.
    pub fn set_rsp0(&mut self, val: u64) {
        unsafe {
            core::ptr::addr_of_mut!(self.rsp0).write_unaligned(val);
        }
    }
}

/// Loads the Global Descriptor Table register (`LGDT`).
///
/// # Safety
///
/// Loading an invalid GDT will crash the CPU upon any subsequent segment reload.
pub unsafe fn lgdt(gdtr: &DescriptorTablePointer) {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        core::arch::asm!("lgdt [{}]", in(reg) gdtr, options(readonly, nostack, preserves_flags));
    }
    #[cfg(not(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none"))))]
    let _ = gdtr;
}

/// Stores the Global Descriptor Table register (`SGDT`).
#[must_use]
pub fn sgdt() -> DescriptorTablePointer {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let mut gdtr = DescriptorTablePointer::default();
        core::arch::asm!("sgdt [{}]", in(reg) &raw mut gdtr, options(nostack, preserves_flags));
        gdtr
    }
    #[cfg(not(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none"))))]
    DescriptorTablePointer::default()
}

/// Loads the Task Register (`LTR`).
///
/// # Safety
///
/// `selector` must point to a valid Available 64-bit TSS descriptor in the active GDT.
pub unsafe fn ltr(selector: SegmentSelector) {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        core::arch::asm!("ltr {0:x}", in(reg) selector.0, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none"))))]
    let _ = selector;
}

/// Reads the Code Segment selector (`CS`).
#[must_use]
pub fn read_cs() -> SegmentSelector {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let sel: u16;
        core::arch::asm!("mov {0:x}, cs", out(reg) sel, options(nomem, nostack, preserves_flags));
        SegmentSelector(sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    SegmentSelector(0)
}

/// Reads the Data Segment selector (`DS`).
#[must_use]
pub fn read_ds() -> SegmentSelector {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let sel: u16;
        core::arch::asm!("mov {0:x}, ds", out(reg) sel, options(nomem, nostack, preserves_flags));
        SegmentSelector(sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    SegmentSelector(0)
}

/// Reads the Stack Segment selector (`SS`).
#[must_use]
pub fn read_ss() -> SegmentSelector {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let sel: u16;
        core::arch::asm!("mov {0:x}, ss", out(reg) sel, options(nomem, nostack, preserves_flags));
        SegmentSelector(sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    SegmentSelector(0)
}

/// Reads the Extra Segment selector (`ES`).
#[must_use]
pub fn read_es() -> SegmentSelector {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let sel: u16;
        core::arch::asm!("mov {0:x}, es", out(reg) sel, options(nomem, nostack, preserves_flags));
        SegmentSelector(sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    SegmentSelector(0)
}

/// Reads the FS segment selector (`FS`).
#[must_use]
pub fn read_fs() -> SegmentSelector {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let sel: u16;
        core::arch::asm!("mov {0:x}, fs", out(reg) sel, options(nomem, nostack, preserves_flags));
        SegmentSelector(sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    SegmentSelector(0)
}

/// Reads the GS segment selector (`GS`).
#[must_use]
pub fn read_gs() -> SegmentSelector {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let sel: u16;
        core::arch::asm!("mov {0:x}, gs", out(reg) sel, options(nomem, nostack, preserves_flags));
        SegmentSelector(sel)
    }
    #[cfg(not(target_arch = "x86_64"))]
    SegmentSelector(0)
}

/// Halts the processor until the next external interrupt arrives (`HLT`).
#[inline]
pub fn halt() {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        core::arch::asm!("hlt", options(nomem, nostack));
    }
}
