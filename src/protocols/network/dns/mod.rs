//! # DNS
//!
//! DNS protocol access.

pub mod protocol;

pub use protocol::{Dns4, Dns6, DnsProtocol, EfiDns4Protocol, EfiDns6Protocol};
