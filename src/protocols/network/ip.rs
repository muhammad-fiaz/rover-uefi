//! # IP helpers
//!
//! IP address helpers.

use crate::protocols::network::types::Ipv4Address;

/// Returns whether the address is unspecified.
#[must_use]
pub fn is_unspecified(addr: &Ipv4Address) -> bool {
    addr.0 == [0, 0, 0, 0]
}
