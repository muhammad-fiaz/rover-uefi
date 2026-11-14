//! # HII Image and Extended Image Protocols
//!
//! Provides services to retrieve, register, update, and render graphical images
//! stored within HII image package lists.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::firmware::hii::{EFI_HII_IMAGE_PROTOCOL_GUID, EfiHiiHandle, EfiImageOutput};
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

/// HII Extended Image Protocol GUID (`EFI_HII_IMAGE_EX_PROTOCOL_GUID`).
pub const EFI_HII_IMAGE_EX_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1a12_41e6,
    0x8f19,
    0x41a9,
    [0xbc, 0x0e, 0xe8, 0xef, 0x39, 0xe0, 0x65, 0x46],
);

/// Image is designed for transparent display.
pub const EFI_IMAGE_TRANSPARENT: u32 = 0x0000_0001;

/// Clip drawn image to bounding box.
pub const EFI_HII_DRAW_FLAG_CLIP: u32 = 0x0000_0001;
/// Force transparent rendering.
pub const EFI_HII_DRAW_FLAG_FORCE_TRANS: u32 = 0x0000_0010;
/// Force opaque rendering.
pub const EFI_HII_DRAW_FLAG_FORCE_OPAQUE: u32 = 0x0000_0020;
/// Draw transparently using existing pixel values.
pub const EFI_HII_DRAW_FLAG_TRANSPARENT: u32 = 0x0000_0030;
/// Render directly to output screen device.
pub const EFI_HII_DIRECT_TO_SCREEN: u32 = 0x0000_0080;
/// Default rendering behavior.
pub const EFI_HII_DRAW_FLAG_DEFAULT: u32 = 0x0000_0000;

/// Unique identifier for an image within an HII package list.
pub type EfiImageId = u16;

/// Image input descriptor (`EFI_IMAGE_INPUT`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiImageInput {
    /// Image characteristics (e.g. `EFI_IMAGE_TRANSPARENT`).
    pub flags: u32,
    /// Image width in pixels.
    pub width: u16,
    /// Image height in pixels.
    pub height: u16,
    /// Pointer to bitmap array of pixels.
    pub bitmap: *mut EfiGraphicsOutputBltPixel,
}

// Function pointer definitions for standard HII Image Protocol

/// Adds an image to an HII package list.
pub type EfiHiiNewImage = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageProtocol,
    package_list: EfiHiiHandle,
    image_id: *mut EfiImageId,
    image: *const EfiImageInput,
) -> EfiStatus;

/// Retrieves an image from an HII package list.
pub type EfiHiiGetImage = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageProtocol,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *mut EfiImageInput,
) -> EfiStatus;

/// Modifies an existing image in an HII package list.
pub type EfiHiiSetImage = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageProtocol,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *const EfiImageInput,
) -> EfiStatus;

/// Draws an image onto a destination surface or screen.
pub type EfiHiiDrawImage = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageProtocol,
    flags: u32,
    image: *const EfiImageInput,
    blt: *mut *mut EfiImageOutput,
    blt_x: UIntN,
    blt_y: UIntN,
) -> EfiStatus;

/// Draws an image looked up from a package list by ID.
pub type EfiHiiDrawImageId = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageProtocol,
    flags: u32,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    blt: *mut *mut EfiImageOutput,
    blt_x: UIntN,
    blt_y: UIntN,
) -> EfiStatus;

/// Raw interface structure for `EFI_HII_IMAGE_PROTOCOL`.
#[repr(C)]
pub struct EfiHiiImageProtocol {
    /// Adds an image to package list.
    pub new_image: EfiHiiNewImage,
    /// Retrieves an image.
    pub get_image: EfiHiiGetImage,
    /// Modifies an image.
    pub set_image: EfiHiiSetImage,
    /// Renders an image buffer.
    pub draw_image: EfiHiiDrawImage,
    /// Renders an image by identifier.
    pub draw_image_id: EfiHiiDrawImageId,
}

// Function pointer definitions for Extended HII Image Protocol

/// Adds an image to an HII package list using extended protocol.
pub type EfiHiiNewImageEx = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageExProtocol,
    package_list: EfiHiiHandle,
    image_id: *mut EfiImageId,
    image: *const EfiImageInput,
) -> EfiStatus;

/// Retrieves an image from an HII package list, invoking installed decoders if needed.
pub type EfiHiiGetImageEx = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageExProtocol,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *mut EfiImageInput,
) -> EfiStatus;

/// Modifies an existing image in an HII package list.
pub type EfiHiiSetImageEx = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageExProtocol,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *const EfiImageInput,
) -> EfiStatus;

/// Draws an image onto a destination surface or screen with decoder support.
pub type EfiHiiDrawImageEx = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageExProtocol,
    flags: u32,
    image: *const EfiImageInput,
    blt: *mut *mut EfiImageOutput,
    blt_x: UIntN,
    blt_y: UIntN,
) -> EfiStatus;

/// Draws an image looked up from a package list by ID with decoder support.
pub type EfiHiiDrawImageIdEx = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageExProtocol,
    flags: u32,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    blt: *mut *mut EfiImageOutput,
    blt_x: UIntN,
    blt_y: UIntN,
) -> EfiStatus;

/// Returns geometry and dimensions of an image without full decoding.
pub type EfiHiiGetImageInfo = unsafe extern "efiapi" fn(
    this: *const EfiHiiImageExProtocol,
    package_list: EfiHiiHandle,
    image_id: EfiImageId,
    image: *mut EfiImageOutput,
) -> EfiStatus;

/// Raw interface structure for `EFI_HII_IMAGE_EX_PROTOCOL`.
#[repr(C)]
pub struct EfiHiiImageExProtocol {
    /// Adds an image to package list.
    pub new_image_ex: EfiHiiNewImageEx,
    /// Retrieves an image.
    pub get_image_ex: EfiHiiGetImageEx,
    /// Modifies an image.
    pub set_image_ex: EfiHiiSetImageEx,
    /// Renders an image buffer.
    pub draw_image_ex: EfiHiiDrawImageEx,
    /// Renders an image by identifier.
    pub draw_image_id_ex: EfiHiiDrawImageIdEx,
    /// Retrieves image dimensions and geometry.
    pub get_image_info: EfiHiiGetImageInfo,
}

/// Safe client wrapper around `EfiHiiImageProtocol`.
pub struct HiiImage<'a> {
    proto: &'a EfiHiiImageProtocol,
}

impl HiiImage<'_> {
    /// Protocol GUID.
    pub const GUID: EfiGuid = EFI_HII_IMAGE_PROTOCOL_GUID;

    /// Wraps a raw pointer to an `EfiHiiImageProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must ensure `proto` points to a valid, live `EfiHiiImageProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiHiiImageProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Adds a new image to the specified package list.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if adding the image failed.
    pub fn new_image(
        &self,
        package_list: EfiHiiHandle,
        image: &EfiImageInput,
    ) -> Result<EfiImageId, EfiStatus> {
        let mut image_id: EfiImageId = 0;
        let status = unsafe {
            (self.proto.new_image)(
                self.proto,
                package_list,
                &raw mut image_id,
                core::ptr::from_ref::<EfiImageInput>(image),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(image_id)
        } else {
            Err(status)
        }
    }

    /// Retrieves an image by identifier.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if retrieving the image failed.
    pub fn get_image(
        &self,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
    ) -> Result<EfiImageInput, EfiStatus> {
        let mut image = EfiImageInput {
            flags: 0,
            width: 0,
            height: 0,
            bitmap: core::ptr::null_mut(),
        };
        let status =
            unsafe { (self.proto.get_image)(self.proto, package_list, image_id, &raw mut image) };
        if status == EfiStatus::SUCCESS {
            Ok(image)
        } else {
            Err(status)
        }
    }

    /// Updates an existing image in the specified package list.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if setting the image failed.
    pub fn set_image(
        &self,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
        image: &EfiImageInput,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.set_image)(
                self.proto,
                package_list,
                image_id,
                core::ptr::from_ref::<EfiImageInput>(image),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Draws an image onto the destination surface.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if rendering failed.
    pub fn draw_image(
        &self,
        flags: u32,
        image: &EfiImageInput,
        blt: &mut *mut EfiImageOutput,
        blt_x: UIntN,
        blt_y: UIntN,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.draw_image)(
                self.proto,
                flags,
                core::ptr::from_ref::<EfiImageInput>(image),
                blt,
                blt_x,
                blt_y,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Draws an image looked up by identifier.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if rendering failed.
    pub fn draw_image_id(
        &self,
        flags: u32,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
        blt: &mut *mut EfiImageOutput,
        blt_x: UIntN,
        blt_y: UIntN,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.draw_image_id)(self.proto, flags, package_list, image_id, blt, blt_x, blt_y)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiHiiImageExProtocol`.
pub struct HiiImageEx<'a> {
    proto: &'a EfiHiiImageExProtocol,
}

impl HiiImageEx<'_> {
    /// Extended Image Protocol GUID.
    pub const GUID: EfiGuid = EFI_HII_IMAGE_EX_PROTOCOL_GUID;

    /// Wraps a raw pointer to an `EfiHiiImageExProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must ensure `proto` points to a valid, live `EfiHiiImageExProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiHiiImageExProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Adds a new image using extended protocol.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if adding the image failed.
    pub fn new_image_ex(
        &self,
        package_list: EfiHiiHandle,
        image: &EfiImageInput,
    ) -> Result<EfiImageId, EfiStatus> {
        let mut image_id: EfiImageId = 0;
        let status = unsafe {
            (self.proto.new_image_ex)(
                self.proto,
                package_list,
                &raw mut image_id,
                core::ptr::from_ref::<EfiImageInput>(image),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(image_id)
        } else {
            Err(status)
        }
    }

    /// Retrieves an image by identifier with decoder invocation.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if retrieval failed.
    pub fn get_image_ex(
        &self,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
    ) -> Result<EfiImageInput, EfiStatus> {
        let mut image = EfiImageInput {
            flags: 0,
            width: 0,
            height: 0,
            bitmap: core::ptr::null_mut(),
        };
        let status = unsafe {
            (self.proto.get_image_ex)(self.proto, package_list, image_id, &raw mut image)
        };
        if status == EfiStatus::SUCCESS {
            Ok(image)
        } else {
            Err(status)
        }
    }

    /// Updates an existing image using extended protocol.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if updating the image failed.
    pub fn set_image_ex(
        &self,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
        image: &EfiImageInput,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.set_image_ex)(
                self.proto,
                package_list,
                image_id,
                core::ptr::from_ref::<EfiImageInput>(image),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Draws an image onto the destination surface using extended decoder support.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if rendering failed.
    pub fn draw_image_ex(
        &self,
        flags: u32,
        image: &EfiImageInput,
        blt: &mut *mut EfiImageOutput,
        blt_x: UIntN,
        blt_y: UIntN,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.draw_image_ex)(
                self.proto,
                flags,
                core::ptr::from_ref::<EfiImageInput>(image),
                blt,
                blt_x,
                blt_y,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Draws an image looked up by identifier with extended decoder support.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if rendering failed.
    pub fn draw_image_id_ex(
        &self,
        flags: u32,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
        blt: &mut *mut EfiImageOutput,
        blt_x: UIntN,
        blt_y: UIntN,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.draw_image_id_ex)(
                self.proto,
                flags,
                package_list,
                image_id,
                blt,
                blt_x,
                blt_y,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Gets geometry and dimensions of an image without full rasterization.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if query failed.
    pub fn get_image_info(
        &self,
        package_list: EfiHiiHandle,
        image_id: EfiImageId,
    ) -> Result<EfiImageOutput, EfiStatus> {
        let mut output = EfiImageOutput {
            width: 0,
            height: 0,
            image_surface: core::ptr::null_mut(),
        };
        let status = unsafe {
            (self.proto.get_image_info)(self.proto, package_list, image_id, &raw mut output)
        };
        if status == EfiStatus::SUCCESS {
            Ok(output)
        } else {
            Err(status)
        }
    }
}
