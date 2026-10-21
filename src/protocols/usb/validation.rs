//! # USB validation
//!
//! Pure validation helpers.

/// Returns whether a descriptor length is
/// valid.
#[must_use]
pub fn validate_descriptor_length(length: u8, minimum: u8) -> bool {
    length >= minimum && length != 0
}
