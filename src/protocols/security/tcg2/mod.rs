//! # TCG2
//!
//! Platform measurement operations.

pub mod capability;
pub mod event;
pub mod event_log;
pub mod pcr;
pub mod protocol;

pub use protocol::{EfiTcg2Protocol, Tcg2Protocol};
