//! # Platform Specific Reset Filter and Handler Protocols
//!
//! Provides platform-level interceptors and handlers for system resets
//! (Cold, Warm, Shutdown, `PlatformSpecific`).

use crate::base::{EfiGuid, EfiStatus, ResetType};
use core::ffi::c_void;

/// Platform Specific Reset Filter Protocol GUID: `{695d7835-8d47-4c11-ab22-fa8acce7ae7a}`.
pub const PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x695D_7835,
    0x8D47,
    0x4C11,
    [0xAB, 0x22, 0xFA, 0x8A, 0xCC, 0xE7, 0xAE, 0x7A],
);

/// Platform Specific Reset Handler Protocol GUID: `{2df6ba0b-7092-440d-bd04-fb091ec3f3c1}`.
pub const PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2DF6_BA0B,
    0x7092,
    0x440D,
    [0xBD, 0x04, 0xFB, 0x09, 0x1E, 0xC3, 0xF3, 0xC1],
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
