//! # GPT validation
//!
//! Pure validation helpers.

use crate::protocols::storage::partition::gpt::header::GptHeader;

/// Returns whether the header LBAs are in
/// order.
#[must_use]
pub fn validate_header_lbas(header: &GptHeader) -> bool {
    header.first_usable_lba <= header.last_usable_lba && header.current_lba != header.backup_lba
}

/// Returns the checked entry array size.
///
/// Returns `None` on overflow.
#[must_use]
pub fn entry_array_size(header: &GptHeader) -> Option<usize> {
    (header.number_of_entries as usize).checked_mul(header.size_of_entry as usize)
}
