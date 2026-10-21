//! # Security architecture protocol
//!
//! The dispatcher authentication callback.
//!
//! The protocol has exactly one entry. The callback receives
//! the authentication status returned by section extraction and
//! an optional device-path pointer for logging. Firmware owns
//! the callback and the dispatcher invokes it; application code
//! does not invoke it directly as part of normal loading.
//! A null file pointer is rejected with `EFI_INVALID_PARAMETER`
//! by the specification contract.

use crate::base::EfiStatus;
use crate::protocols::device_path::EfiDevicePath;

/// The authentication state callback.
pub type EfiSecurityFileAuthenticationState =
    unsafe extern "efiapi" fn(*mut EfiSecurityArchProtocol, u32, *mut EfiDevicePath) -> EfiStatus;

/// The security architecture protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSecurityArchProtocol {
    /// The authentication state callback.
    pub file_authentication_state: EfiSecurityFileAuthenticationState,
}

/// A borrowed security view.
pub struct SecurityArch<'a> {
    protocol: &'a EfiSecurityArchProtocol,
}

impl<'a> SecurityArch<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiSecurityArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSecurityArchProtocol {
        self.protocol
    }

    /// Invokes the authentication state callback.
    ///
    /// # Safety
    ///
    /// The device-path pointer must be null or point to a valid
    /// firmware device path that remains alive for the call, and
    /// the caller must be acting in a dispatcher context where
    /// this callback is permitted.
    #[must_use]
    pub unsafe fn authenticate(&self, status: u32, file: *mut EfiDevicePath) -> EfiStatus {
        // SAFETY: The caller guarantees the file pointer contract.
        unsafe {
            (self.protocol.file_authentication_state)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                status,
                file,
            )
        }
    }
}

impl core::fmt::Debug for SecurityArch<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SecurityArch")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
