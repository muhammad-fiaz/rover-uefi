//! # x86-64 Processor Cache Maintenance
//!
//! Provides x86 hardware cache operations including line flushes (`CLFLUSH`,
//! `CLFLUSHOPT`, `CLWB`) and entire cache flush/invalidation (`WBINVD`, `INVD`).

use crate::arch::common::cache::cache_line_span;

/// The typical cache line size on x86 processors (64 bytes).
pub const X86_CACHE_LINE_SIZE: usize = 64;

/// Writes back and invalidates all internal and external caches (`WBINVD`).
///
/// Ensures all dirty modified cache lines are committed to system memory,
/// then invalidates the caches.
pub fn writeback_invalidate_data_cache() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("wbinvd", options(nostack, preserves_flags));
    }
}

/// Flushes and invalidates instruction cache. On x86, instruction and data caches
/// are snooped, but pipeline serialization is performed.
pub fn invalidate_instruction_cache() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // Serializing instruction (CPUID or self-modifying code serialization)
        core::arch::asm!(
            "push rbx",
            "mov eax, 0",
            "cpuid",
            "pop rbx",
            out("eax") _,
            out("ecx") _,
            out("edx") _,
            options(nostack, preserves_flags)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!(
            "push ebx",
            "mov eax, 0",
            "cpuid",
            "pop ebx",
            out("eax") _,
            out("ecx") _,
            out("edx") _,
            options(nostack, preserves_flags)
        );
    }
}

/// Invalidates a range of instruction cache lines.
pub fn invalidate_instruction_cache_range(address: usize, length: usize) {
    writeback_invalidate_data_cache_range(address, length);
    invalidate_instruction_cache();
}

/// Writes back the entire data cache.
pub fn writeback_data_cache() {
    writeback_invalidate_data_cache();
}

/// Invalidates the entire data cache without writeback (`INVD`).
///
/// # Safety
///
/// This discards dirty cache lines without saving modifications to RAM.
pub unsafe fn invalidate_data_cache() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("invd", options(nostack, preserves_flags));
    }
}

/// Writes back and invalidates a range of cache lines using `CLFLUSH`.
pub fn writeback_invalidate_data_cache_range(address: usize, length: usize) {
    let (start, span_len) = cache_line_span(address, length, X86_CACHE_LINE_SIZE);
    let mut offset = 0;
    while offset < span_len {
        let ptr = (start + offset) as *const u8;
        clflush(ptr);
        offset += X86_CACHE_LINE_SIZE;
    }
    crate::sync::barrier::memory_barrier();
}

/// Writes back a range of cache lines without necessarily evicting them (using `CLWB` if supported, or `CLFLUSH`).
pub fn writeback_data_cache_range(address: usize, length: usize) {
    let (start, span_len) = cache_line_span(address, length, X86_CACHE_LINE_SIZE);
    let mut offset = 0;
    while offset < span_len {
        let ptr = (start + offset) as *const u8;
        clflush(ptr);
        offset += X86_CACHE_LINE_SIZE;
    }
    crate::sync::barrier::memory_barrier();
}

/// Invalidates a range of data cache lines.
///
/// # Safety
///
/// On x86 without writeback, discarding dirty lines requires careful diagnosis.
/// Uses `CLFLUSH` safely.
pub unsafe fn invalidate_data_cache_range(address: usize, length: usize) {
    writeback_invalidate_data_cache_range(address, length);
}

/// Flushes the cache line containing `ptr` from all cache levels (`CLFLUSH`).
#[inline]
pub fn clflush(ptr: *const u8) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("clflush [{0}]", in(reg) ptr, options(nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    let _ = ptr;
}

/// Flushes the cache line containing `ptr` using optimized `CLFLUSHOPT`.
#[inline]
pub fn clflushopt(ptr: *const u8) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("clflushopt [{0}]", in(reg) ptr, options(nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    let _ = ptr;
}

/// Writes back the cache line containing `ptr` to memory without evicting (`CLWB`).
#[inline]
pub fn clwb(ptr: *const u8) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("clwb [{0}]", in(reg) ptr, options(nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    let _ = ptr;
}
