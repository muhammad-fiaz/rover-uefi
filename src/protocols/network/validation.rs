//! # Network validation
//!
//! Pure validation helpers.

/// Returns whether a packet length is valid.
#[must_use]
pub fn validate_packet_length(length: usize) -> bool {
    length != 0
}
