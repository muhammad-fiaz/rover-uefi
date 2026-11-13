//! # ARM64 Stage 1 VMSA MMU and Translation Tables
//!
//! Provides translation table descriptor formats, memory attribute indirection (`MAIR_EL1`),
//! Translation Control Register (`TCR_EL1`), Translation Table Base Registers (`TTBR0_EL1`),
//! and Translation Lookaside Buffer (`TLBI`) maintenance primitives.

/// 4 KiB standard page size in bytes.
pub const ARM64_PAGE_SIZE_4K: usize = 4096;
/// 2 MiB large block size in bytes.
pub const ARM64_BLOCK_SIZE_2M: usize = 2 * 1024 * 1024;
/// 1 GiB huge block size in bytes.
pub const ARM64_BLOCK_SIZE_1G: usize = 1024 * 1024 * 1024;
/// Number of entries contained in a single 4 KiB translation table.
pub const ARM64_ENTRIES_PER_TABLE: usize = 512;

/// Standard memory attribute index slots in `MAIR_EL1`.
pub const MAIR_ATTR_DEVICE_NGNRNE: u8 = 0;
/// Device-nGnRE memory attribute index slot.
pub const MAIR_ATTR_DEVICE_NGNRE: u8 = 1;
/// Normal Non-Cacheable memory attribute index slot.
pub const MAIR_ATTR_NORMAL_NON_CACHEABLE: u8 = 2;
/// Normal Write-Back Cacheable memory attribute index slot.
pub const MAIR_ATTR_NORMAL_WRITE_BACK: u8 = 3;

/// Architectural bitflags defining Stage 1 page/block descriptor attributes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ArmDescriptorFlags(pub u64);

impl ArmDescriptorFlags {
    /// Valid entry bit (`bit 0`). If clear, the entry triggers a Translation Fault.
    pub const VALID: Self = Self(1 << 0);
    /// Table or Page descriptor indicator (`bit 1`).
    ///
    /// At levels 0-2: `1` = Table descriptor pointing to next level, `0` = Block descriptor.
    /// At level 3: `1` = 4 KiB Page descriptor.
    pub const TABLE_OR_PAGE: Self = Self(1 << 1);

    /// Non-Secure access bit (`NS`, bit 5).
    pub const NON_SECURE: Self = Self(1 << 5);

    /// Access Permission: Privileged Read/Write, Unprivileged None (`0b00 << 6`).
    pub const AP_RW_PRIVILEGED: Self = Self(0b00 << 6);
    /// Access Permission: Privileged Read/Write, Unprivileged Read/Write (`0b01 << 6`).
    pub const AP_RW_ALL: Self = Self(0b01 << 6);
    /// Access Permission: Privileged Read-Only, Unprivileged None (`0b10 << 6`).
    pub const AP_RO_PRIVILEGED: Self = Self(0b10 << 6);
    /// Access Permission: Privileged Read-Only, Unprivileged Read-Only (`0b11 << 6`).
    pub const AP_RO_ALL: Self = Self(0b11 << 6);

    /// Shareability: Non-shareable (`0b00 << 8`).
    pub const SH_NON_SHAREABLE: Self = Self(0b00 << 8);
    /// Shareability: Outer Shareable (`0b10 << 8`).
    pub const SH_OUTER_SHAREABLE: Self = Self(0b10 << 8);
    /// Shareability: Inner Shareable (`0b11 << 8`).
    pub const SH_INNER_SHAREABLE: Self = Self(0b11 << 8);

    /// Access Flag (`AF`, bit 10). Must be set to 1 to avoid an Access Flag fault.
    pub const ACCESS_FLAG: Self = Self(1 << 10);
    /// Not Global (`nG`, bit 11).
    pub const NOT_GLOBAL: Self = Self(1 << 11);

    /// Privileged Execute-Never (`PXN`, bit 53).
    pub const PRIVILEGED_EXECUTE_NEVER: Self = Self(1 << 53);
    /// User Execute-Never (`UXN`, bit 54).
    pub const USER_EXECUTE_NEVER: Self = Self(1 << 54);

    /// Sets the Memory Attribute Indirection index (`AttrIndx[2:0]`, bits 2..4).
    #[must_use]
    pub const fn with_attr_index(self, index: u8) -> Self {
        Self((self.0 & !(0b111 << 2)) | (((index & 0x07) as u64) << 2))
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

/// A 64-bit Stage 1 Translation Table Descriptor in ARMv8-A.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArmTableDescriptor(pub u64);

impl ArmTableDescriptor {
    const OUTPUT_ADDRESS_MASK: u64 = 0x0000_FFFF_FFFF_F000;

    /// Creates a table descriptor pointing to the next-level page table.
    #[must_use]
    pub const fn new_table(next_table_addr: u64) -> Self {
        Self(
            (next_table_addr & Self::OUTPUT_ADDRESS_MASK)
                | ArmDescriptorFlags::VALID.0
                | ArmDescriptorFlags::TABLE_OR_PAGE.0,
        )
    }

    /// Creates a block descriptor mapping a 1 GiB or 2 MiB physical region.
    #[must_use]
    pub const fn new_block(phys_addr: u64, flags: ArmDescriptorFlags) -> Self {
        Self((phys_addr & Self::OUTPUT_ADDRESS_MASK) | flags.0 | ArmDescriptorFlags::VALID.0)
    }

    /// Creates a 4 KiB page descriptor for level 3.
    #[must_use]
    pub const fn new_page(phys_addr: u64, flags: ArmDescriptorFlags) -> Self {
        Self(
            (phys_addr & Self::OUTPUT_ADDRESS_MASK)
                | flags.0
                | ArmDescriptorFlags::VALID.0
                | ArmDescriptorFlags::TABLE_OR_PAGE.0,
        )
    }

    /// Creates an empty (invalid / faulting) descriptor.
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns `true` if the descriptor is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        (self.0 & ArmDescriptorFlags::VALID.0) != 0
    }

    /// Returns `true` if this descriptor points to a next-level table (levels 0-2).
    #[must_use]
    pub const fn is_table(self) -> bool {
        self.is_valid() && ((self.0 & ArmDescriptorFlags::TABLE_OR_PAGE.0) != 0)
    }

    /// Returns the 4 KiB aligned physical address referenced by this descriptor.
    #[must_use]
    pub const fn output_address(self) -> u64 {
        self.0 & Self::OUTPUT_ADDRESS_MASK
    }

    /// Clears the descriptor to zero.
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl core::fmt::Debug for ArmTableDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ArmTableDescriptor")
            .field(
                "output_address",
                &format_args!("{:#016x}", self.output_address()),
            )
            .field("valid", &self.is_valid())
            .field(
                "table_or_page",
                &((self.0 & ArmDescriptorFlags::TABLE_OR_PAGE.0) != 0),
            )
            .field("raw", &format_args!("{:#016x}", self.0))
            .finish()
    }
}

/// Memory Attribute Indirection Register (`MAIR_EL1`) holding 8 attribute slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MairEl1(pub u64);

impl MairEl1 {
    /// Default firmware MAIR configuration:
    /// - Slot 0: Device-nGnRnE (`0x00`)
    /// - Slot 1: Device-nGnRE (`0x04`)
    /// - Slot 2: Normal Non-Cacheable (`0x44`)
    /// - Slot 3: Normal Outer & Inner Write-Back (`0xFF`)
    pub const DEFAULT: Self = Self(
        (0x00 << (MAIR_ATTR_DEVICE_NGNRNE * 8))
            | (0x04 << (MAIR_ATTR_DEVICE_NGNRE * 8))
            | (0x44 << (MAIR_ATTR_NORMAL_NON_CACHEABLE * 8))
            | (0xFF << (MAIR_ATTR_NORMAL_WRITE_BACK * 8)),
    );

    /// Constructs a `MAIR_EL1` register with a specific attribute byte at `slot` (0..7).
    #[must_use]
    pub const fn with_slot(self, slot: usize, attr: u8) -> Self {
        let mask = !(0xFF << (slot * 8));
        Self((self.0 & mask) | ((attr as u64) << (slot * 8)))
    }
}

/// Translation Control Register (`TCR_EL1`) for 48-bit virtual addressing with 4 KiB granule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcrEl1(pub u64);

impl TcrEl1 {
    /// Standard 48-bit virtual address configuration:
    /// - T0SZ = 16 (64 - 48 = 16)
    /// - TG0 = 0b00 (4 KiB granule)
    /// - SH0 = 0b11 (Inner Shareable)
    /// - ORGN0 = 0b01 (Normal Outer Write-Back Cacheable)
    /// - IRGN0 = 0b01 (Normal Inner Write-Back Cacheable)
    /// - IPS = 0b101 (48-bit Intermediate Physical Address size)
    pub const DEFAULT_48BIT: Self = Self(
        16 // T0SZ
        | (0b01 << 8)  // IRGN0: Normal WB
        | (0b01 << 10) // ORGN0: Normal WB
        | (0b11 << 12) // SH0: Inner Shareable
        // TG0: 0b00 (4 KiB granule)
        | (0b101 << 32), // IPS: 48-bit PA
    );
}

/// Reads the Memory Attribute Indirection Register (`MAIR_EL1`).
#[must_use]
pub fn read_mair_el1() -> MairEl1 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, mair_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        MairEl1(raw)
    }
    #[cfg(not(target_arch = "aarch64"))]
    MairEl1::DEFAULT
}

/// Writes `MAIR_EL1`.
///
/// # Safety
///
/// Modifying memory attribute slots alters cacheability and hardware ordering rules.
pub unsafe fn write_mair_el1(mair: MairEl1) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("msr mair_el1, {}", in(reg) mair.0, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = mair;
}

/// Reads the Translation Control Register (`TCR_EL1`).
#[must_use]
pub fn read_tcr_el1() -> TcrEl1 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, tcr_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        TcrEl1(raw)
    }
    #[cfg(not(target_arch = "aarch64"))]
    TcrEl1::DEFAULT_48BIT
}

/// Writes `TCR_EL1`.
///
/// # Safety
///
/// Modifying `TCR_EL1` changes translation granules and addressing bounds.
pub unsafe fn write_tcr_el1(tcr: TcrEl1) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("msr tcr_el1, {}", in(reg) tcr.0, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = tcr;
}

/// Reads Translation Table Base Register 0 (`TTBR0_EL1`).
#[must_use]
pub fn read_ttbr0_el1() -> u64 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let raw: u64;
        core::arch::asm!("mrs {}, ttbr0_el1", out(reg) raw, options(nomem, nostack, preserves_flags));
        raw
    }
    #[cfg(not(target_arch = "aarch64"))]
    0
}

/// Writes Translation Table Base Register 0 (`TTBR0_EL1`).
///
/// # Safety
///
/// Modifying `TTBR0_EL1` sets the active stage 1 root translation table base.
pub unsafe fn write_ttbr0_el1(table_base: u64) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("msr ttbr0_el1, {}", in(reg) table_base, options(nomem, nostack, preserves_flags));
        core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = table_base;
}

/// Invalidates all Stage 1 translation table entries at EL1 across all inner-shareable cores (`TLBI VMALLE1IS`).
pub fn tlbi_vmalle1() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "dsb ishst",
            "tlbi vmalle1is",
            "dsb ish",
            "isb",
            options(nostack, preserves_flags)
        );
    }
}

/// Invalidates TLB entries matching a virtual address for all ASIDs at EL1 (`TLBI VAAE1IS`).
pub fn tlbi_vaae1(vaddr: u64) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let page = vaddr >> 12;
        core::arch::asm!(
            "dsb ishst",
            "tlbi vaae1is, {0}",
            "dsb ish",
            "isb",
            in(reg) page,
            options(nostack, preserves_flags)
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = vaddr;
}
