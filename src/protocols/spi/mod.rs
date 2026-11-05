//! # SPI Protocols
//!
//! UEFI PI SPI Host Controller, NOR Flash, and Configuration protocols.

pub mod guid;
pub mod hc;
pub mod nor_flash;

pub use guid::*;
pub use hc::*;
pub use nor_flash::*;
