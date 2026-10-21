//! # Loaded image
//!
//! The firmware interface describing a loaded image.

use crate::abi::types::Char16;
use crate::base::{EfiHandle, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;
use crate::tables::system::EfiSystemTable;

/// The loaded image protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiLoadedImageProtocol {
    /// The structure revision.
    pub revision: u32,
    /// The parent image handle.
    pub parent_handle: EfiHandle,
    /// The system table pointer.
    pub system_table: *mut EfiSystemTable,
    /// The device handle.
    pub device_handle: EfiHandle,
    /// The file path pointer.
    pub file_path: *mut EfiDevicePath,
    /// Reserved.
    pub reserved: *mut core::ffi::c_void,
    /// The load-options size in bytes.
    pub load_options_size: u32,
    /// The load-options pointer.
    pub load_options: *mut core::ffi::c_void,
    /// The image base address.
    pub image_base: *mut core::ffi::c_void,
    /// The image size in bytes.
    pub image_size: u64,
    /// The image code memory type.
    pub image_code_type: u32,
    /// The image data memory type.
    pub image_data_type: u32,
    /// The image unload function.
    pub unload: Option<unsafe extern "efiapi" fn(EfiHandle) -> EfiStatus>,
}

/// A borrowed loaded image view.
pub struct LoadedImage<'a> {
    protocol: &'a EfiLoadedImageProtocol,
}

impl<'a> LoadedImage<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiLoadedImageProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiLoadedImageProtocol {
        self.protocol
    }

    /// Returns the image size in bytes.
    #[must_use]
    pub const fn image_size(&self) -> u64 {
        self.protocol.image_size
    }

    /// Returns the revision.
    #[must_use]
    pub const fn revision(&self) -> u32 {
        self.protocol.revision
    }

    /// Returns the load-options pointer.
    #[must_use]
    pub const fn load_options(&self) -> *mut core::ffi::c_void {
        self.protocol.load_options
    }

    /// Returns the load-options size in bytes.
    #[must_use]
    pub const fn load_options_size(&self) -> u32 {
        self.protocol.load_options_size
    }
}

impl core::fmt::Debug for LoadedImage<'_> {
    /// Formats without dereferencing firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("LoadedImage")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// The vendor-string pointer type used by component naming.
pub type SupportedLanguages = *mut core::ffi::c_void;

/// Compatibility alias for vendor strings.
pub type LanguageString = *mut Char16;
