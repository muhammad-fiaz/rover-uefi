//! # Memory-mapped input and output
//!
//! Volatile helpers for device registers that are mapped into
//! the physical address space.
//!
//! Device registers are not ordinary memory. A read can clear a
//! status bit or acknowledge an event, and the compiler must not
//! cache, duplicate or reorder the accesses. Every helper in
//! this module therefore uses the volatile read and write
//! operations provided by `core::ptr`.
//!
//! The helpers take the register address as a plain integer.
//! The caller stays responsible for knowing that the address is
//! mapped and that the access width matches the register. Each
//! helper is therefore an unsafe function.

use core::ptr::{read_volatile, write_volatile};

/// Reads an 8-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be read
/// with 8-bit granularity. The location must be valid for the
/// duration of the access.
#[must_use]
pub unsafe fn read_u8(address: usize) -> u8 {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for an 8-bit volatile read.
    unsafe { read_volatile(address as *const u8) }
}

/// Reads a 16-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be read
/// with 16-bit granularity. The location must be valid for the
/// duration of the access.
#[must_use]
pub unsafe fn read_u16(address: usize) -> u16 {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a 16-bit volatile read.
    unsafe { read_volatile(address as *const u16) }
}

/// Reads a 32-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be read
/// with 32-bit granularity. The location must be valid for the
/// duration of the access.
#[must_use]
pub unsafe fn read_u32(address: usize) -> u32 {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a 32-bit volatile read.
    unsafe { read_volatile(address as *const u32) }
}

/// Reads a 64-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be read
/// with 64-bit granularity. The location must be valid for the
/// duration of the access.
#[must_use]
pub unsafe fn read_u64(address: usize) -> u64 {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a 64-bit volatile read.
    unsafe { read_volatile(address as *const u64) }
}

/// Reads a register value with the native pointer width.
///
/// On a 32-bit target the access is 32 bits wide, on a 64-bit
/// target it is 64 bits wide.
///
/// # Safety
///
/// `address` must denote a mapped location that can be read
/// with pointer granularity. The location must be valid for the
/// duration of the access.
#[must_use]
pub unsafe fn read_usize(address: usize) -> usize {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a pointer-sized volatile read.
    unsafe { read_volatile(address as *const usize) }
}

/// Writes an 8-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be written
/// with 8-bit granularity. The location must be valid for the
/// duration of the access.
pub unsafe fn write_u8(address: usize, value: u8) {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for an 8-bit volatile write.
    unsafe { write_volatile(address as *mut u8, value) }
}

/// Writes a 16-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be written
/// with 16-bit granularity. The location must be valid for the
/// duration of the access.
pub unsafe fn write_u16(address: usize, value: u16) {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a 16-bit volatile write.
    unsafe { write_volatile(address as *mut u16, value) }
}

/// Writes a 32-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be written
/// with 32-bit granularity. The location must be valid for the
/// duration of the access.
pub unsafe fn write_u32(address: usize, value: u32) {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a 32-bit volatile write.
    unsafe { write_volatile(address as *mut u32, value) }
}

/// Writes a 64-bit register value.
///
/// # Safety
///
/// `address` must denote a mapped location that can be written
/// with 64-bit granularity. The location must be valid for the
/// duration of the access.
pub unsafe fn write_u64(address: usize, value: u64) {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a 64-bit volatile write.
    unsafe { write_volatile(address as *mut u64, value) }
}

/// Writes a register value with the native pointer width.
///
/// On a 32-bit target the access is 32 bits wide, on a 64-bit
/// target it is 64 bits wide.
///
/// # Safety
///
/// `address` must denote a mapped location that can be written
/// with pointer granularity. The location must be valid for the
/// duration of the access.
pub unsafe fn write_usize(address: usize, value: usize) {
    // SAFETY: The caller guarantees that the address is mapped
    // and suitable for a pointer-sized volatile write.
    unsafe { write_volatile(address as *mut usize, value) }
}

/// Modifies an 8-bit register by clearing bits specified in `clear_mask` and setting bits in `set_mask`.
///
/// Returns the original value of the register prior to the modification.
///
/// # Safety
///
/// `address` must denote a valid mapped 8-bit register.
#[must_use]
pub unsafe fn modify_u8(address: usize, clear_mask: u8, set_mask: u8) -> u8 {
    let old = unsafe { read_u8(address) };
    let new = (old & !clear_mask) | set_mask;
    unsafe { write_u8(address, new) };
    old
}

/// Bitwise ANDs an 8-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 8-bit register.
#[must_use]
pub unsafe fn and_u8(address: usize, mask: u8) -> u8 {
    unsafe { modify_u8(address, !mask, 0) }
}

/// Bitwise ORs an 8-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 8-bit register.
#[must_use]
pub unsafe fn or_u8(address: usize, mask: u8) -> u8 {
    unsafe { modify_u8(address, 0, mask) }
}

/// Modifies a 16-bit register by clearing bits specified in `clear_mask` and setting bits in `set_mask`.
///
/// Returns the original value of the register prior to the modification.
///
/// # Safety
///
/// `address` must denote a valid mapped 16-bit register.
#[must_use]
pub unsafe fn modify_u16(address: usize, clear_mask: u16, set_mask: u16) -> u16 {
    let old = unsafe { read_u16(address) };
    let new = (old & !clear_mask) | set_mask;
    unsafe { write_u16(address, new) };
    old
}

/// Bitwise ANDs a 16-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 16-bit register.
#[must_use]
pub unsafe fn and_u16(address: usize, mask: u16) -> u16 {
    unsafe { modify_u16(address, !mask, 0) }
}

/// Bitwise ORs a 16-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 16-bit register.
#[must_use]
pub unsafe fn or_u16(address: usize, mask: u16) -> u16 {
    unsafe { modify_u16(address, 0, mask) }
}

/// Modifies a 32-bit register by clearing bits specified in `clear_mask` and setting bits in `set_mask`.
///
/// Returns the original value of the register prior to the modification.
///
/// # Safety
///
/// `address` must denote a valid mapped 32-bit register.
#[must_use]
pub unsafe fn modify_u32(address: usize, clear_mask: u32, set_mask: u32) -> u32 {
    let old = unsafe { read_u32(address) };
    let new = (old & !clear_mask) | set_mask;
    unsafe { write_u32(address, new) };
    old
}

/// Bitwise ANDs a 32-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 32-bit register.
#[must_use]
pub unsafe fn and_u32(address: usize, mask: u32) -> u32 {
    unsafe { modify_u32(address, !mask, 0) }
}

/// Bitwise ORs a 32-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 32-bit register.
#[must_use]
pub unsafe fn or_u32(address: usize, mask: u32) -> u32 {
    unsafe { modify_u32(address, 0, mask) }
}

/// Modifies a 64-bit register by clearing bits specified in `clear_mask` and setting bits in `set_mask`.
///
/// Returns the original value of the register prior to the modification.
///
/// # Safety
///
/// `address` must denote a valid mapped 64-bit register.
#[must_use]
pub unsafe fn modify_u64(address: usize, clear_mask: u64, set_mask: u64) -> u64 {
    let old = unsafe { read_u64(address) };
    let new = (old & !clear_mask) | set_mask;
    unsafe { write_u64(address, new) };
    old
}

/// Bitwise ANDs a 64-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 64-bit register.
#[must_use]
pub unsafe fn and_u64(address: usize, mask: u64) -> u64 {
    unsafe { modify_u64(address, !mask, 0) }
}

/// Bitwise ORs a 64-bit register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped 64-bit register.
#[must_use]
pub unsafe fn or_u64(address: usize, mask: u64) -> u64 {
    unsafe { modify_u64(address, 0, mask) }
}

/// Modifies a pointer-width register by clearing bits specified in `clear_mask` and setting bits in `set_mask`.
///
/// Returns the original value of the register prior to the modification.
///
/// # Safety
///
/// `address` must denote a valid mapped pointer-width register.
#[must_use]
pub unsafe fn modify_usize(address: usize, clear_mask: usize, set_mask: usize) -> usize {
    let old = unsafe { read_usize(address) };
    let new = (old & !clear_mask) | set_mask;
    unsafe { write_usize(address, new) };
    old
}

/// Bitwise ANDs a pointer-width register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped pointer-width register.
#[must_use]
pub unsafe fn and_usize(address: usize, mask: usize) -> usize {
    unsafe { modify_usize(address, !mask, 0) }
}

/// Bitwise ORs a pointer-width register with `mask`.
///
/// Returns the original value prior to the operation.
///
/// # Safety
///
/// `address` must denote a valid mapped pointer-width register.
#[must_use]
pub unsafe fn or_usize(address: usize, mask: usize) -> usize {
    unsafe { modify_usize(address, 0, mask) }
}

/// Prevents the compiler from reordering read and write operations across this boundary.
#[inline]
pub fn compiler_barrier() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/// Full hardware memory barrier ensuring all previous memory accesses (reads and writes)
/// complete before any subsequent memory accesses are initiated.
#[inline]
pub fn memory_barrier() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dmb sy", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!(
            "mcr p15, 0, r0, c7, c10, 5",
            options(nostack, preserves_flags)
        );
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        core::arch::asm!("fence rw, rw", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "loongarch64")]
    unsafe {
        core::arch::asm!("dbar 0", options(nostack, preserves_flags));
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )))]
    compiler_barrier();
}

/// Hardware read memory barrier ensuring all previous read/load operations complete
/// before any subsequent read/load operations are initiated.
#[inline]
pub fn read_memory_barrier() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("lfence", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dmb ld", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!(
            "mcr p15, 0, r0, c7, c10, 5",
            options(nostack, preserves_flags)
        );
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        core::arch::asm!("fence r, r", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "loongarch64")]
    unsafe {
        core::arch::asm!("dbar 0", options(nostack, preserves_flags));
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )))]
    compiler_barrier();
}

/// Hardware write memory barrier ensuring all previous write/store operations complete
/// before any subsequent write/store operations are initiated.
#[inline]
pub fn write_memory_barrier() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("sfence", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dmb st", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!(
            "mcr p15, 0, r0, c7, c10, 5",
            options(nostack, preserves_flags)
        );
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        core::arch::asm!("fence w, w", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "loongarch64")]
    unsafe {
        core::arch::asm!("dbar 0", options(nostack, preserves_flags));
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )))]
    compiler_barrier();
}

/// Strongly typed, zero-overhead accessor for a memory-mapped device register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MmioReg<T> {
    address: usize,
    _marker: core::marker::PhantomData<*mut T>,
}

impl<T> MmioReg<T> {
    /// Creates a new register accessor at the given physical memory address.
    #[must_use]
    pub const fn new(address: usize) -> Self {
        Self {
            address,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the raw memory address of the register.
    #[must_use]
    pub const fn address(&self) -> usize {
        self.address
    }

    /// Computes a new register accessor offset by `bytes` from this register.
    #[must_use]
    pub const fn offset(&self, bytes: usize) -> Self {
        Self {
            address: self.address.wrapping_add(bytes),
            _marker: core::marker::PhantomData,
        }
    }
}

impl MmioReg<u8> {
    /// Volatile read from this 8-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit reads.
    #[must_use]
    pub unsafe fn read(&self) -> u8 {
        unsafe { read_u8(self.address) }
    }

    /// Volatile write to this 8-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit writes.
    pub unsafe fn write(&self, value: u8) {
        unsafe { write_u8(self.address, value) }
    }

    /// Modifies this 8-bit register with clear and set bitmasks.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit read/write.
    #[must_use]
    pub unsafe fn modify(&self, clear_mask: u8, set_mask: u8) -> u8 {
        unsafe { modify_u8(self.address, clear_mask, set_mask) }
    }

    /// Bitwise ANDs this 8-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit read/write.
    #[must_use]
    pub unsafe fn and(&self, mask: u8) -> u8 {
        unsafe { and_u8(self.address, mask) }
    }

    /// Bitwise ORs this 8-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit read/write.
    #[must_use]
    pub unsafe fn or(&self, mask: u8) -> u8 {
        unsafe { or_u8(self.address, mask) }
    }

    /// Extracts a bitfield from this 8-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit reads.
    #[must_use]
    pub unsafe fn read_field(&self, mask: u8, shift: u32) -> u8 {
        let val = unsafe { self.read() };
        (val & mask) >> shift
    }

    /// Writes a bitfield to this 8-bit register while preserving other bits.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 8-bit read/write.
    pub unsafe fn write_field(&self, mask: u8, shift: u32, field_value: u8) {
        let set_mask = (field_value << shift) & mask;
        unsafe {
            let _ = self.modify(mask, set_mask);
        }
    }
}

impl MmioReg<u16> {
    /// Volatile read from this 16-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit reads.
    #[must_use]
    pub unsafe fn read(&self) -> u16 {
        unsafe { read_u16(self.address) }
    }

    /// Volatile write to this 16-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit writes.
    pub unsafe fn write(&self, value: u16) {
        unsafe { write_u16(self.address, value) }
    }

    /// Modifies this 16-bit register with clear and set bitmasks.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit read/write.
    #[must_use]
    pub unsafe fn modify(&self, clear_mask: u16, set_mask: u16) -> u16 {
        unsafe { modify_u16(self.address, clear_mask, set_mask) }
    }

    /// Bitwise ANDs this 16-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit read/write.
    #[must_use]
    pub unsafe fn and(&self, mask: u16) -> u16 {
        unsafe { and_u16(self.address, mask) }
    }

    /// Bitwise ORs this 16-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit read/write.
    #[must_use]
    pub unsafe fn or(&self, mask: u16) -> u16 {
        unsafe { or_u16(self.address, mask) }
    }

    /// Extracts a bitfield from this 16-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit reads.
    #[must_use]
    pub unsafe fn read_field(&self, mask: u16, shift: u32) -> u16 {
        let val = unsafe { self.read() };
        (val & mask) >> shift
    }

    /// Writes a bitfield to this 16-bit register while preserving other bits.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 16-bit read/write.
    pub unsafe fn write_field(&self, mask: u16, shift: u32, field_value: u16) {
        let set_mask = (field_value << shift) & mask;
        unsafe {
            let _ = self.modify(mask, set_mask);
        }
    }
}

impl MmioReg<u32> {
    /// Volatile read from this 32-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit reads.
    #[must_use]
    pub unsafe fn read(&self) -> u32 {
        unsafe { read_u32(self.address) }
    }

    /// Volatile write to this 32-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit writes.
    pub unsafe fn write(&self, value: u32) {
        unsafe { write_u32(self.address, value) }
    }

    /// Modifies this 32-bit register with clear and set bitmasks.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit read/write.
    #[must_use]
    pub unsafe fn modify(&self, clear_mask: u32, set_mask: u32) -> u32 {
        unsafe { modify_u32(self.address, clear_mask, set_mask) }
    }

    /// Bitwise ANDs this 32-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit read/write.
    #[must_use]
    pub unsafe fn and(&self, mask: u32) -> u32 {
        unsafe { and_u32(self.address, mask) }
    }

    /// Bitwise ORs this 32-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit read/write.
    #[must_use]
    pub unsafe fn or(&self, mask: u32) -> u32 {
        unsafe { or_u32(self.address, mask) }
    }

    /// Extracts a bitfield from this 32-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit reads.
    #[must_use]
    pub unsafe fn read_field(&self, mask: u32, shift: u32) -> u32 {
        let val = unsafe { self.read() };
        (val & mask) >> shift
    }

    /// Writes a bitfield to this 32-bit register while preserving other bits.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 32-bit read/write.
    pub unsafe fn write_field(&self, mask: u32, shift: u32, field_value: u32) {
        let set_mask = (field_value << shift) & mask;
        unsafe {
            let _ = self.modify(mask, set_mask);
        }
    }
}

impl MmioReg<u64> {
    /// Volatile read from this 64-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit reads.
    #[must_use]
    pub unsafe fn read(&self) -> u64 {
        unsafe { read_u64(self.address) }
    }

    /// Volatile write to this 64-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit writes.
    pub unsafe fn write(&self, value: u64) {
        unsafe { write_u64(self.address, value) }
    }

    /// Modifies this 64-bit register with clear and set bitmasks.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit read/write.
    #[must_use]
    pub unsafe fn modify(&self, clear_mask: u64, set_mask: u64) -> u64 {
        unsafe { modify_u64(self.address, clear_mask, set_mask) }
    }

    /// Bitwise ANDs this 64-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit read/write.
    #[must_use]
    pub unsafe fn and(&self, mask: u64) -> u64 {
        unsafe { and_u64(self.address, mask) }
    }

    /// Bitwise ORs this 64-bit register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit read/write.
    #[must_use]
    pub unsafe fn or(&self, mask: u64) -> u64 {
        unsafe { or_u64(self.address, mask) }
    }

    /// Extracts a bitfield from this 64-bit register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit reads.
    #[must_use]
    pub unsafe fn read_field(&self, mask: u64, shift: u32) -> u64 {
        let val = unsafe { self.read() };
        (val & mask) >> shift
    }

    /// Writes a bitfield to this 64-bit register while preserving other bits.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for 64-bit read/write.
    pub unsafe fn write_field(&self, mask: u64, shift: u32, field_value: u64) {
        let set_mask = (field_value << shift) & mask;
        unsafe {
            let _ = self.modify(mask, set_mask);
        }
    }
}

impl MmioReg<usize> {
    /// Volatile read from this pointer-width register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for pointer-width reads.
    #[must_use]
    pub unsafe fn read(&self) -> usize {
        unsafe { read_usize(self.address) }
    }

    /// Volatile write to this pointer-width register.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for pointer-width writes.
    pub unsafe fn write(&self, value: usize) {
        unsafe { write_usize(self.address, value) }
    }

    /// Modifies this pointer-width register with clear and set bitmasks.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for pointer-width read/write.
    #[must_use]
    pub unsafe fn modify(&self, clear_mask: usize, set_mask: usize) -> usize {
        unsafe { modify_usize(self.address, clear_mask, set_mask) }
    }

    /// Bitwise ANDs this pointer-width register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for pointer-width read/write.
    #[must_use]
    pub unsafe fn and(&self, mask: usize) -> usize {
        unsafe { and_usize(self.address, mask) }
    }

    /// Bitwise ORs this pointer-width register with `mask`.
    ///
    /// # Safety
    ///
    /// Address must be mapped and valid for pointer-width read/write.
    #[must_use]
    pub unsafe fn or(&self, mask: usize) -> usize {
        unsafe { or_usize(self.address, mask) }
    }
}
