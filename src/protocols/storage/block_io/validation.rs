//! # Block validation
//!
//! Pure validation for block operations.
//!
//! All arithmetic is checked.

use crate::abi::types::UIntN;

/// Returns whether a buffer address satisfies
/// the alignment requirement.
#[must_use]
pub fn is_buffer_aligned(address: UIntN, full_align: u32) -> bool {
    if full_align == 0 {
        return true;
    }
    let Ok(align) = UIntN::try_from(full_align) else {
        return false;
    };
    if align == 0 {
        return true;
    }
    address.is_multiple_of(align)
}

/// Returns the block count for a buffer size.
///
/// Returns `None` when the size is not a
/// multiple of the block size or overflows.
#[must_use]
pub fn block_count(buffer_size: UIntN, block_size: u32) -> Option<UIntN> {
    if block_size == 0 {
        return None;
    }
    let block = UIntN::try_from(block_size).ok()?;
    if block == 0 || !buffer_size.is_multiple_of(block) {
        return None;
    }
    Some(buffer_size / block)
}

/// Returns whether an LBA range is valid.
///
/// The range must not overflow and must end
/// at or below the last block.
#[must_use]
pub fn validate_lba_range(lba: u64, count: UIntN, last_block: u64) -> bool {
    let Ok(count_u64) = u64::try_from(count) else {
        return false;
    };
    if count_u64 == 0 {
        return true;
    }
    let Some(end) = lba.checked_add(count_u64) else {
        return false;
    };
    let Some(last_plus_one) = last_block.checked_add(1) else {
        return last_block == u64::MAX;
    };
    end <= last_plus_one
}

/// Returns the checked end LBA.
#[must_use]
pub fn checked_last_lba(lba: u64, count: UIntN) -> Option<u64> {
    let count_u64 = u64::try_from(count).ok()?;
    if count_u64 == 0 {
        return Some(lba);
    }
    lba.checked_add(count_u64)
}
