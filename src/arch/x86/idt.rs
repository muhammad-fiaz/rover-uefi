//! # x86-64 Interrupt Descriptor Table (IDT)
//!
//! Provides the 64-bit Interrupt Descriptor Table (`IDT`), gate descriptors,
//! privilege gate attributes, and `LIDT`/`SIDT` instruction abstractions.

use super::gdt::{DescriptorTablePointer, SegmentSelector};

/// Total number of interrupt and exception entries in the x86 IDT.
pub const IDT_ENTRY_COUNT: usize = 256;

/// Gate type for a 64-bit Interrupt Gate (clears IF on entry).
pub const GATE_TYPE_INTERRUPT64: u8 = 0x0E;
/// Gate type for a 64-bit Trap Gate (preserves IF on entry).
pub const GATE_TYPE_TRAP64: u8 = 0x0F;

/// Attribute bit indicating the gate is present in memory.
pub const GATE_PRESENT: u8 = 1 << 7;

/// A 16-byte Interrupt Gate Descriptor in 64-bit mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct IdtEntry {
    /// Target handler address bits 0..15.
    pub offset_low: u16,
    /// Code segment selector to load into CS.
    pub selector: SegmentSelector,
    /// Interrupt Stack Table index (bits 0..2) and reserved zero bits (3..7).
    pub ist: u8,
    /// Type and attributes (Present, DPL, Gate Type).
    pub type_attr: u8,
    /// Target handler address bits 16..31.
    pub offset_mid: u16,
    /// Target handler address bits 32..63.
    pub offset_high: u32,
    _reserved: u32,
}

impl IdtEntry {
    /// Creates an empty, non-present IDT entry.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            offset_low: 0,
            selector: SegmentSelector(0),
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            _reserved: 0,
        }
    }

    /// Creates an active 64-bit interrupt gate descriptor.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new_interrupt(
        handler_address: usize,
        code_selector: SegmentSelector,
        dpl: u8,
        ist_index: u8,
    ) -> Self {
        let addr = handler_address as u64;
        let type_attr = GATE_PRESENT | ((dpl & 0x03) << 5) | GATE_TYPE_INTERRUPT64;
        Self {
            offset_low: (addr & 0xFFFF) as u16,
            selector: code_selector,
            ist: ist_index & 0x07,
            type_attr,
            offset_mid: ((addr >> 16) & 0xFFFF) as u16,
            offset_high: (addr >> 32) as u32,
            _reserved: 0,
        }
    }

    /// Safely reads the segment selector without creating unaligned references.
    #[must_use]
    pub const fn selector(&self) -> SegmentSelector {
        unsafe { core::ptr::addr_of!(self.selector).read_unaligned() }
    }

    /// Safely reads the IST index.
    #[must_use]
    pub const fn ist(&self) -> u8 {
        unsafe { core::ptr::addr_of!(self.ist).read_unaligned() }
    }

    /// Safely reads the type and attribute byte.
    #[must_use]
    pub const fn type_attr(&self) -> u8 {
        unsafe { core::ptr::addr_of!(self.type_attr).read_unaligned() }
    }

    /// Returns `true` if the entry is marked present.
    #[must_use]
    pub const fn is_present(&self) -> bool {
        (self.type_attr() & GATE_PRESENT) != 0
    }

    /// Returns the target Descriptor Privilege Level (`DPL`, 0..3).
    #[must_use]
    pub const fn dpl(&self) -> u8 {
        (self.type_attr() >> 5) & 0x03
    }

    /// Returns the complete 64-bit handler address pointed to by this descriptor.
    #[must_use]
    pub fn handler_address(&self) -> u64 {
        let low = unsafe { core::ptr::addr_of!(self.offset_low).read_unaligned() };
        let mid = unsafe { core::ptr::addr_of!(self.offset_mid).read_unaligned() };
        let high = unsafe { core::ptr::addr_of!(self.offset_high).read_unaligned() };
        u64::from(low) | (u64::from(mid) << 16) | (u64::from(high) << 32)
    }
}

/// The 256-entry Interrupt Descriptor Table for x86-64.
#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Idt {
    /// Array of 256 interrupt and trap gate descriptors.
    pub entries: [IdtEntry; IDT_ENTRY_COUNT],
}

impl Default for Idt {
    fn default() -> Self {
        Self::new()
    }
}

impl Idt {
    /// Creates a completely zeroed, empty IDT.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::empty(); IDT_ENTRY_COUNT],
        }
    }

    /// Creates a pointer structure for loading via `lidt`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: core::ptr::from_ref(self).cast::<()>() as usize as u64,
            limit: (size_of::<Self>() - 1) as u16,
        }
    }

    /// Configures an interrupt gate at `vector`.
    pub fn set_handler(
        &mut self,
        vector: u8,
        handler: usize,
        code_selector: SegmentSelector,
        ist: u8,
    ) {
        self.entries[vector as usize] = IdtEntry::new_interrupt(handler, code_selector, 0, ist);
    }

    /// Returns a reference to the entry for `vector`.
    #[must_use]
    pub fn get(&self, vector: u8) -> &IdtEntry {
        &self.entries[vector as usize]
    }

    /// Returns a mutable reference to the entry for `vector`.
    pub fn get_mut(&mut self, vector: u8) -> &mut IdtEntry {
        &mut self.entries[vector as usize]
    }
}

impl core::ops::Index<usize> for Idt {
    type Output = IdtEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl core::ops::IndexMut<usize> for Idt {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

/// Loads the Interrupt Descriptor Table register (`LIDT`).
///
/// # Safety
///
/// Pointing `idtr` to an invalid table can cause fatal processor faults on interrupts.
pub unsafe fn lidt(idtr: &DescriptorTablePointer) {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) idtr, options(readonly, nostack, preserves_flags));
    }
    #[cfg(not(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none"))))]
    let _ = idtr;
}

/// Stores the Interrupt Descriptor Table register (`SIDT`).
#[must_use]
pub fn sidt() -> DescriptorTablePointer {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let mut idtr = DescriptorTablePointer::default();
        core::arch::asm!("sidt [{}]", in(reg) &raw mut idtr, options(nostack, preserves_flags));
        idtr
    }
    #[cfg(not(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none"))))]
    DescriptorTablePointer::default()
}
