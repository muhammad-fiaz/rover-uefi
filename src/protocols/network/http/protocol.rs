//! # HTTP protocol
//!
//! The firmware interface for HTTP requests.
//!
//! The struct has six function entries in specification order.

use core::ffi::c_void;

use crate::base::EfiStatus;

/// The HTTP protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiHttpProtocol {
    /// Gets mode data.
    pub get_mode_data: unsafe extern "efiapi" fn(*mut EfiHttpProtocol, *mut c_void) -> EfiStatus,
    /// Configures the instance.
    pub configure: unsafe extern "efiapi" fn(*mut EfiHttpProtocol, *mut c_void) -> EfiStatus,
    /// Sends a request.
    pub request: unsafe extern "efiapi" fn(
        *mut EfiHttpProtocol,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> EfiStatus,
    /// Cancels a token.
    pub cancel: unsafe extern "efiapi" fn(*mut EfiHttpProtocol, *mut c_void) -> EfiStatus,
    /// Gets a response.
    pub response:
        unsafe extern "efiapi" fn(*mut EfiHttpProtocol, *mut c_void, *mut c_void) -> EfiStatus,
    /// Polls for progress.
    pub poll: unsafe extern "efiapi" fn(*mut EfiHttpProtocol) -> EfiStatus,
}

/// A borrowed HTTP view.
pub struct Http<'a> {
    protocol: &'a EfiHttpProtocol,
}

impl<'a> Http<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiHttpProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiHttpProtocol {
        self.protocol
    }

    /// Polls for progress.
    #[must_use]
    pub fn poll(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.poll)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for Http<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Http")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// Legacy alias for the HTTP view.
pub type HttpProtocol<'a> = Http<'a>;
