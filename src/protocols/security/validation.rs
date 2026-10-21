//! # Security validation
//!
//! Pure validation helpers for buffers and sizes that do not
//! require firmware.

/// Returns whether a digest size matches the expected size.
#[must_use]
pub fn validate_digest_size(size: usize, expected: usize) -> bool {
    size == expected && expected != 0
}

/// Returns whether a requested output length is valid for a
/// firmware-generated value.
#[must_use]
pub fn validate_output_length(length: usize) -> bool {
    length != 0
}

/// Returns whether a checked addition of a header size and a
/// payload size is within the buffer.
#[must_use]
pub fn checked_total_size(header_size: usize, payload_size: usize) -> Option<usize> {
    header_size.checked_add(payload_size)
}
