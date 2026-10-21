//! # Images
//!
//! Loading, starting, exiting, and unloading
//! images.
//!
//! Image operations require caller pointers
//! for device paths, source buffers, and exit
//! data. The methods are therefore unsafe.
//! Callers must guarantee that pointers are
//! valid and that buffers remain alive for the
//! duration of the load or start.

use core::ffi::c_void;

use crate::abi::types::{Bool, Char16, UIntN};
use crate::base::{EfiHandle, EfiStatus};
use crate::services::boot::BootServices;
use crate::tables::boot::EfiDevicePath;

impl BootServices<'_> {
    /// Loads an image into memory.
    ///
    /// # Safety
    ///
    /// The device path and source buffer
    /// pointers must be valid when non-null, and
    /// the image handle pointer must be valid
    /// for writing.
    #[must_use]
    pub unsafe fn load_image(
        &self,
        boot_policy: Bool,
        parent_image_handle: EfiHandle,
        device_path: *mut EfiDevicePath,
        source_buffer: *mut c_void,
        source_size: UIntN,
        image_handle: *mut EfiHandle,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe {
            (self.table.load_image)(
                boot_policy,
                parent_image_handle,
                device_path,
                source_buffer,
                source_size,
                image_handle,
            )
        }
    }

    /// Starts a loaded image.
    ///
    /// # Safety
    ///
    /// The image handle must name a loaded
    /// image and both out pointers must be valid
    /// for writing. The returned exit data must
    /// be freed according to the pool ownership rules.
    #[must_use]
    pub unsafe fn start_image(
        &self,
        image_handle: EfiHandle,
        exit_data_size: *mut UIntN,
        exit_data: *mut *mut Char16,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees handle and pointer validity.
        unsafe { (self.table.start_image)(image_handle, exit_data_size, exit_data) }
    }

    /// Exits a loaded image.
    ///
    /// # Safety
    ///
    /// The image handle must name the currently
    /// executing image and the exit data pointer
    /// must be valid when non-null.
    #[must_use]
    pub unsafe fn exit(
        &self,
        image_handle: EfiHandle,
        exit_status: EfiStatus,
        exit_data_size: UIntN,
        exit_data: *mut Char16,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees handle and pointer validity.
        unsafe { (self.table.exit)(image_handle, exit_status, exit_data_size, exit_data) }
    }

    /// Unloads an image.
    #[must_use]
    pub fn unload_image(&self, image_handle: EfiHandle) -> EfiStatus {
        // SAFETY: Firmware returns an error for an invalid handle.
        unsafe { (self.table.unload_image)(image_handle) }
    }
}
