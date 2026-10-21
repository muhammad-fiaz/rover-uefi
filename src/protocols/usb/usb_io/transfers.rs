//! # Transfers
//!
//! Transfer helpers.

/// Returns whether an endpoint is valid.
#[must_use]
pub fn validate_endpoint(address: u8) -> bool {
    (0x01..=0x0F).contains(&(address & 0x0F)) || (0x81..=0x8F).contains(&address)
}
