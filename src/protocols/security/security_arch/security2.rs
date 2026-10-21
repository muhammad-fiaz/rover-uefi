//! # Security2 architecture protocol
//!
//! The image authentication callback.
//!
//! The protocol has exactly one entry. The callback receives an
//! optional file device path, an optional image buffer with its
//! size, and a boot policy flag. A null buffer selects the
//! device-path-only policy path of the contract. This protocol
//! is distinct from the security architecture protocol and its
//! callback signature is not interchangeable with it.

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;
use crate::protocols::device_path::EfiDevicePath;

/// The Security2 file authentication callback.
pub type EfiSecurity2FileAuthentication = unsafe extern "efiapi" fn(
    *mut EfiSecurity2ArchProtocol,
    *mut EfiDevicePath,
    *mut core::ffi::c_void,
    UIntN,
    Bool,
) -> EfiStatus;

/// The Security2 architecture protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSecurity2ArchProtocol {
    /// The file authentication callback.
    pub file_authentication: EfiSecurity2FileAuthentication,
}

/// A borrowed Security2 view.
pub struct Security2Arch<'a> {
    protocol: &'a EfiSecurity2ArchProtocol,
}

impl<'a> Security2Arch<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiSecurity2ArchProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSecurity2ArchProtocol {
        self.protocol
    }

    /// Invokes the file authentication callback.
    ///
    /// # Safety
    ///
    /// The file pointer must be null or point to a valid
    /// device path, and when the buffer is non-null it must
    /// remain alive for `file_size` bytes during the call.
    #[must_use]
    pub unsafe fn authenticate(
        &self,
        file: *mut EfiDevicePath,
        file_buffer: *mut core::ffi::c_void,
        file_size: UIntN,
        boot_policy: Bool,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer and lifetime
        // contracts.
        unsafe {
            (self.protocol.file_authentication)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                file,
                file_buffer,
                file_size,
                boot_policy,
            )
        }
    }
}

impl core::fmt::Debug for Security2Arch<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Security2Arch")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
