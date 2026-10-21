//! # UDP
//!
//! UDP protocol access.

pub mod config;
pub mod protocol;

pub use protocol::{EfiUdp4Protocol, EfiUdp6Protocol, Udp4, Udp6, UdpProtocol};
