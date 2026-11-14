//! # Capsule Architectural Protocol and Capsule PPIs
//!
//! Defined in PI Specification Volume 2: DXE and Volume 1: PEI, and Microsoft
//! Windows UEFI Firmware Update Platform Specification.
//!
//! Provides the DXE Capsule Architectural Protocol, PEI Capsule PPI for early
//! memory coalescing, Windows UX Display Capsule structures for graphical update progress,
//! and JSON payload capsule tables.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::firmware::pei::EfiPeiServices;

/// The capsule architectural protocol `GUID`.
///
/// The DXE driver producing this protocol initializes `UpdateCapsule` and
/// `QueryCapsuleCapabilities` in the UEFI Runtime Services Table and installs
/// this protocol on a new handle with a NULL interface pointer to signal
/// that capsule services are available.
pub const EFI_CAPSULE_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5053_697E,
    0x2CBC,
    0x4819,
    [0x90, 0xD9, 0x05, 0x80, 0xDE, 0xEE, 0x57, 0x54],
);

/// The PEI Capsule PPI `GUID`: `{3acf33ee-d892-40f4-a2fc-3854d2e1323d}`.
pub const EFI_PEI_CAPSULE_PPI_GUID: EfiGuid = EfiGuid::new(
    0x3ACF_33EE,
    0xD892,
    0x40F4,
    [0xA2, 0xFC, 0x38, 0x54, 0xD2, 0xE1, 0x32, 0x3D],
);

/// The PEI Capsule on Disk PPI `GUID`: `{2d770c80-ee12-4299-8806-03c031c5eebe}`.
pub const EFI_PEI_CAPSULE_ON_DISK_PPI_GUID: EfiGuid = EfiGuid::new(
    0x2D77_0C80,
    0xEE12,
    0x4299,
    [0x88, 0x06, 0x03, 0xC0, 0x31, 0xC5, 0xEE, 0xBE],
);

/// Function pointer for `EFI_PEI_CAPSULE_COALESCE`.
pub type EfiPeiCapsuleCoalesce = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    memory_base: *mut *mut c_void,
    mem_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_CAPSULE_CHECK_CAPSULE_UPDATE`.
pub type EfiPeiCapsuleCheckCapsuleUpdate =
    unsafe extern "efiapi" fn(pei_services: *const *const EfiPeiServices) -> EfiStatus;

/// Function pointer for `EFI_PEI_CAPSULE_CREATE_STATE`.
pub type EfiPeiCapsuleCreateState = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    capsule_base: *mut c_void,
    capsule_size: UIntN,
) -> EfiStatus;

/// PEI Capsule PPI interface table.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiCapsulePpi {
    /// Coalesces capsule fragments into contiguous system memory.
    pub coalesce: EfiPeiCapsuleCoalesce,
    /// Determines whether a capsule is available for processing.
    pub check_capsule_update: EfiPeiCapsuleCheckCapsuleUpdate,
    /// Creates capsule state descriptors.
    pub create_state: EfiPeiCapsuleCreateState,
}

/// Windows UX Capsule GUID: `{3b8c8162-188c-46a4-aec9-be43f1d65697}`.
pub const WINDOWS_UX_CAPSULE_GUID: EfiGuid = EfiGuid::new(
    0x3B8C_8162,
    0x188C,
    0x46A4,
    [0xAE, 0xC9, 0xBE, 0x43, 0xF1, 0xD6, 0x56, 0x97],
);

/// Windows UX display image payload header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayDisplayPayload {
    /// Payload structure version (typically 1).
    pub version: u8,
    /// Checksum of payload.
    pub checksum: u8,
    /// Image format type (0 = Bitmap / BMP).
    pub image_type: u8,
    /// Reserved byte.
    pub reserved: u8,
    /// Display graphics mode.
    pub mode: u32,
    /// Horizontal offset X.
    pub offset_x: u32,
    /// Vertical offset Y.
    pub offset_y: u32,
}

/// EFI JSON Configuration Data Table GUID: `{87367f87-1119-41ce-aaec-8be0111f558a}`.
pub const EFI_JSON_CONFIG_DATA_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x8736_7F87,
    0x1119,
    0x41CE,
    [0xAA, 0xEC, 0x8B, 0xE0, 0x11, 0x1F, 0x55, 0x8A],
);

/// EFI JSON Capsule Data Table GUID: `{35e7a725-8dd2-4cac-8011-33cda8109056}`.
pub const EFI_JSON_CAPSULE_DATA_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x35E7_A725,
    0x8DD2,
    0x4CAC,
    [0x80, 0x11, 0x33, 0xCD, 0xA8, 0x10, 0x90, 0x56],
);

/// EFI JSON Capsule Result Table GUID: `{dbc461c3-b3de-422a-b9b4-9886fd49a1e5}`.
pub const EFI_JSON_CAPSULE_RESULT_TABLE_GUID: EfiGuid = EfiGuid::new(
    0xDBC4_61C3,
    0xB3DE,
    0x422A,
    [0xB9, 0xB4, 0x98, 0x86, 0xFD, 0x49, 0xA1, 0xE5],
);

/// EFI JSON Capsule ID GUID: `{67d6f4cd-d6b8-4573-bf4a-de5e252d61ae}`.
pub const EFI_JSON_CAPSULE_ID_GUID: EfiGuid = EfiGuid::new(
    0x67D6_F4CD,
    0xD6B8,
    0x4573,
    [0xBF, 0x4A, 0xDE, 0x5E, 0x25, 0x2D, 0x61, 0xAE],
);

/// EFI JSON Capsule Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiJsonCapsuleHeader {
    /// Structure version (initially 0x00000001).
    pub version: u32,
    /// Unique capsule identifier.
    pub capsule_id: u32,
    /// Byte length of following JSON payload.
    pub payload_length: u32,
}

/// EFI JSON Configuration Data Item Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiJsonConfigDataItemHeader {
    /// Byte length of config data.
    pub config_data_length: u32,
}

/// EFI JSON Capsule Configuration Data Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiJsonCapsuleConfigDataHeader {
    /// Structure version (initially 0x00000001).
    pub version: u32,
    /// Total byte length of configuration payload.
    pub total_length: u32,
}
