//! # PXE
//!
//! PXE base code access.

pub mod mode;
pub mod packets;
pub mod protocol;

pub use protocol::{EfiPxeBaseCodeProtocol, PxeBaseCode};
