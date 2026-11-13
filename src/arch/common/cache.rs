//! # Cache Maintenance Architecture Independent Primitives
//!
//! Provides platform-agnostic abstractions and utilities for maintaining processor
//! instruction and data caches, coherency domains, and DMA buffer alignment.

/// Default cache line size assumed for general-purpose 64-bit architectures (64 bytes).
pub const CACHE_LINE_SIZE_DEFAULT: usize = 64;

/// Align an address down to the nearest cache line boundary.
#[must_use]
#[inline]
pub const fn cache_line_align_down(addr: usize, line_size: usize) -> usize {
    addr & !(line_size - 1)
}

/// Align an address up to the nearest cache line boundary.
#[must_use]
#[inline]
pub const fn cache_line_align_up(addr: usize, line_size: usize) -> usize {
    (addr + line_size - 1) & !(line_size - 1)
}

/// Computes the span of cache lines covered by `[address, address + length)`.
///
/// Returns `(aligned_base, aligned_length)` aligned to `line_size`.
#[must_use]
pub fn cache_line_span(address: usize, length: usize, line_size: usize) -> (usize, usize) {
    if length == 0 {
        return (address, 0);
    }
    let start = cache_line_align_down(address, line_size);
    let end = cache_line_align_up(address.saturating_add(length), line_size);
    (start, end.saturating_sub(start))
}

/// Invalidates the entire instruction cache in the coherency domain of the calling processor.
pub fn invalidate_instruction_cache() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    crate::arch::x86::cache::invalidate_instruction_cache();

    #[cfg(target_arch = "aarch64")]
    crate::arch::arm64::cache::invalidate_instruction_cache();
}

/// Invalidates a range of instruction cache lines.
pub fn invalidate_instruction_cache_range(address: usize, length: usize) {
    if length == 0 {
        return;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    crate::arch::x86::cache::invalidate_instruction_cache_range(address, length);

    #[cfg(target_arch = "aarch64")]
    crate::arch::arm64::cache::invalidate_instruction_cache_range(address, length);

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = (address, length);
}

/// Writes back the entire data cache to main memory.
pub fn writeback_data_cache() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    crate::arch::x86::cache::writeback_data_cache();

    #[cfg(target_arch = "aarch64")]
    crate::arch::arm64::cache::writeback_data_cache();
}

/// Writes back a range of data cache lines to main memory.
pub fn writeback_data_cache_range(address: usize, length: usize) {
    if length == 0 {
        return;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    crate::arch::x86::cache::writeback_data_cache_range(address, length);

    #[cfg(target_arch = "aarch64")]
    crate::arch::arm64::cache::writeback_data_cache_range(address, length);

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = (address, length);
}

/// Invalidates the entire data cache.
///
/// # Safety
///
/// Dirty cache lines are discarded without being written back to memory.
/// This function must be used with caution, typically for diagnostics.
pub unsafe fn invalidate_data_cache() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        crate::arch::x86::cache::invalidate_data_cache();
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        crate::arch::arm64::cache::invalidate_data_cache();
    }
}

/// Invalidates a range of data cache lines.
///
/// # Safety
///
/// Dirty cache lines in the specified range are discarded without being written back.
pub unsafe fn invalidate_data_cache_range(address: usize, length: usize) {
    if length == 0 {
        return;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        crate::arch::x86::cache::invalidate_data_cache_range(address, length);
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        crate::arch::arm64::cache::invalidate_data_cache_range(address, length);
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = (address, length);
}

/// Writes back and invalidates the entire data cache.
pub fn writeback_invalidate_data_cache() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    crate::arch::x86::cache::writeback_invalidate_data_cache();

    #[cfg(target_arch = "aarch64")]
    crate::arch::arm64::cache::writeback_invalidate_data_cache();
}

/// Writes back and invalidates a range of data cache lines.
pub fn writeback_invalidate_data_cache_range(address: usize, length: usize) {
    if length == 0 {
        return;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    crate::arch::x86::cache::writeback_invalidate_data_cache_range(address, length);

    #[cfg(target_arch = "aarch64")]
    crate::arch::arm64::cache::writeback_invalidate_data_cache_range(address, length);

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = (address, length);
}
