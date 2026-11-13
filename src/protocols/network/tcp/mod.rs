//! # TCP
//!
//! TCP protocol access.

pub mod config;
pub mod protocol;

pub use protocol::{EfiTcp4Protocol, EfiTcp6Protocol, Tcp4, Tcp6};
