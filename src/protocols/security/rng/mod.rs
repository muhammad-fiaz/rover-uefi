//! # RNG
//!
//! Random number generation.

pub mod algorithm;
pub mod protocol;

pub use protocol::{EfiRngProtocol, RngProtocol};
