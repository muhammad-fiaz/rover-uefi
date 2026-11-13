//! # DNS
//!
//! DNS protocol access.

pub mod protocol;

pub use protocol::{Dns4, Dns6, EfiDns4Protocol, EfiDns6Protocol};
