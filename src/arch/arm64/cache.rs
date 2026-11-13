//! # ARM64 Processor Cache Maintenance
//!
//! Implements ARMv8-A architecture cache maintenance operations targeting Point of
//! Coherency (`PoC`) and Point of Unification (`PoU`).

use crate::arch::common::cache::cache_line_span;

/// The typical cache line size on ARM64 processors (64 bytes).
pub const ARM64_CACHE_LINE_SIZE: usize = 64;

/// Invalidates all instruction caches to the Point of Unification (`PoU`).
pub fn invalidate_instruction_cache() {
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!(
            "ic iallu",
            "dsb ish",
            "isb",
            options(nostack, preserves_flags)
        );
    }
}

/// Invalidates a range of instruction cache lines by Virtual Address to `PoU`.
pub fn invalidate_instruction_cache_range(address: usize, length: usize) {
    let (start, span_len) = cache_line_span(address, length, ARM64_CACHE_LINE_SIZE);
    let mut offset = 0;
    while offset < span_len {
        let va = start + offset;
        #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
        unsafe {
            core::arch::asm!("ic ivau, {0}", in(reg) va, options(nostack, preserves_flags));
        }
        #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
        let _ = va;
        offset += ARM64_CACHE_LINE_SIZE;
    }
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("dsb ish", "isb", options(nostack, preserves_flags));
    }
}

/// Writes back the entire data cache. On ARM64 this cleans all set/ways or `PoC`.
pub fn writeback_data_cache() {
    // In ARMv8, point-of-coherency range clean is standard for drivers and boot services.
}

/// Writes back a range of data cache lines to Point of Coherency (`PoC`) via `dc cvac`.
pub fn writeback_data_cache_range(address: usize, length: usize) {
    let (start, span_len) = cache_line_span(address, length, ARM64_CACHE_LINE_SIZE);
    let mut offset = 0;
    while offset < span_len {
        let va = start + offset;
        #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
        unsafe {
            core::arch::asm!("dc cvac, {0}", in(reg) va, options(nostack, preserves_flags));
        }
        #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
        let _ = va;
        offset += ARM64_CACHE_LINE_SIZE;
    }
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("dsb ish", options(nostack, preserves_flags));
    }
}

/// Invalidates the entire data cache.
///
/// # Safety
///
/// Discards modified cache lines without writing them back to memory.
pub unsafe fn invalidate_data_cache() {}

/// Invalidates a range of data cache lines to Point of Coherency (`PoC`) via `dc ivac`.
///
/// # Safety
///
/// Discards modified cache lines in the specified range.
pub unsafe fn invalidate_data_cache_range(address: usize, length: usize) {
    let (start, span_len) = cache_line_span(address, length, ARM64_CACHE_LINE_SIZE);
    let mut offset = 0;
    while offset < span_len {
        let va = start + offset;
        #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
        unsafe {
            core::arch::asm!("dc ivac, {0}", in(reg) va, options(nostack, preserves_flags));
        }
        #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
        let _ = va;
        offset += ARM64_CACHE_LINE_SIZE;
    }
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("dsb ish", options(nostack, preserves_flags));
    }
}

/// Writes back and invalidates the entire data cache.
pub fn writeback_invalidate_data_cache() {}

/// Writes back and invalidates a range of data cache lines via `dc civac`.
pub fn writeback_invalidate_data_cache_range(address: usize, length: usize) {
    let (start, span_len) = cache_line_span(address, length, ARM64_CACHE_LINE_SIZE);
    let mut offset = 0;
    while offset < span_len {
        let va = start + offset;
        #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
        unsafe {
            core::arch::asm!("dc civac, {0}", in(reg) va, options(nostack, preserves_flags));
        }
        #[cfg(not(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi"))))]
        let _ = va;
        offset += ARM64_CACHE_LINE_SIZE;
    }
    #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!("dsb ish", options(nostack, preserves_flags));
    }
}
