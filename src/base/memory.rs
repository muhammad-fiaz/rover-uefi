//! # Low-Level Memory Primitives and Address Range Abstractions
//!
//! Provides fundamental memory range validation, overlapping copy, fill, comparison,
//! volatile MMIO memory access, and compiler-barrier-protected secure zeroing.

use core::cmp::Ordering;
use core::sync::atomic;

/// Represents a contiguous range of physical or virtual memory `[start, start + length)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MemoryRange {
    /// Starting base address.
    pub start: u64,
    /// Length of the range in bytes.
    pub length: u64,
}

impl MemoryRange {
    /// Creates a new memory range descriptor.
    #[must_use]
    pub const fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    /// Returns the start base address.
    #[must_use]
    pub const fn start(&self) -> u64 {
        self.start
    }

    /// Returns the length of the range in bytes.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// Returns the end address (exclusive) `start + length`.
    #[must_use]
    pub const fn end(&self) -> u64 {
        self.start.saturating_add(self.length)
    }

    /// Checks if the memory range is empty (length == 0).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Checks if the range contains a specific address.
    #[must_use]
    pub const fn contains_address(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.end()
    }

    /// Checks if this range completely encloses `other`.
    #[must_use]
    pub const fn contains_range(&self, other: &Self) -> bool {
        if other.is_empty() {
            return true;
        }
        other.start >= self.start && other.end() <= self.end()
    }

    /// Checks if this range overlaps with `other`.
    #[must_use]
    pub const fn overlaps(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }
        self.start < other.end() && other.start < self.end()
    }

    /// Checks if the base address and length are aligned to `alignment`.
    #[must_use]
    pub const fn is_aligned(&self, alignment: u64) -> bool {
        if alignment == 0 {
            return false;
        }
        (self.start % alignment == 0) && (self.length % alignment == 0)
    }

    /// Aligns the range boundaries outward to the nearest multiple of `alignment`.
    #[must_use]
    pub const fn align_outward(&self, alignment: u64) -> Self {
        if alignment <= 1 || self.is_empty() {
            return *self;
        }
        let aligned_start = self.start & !(alignment - 1);
        let end = self.end();
        let aligned_end = (end + alignment - 1) & !(alignment - 1);
        Self {
            start: aligned_start,
            length: aligned_end - aligned_start,
        }
    }

    /// Splits the range into two at `offset` relative to `self.start`.
    ///
    /// Returns `Some((left, right))` if `offset <= length`, or `None` if offset is out of bounds.
    #[must_use]
    pub const fn split_at(&self, offset: u64) -> Option<(Self, Self)> {
        if offset > self.length {
            return None;
        }
        let left = Self::new(self.start, offset);
        let right = Self::new(self.start + offset, self.length - offset);
        Some((left, right))
    }
}

/// Copies `count` bytes from `src` to `dest`.
///
/// # Safety
///
/// `src` and `dest` must be valid for reads/writes of `count` bytes and must not overlap.
#[inline]
pub unsafe fn copy_bytes(dest: *mut u8, src: *const u8, count: usize) {
    if count == 0 || core::ptr::eq(dest, src.cast_mut()) {
        return;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(src, dest, count);
    }
}

/// Copies `count` bytes from `src` to `dest`, safely handling overlapping memory regions.
///
/// # Safety
///
/// `src` and `dest` must be valid for reads/writes of `count` bytes.
#[inline]
pub unsafe fn move_bytes(dest: *mut u8, src: *const u8, count: usize) {
    if count == 0 || core::ptr::eq(dest, src.cast_mut()) {
        return;
    }
    unsafe {
        core::ptr::copy(src, dest, count);
    }
}

/// Fills `count` bytes starting at `dest` with `value`.
///
/// # Safety
///
/// `dest` must be valid for writes of `count` bytes.
#[inline]
pub unsafe fn fill_bytes(dest: *mut u8, value: u8, count: usize) {
    if count == 0 {
        return;
    }
    unsafe {
        core::ptr::write_bytes(dest, value, count);
    }
}

/// Sets `count` bytes starting at `dest` to zero.
///
/// # Safety
///
/// `dest` must be valid for writes of `count` bytes.
#[inline]
pub unsafe fn zero_bytes(dest: *mut u8, count: usize) {
    unsafe {
        fill_bytes(dest, 0, count);
    }
}

/// Lexicographically compares two memory buffers of `count` bytes.
///
/// # Safety
///
/// Both `a` and `b` must be valid for reads of `count` bytes.
#[must_use]
pub unsafe fn compare_bytes(a: *const u8, b: *const u8, count: usize) -> Ordering {
    if count == 0 || core::ptr::eq(a, b) {
        return Ordering::Equal;
    }
    let mut i = 0;
    while i < count {
        let val_a = unsafe { *a.add(i) };
        let val_b = unsafe { *b.add(i) };
        if val_a != val_b {
            return val_a.cmp(&val_b);
        }
        i += 1;
    }
    Ordering::Equal
}

/// Compares two memory buffers and returns an integer (-1, 0, 1) for raw C-style comparison.
///
/// # Safety
///
/// Both `a` and `b` must be valid for reads of `count` bytes.
#[must_use]
pub unsafe fn compare_bytes_raw(a: *const u8, b: *const u8, count: usize) -> i32 {
    match unsafe { compare_bytes(a, b, count) } {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Performs a volatile memory copy byte-by-byte.
///
/// Guarantees that compiler optimizations do not coalesce or eliminate accesses,
/// required for MMIO peripheral FIFOs and volatile memory areas.
///
/// # Safety
///
/// `src` and `dest` must be valid for reads/writes of `count` bytes.
pub unsafe fn volatile_copy(dest: *mut u8, src: *const u8, count: usize) {
    let mut i = 0;
    while i < count {
        unsafe {
            let val = core::ptr::read_volatile(src.add(i));
            core::ptr::write_volatile(dest.add(i), val);
        }
        i += 1;
    }
}

/// Performs a volatile byte fill.
///
/// # Safety
///
/// `dest` must be valid for writes of `count` bytes.
pub unsafe fn volatile_fill(dest: *mut u8, value: u8, count: usize) {
    let mut i = 0;
    while i < count {
        unsafe {
            core::ptr::write_volatile(dest.add(i), value);
        }
        i += 1;
    }
}

/// Securely clears sensitive memory using volatile writes followed by a compiler fence.
///
/// Guarantees that the dead-store elimination compiler pass does not eliminate the wipe.
///
/// # Safety
///
/// `dest` must be valid for writes of `count` bytes.
pub unsafe fn secure_zero_memory(dest: *mut u8, count: usize) {
    unsafe {
        volatile_fill(dest, 0, count);
    }
    atomic::compiler_fence(atomic::Ordering::SeqCst);
}
