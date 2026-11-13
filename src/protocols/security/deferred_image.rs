//! UEFI Deferred Image Load Protocol definition and safe wrapper.
//!
//! Defined in UEFI Specification 2.2+ Section 31.
//! This protocol manages images whose execution was deferred due to security
//! policies (e.g. untrusted certificates, missing user credentials, or secure boot
//! evaluation). When security state changes (e.g. a user authenticates), the
//! Boot Manager can query deferred images and retry loading them.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::device_path::types::EfiDevicePath;
use core::ffi::c_void;

/// UEFI Deferred Image Load Protocol GUID: `{15853d7c-3ddf-43e0-a1cb-ebf85b8f872c}`.
pub const EFI_DEFERRED_IMAGE_LOAD_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1585_3D7C,
    0x3DDF,
    0x43E0,
    [0xA1, 0xCB, 0xEB, 0xF8, 0x5B, 0x8F, 0x87, 0x2C],
);

/// Function prototype for querying deferred image details.
pub type EfiDeferredImageInfo = unsafe extern "efiapi" fn(
    this: *const EfiDeferredImageLoadProtocol,
    image_index: UIntN,
    image_device_path: *mut *const EfiDevicePath,
    image: *mut *const c_void,
    image_size: *mut UIntN,
    boot_option: *mut bool,
) -> EfiStatus;

/// UEFI Deferred Image Load Protocol structure.
#[repr(C)]
pub struct EfiDeferredImageLoadProtocol {
    /// Retrieves information about a deferred image at index `ImageIndex`.
    pub get_image_info: EfiDeferredImageInfo,
}

/// Information record describing a deferred executable image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeferredImageDetails {
    /// Device path of the image.
    pub device_path: *const EfiDevicePath,
    /// In-memory image buffer (or null if deferred by location).
    pub image: *const c_void,
    /// Image buffer length in bytes.
    pub image_size: usize,
    /// True if the image was intended as a boot option.
    pub boot_option: bool,
}

/// Safe idiomatic wrapper around `EfiDeferredImageLoadProtocol`.
pub struct DeferredImageLoad {
    proto: *const EfiDeferredImageLoadProtocol,
}

impl DeferredImageLoad {
    /// Creates a new `DeferredImageLoad` client wrapper.
    #[must_use]
    pub const fn new(proto: *const EfiDeferredImageLoadProtocol) -> Self {
        Self { proto }
    }

    /// Returns the raw protocol pointer.
    #[must_use]
    pub const fn raw(&self) -> *const EfiDeferredImageLoadProtocol {
        self.proto
    }

    /// Retrieves information about the deferred image at zero-based `image_index`.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if `image_index` is beyond the number of deferred images,
    /// or an error status on failure.
    pub fn get_image_info(&self, image_index: usize) -> Result<DeferredImageDetails, EfiStatus> {
        let mut dev_path = core::ptr::null();
        let mut image_ptr = core::ptr::null();
        let mut size = 0 as UIntN;
        let mut is_boot = false;

        let status = unsafe {
            ((*self.proto).get_image_info)(
                self.proto,
                image_index as UIntN,
                core::ptr::from_mut(&mut dev_path),
                core::ptr::from_mut(&mut image_ptr),
                core::ptr::from_mut(&mut size),
                core::ptr::from_mut(&mut is_boot),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(DeferredImageDetails {
                device_path: dev_path,
                image: image_ptr,
                image_size: size as usize,
                boot_option: is_boot,
            })
        } else {
            Err(status)
        }
    }
}
