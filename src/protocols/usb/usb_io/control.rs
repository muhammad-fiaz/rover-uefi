//! # Control transfers
//!
//! Control request helpers.

/// Returns whether a control request is
/// valid.
#[must_use]
pub fn validate_control_length(length: u16) -> bool {
    length != 0
}
