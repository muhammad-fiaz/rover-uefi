//! # Management Mode (SMM) Event Notification Protocols
//!
//! Provides protocol identifiers published by the SMM foundation core to notify
//! SMM drivers of system state transitions corresponding to DXE event groups:
//! - Exit Boot Services
//! - Ready To Boot
//! - Legacy Boot

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
