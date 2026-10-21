//! # Simple network
//!
//! Simple network protocol access.

pub mod mode;
pub mod protocol;

pub use protocol::{EfiSimpleNetworkProtocol, SimpleNetwork};
