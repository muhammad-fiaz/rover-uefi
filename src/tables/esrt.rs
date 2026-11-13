//! # EFI System Resource Table (ESRT)
//!
//! Defined in UEFI Specification 2.5+ Section 23.
//!
//! Describes non-volatile updatable system and device firmware components,
//! versioning, anti-rollback lowest supported versions, and capsule update status.

use crate::base::{EfiGuid, EfiStatus};

/// EFI System Resource Table GUID: `{b122a263-3661-4f68-9929-78f8b0d62180}`.
pub const EFI_SYSTEM_RESOURCE_TABLE_GUID: EfiGuid = EfiGuid::new(
    0xB122_A263,
    0x3661,
    0x4F68,
    [0x99, 0x29, 0x78, 0xF8, 0xB0, 0xD6, 0x21, 0x80],
);

/// ESRT Management Protocol GUID: `{a340c064-723c-4a9c-a4dd-d5b47a26fbb0}`.
pub const ESRT_MANAGEMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA340_C064,
    0x723C,
    0x4A9C,
    [0xA4, 0xDD, 0xD5, 0xB4, 0x7A, 0x26, 0xFB, 0xB0],
);

/// Current ESRT entry format version.
pub const EFI_SYSTEM_RESOURCE_TABLE_FIRMWARE_RESOURCE_VERSION: u32 = 1;

/// Unknown firmware resource type.
pub const ESRT_FW_TYPE_UNKNOWN: u32 = 0x0000_0000;
/// System firmware (BIOS/UEFI main image).
pub const ESRT_FW_TYPE_SYSTEMFIRMWARE: u32 = 0x0000_0001;
/// Device firmware (`PCIe` peripheral, controller, EC).
pub const ESRT_FW_TYPE_DEVICEFIRMWARE: u32 = 0x0000_0002;
/// UEFI Driver in option ROM or flash.
pub const ESRT_FW_TYPE_UEFIDRIVER: u32 = 0x0000_0003;

/// Last attempt status: Success.
pub const LAST_ATTEMPT_STATUS_SUCCESS: u32 = 0x0000_0000;
/// Last attempt status: Error unsuccessful.
pub const LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL: u32 = 0x0000_0001;
/// Last attempt status: Error insufficient resources.
pub const LAST_ATTEMPT_STATUS_ERROR_INSUFFICIENT_RESOURCES: u32 = 0x0000_0002;
/// Last attempt status: Error incorrect version.
pub const LAST_ATTEMPT_STATUS_ERROR_INCORRECT_VERSION: u32 = 0x0000_0003;
/// Last attempt status: Error invalid format.
pub const LAST_ATTEMPT_STATUS_ERROR_INVALID_FORMAT: u32 = 0x0000_0004;
/// Last attempt status: Error authentication failure.
pub const LAST_ATTEMPT_STATUS_ERROR_AUTH_ERROR: u32 = 0x0000_0005;
/// Last attempt status: Error power event AC disconnected.
pub const LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_AC: u32 = 0x0000_0006;
/// Last attempt status: Error power event battery low.
pub const LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_BATT: u32 = 0x0000_0007;
/// Last attempt status: Error unsatisfied dependencies.
pub const LAST_ATTEMPT_STATUS_ERROR_UNSATISFIED_DEPENDENCIES: u32 = 0x0000_0008;

/// Minimum status code for vendor-defined failure range.
pub const LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MIN: u32 = 0x0000_1000;
/// Maximum status code for vendor-defined failure range.
pub const LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MAX: u32 = 0x0000_3FFF;

/// An individual ESRT firmware resource description entry (`EFI_SYSTEM_RESOURCE_ENTRY`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiSystemResourceEntry {
    /// Unique GUID identifying the firmware component targeted by `UpdateCapsule()`.
    pub fw_class: EfiGuid,
    /// Firmware type (`ESRT_FW_TYPE_*`).
    pub fw_type: u32,
    /// Monotonically increasing current version of this firmware component.
    pub fw_version: u32,
    /// Lowest version to which rollback is permitted for anti-rollback security.
    pub lowest_supported_fw_version: u32,
    /// Capsule header flags associated with this entry.
    pub capsule_flags: u32,
    /// Last version for which an update attempt was executed.
    pub last_attempt_version: u32,
    /// Result status of the last update attempt (`LAST_ATTEMPT_STATUS_*`).
    pub last_attempt_status: u32,
}

/// Header of the EFI System Resource Table (`EFI_SYSTEM_RESOURCE_TABLE`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiSystemResourceTable {
    /// Number of firmware resource entries present in the table.
    pub fw_resource_count: u32,
    /// Maximum number of resource entries allocatable without reallocating table.
    pub fw_resource_count_max: u32,
    /// Version of the entry structures (currently 1).
    pub fw_resource_version: u64,
}

/// Retrieves an ESRT entry by firmware class GUID.
pub type EsrtGetEntry = unsafe extern "efiapi" fn(
    fw_class: *const EfiGuid,
    entry: *mut EfiSystemResourceEntry,
) -> EfiStatus;

/// Updates an ESRT entry in the firmware cache.
pub type EsrtUpdateEntry =
    unsafe extern "efiapi" fn(entry: *const EfiSystemResourceEntry) -> EfiStatus;

/// Registers a new ESRT entry into the firmware cache.
pub type EsrtRegisterEntry =
    unsafe extern "efiapi" fn(entry: *const EfiSystemResourceEntry) -> EfiStatus;

/// Unregisters an ESRT entry from the firmware cache.
pub type EsrtUnregisterEntry = unsafe extern "efiapi" fn(fw_class: *const EfiGuid) -> EfiStatus;

/// Synchronizes cached ESRT entries with discovered FMP protocol instances.
pub type EsrtSyncFmp = unsafe extern "efiapi" fn() -> EfiStatus;

/// Locks the ESRT repository to prevent modification before `EndOfDxe`.
pub type EsrtLockRepository = unsafe extern "efiapi" fn() -> EfiStatus;

/// ESRT Management Protocol interface (`ESRT_MANAGEMENT_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EsrtManagementProtocol {
    /// Retrieves an entry by GUID.
    pub get_esrt_entry: EsrtGetEntry,
    /// Updates an existing entry.
    pub update_esrt_entry: EsrtUpdateEntry,
    /// Registers a new entry.
    pub register_esrt_entry: EsrtRegisterEntry,
    /// Unregisters an existing entry.
    pub unregister_esrt_entry: EsrtUnregisterEntry,
    /// Synchronizes entries with FMP protocols.
    pub sync_esrt_fmp: EsrtSyncFmp,
    /// Locks the repository.
    pub lock_esrt_repository: EsrtLockRepository,
}
