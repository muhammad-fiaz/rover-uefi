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
