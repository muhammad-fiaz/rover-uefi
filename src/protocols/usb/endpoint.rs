//! # USB endpoints
//!
//! Endpoint address helpers.

/// Returns whether the address is an IN
/// endpoint.
#[must_use]
pub fn is_in(address: u8) -> bool {
    address & 0x80 != 0
}

/// Returns the endpoint number.
#[must_use]
pub fn endpoint_number(address: u8) -> u8 {
    address & 0x0F
}

/// Returns the transfer type.
#[must_use]
pub fn transfer_type(attributes: u8) -> u8 {
    attributes & 0x03
}
