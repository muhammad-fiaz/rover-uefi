//! # MBR validation
//!
//! Pure validation for MBR entries.

use crate::protocols::storage::partition::mbr::entry::MbrEntry;

/// Returns whether two entries overlap.
#[must_use]
pub fn entries_overlap(first: &MbrEntry, second: &MbrEntry) -> bool {
    if first.sector_count == 0 || second.sector_count == 0 {
        return false;
    }
    let first_start = u64::from(first.starting_lba);
    let second_start = u64::from(second.starting_lba);
    let first_end = first_start + u64::from(first.sector_count);
    let second_end = second_start + u64::from(second.sector_count);
    first_start < second_end && second_start < first_end
}

/// Returns the checked end LBA.
#[must_use]
pub fn checked_end_lba(starting_lba: u32, sector_count: u32) -> Option<u64> {
    u64::from(starting_lba).checked_add(u64::from(sector_count))
}
