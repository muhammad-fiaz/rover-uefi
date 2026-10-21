//! # File I/O token
//!
//! The token for Ex file operations.
//!
//! A null event means blocking I/O. A
//! non-null event means non-blocking I/O when
//! supported. The struct is `#[repr(C)]`.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiStatus};

/// A file I/O token.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFileIoToken {
    /// The completion event.
    pub event: EfiEvent,
    /// The operation status.
    pub status: EfiStatus,
    /// The buffer size in bytes.
    pub buffer_size: UIntN,
    /// The data buffer pointer.
    pub buffer: *mut c_void,
}

const _: () = {
    assert!(core::mem::offset_of!(EfiFileIoToken, event) == 0);
};
