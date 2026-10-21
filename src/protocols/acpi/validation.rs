//! # ACPI validation
//!
//! Pure validation helpers.

/// Returns whether a length fits the buffer.
#[must_use]
pub fn validate_length(length: u32, buffer_len: usize) -> bool {
    (length as usize) <= buffer_len && length >= 36
}
