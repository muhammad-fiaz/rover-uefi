//! # Driver Supported EFI Version Protocol
//!
//! Defined by the UEFI Specification Section 11.9: Driver Supported EFI Version Protocol.
//!
//! Advertises the specification version that a driver image was designed and tested against.

use crate::base::EfiGuid;

/// The `EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID`.
pub const EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5C19_87D7,
    0x4CD1,
    0x4383,
    [0xA3, 0xD4, 0x61, 0x2B, 0x17, 0x86, 0x41, 0x44],
);

/// The raw ABI structure for `EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiDriverSupportedEfiVersionProtocol {
    /// Size of this structure in bytes.
    pub length: u32,
    /// Supported specification version (matches System Table revision).
    pub firmware_version: u32,
}
