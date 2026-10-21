//! # SMBIOS validation
//!
//! Pure validation helpers.

/// Returns whether a formatted length is
/// valid.
#[must_use]
pub fn validate_formatted_length(length: u8) -> bool {
    length >= 3
}
