//! # Extended boot records
//!
//! EBR chain validation.
//!
//! The helper validates a chain without
//! performing Block I/O. Loop detection uses
//! a bounded visited count.

use crate::protocols::storage::partition::mbr::entry::MbrEntry;

/// Returns whether a chain is acyclic and
/// bounded.
///
/// The offsets are the LBAs of each EBR in
/// traversal order.
#[must_use]
pub fn validate_ebr_chain(offsets: &[u64], full_limit: usize) -> bool {
    if offsets.len() > full_limit {
        return false;
    }
    for (index, offset) in offsets.iter().enumerate() {
        for previous in &offsets[..index] {
            if previous == offset {
                return false;
            }
        }
    }
    true
}

/// Returns the absolute LBA of a logical
/// partition.
///
/// The result is `extended_start + relative`.
#[must_use]
pub fn absolute_lba(extended_start: u64, relative: u32) -> Option<u64> {
    extended_start.checked_add(u64::from(relative))
}

/// Returns whether an entry is an extended
/// container.
#[must_use]
pub fn is_extended(entry: &MbrEntry) -> bool {
    entry.partition_type == 0x05 || entry.partition_type == 0x0F
}
