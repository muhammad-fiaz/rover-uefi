//! # Handles
//!
//! Handle discovery and driver connection.
//!
//! The safe buffer methods take slices so the
//! caller provides valid storage. The unsafe
//! variants expose the raw interface for
//! firmware-provided buffers.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus, LocateSearchType};
use crate::services::boot::BootServices;
use crate::tables::boot::EfiDevicePath;

impl BootServices<'_> {
    /// Locates handles matching a search.
    ///
    /// The buffer must be large enough. The
    /// size is updated to the required size
    /// when the buffer is too small.
    ///
    /// # Safety
    ///
    /// The protocol and search key pointers
    /// must be valid, the size pointer must be
    /// valid, and the buffer pointer must be
    /// valid for writing the handles.
    #[must_use]
    pub unsafe fn locate_handle(
        &self,
        search_type: LocateSearchType,
        protocol: *mut EfiGuid,
        search_key: *mut core::ffi::c_void,
        buffer_size: *mut UIntN,
        buffer: *mut EfiHandle,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe {
            (self.table.locate_handle)(search_type, protocol, search_key, buffer_size, buffer)
        }
    }

    /// Locates handles by protocol.
    ///
    /// The slice receives the handles. The
    /// required size is returned as an error
    /// status value through `Err` when the slice
    /// is too small, which matches the firmware
    /// `BUFFER_TOO_SMALL` behavior.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the call
    /// fails.
    pub fn locate_handles(
        &self,
        search_type: LocateSearchType,
        protocol: Option<&EfiGuid>,
        buffer: &mut [EfiHandle],
    ) -> Result<UIntN, EfiStatus> {
        let mut buffer_size = buffer.len();
        let protocol_ptr = match protocol {
            Some(guid) => core::ptr::from_ref(guid).cast_mut(),
            None => core::ptr::null_mut(),
        };
        // SAFETY: The slice pointer is valid for `buffer_size` handles and the stack local is valid for the out parameter.
        let status = unsafe {
            (self.table.locate_handle)(
                search_type,
                protocol_ptr,
                core::ptr::null_mut(),
                &raw mut buffer_size,
                buffer.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(buffer_size)
        } else {
            Err(status)
        }
    }

    /// Locates handles into a pool buffer.
    ///
    /// # Safety
    ///
    /// The protocol and search key pointers
    /// must be valid, and both out pointers must
    /// be valid for writing. The returned buffer
    /// is owned by firmware and must be freed
    /// according to the pool ownership rules.
    #[must_use]
    pub unsafe fn locate_handle_buffer(
        &self,
        search_type: LocateSearchType,
        protocol: *mut EfiGuid,
        search_key: *mut core::ffi::c_void,
        no_handles: *mut UIntN,
        buffer: *mut *mut EfiHandle,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe {
            (self.table.locate_handle_buffer)(search_type, protocol, search_key, no_handles, buffer)
        }
    }

    /// Connects drivers to a controller.
    ///
    /// # Safety
    ///
    /// The driver and device path pointers
    /// must be valid when non-null, and the
    /// controller handle must be valid.
    #[must_use]
    pub unsafe fn connect_controller(
        &self,
        controller_handle: EfiHandle,
        driver_image_handle: *mut EfiHandle,
        remaining_device_path: *mut EfiDevicePath,
        recursive: u8,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees handle and pointer validity.
        unsafe {
            (self.table.connect_controller)(
                controller_handle,
                driver_image_handle,
                remaining_device_path,
                recursive,
            )
        }
    }

    /// Disconnects drivers from a controller.
    #[must_use]
    pub fn disconnect_controller(
        &self,
        controller_handle: EfiHandle,
        driver_image_handle: EfiHandle,
        child_handle: EfiHandle,
    ) -> EfiStatus {
        // SAFETY: Firmware returns an error for an invalid disconnect instead of undefined behavior.
        unsafe {
            (self.table.disconnect_controller)(controller_handle, driver_image_handle, child_handle)
        }
    }
}
