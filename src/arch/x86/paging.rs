//! # x86-64 MMU and Paging Structures
//!
//! Provides 4-level and 5-level page table structures, entry flags, virtual address
//! decomposition, translation table manipulation, and Translation Lookaside Buffer (`TLB`)
//! invalidation primitives.

/// Page size in bytes for a standard 4 KiB small page.
pub const PAGE_SIZE_4K: usize = 4096;
/// Page size in bytes for a 2 MiB large page.
pub const PAGE_SIZE_2M: usize = 2 * 1024 * 1024;
/// Page size in bytes for a 1 GiB huge page.
pub const PAGE_SIZE_1G: usize = 1024 * 1024 * 1024;
/// Number of entries contained in a single page table (PML5, PML4, PDPT, PD, or PT).
pub const PAGE_ENTRIES_PER_TABLE: usize = 512;

/// Operating modes for the x86 processor paging unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PagingMode {
    /// Legacy 32-bit two-level paging without PAE.
    Paging32Bit,
    /// 32-bit Physical Address Extension (PAE) three-level paging.
    PagingPae,
    /// Standard 64-bit 4-level paging (48-bit canonical virtual address space).
    Paging4Level,
    /// Extended 64-bit 5-level paging (57-bit virtual address space, `LA57`).
    Paging5Level,
}

/// Architectural bitflags defining attributes for page table entries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PageTableFlags(pub u64);

impl PageTableFlags {
    /// Present (`P`): page or table is present in physical memory.
    pub const PRESENT: Self = Self(1 << 0);
    /// Read/Write (`R/W`): page is writable if set, read-only if clear.
    pub const WRITABLE: Self = Self(1 << 1);
    /// User/Supervisor (`U/S`): page is accessible by ring 3 user code.
    pub const USER_ACCESSIBLE: Self = Self(1 << 2);
    /// Page-level Write-Through (`PWT`).
    pub const WRITE_THROUGH: Self = Self(1 << 3);
    /// Page-level Cache Disable (`PCD`).
    pub const NO_CACHE: Self = Self(1 << 4);
    /// Accessed (`A`): CPU sets this bit when the entry is used in translation.
    pub const ACCESSED: Self = Self(1 << 5);
    /// Dirty (`D`): CPU sets this bit upon write access to a leaf page.
    pub const DIRTY: Self = Self(1 << 6);
    /// Huge Page (`PS`): indicates a 2 MiB page in PD or 1 GiB page in PDPT.
    pub const HUGE_PAGE: Self = Self(1 << 7);
    /// Global (`G`): prevents eviction from TLB on CR3 reloads when CR4.PGE is enabled.
    pub const GLOBAL: Self = Self(1 << 8);
    /// Page Attribute Table index bit (`PAT`) for 4 KiB leaf pages.
    pub const PAT: Self = Self(1 << 7);
    /// Page Attribute Table index bit for huge pages (bit 12).
    pub const HUGE_PAT: Self = Self(1 << 12);
    /// No-Execute (`NX` / `XD`): prevents code execution from this page.
    pub const NO_EXECUTE: Self = Self(1 << 63);

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

    /// Returns the raw 64-bit integer value.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

/// An entry in an x86-64 page table.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    const ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;

    /// Creates a new page table entry mapping `address` with given `flags`.
    #[must_use]
    pub const fn new(address: u64, flags: PageTableFlags) -> Self {
        Self((address & Self::ADDRESS_MASK) | flags.0)
    }

    /// Creates an empty (not present) page table entry.
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the 4 KiB aligned physical address referenced by this entry.
    #[must_use]
    pub const fn address(self) -> u64 {
        self.0 & Self::ADDRESS_MASK
    }

    /// Returns the flags configured for this entry.
    #[must_use]
    pub const fn flags(self) -> PageTableFlags {
        PageTableFlags(self.0 & !Self::ADDRESS_MASK)
    }

    /// Returns `true` if the entry has the Present bit asserted.
    #[must_use]
    pub const fn is_present(self) -> bool {
        (self.0 & PageTableFlags::PRESENT.0) != 0
    }

    /// Returns `true` if the entry has the Writable bit asserted.
    #[must_use]
    pub const fn is_writable(self) -> bool {
        (self.0 & PageTableFlags::WRITABLE.0) != 0
    }

    /// Returns `true` if this entry designates a huge leaf page (2 MiB or 1 GiB).
    #[must_use]
    pub const fn is_huge(self) -> bool {
        (self.0 & PageTableFlags::HUGE_PAGE.0) != 0
    }

    /// Sets the physical address pointed to by this entry, preserving flags.
    pub fn set_address(&mut self, address: u64) {
        self.0 = (address & Self::ADDRESS_MASK) | (self.0 & !Self::ADDRESS_MASK);
    }

    /// Sets the flags for this entry, preserving the physical address.
    pub fn set_flags(&mut self, flags: PageTableFlags) {
        self.0 = (self.0 & Self::ADDRESS_MASK) | (flags.0 & !Self::ADDRESS_MASK);
    }

    /// Clears the entry completely to zero.
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageTableEntry")
            .field("address", &format_args!("{:#016x}", self.address()))
            .field("present", &self.is_present())
            .field("writable", &self.is_writable())
            .field("huge", &self.is_huge())
            .field("flags", &self.flags())
            .finish()
    }
}

/// A 4 KiB aligned x86-64 page table containing 512 entries.
#[repr(C, align(4096))]
pub struct PageTable {
    /// The 512 entries of the table.
    pub entries: [PageTableEntry; PAGE_ENTRIES_PER_TABLE],
}

impl PageTable {
    /// Creates a completely zeroed, empty page table.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            entries: [PageTableEntry(0); PAGE_ENTRIES_PER_TABLE],
        }
    }

    /// Clears all entries in this table.
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            entry.clear();
        }
    }
}

impl core::ops::Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl core::ops::IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

/// Extracts the PML5 index (level 5) from a 57-bit virtual address (bits 48..56).
#[must_use]
#[inline]
pub const fn pml5_index(vaddr: u64) -> usize {
    ((vaddr >> 48) & 0x1FF) as usize
}

/// Extracts the PML4 index (level 4) from a virtual address (bits 39..47).
#[must_use]
#[inline]
pub const fn pml4_index(vaddr: u64) -> usize {
    ((vaddr >> 39) & 0x1FF) as usize
}

/// Extracts the Page Directory Pointer Table (`PDPT`) index (level 3) from a virtual address (bits 30..38).
#[must_use]
#[inline]
pub const fn pdpt_index(vaddr: u64) -> usize {
    ((vaddr >> 30) & 0x1FF) as usize
}

/// Extracts the Page Directory (`PD`) index (level 2) from a virtual address (bits 21..29).
#[must_use]
#[inline]
pub const fn pd_index(vaddr: u64) -> usize {
    ((vaddr >> 21) & 0x1FF) as usize
}

/// Extracts the Page Table (`PT`) index (level 1) from a virtual address (bits 12..20).
#[must_use]
#[inline]
pub const fn pt_index(vaddr: u64) -> usize {
    ((vaddr >> 12) & 0x1FF) as usize
}

/// Extracts the 4 KiB page offset (bits 0..11) from a virtual address.
#[must_use]
#[inline]
pub const fn page_offset_4k(vaddr: u64) -> usize {
    (vaddr & 0xFFF) as usize
}

/// Extracts the 2 MiB page offset (bits 0..20) from a virtual address.
#[must_use]
#[inline]
pub const fn page_offset_2m(vaddr: u64) -> usize {
    (vaddr & 0x1F_FFFF) as usize
}

/// Extracts the 1 GiB page offset (bits 0..29) from a virtual address.
#[must_use]
#[inline]
pub const fn page_offset_1g(vaddr: u64) -> usize {
    (vaddr & 0x3FFF_FFFF) as usize
}

/// Canonicalizes a 48-bit virtual address by sign-extending bit 47 to bits 48..63.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub const fn canonicalize_48(vaddr: u64) -> u64 {
    ((vaddr << 16) as i64 >> 16) as u64
}

/// Canonicalizes a 57-bit virtual address by sign-extending bit 56 to bits 57..63.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub const fn canonicalize_57(vaddr: u64) -> u64 {
    ((vaddr << 7) as i64 >> 7) as u64
}

/// Invalidates the TLB entry corresponding to the virtual address `vaddr` (`INVLPG`).
#[inline]
pub fn invlpg(vaddr: u64) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("invlpg [{0}]", in(reg) vaddr, options(nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = vaddr;
}

/// Flushes all non-global TLB entries by reloading the `CR3` register.
pub fn flush_tlb() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let cr3 = super::control_regs::read_cr3();
        super::control_regs::write_cr3(cr3);
    }
}
