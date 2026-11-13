//! # Multiprocessor Atomic and Interlocked Primitives
//!
//! Provides interlocked operations across primitive integer types, matching low-level
//! firmware synchronization and hardware bus locking requirements.

use core::sync::atomic::{
    AtomicI32, AtomicI64, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
};

/// Atomically increments a 32-bit unsigned integer and returns the incremented value.
#[inline]
pub fn atomic_increment_u32(target: &AtomicU32) -> u32 {
    target.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

/// Atomically decrements a 32-bit unsigned integer and returns the decremented value.
#[inline]
pub fn atomic_decrement_u32(target: &AtomicU32) -> u32 {
    target.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
}

/// Atomically increments a 64-bit unsigned integer and returns the incremented value.
#[inline]
pub fn atomic_increment_u64(target: &AtomicU64) -> u64 {
    target.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

/// Atomically decrements a 64-bit unsigned integer and returns the decremented value.
#[inline]
pub fn atomic_decrement_u64(target: &AtomicU64) -> u64 {
    target.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
}

/// Atomically increments a signed 32-bit integer and returns the incremented value.
#[inline]
pub fn atomic_increment_i32(target: &AtomicI32) -> i32 {
    target.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

/// Atomically decrements a signed 32-bit integer and returns the decremented value.
#[inline]
pub fn atomic_decrement_i32(target: &AtomicI32) -> i32 {
    target.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
}

/// Atomically increments a signed 64-bit integer and returns the incremented value.
#[inline]
pub fn atomic_increment_i64(target: &AtomicI64) -> i64 {
    target.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

/// Atomically decrements a signed 64-bit integer and returns the decremented value.
#[inline]
pub fn atomic_decrement_i64(target: &AtomicI64) -> i64 {
    target.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
}

/// Atomically performs compare-and-exchange on a 16-bit unsigned integer.
///
/// Returns the initial value of `target` prior to the attempted exchange.
#[inline]
pub fn atomic_compare_exchange_u16(target: &AtomicU16, compare: u16, exchange: u16) -> u16 {
    match target.compare_exchange(compare, exchange, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(prev) | Err(prev) => prev,
    }
}

/// Atomically performs compare-and-exchange on a 32-bit unsigned integer.
///
/// Returns the initial value of `target` prior to the attempted exchange.
#[inline]
pub fn atomic_compare_exchange_u32(target: &AtomicU32, compare: u32, exchange: u32) -> u32 {
    match target.compare_exchange(compare, exchange, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(prev) | Err(prev) => prev,
    }
}

/// Atomically performs compare-and-exchange on a 64-bit unsigned integer.
///
/// Returns the initial value of `target` prior to the attempted exchange.
#[inline]
pub fn atomic_compare_exchange_u64(target: &AtomicU64, compare: u64, exchange: u64) -> u64 {
    match target.compare_exchange(compare, exchange, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(prev) | Err(prev) => prev,
    }
}

/// Atomically adds `value` to a 64-bit unsigned integer and returns the new value.
#[inline]
pub fn atomic_add_u64(target: &AtomicU64, value: u64) -> u64 {
    target
        .fetch_add(value, Ordering::SeqCst)
        .wrapping_add(value)
}

/// Atomically subtracts `value` from a 64-bit unsigned integer and returns the new value.
#[inline]
pub fn atomic_sub_u64(target: &AtomicU64, value: u64) -> u64 {
    target
        .fetch_sub(value, Ordering::SeqCst)
        .wrapping_sub(value)
}

/// Atomically bitwise-ANDs `mask` into `target` and returns the resulting value.
#[inline]
pub fn atomic_and_u32(target: &AtomicU32, mask: u32) -> u32 {
    target.fetch_and(mask, Ordering::SeqCst) & mask
}

/// Atomically bitwise-ORs `mask` into `target` and returns the resulting value.
#[inline]
pub fn atomic_or_u32(target: &AtomicU32, mask: u32) -> u32 {
    target.fetch_or(mask, Ordering::SeqCst) | mask
}

/// Atomically bitwise-XORs `mask` into `target` and returns the resulting value.
#[inline]
pub fn atomic_xor_u32(target: &AtomicU32, mask: u32) -> u32 {
    target.fetch_xor(mask, Ordering::SeqCst) ^ mask
}

/// Atomically adds `value` to a `usize` and returns the new value.
#[inline]
pub fn atomic_add_usize(target: &AtomicUsize, value: usize) -> usize {
    target
        .fetch_add(value, Ordering::SeqCst)
        .wrapping_add(value)
}
