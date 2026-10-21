//! # File validation
//!
//! Pure validation for file operations.

use crate::abi::types::Char16;

/// Returns whether the name is a valid
/// null-terminated UTF-16 name.
#[must_use]
pub fn validate_file_name(name: &[Char16]) -> bool {
    !name.is_empty() && name.last().is_some_and(|last| *last == 0)
}

/// Returns the checked data size in bytes.
///
/// Returns `None` on overflow.
#[must_use]
pub fn checked_data_size(count: usize, unit: usize) -> Option<usize> {
    count.checked_mul(unit)
}

/// Returns whether an open mode is valid.
///
/// Valid combinations are read, read/write,
/// and create/read/write.
#[must_use]
pub fn validate_open_mode(open_mode: u64) -> bool {
    const READ: u64 = 0x0000_0000_0000_0001;
    const WRITE: u64 = 0x0000_0000_0000_0002;
    const CREATE: u64 = 0x8000_0000_0000_0000;
    let read = open_mode & READ;
    let write = open_mode & WRITE;
    let create = open_mode & CREATE;
    let known = read | write | create;
    if open_mode & !known != 0 {
        return false;
    }
    matches!(
        (read != 0, write != 0, create != 0),
        (true, false | true, false) | (true, true, true)
    )
}
