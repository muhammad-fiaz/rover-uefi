//! # IPv6
//!
//! IPv6 configuration helpers.

use crate::protocols::network::types::Ipv6Address;

/// A minimal IPv6 configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ipv6Config {
    /// The station address.
    pub station_address: Ipv6Address,
    /// The prefix length.
    pub prefix_length: u8,
}

/// Returns whether the address is unspecified.
#[must_use]
pub fn is_unspecified(addr: &Ipv6Address) -> bool {
    addr.0 == [0; 16]
}

/// Returns whether the address is loopback.
#[must_use]
pub fn is_loopback(addr: &Ipv6Address) -> bool {
    addr.0[..15] == [0; 15] && addr.0[15] == 1
}
