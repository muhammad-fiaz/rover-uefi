//! # Platform Logo Protocol
//!
//! Provides platform OEM graphics logos displayed during system initialization and boot.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

/// Platform Logo Protocol GUID: `{53cd299f-2c0f-4ae5-a5e0-b80dd804dd57}`.
pub const PLATFORM_LOGO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x53CD_299F,
    0x2C0F,
    0x4AE5,
    [0xA5, 0xE0, 0xB8, 0x0D, 0xD8, 0x04, 0xDD, 0x57],
);

/// Image type for platform logo.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PlatformLogoImageType {
    /// Primary platform logo.
    PlatformLogo = 0,
    /// OEM vendor brand badge.
    OemBadge = 1,
    /// System certification badge.
    CertificationBadge = 2,
    /// Diagnostic / firmware recovery splash.
    DiagnosticLogo = 3,
}

/// Function pointer for `GetImage`.
pub type EfiPlatformLogoGetImage = unsafe extern "efiapi" fn(
    this: *const EfiPlatformLogoProtocol,
    instance: u32,
    image_type: *mut PlatformLogoImageType,
    image: *mut *mut EfiGraphicsOutputBltPixel,
    coord_x: *mut usize,
    coord_y: *mut usize,
    width: *mut usize,
    height: *mut usize,
) -> EfiStatus;

/// Low-level Platform Logo Protocol ABI table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPlatformLogoProtocol {
    /// Retrieves a platform logo image.
    pub get_image: EfiPlatformLogoGetImage,
}

/// A retrieved platform logo image view.
#[derive(Debug, Clone, Copy)]
pub struct PlatformLogoImage {
    /// Type of the retrieved image.
    pub image_type: PlatformLogoImageType,
    /// Pointer to `BltPixel` buffer.
    pub image_data: *mut EfiGraphicsOutputBltPixel,
    /// Screen X-coordinate.
    pub coord_x: usize,
    /// Screen Y-coordinate.
    pub coord_y: usize,
    /// Pixel width.
    pub width: usize,
    /// Pixel height.
    pub height: usize,
}

/// Safe client wrapper around the Platform Logo Protocol.
#[derive(Debug, Clone, Copy)]
pub struct PlatformLogo {
    proto: *const EfiPlatformLogoProtocol,
}

impl PlatformLogo {
    /// Wraps a Platform Logo protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiPlatformLogoProtocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPlatformLogoProtocol) -> Self {
        Self { proto }
    }

    /// Retrieves the platform logo image at the given index.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if `instance` exceeds available images.
    /// Returns `EfiStatus::INVALID_PARAMETER` if parameters are invalid.
    pub fn get_image(&self, instance: u32) -> Result<PlatformLogoImage, EfiStatus> {
        let mut image_type = PlatformLogoImageType::PlatformLogo;
        let mut image_ptr: *mut EfiGraphicsOutputBltPixel = core::ptr::null_mut();
        let mut coord_x = 0_usize;
        let mut coord_y = 0_usize;
        let mut width = 0_usize;
        let mut height = 0_usize;

        let status = unsafe {
            let func = (*self.proto).get_image;
            func(
                self.proto,
                instance,
                &raw mut image_type,
                &raw mut image_ptr,
                &raw mut coord_x,
                &raw mut coord_y,
                &raw mut width,
                &raw mut height,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(PlatformLogoImage {
                image_type,
                image_data: image_ptr,
                coord_x,
                coord_y,
                width,
                height,
            })
        } else {
            Err(status)
        }
    }
}
