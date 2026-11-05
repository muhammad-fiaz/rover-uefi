//! # I2C Protocols
//!
//! UEFI PI I2C Master and I2C I/O protocols.

pub mod guid;
pub mod master;

pub use guid::*;
pub use master::*;
