//! # IPv4
//!
//! IPv4 configuration helpers.

use crate::protocols::network::types::Ipv4Address;

/// A minimal IPv4 configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ipv4Config {
    /// The station address.
    pub station_address: Ipv4Address,
    /// The subnet mask.
    pub subnet_mask: Ipv4Address,
}

/// Returns whether the address is unspecified.
#[must_use]
pub fn is_unspecified(addr: &Ipv4Address) -> bool {
    addr.0 == [0, 0, 0, 0]
}

/// Returns whether the address is broadcast.
#[must_use]
pub fn is_broadcast(addr: &Ipv4Address) -> bool {
    addr.0 == [0xFF, 0xFF, 0xFF, 0xFF]
}
