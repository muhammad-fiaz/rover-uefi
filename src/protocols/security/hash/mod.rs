//! # Hash
//!
//! Hashing services.

pub mod algorithm;
pub mod hash2;
pub mod protocol;

pub use hash2::Hash2Protocol;
pub use protocol::{EfiHashProtocol, HashProtocol};
