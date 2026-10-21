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
