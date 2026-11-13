//! # Platform Specific Reset Filter and Handler Protocols
//!
//! Provides platform-level interceptors and handlers for system resets
//! (Cold, Warm, Shutdown, `PlatformSpecific`).

use crate::base::{EfiGuid, EfiStatus, ResetType};
use core::ffi::c_void;

/// Platform Specific Reset Filter Protocol GUID: `{695d36cf-da7d-4c06-b3da-7d1c2c3a519e}`.
pub const PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x695D_36CF,
    0xDA7D,
    0x4C06,
    [0xB3, 0xDA, 0x7D, 0x1C, 0x2C, 0x3A, 0x51, 0x9E],
);

/// Platform Specific Reset Handler Protocol GUID: `{9d410013-05da-4471-ad32-22421684a2db}`.
pub const PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9D41_0013,
    0x05DA,
    0x4471,
    [0xAD, 0x32, 0x22, 0x42, 0x16, 0x84, 0xA2, 0xDB],
);

/// Function pointer for `ResetFilter`.
pub type EfiResetFilter = unsafe extern "efiapi" fn(
    reset_type: ResetType,
    reset_status: EfiStatus,
    data_size: usize,
    reset_data: *mut c_void,
);

/// Low-level Platform Specific Reset Filter Protocol ABI table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPlatformSpecificResetFilterProtocol {
    /// Filter function called before resetting system.
    pub reset_filter: EfiResetFilter,
}

/// Safe client wrapper around the Platform Specific Reset Filter Protocol.
#[derive(Debug, Clone, Copy)]
pub struct PlatformSpecificResetFilter {
    proto: *const EfiPlatformSpecificResetFilterProtocol,
}

impl PlatformSpecificResetFilter {
    /// Wraps a Reset Filter protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiPlatformSpecificResetFilterProtocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPlatformSpecificResetFilterProtocol) -> Self {
        Self { proto }
    }

    /// Invokes the platform reset filter hook.
    pub fn invoke_filter(
        &self,
        reset_type: ResetType,
        reset_status: EfiStatus,
        reset_data: Option<&mut [u8]>,
    ) {
        let (ptr, size) = match reset_data {
            Some(buf) => (buf.as_mut_ptr().cast::<c_void>(), buf.len()),
            None => (core::ptr::null_mut(), 0),
        };
        unsafe {
            let func = (*self.proto).reset_filter;
            func(reset_type, reset_status, size, ptr);
        }
    }
}
