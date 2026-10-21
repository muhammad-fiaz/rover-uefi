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

/// Canonical alias for the Firmware Management Protocol GUID.
pub const FMP_PROTOCOL_GUID: EfiGuid = FIRMWARE_MANAGEMENT_PROTOCOL_GUID;

/// Legacy/capsule identifier GUID for backwards compatibility.
pub const FMP_GUID: EfiGuid = FIRMWARE_MANAGEMENT_PROTOCOL_GUID;

/// The Firmware Management Capsule ID GUID (`EFI_FIRMWARE_MANAGEMENT_CAPSULE_ID_GUID`).
pub const FMP_CAPSULE_GUID: EfiGuid = EfiGuid::new(
    0x6DCB_D5ED,
    0xE82D,
    0x4C44,
    [0xBD, 0xA1, 0x71, 0x94, 0x19, 0x9A, 0xD9, 0x2A],
);

/// Legacy constant alias for capsule identifier GUID.
pub const EFI_FIRMWARE_MANAGEMENT_CAPSULE_ID_GUID: EfiGuid = FMP_CAPSULE_GUID;

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
