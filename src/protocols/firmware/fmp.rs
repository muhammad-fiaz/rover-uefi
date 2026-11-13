//! # Firmware Management Protocol
//!
//! `Firmware Management Protocol (FMP)` identifiers, structures, and
//! capsule header parsing according to the UEFI specification.
//!
//! Applying a capsule remains a firmware operation; this module provides
//! the typed specification ABI structures and parsing primitives.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The standard Firmware Management Protocol GUID (`EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID`).
pub const FIRMWARE_MANAGEMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x86C7_7A67,
    0x0B97,
    0x4633,
    [0xA1, 0x87, 0x49, 0x10, 0x4D, 0x06, 0x85, 0xC7],
);

/// Exact UEFI specification naming for Firmware Management Protocol GUID.
pub const EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID: EfiGuid = FIRMWARE_MANAGEMENT_PROTOCOL_GUID;

/// Canonical alias for the Firmware Management Protocol GUID.
pub const FMP_PROTOCOL_GUID: EfiGuid = FIRMWARE_MANAGEMENT_PROTOCOL_GUID;

/// The Firmware Management Capsule ID GUID.
pub const FMP_CAPSULE_GUID: EfiGuid = EfiGuid::new(
    0x6DCB_D5ED,
    0xE82D,
    0x4C44,
    [0xBD, 0xA1, 0x71, 0x94, 0x19, 0x9A, 0xD9, 0x2A],
);

// Image Attribute Definitions
/// The device supports firmware image update.
pub const IMAGE_ATTRIBUTE_IMAGE_UPDATABLE: u64 = 0x0000_0000_0000_0001;
/// Reset of the device is required for the new firmware image to take effect.
pub const IMAGE_ATTRIBUTE_RESET_REQUIRED: u64 = 0x0000_0000_0000_0002;
/// Authentication is required to perform image operations.
pub const IMAGE_ATTRIBUTE_AUTHENTICATION_REQUIRED: u64 = 0x0000_0000_0000_0004;
/// The firmware image is currently in use.
pub const IMAGE_ATTRIBUTE_IN_USE: u64 = 0x0000_0000_0000_0008;
/// The image is an EFI compatible image.
pub const IMAGE_ATTRIBUTE_UEFI_IMAGE: u64 = 0x0000_0000_0000_0010;
/// A dependency section is associated with the image.
pub const IMAGE_ATTRIBUTE_DEPENDENCY: u64 = 0x0000_0000_0000_0020;

// Image Updatable Definitions
/// `SetImage` will accept the new image.
pub const IMAGE_UPDATABLE_VALID: u32 = 0x0000_0001;
/// `SetImage` will reject the new image.
pub const IMAGE_UPDATABLE_INVALID: u32 = 0x0000_0002;
/// Rejection due to unrecognized image type for this device.
pub const IMAGE_UPDATABLE_INVALID_TYPE: u32 = 0x0000_0004;
/// Rejection due to version older than current image (downgrade prohibited).
pub const IMAGE_UPDATABLE_INVALID_OLD: u32 = 0x0000_0008;
/// `SetImage` will accept only if correct `VendorCode` is provided.
pub const IMAGE_UPDATABLE_VALID_WITH_VENDOR_CODE: u32 = 0x0000_0010;

// Package Attribute Definitions
/// Device supports update of firmware package version.
pub const PACKAGE_ATTRIBUTE_VERSION_UPDATABLE: u64 = 0x0000_0000_0000_0001;
/// Reset of the device is required for new package info to take effect.
pub const PACKAGE_ATTRIBUTE_RESET_REQUIRED: u64 = 0x0000_0000_0000_0002;
/// Authentication is required to update package info.
pub const PACKAGE_ATTRIBUTE_AUTHENTICATION_REQUIRED: u64 = 0x0000_0000_0000_0004;

// Capsule Support flags for EFI_FIRMWARE_MANAGEMENT_CAPSULE_IMAGE_HEADER
/// Authentication structure is included in the capsule.
pub const CAPSULE_SUPPORT_AUTHENTICATION: u64 = 0x0000_0000_0000_0001;
/// Dependency expression is included in the capsule.
pub const CAPSULE_SUPPORT_DEPENDENCY: u64 = 0x0000_0000_0000_0002;

/// Firmware image dependency expression structure header.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFirmwareImageDep {
    /// Array of dependency expression opcodes.
    pub dependencies: [u8; 1],
}

/// Firmware image descriptor structure describing device firmware capabilities.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFirmwareImageDescriptor {
    /// Unique number identifying the image within the device (1..=DescriptorCount).
    pub image_index: u8,
    /// Unique GUID identifying the firmware image type.
    pub image_type_id: EfiGuid,
    /// Unique number identifying the firmware image.
    pub image_id: u64,
    /// Null-terminated UTF-16 string representing the firmware image name.
    pub image_id_name: *const Char16,
    /// Version number of the device firmware.
    pub version: u32,
    /// Null-terminated UTF-16 string representing version name.
    pub version_name: *const Char16,
    /// Size of the image in bytes.
    pub size: UIntN,
    /// Supported image attribute bitmask.
    pub attributes_supported: u64,
    /// Current image attribute settings.
    pub attributes_setting: u64,
    /// Image compatibilities bitmask.
    pub compatibilities: u64,
    /// Lowest version accepted (version 2+).
    pub lowest_supported_image_version: u32,
    /// Version last attempted to update (version 3+).
    pub last_attempt_version: u32,
    /// Status of last attempted update (version 3+).
    pub last_attempt_status: u32,
    /// Unique hardware instance identifier (version 3+).
    pub hardware_instance: u64,
    /// Pointer to dependency structure (version 4+).
    pub dependencies: *const EfiFirmwareImageDep,
}

/// Callback function to report progress of a firmware update (1..=100%).
pub type EfiFirmwareManagementUpdateImageProgress =
    unsafe extern "efiapi" fn(completion: UIntN) -> EfiStatus;

/// Function pointer for `EFI_FIRMWARE_MANAGEMENT_PROTOCOL.GetImageInfo`.
pub type EfiFirmwareManagementGetImageInfo = unsafe extern "efiapi" fn(
    this: *mut EfiFirmwareManagementProtocol,
    image_info_size: *mut UIntN,
    image_info: *mut EfiFirmwareImageDescriptor,
    descriptor_version: *mut u32,
    descriptor_count: *mut u8,
    descriptor_size: *mut UIntN,
    package_version: *mut u32,
    package_version_name: *mut *mut Char16,
) -> EfiStatus;

/// Function pointer for `EFI_FIRMWARE_MANAGEMENT_PROTOCOL.GetImage`.
pub type EfiFirmwareManagementGetImage = unsafe extern "efiapi" fn(
    this: *mut EfiFirmwareManagementProtocol,
    image_index: u8,
    image: *mut c_void,
    image_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_FIRMWARE_MANAGEMENT_PROTOCOL.SetImage`.
pub type EfiFirmwareManagementSetImage = unsafe extern "efiapi" fn(
    this: *mut EfiFirmwareManagementProtocol,
    image_index: u8,
    image: *const c_void,
    image_size: UIntN,
    vendor_code: *const c_void,
    progress: Option<EfiFirmwareManagementUpdateImageProgress>,
    abort_reason: *mut *mut Char16,
) -> EfiStatus;

/// Function pointer for `EFI_FIRMWARE_MANAGEMENT_PROTOCOL.CheckImage`.
pub type EfiFirmwareManagementCheckImage = unsafe extern "efiapi" fn(
    this: *mut EfiFirmwareManagementProtocol,
    image_index: u8,
    image: *const c_void,
    image_size: UIntN,
    image_updatable: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_FIRMWARE_MANAGEMENT_PROTOCOL.GetPackageInfo`.
pub type EfiFirmwareManagementGetPackageInfo = unsafe extern "efiapi" fn(
    this: *mut EfiFirmwareManagementProtocol,
    package_version: *mut u32,
    package_version_name: *mut *mut Char16,
    package_version_name_max_len: *mut u32,
    attributes_supported: *mut u64,
    attributes_setting: *mut u64,
) -> EfiStatus;

/// Function pointer for `EFI_FIRMWARE_MANAGEMENT_PROTOCOL.SetPackageInfo`.
pub type EfiFirmwareManagementSetPackageInfo = unsafe extern "efiapi" fn(
    this: *mut EfiFirmwareManagementProtocol,
    image: *const c_void,
    image_size: UIntN,
    vendor_code: *const c_void,
    package_version: u32,
    package_version_name: *const Char16,
) -> EfiStatus;

/// The `EFI_FIRMWARE_MANAGEMENT_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFirmwareManagementProtocol {
    /// Retrieves information about the device's current firmware images.
    pub get_image_info: EfiFirmwareManagementGetImageInfo,
    /// Retrieves a copy of the current firmware image.
    pub get_image: EfiFirmwareManagementGetImage,
    /// Updates the device firmware with a new image.
    pub set_image: EfiFirmwareManagementSetImage,
    /// Validates a firmware image without updating hardware.
    pub check_image: EfiFirmwareManagementCheckImage,
    /// Retrieves firmware package information.
    pub get_package_info: EfiFirmwareManagementGetPackageInfo,
    /// Updates firmware package information.
    pub set_package_info: EfiFirmwareManagementSetPackageInfo,
}

/// A parsed firmware management capsule header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FmpCapsuleHeader {
    /// The header size.
    pub header_size: u32,
    /// The flags.
    pub flags: u32,
    /// The image type identifier.
    pub image_type: EfiGuid,
    /// The image version.
    pub image_version: u32,
    /// The image size.
    pub image_size: u32,
}

/// Parses a firmware management capsule header.
///
/// The layout is `CapsuleGuid(16)`, `HeaderSize(u32)`,
/// `Flags(u32)`, `ImageType(16)`, `ImageVersion(u32)`,
/// `ImageSize(u32)`. Returns `None` when the buffer is shorter than
/// 44 bytes or the header size is invalid.
#[must_use]
pub fn parse_fmp_capsule_header(input: &[u8]) -> Option<FmpCapsuleHeader> {
    if input.len() < 44 {
        return None;
    }
    let mut header_size_raw = [0_u8; 4];
    header_size_raw.copy_from_slice(input.get(16..20)?);
    let header_size = u32::from_le_bytes(header_size_raw);
    if header_size < 44 || (header_size as usize) > input.len() {
        return None;
    }
    let mut flags_raw = [0_u8; 4];
    flags_raw.copy_from_slice(input.get(20..24)?);
    let flags = u32::from_le_bytes(flags_raw);
    let mut type_bytes = [0_u8; 16];
    type_bytes.copy_from_slice(input.get(24..40)?);
    let image_type = EfiGuid::from_bytes(type_bytes);
    let mut version_raw = [0_u8; 4];
    version_raw.copy_from_slice(input.get(40..44)?);
    let mut size_raw = [0_u8; 4];
    let image_size = if input.len() >= 48 {
        size_raw.copy_from_slice(input.get(44..48)?);
        u32::from_le_bytes(size_raw)
    } else {
        0
    };
    Some(FmpCapsuleHeader {
        header_size,
        flags,
        image_type,
        image_version: u32::from_le_bytes(version_raw),
        image_size,
    })
}

/// Returns whether a capsule flags value is reserved-only.
#[must_use]
pub const fn is_reserved_flags(flags: u32) -> bool {
    flags == 0
}

/// Safe idiomatic wrapper around `EfiFirmwareManagementProtocol`.
pub struct FirmwareManagement<'a> {
    proto: &'a mut EfiFirmwareManagementProtocol,
}

impl<'a> FirmwareManagement<'a> {
    /// Creates a new `FirmwareManagement` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiFirmwareManagementProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiFirmwareManagementProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Queries information about the device's current firmware images.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `image_info_buffer` is insufficient, or an error status.
    pub fn get_image_info(
        &mut self,
        image_info_buffer: &mut [EfiFirmwareImageDescriptor],
    ) -> Result<(u32, u8, usize, u32), EfiStatus> {
        let mut size = core::mem::size_of_val(image_info_buffer) as UIntN;
        let mut descriptor_version = 0u32;
        let mut descriptor_count = 0u8;
        let mut descriptor_size = 0 as UIntN;
        let mut package_version = 0u32;
        let mut package_version_name = core::ptr::null_mut();

        let status = unsafe {
            (self.proto.get_image_info)(
                self.proto,
                core::ptr::from_mut(&mut size),
                image_info_buffer.as_mut_ptr(),
                core::ptr::from_mut(&mut descriptor_version),
                core::ptr::from_mut(&mut descriptor_count),
                core::ptr::from_mut(&mut descriptor_size),
                core::ptr::from_mut(&mut package_version),
                core::ptr::from_mut(&mut package_version_name),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((
                descriptor_version,
                descriptor_count,
                descriptor_size as usize,
                package_version,
            ))
        } else {
            Err(status)
        }
    }

    /// Retrieves a copy of the current firmware image.
    ///
    /// Returns the number of bytes written to `buffer`.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` is too small, or an error status.
    pub fn get_image(&mut self, image_index: u8, buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            (self.proto.get_image)(
                self.proto,
                image_index,
                buffer.as_mut_ptr().cast::<c_void>(),
                core::ptr::from_mut(&mut size),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(size as usize)
        } else {
            Err(status)
        }
    }

    /// Updates the device firmware with a new image.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware update fails or is rejected.
    pub fn set_image(
        &mut self,
        image_index: u8,
        image: &[u8],
        vendor_code: Option<&[u8]>,
        progress: Option<EfiFirmwareManagementUpdateImageProgress>,
    ) -> Result<(), EfiStatus> {
        let vendor_ptr = match vendor_code {
            Some(slice) => slice.as_ptr().cast::<c_void>(),
            None => core::ptr::null(),
        };
        let mut abort_reason = core::ptr::null_mut();

        let status = unsafe {
            (self.proto.set_image)(
                self.proto,
                image_index,
                image.as_ptr().cast::<c_void>(),
                image.len() as UIntN,
                vendor_ptr,
                progress,
                core::ptr::from_mut(&mut abort_reason),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Validates a firmware image without updating hardware.
    ///
    /// Returns the `image_updatable` status bitmask.
    ///
    /// # Errors
    ///
    /// Returns an error status on validation failure.
    pub fn check_image(&mut self, image_index: u8, image: &[u8]) -> Result<u32, EfiStatus> {
        let mut updatable = 0u32;
        let status = unsafe {
            (self.proto.check_image)(
                self.proto,
                image_index,
                image.as_ptr().cast::<c_void>(),
                image.len() as UIntN,
                core::ptr::from_mut(&mut updatable),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(updatable)
        } else {
            Err(status)
        }
    }

    /// Retrieves firmware package version and attributes.
    ///
    /// Returns `(package_version, attributes_supported, attributes_setting)`.
    ///
    /// # Errors
    ///
    /// Returns an error status if package info is unsupported or query fails.
    pub fn get_package_info(&mut self) -> Result<(u32, u64, u64), EfiStatus> {
        let mut pkg_version = 0u32;
        let mut pkg_name = core::ptr::null_mut();
        let mut pkg_name_max_len = 0u32;
        let mut attr_supported = 0u64;
        let mut attr_setting = 0u64;

        let status = unsafe {
            (self.proto.get_package_info)(
                self.proto,
                core::ptr::from_mut(&mut pkg_version),
                core::ptr::from_mut(&mut pkg_name),
                core::ptr::from_mut(&mut pkg_name_max_len),
                core::ptr::from_mut(&mut attr_supported),
                core::ptr::from_mut(&mut attr_setting),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((pkg_version, attr_supported, attr_setting))
        } else {
            Err(status)
        }
    }

    /// Updates firmware package version and attributes.
    ///
    /// # Errors
    ///
    /// Returns an error status if package update fails.
    pub fn set_package_info(
        &mut self,
        image: &[u8],
        vendor_code: Option<&[u8]>,
        package_version: u32,
        package_version_name: &[Char16],
    ) -> Result<(), EfiStatus> {
        let vendor_ptr = match vendor_code {
            Some(slice) => slice.as_ptr().cast::<c_void>(),
            None => core::ptr::null(),
        };

        let status = unsafe {
            (self.proto.set_package_info)(
                self.proto,
                image.as_ptr().cast::<c_void>(),
                image.len() as UIntN,
                vendor_ptr,
                package_version,
                package_version_name.as_ptr(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
