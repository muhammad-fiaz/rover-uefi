//! # Core protocol support
//!
//! Shared support for all protocol modules.
//!
//! The module provides protocol `GUID`
//! constants, lifetime rules, and open and
//! close helpers. Concrete protocols provide
//! their own wrappers that borrow the interface
//! for an explicit lifetime.

pub mod guid;
pub mod interface;
pub mod open;

pub use guid::{DEVICE_PATH_GUID, SIMPLE_TEXT_INPUT_GUID, SIMPLE_TEXT_OUTPUT_GUID};
pub use interface::{is_handle_present, is_interface_present, locate_protocol};
pub use open::{close, open};
