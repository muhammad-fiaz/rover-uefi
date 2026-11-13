//! # SPI Protocols
//!
//! UEFI PI SPI Host Controller, NOR Flash, and Configuration protocols.

pub mod configuration;
pub mod guid;
pub mod hc;
pub mod io;
pub mod nor_flash;

pub use configuration::*;
pub use guid::*;
pub use hc::*;
pub use io::*;
pub use nor_flash::*;
