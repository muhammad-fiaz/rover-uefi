//! # Android Bootloader Debug Protocol
//!
//! Provides an interface for reporting fatal errors, assertions, and crash events
//! from Android bootloader components to platform diagnostic collectors.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Protocol GUID for the Android Bootloader Debug interface.
///
/// `{98ca3da1-c1ac-4402-9c16-7558d3ed5705}`
pub const GBL_EFI_DEBUG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x98ca_3da1,
    0xc1ac,
    0x4402,
    [0x9c, 0x16, 0x75, 0x58, 0xd3, 0xed, 0x57, 0x05],
);

/// Initial revision of the Debug protocol.
pub const GBL_EFI_DEBUG_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Error classifications reported during bootloader execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum GblEfiDebugErrorTag {
    /// Panic or assertion failure.
    AssertionError = 0,
    /// Storage partition table or I/O failure.
    Partition = 1,
    /// OS image verification or loading failure.
    LoadImage = 2,
    /// Unrecoverable boot handoff error.
    BootError = 3,
}

/// Raw function pointer for signaling a fatal bootloader event.
pub type GblEfiDebugFatalError = unsafe extern "efiapi" fn(
    this: *mut GblEfiDebugProtocol,
    frame_ptr: *const c_void,
    tag: u64,
) -> EfiStatus;

/// Raw Android Bootloader Debug Protocol structure.
#[derive(Debug)]
#[repr(C)]
pub struct GblEfiDebugProtocol {
    /// Revision of the protocol.
    pub revision: u64,
    /// Function to report a fatal error event.
    pub fatal_error: GblEfiDebugFatalError,
}

/// High-level safe wrapper around [`GblEfiDebugProtocol`].
pub struct GblDebug<'a> {
    proto: &'a mut GblEfiDebugProtocol,
}

impl<'a> GblDebug<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut GblEfiDebugProtocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.proto.revision
    }

    /// Reports a fatal bootloader condition.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn fatal_error(&mut self, frame_ptr: *const c_void, tag: GblEfiDebugErrorTag) -> EfiStatus {
        unsafe { (self.proto.fatal_error)(self.proto, frame_ptr, tag as u64) }
    }
}
