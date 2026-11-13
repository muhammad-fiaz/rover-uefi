//! # RISC-V Sv39 and Sv48 MMU and Paging
//!
//! Provides page table entry bitfields, Sv39 virtual address decomposition,
//! and page directory structures for 64-bit RISC-V firmware systems.

/// Standard 4 KiB small page size in bytes.
pub const RISCV_PAGE_SIZE_4K: usize = 4096;
/// 2 MiB megapage size in bytes.
pub const RISCV_PAGE_SIZE_2M: usize = 2 * 1024 * 1024;
/// 1 GiB gigapage size in bytes.
pub const RISCV_PAGE_SIZE_1G: usize = 1024 * 1024 * 1024;
/// Number of entries contained in a single 4 KiB RISC-V page table.
pub const RISCV_ENTRIES_PER_TABLE: usize = 512;

/// Bitflags defining permissions and attributes for a RISC-V Page Table Entry (`PTE`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RiscvPteFlags(pub u64);

impl RiscvPteFlags {
    /// Valid bit (`V`, bit 0). If clear, entry causes a page fault.
    pub const VALID: Self = Self(1 << 0);
    /// Readable bit (`R`, bit 1).
    pub const READABLE: Self = Self(1 << 1);
    /// Writable bit (`W`, bit 2).
    pub const WRITABLE: Self = Self(1 << 2);
    /// Executable bit (`X`, bit 3).
    pub const EXECUTABLE: Self = Self(1 << 3);
    /// User accessible bit (`U`, bit 4).
    pub const USER: Self = Self(1 << 4);
    /// Global mapping bit (`G`, bit 5).
    pub const GLOBAL: Self = Self(1 << 5);
    /// Accessed bit (`A`, bit 6).
    pub const ACCESSED: Self = Self(1 << 6);
    /// Dirty bit (`D`, bit 7).
    pub const DIRTY: Self = Self(1 << 7);

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

    /// Returns `true` if this PTE is a leaf page (R=1 or X=1).
    #[must_use]
    pub const fn is_leaf(self) -> bool {
        (self.0 & (Self::READABLE.0 | Self::EXECUTABLE.0)) != 0
    }

    /// Returns `true` if this PTE points to the next-level page table (V=1 and R=W=X=0).
    #[must_use]
    pub const fn is_branch(self) -> bool {
        self.contains(Self::VALID) && !self.is_leaf()
    }
}

impl core::ops::BitOr for RiscvPteFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for RiscvPteFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for RiscvPteFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign for RiscvPteFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// A 64-bit Page Table Entry (`PTE`) in Sv39/Sv48.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RiscvPte(pub u64);

impl RiscvPte {
    const PPN_MASK: u64 = 0x003F_FFFF_FFFF_FC00;

    /// Creates a new page table entry mapping a 44-bit physical page number with given flags.
    #[must_use]
    pub const fn new(ppn: u64, flags: RiscvPteFlags) -> Self {
        Self(((ppn << 10) & Self::PPN_MASK) | flags.0)
    }

    /// Creates a pointer entry referencing a child page table at `physical_address`.
    #[must_use]
    pub const fn new_table(physical_address: u64) -> Self {
        Self::new(physical_address >> 12, RiscvPteFlags::VALID)
    }

    /// Creates a leaf mapping entry referencing physical memory at `physical_address`.
    #[must_use]
    pub const fn new_leaf(physical_address: u64, flags: RiscvPteFlags) -> Self {
        Self::new(physical_address >> 12, flags.union(RiscvPteFlags::VALID))
    }

    /// Creates an empty (invalid) PTE.
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns `true` if this PTE is a leaf translation entry (R=1 or X=1).
    #[must_use]
    pub const fn is_leaf(self) -> bool {
        self.flags().is_leaf()
    }

    /// Returns `true` if this PTE is a branch pointing to a next-level page table.
    #[must_use]
    pub const fn is_branch(self) -> bool {
        self.flags().is_branch()
    }

    /// Returns the Physical Page Number (`PPN`, bits 10..53).
    #[must_use]
    pub const fn ppn(self) -> u64 {
        (self.0 & Self::PPN_MASK) >> 10
    }

    /// Returns the physical base address in bytes (`PPN * 4096`).
    #[must_use]
    pub const fn physical_address(self) -> u64 {
        self.ppn() << 12
    }

    /// Returns the flags configured for this entry.
    #[must_use]
    pub const fn flags(self) -> RiscvPteFlags {
        RiscvPteFlags(self.0 & 0x3FF)
    }

    /// Returns `true` if the entry is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.flags().contains(RiscvPteFlags::VALID)
    }
}

impl core::fmt::Debug for RiscvPte {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RiscvPte")
            .field(
                "physical_address",
                &format_args!("{:#016x}", self.physical_address()),
            )
            .field("is_valid", &self.is_valid())
            .field("is_leaf", &self.flags().is_leaf())
            .field("flags", &self.flags())
            .finish()
    }
}

/// Extracts the VPN[2] index (level 2) from an Sv39 virtual address (bits 30..38).
#[must_use]
#[inline]
pub const fn sv39_vpn2_index(vaddr: usize) -> usize {
    (vaddr >> 30) & 0x1FF
}

/// Extracts the VPN[1] index (level 1) from an Sv39 virtual address (bits 21..29).
#[must_use]
#[inline]
pub const fn sv39_vpn1_index(vaddr: usize) -> usize {
    (vaddr >> 21) & 0x1FF
}

/// Extracts the VPN[0] index (level 0) from an Sv39 virtual address (bits 12..20).
#[must_use]
#[inline]
pub const fn sv39_vpn0_index(vaddr: usize) -> usize {
    (vaddr >> 12) & 0x1FF
}

/// Extracts the 4 KiB page offset (bits 0..11) from an Sv39 virtual address.
#[must_use]
#[inline]
pub const fn sv39_page_offset(vaddr: usize) -> usize {
    vaddr & 0xFFF
}
