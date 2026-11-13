//! # Audio Protocol GUIDs
//!
//! Standard EFI Audio I/O Protocol identifiers.

use crate::base::EfiGuid;

/// EFI Audio I/O Protocol GUID: `{f05b5cea-1384-4b83-8490-766e05479146}`.
pub const EFI_AUDIO_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xF05B_5CEA,
    0x1384,
    0x4B83,
    [0x84, 0x90, 0x76, 0x6E, 0x05, 0x47, 0x91, 0x46],
);
