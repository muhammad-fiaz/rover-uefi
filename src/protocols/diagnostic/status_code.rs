//! # Status-code reporting
//!
//! The firmware interface for status-code reporting.

use crate::base::{EfiGuid, EfiStatus};

/// The status-code runtime protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiStatusCodeProtocol {
    /// Reports a status code.
    pub report_status_code: unsafe extern "efiapi" fn(
        *mut EfiStatusCodeProtocol,
        u32,
        u32,
        u32,
        *mut EfiGuid,
        *mut core::ffi::c_void,
    ) -> EfiStatus,
}

/// A borrowed status-code view.
pub struct StatusCode<'a> {
    protocol: &'a EfiStatusCodeProtocol,
}

impl<'a> StatusCode<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiStatusCodeProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiStatusCodeProtocol {
        self.protocol
    }

    /// Reports a status code without extended data.
    #[must_use]
    pub fn report(&self, code_type: u32, value: u32, instance: u32) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.report_status_code)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                code_type,
                value,
                instance,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        }
    }

    /// Reports a status code with a caller identifier.
    #[must_use]
    pub fn report_with_caller(
        &self,
        code_type: u32,
        value: u32,
        instance: u32,
        caller_id: &EfiGuid,
    ) -> EfiStatus {
        // SAFETY: The caller identifier reference is valid for the call.
        unsafe {
            (self.protocol.report_status_code)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                code_type,
                value,
                instance,
                core::ptr::from_ref(caller_id).cast_mut(),
                core::ptr::null_mut(),
            )
        }
    }
}

impl core::fmt::Debug for StatusCode<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("StatusCode")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
