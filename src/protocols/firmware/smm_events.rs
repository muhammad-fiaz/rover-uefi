//! # Management Mode (SMM) Event Notification Protocols
//!
//! Provides protocol identifiers published by the SMM foundation core to notify
//! SMM drivers of system state transitions corresponding to DXE event groups:
//! - Exit Boot Services
//! - Ready To Boot
//! - Legacy Boot
//! - Ready To Lock
//! - End Of DXE
//! - End of S3 Resume

use crate::base::EfiGuid;

/// SMM Exit Boot Services Notification Protocol GUID: `{296eb418-c4c8-4e05-ab59-39e8af56f00a}`.
///
/// Published in the SMM handle database when DXE signals `EFI_EVENT_GROUP_EXIT_BOOT_SERVICES`,
/// alerting SMM drivers that boot services are terminating and OS runtime is imminent.
pub const SMM_EXIT_BOOT_SERVICES_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x296E_B418,
    0xC4C8,
    0x4E05,
    [0xAB, 0x59, 0x39, 0xE8, 0xAF, 0x56, 0xF0, 0x0A],
);

/// SMM Ready To Boot Notification Protocol GUID: `{6e057ecf-fa99-4f39-95bc-59f9921d17e4}`.
///
/// Published in the SMM handle database when DXE signals `EFI_EVENT_GROUP_READY_TO_BOOT`,
/// indicating that the firmware is preparing to launch a boot option.
pub const SMM_READY_TO_BOOT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6E05_7ECF,
    0xFA99,
    0x4F39,
    [0x95, 0xBC, 0x59, 0xF9, 0x92, 0x1D, 0x17, 0xE4],
);

/// SMM Legacy Boot Notification Protocol GUID: `{85a8ab57-0644-4110-850f-981322047070}`.
///
/// Published in the SMM handle database when DXE signals legacy boot transitions,
/// indicating compatibility support module handoff.
pub const SMM_LEGACY_BOOT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x85A8_AB57,
    0x0644,
    0x4110,
    [0x85, 0x0F, 0x98, 0x13, 0x22, 0x04, 0x70, 0x70],
);

/// MM / SMM Ready To Lock Protocol GUID: `{47b7fa8c-f4bd-4af6-8200-333086f0d2c8}`.
///
/// Indicates that SMM resources and services not intended for 3rd-party code are about to be locked.
pub const EFI_MM_READY_TO_LOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x47B7_FA8C,
    0xF4BD,
    0x4AF6,
    [0x82, 0x00, 0x33, 0x30, 0x86, 0xF0, 0xD2, 0xC8],
);

/// Alias for [`EFI_MM_READY_TO_LOCK_PROTOCOL_GUID`].
pub const EFI_SMM_READY_TO_LOCK_PROTOCOL_GUID: EfiGuid = EFI_MM_READY_TO_LOCK_PROTOCOL_GUID;

/// DXE MM / SMM Ready To Lock Protocol GUID: `{60ff8964-e906-41d0-afed-f241e974e08e}`.
///
/// Published in DXE environment immediately after signaling the End of DXE event.
pub const EFI_DXE_MM_READY_TO_LOCK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x60FF_8964,
    0xE906,
    0x41D0,
    [0xAF, 0xED, 0xF2, 0x41, 0xE9, 0x74, 0xE0, 0x8E],
);

/// Alias for [`EFI_DXE_MM_READY_TO_LOCK_PROTOCOL_GUID`].
pub const EFI_DXE_SMM_READY_TO_LOCK_PROTOCOL_GUID: EfiGuid = EFI_DXE_MM_READY_TO_LOCK_PROTOCOL_GUID;

/// MM / SMM End Of DXE Protocol GUID: `{24e70042-d5c5-4260-8c39-0ad3aa32e93d}`.
///
/// Indicates end of the DXE execution phase when all components are under the authority
/// of the platform manufacturer.
pub const EFI_MM_END_OF_DXE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x24E7_0042,
    0xD5C5,
    0x4260,
    [0x8C, 0x39, 0x0A, 0xD3, 0xAA, 0x32, 0xE9, 0x3D],
);

/// Alias for [`EFI_MM_END_OF_DXE_PROTOCOL_GUID`].
pub const EFI_SMM_END_OF_DXE_PROTOCOL_GUID: EfiGuid = EFI_MM_END_OF_DXE_PROTOCOL_GUID;

/// MM End Of S3 Resume Protocol GUID: `{887c9431-7b94-49c0-9a4d-f95d985a1a1c}`.
pub const EFI_MM_END_OF_S3_RESUME_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x887C_9431,
    0x7B94,
    0x49C0,
    [0x9A, 0x4D, 0xF9, 0x5D, 0x98, 0x5A, 0x1A, 0x1C],
);
