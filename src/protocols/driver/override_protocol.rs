//! # Driver Override Protocols
//!
//! Defined by the UEFI Specification Section 11.4: Platform Driver Override Protocol
//! and Section 11.5: Bus Specific Driver Override Protocol.
//!
//! Allows platforms and bus drivers to supply higher priority driver image handles
//! or paths for a given controller.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;

/// The `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID`.
pub const EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6B30_C738,
    0xA391,
    0x401E,
    [0x92, 0x60, 0x8A, 0x76, 0x79, 0x90, 0x67, 0xEB],
);

/// The `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID`.
pub const EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3BC1_B285,
    0x8A15,
    0x4A82,
    [0xAA, 0xBF, 0x4D, 0x7D, 0x13, 0xFB, 0x32, 0x65],
);

/// Function pointer for `EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER`.
pub type EfiPlatformDriverOverrideGetDriver = unsafe extern "efiapi" fn(
    this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_handle: *mut EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER_PATH`.
pub type EfiPlatformDriverOverrideGetDriverPath = unsafe extern "efiapi" fn(
    this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function pointer for `EFI_PLATFORM_DRIVER_OVERRIDE_DRIVER_LOADED`.
pub type EfiPlatformDriverOverrideDriverLoaded = unsafe extern "efiapi" fn(
    this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_path: *const EfiDevicePath,
    driver_image_handle: EfiHandle,
) -> EfiStatus;

/// The raw ABI structure for `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPlatformDriverOverrideProtocol {
    /// Retrieves next override driver image handle.
    pub get_driver: EfiPlatformDriverOverrideGetDriver,
    /// Retrieves next override driver device path.
    pub get_driver_path: EfiPlatformDriverOverrideGetDriverPath,
    /// Notifies that an override driver was successfully loaded.
    pub driver_loaded: EfiPlatformDriverOverrideDriverLoaded,
}

/// Function pointer for `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_GET_DRIVER`.
pub type EfiBusSpecificDriverOverrideGetDriver = unsafe extern "efiapi" fn(
    this: *const EfiBusSpecificDriverOverrideProtocol,
    driver_image_handle: *mut EfiHandle,
) -> EfiStatus;

/// The raw ABI structure for `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBusSpecificDriverOverrideProtocol {
    /// Retrieves next bus-specific driver override image handle.
    pub get_driver: EfiBusSpecificDriverOverrideGetDriver,
}
