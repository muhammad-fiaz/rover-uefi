//! # Network types
//!
//! Address and packet representations.

/// A MAC address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct MacAddress(pub [u8; 6]);

/// An IPv4 address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Ipv4Address(pub [u8; 4]);

/// An IPv6 address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Ipv6Address(pub [u8; 16]);

/// A standard UEFI 32-byte MAC address buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiMacAddress(pub [u8; 32]);

impl EfiMacAddress {
    /// Creates a zeroed MAC address.
    #[must_use]
    pub const fn zero() -> Self {
        Self([0u8; 32])
    }

    /// Creates an `EfiMacAddress` from standard 6-byte Ethernet MAC.
    #[must_use]
    pub const fn from_mac6(mac: [u8; 6]) -> Self {
        let mut addr = [0u8; 32];
        addr[0] = mac[0];
        addr[1] = mac[1];
        addr[2] = mac[2];
        addr[3] = mac[3];
        addr[4] = mac[4];
        addr[5] = mac[5];
        Self(addr)
    }

    /// Extracts the 6-byte Ethernet MAC if applicable.
    #[must_use]
    pub const fn to_mac6(&self) -> [u8; 6] {
        [
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        ]
    }
}
