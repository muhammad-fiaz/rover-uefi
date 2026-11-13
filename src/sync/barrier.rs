//! # Low-Level Memory Barriers and Processor Yielding
//!
//! Provides architecture-specific hardware memory barriers, compiler barriers,
//! instruction synchronization barriers, and CPU spin-loop pause instructions.

use core::sync::atomic::{self, Ordering};

/// Full compiler fence preventing the compiler from reordering memory accesses across the boundary.
#[inline]
pub fn compiler_fence() {
    atomic::compiler_fence(Ordering::SeqCst);
}

/// Executes a full memory barrier.
///
/// Guarantees that all preceding read and write operations are globally visible
/// before any subsequent reads and writes are executed.
///
/// * On `x86_64`: executes `mfence`.
/// * On `aarch64`: executes `dmb sy`.
/// * On other architectures: executes a sequentially consistent atomic memory fence.
#[inline]
pub fn memory_barrier() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dmb sy", options(nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    atomic::fence(Ordering::SeqCst);
}

/// Executes a read (load) memory barrier.
///
/// Guarantees that all preceding read operations are completed before any subsequent
/// reads are initiated.
///
/// * On `x86_64`: executes `lfence`.
/// * On `aarch64`: executes `dmb ld`.
/// * On other architectures: executes an acquire atomic memory fence.
#[inline]
pub fn read_memory_barrier() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("lfence", options(nostack, preserves_flags));
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dmb ld", options(nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    atomic::fence(Ordering::Acquire);
}

/// Executes a write (store) memory barrier.
///
/// Guarantees that all preceding write operations are committed to memory or cache
/// before any subsequent writes are initiated.
///
/// * On `x86_64`: executes `sfence`.
/// * On `aarch64`: executes `dmb st`.
/// * On other architectures: executes a release atomic memory fence.
#[inline]
pub fn write_memory_barrier() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("sfence", options(nostack, preserves_flags));
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dmb st", options(nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    atomic::fence(Ordering::Release);
}

/// Executes an instruction synchronization barrier.
///
/// Flushes the processor pipeline so that all instructions following the barrier
/// are fetched from memory or cache after the barrier completes.
///
/// * On `aarch64`: executes `isb`.
/// * On other architectures: executes a compiler fence and memory barrier.
#[inline]
pub fn instruction_barrier() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("isb", options(nostack, preserves_flags));
    }

    #[cfg(not(target_arch = "aarch64"))]
    {
        compiler_fence();
        memory_barrier();
    }
}

/// Hints to the processor that the executing thread is in a spin-wait loop.
///
/// Reduces power consumption, avoids pipeline stalls, and yields hyperthread
/// pipeline resources to sibling cores.
///
/// * On `x86` / `x86_64`: executes `pause`.
/// * On `aarch64`: executes `yield`.
/// * On other architectures: invokes [`core::hint::spin_loop`].
#[inline]
pub fn cpu_pause() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("pause", options(nomem, nostack, preserves_flags));
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("yield", options(nomem, nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    core::hint::spin_loop();
}

/// Alias for [`cpu_pause`], indicating processor yield during busy-waiting.
#[inline]
pub fn cpu_relax() {
    cpu_pause();
}
