//! # UDP configuration
//!
//! Configuration helpers.

use crate::protocols::network::types::Ipv4Address;

/// A UDP configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UdpConfig {
    /// The station address.
    pub station_address: Ipv4Address,
    /// The station port.
    pub station_port: u16,
}
