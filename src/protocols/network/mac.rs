//! # MAC helpers
//!
//! MAC address helpers.

use crate::protocols::network::types::MacAddress;

/// Returns whether the MAC is multicast.
#[must_use]
pub fn is_multicast(mac: &MacAddress) -> bool {
    mac.0[0] & 0x01 != 0
}
