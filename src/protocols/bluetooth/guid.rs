//! # Bluetooth Protocol GUIDs
//!
//! Defined in UEFI Specification 2.5+ Section 26.

use crate::base::EfiGuid;

/// EFI Bluetooth Host Controller Protocol GUID: `{b3930571-beba-4fc5-9203-9427242e6a43}`.
pub const EFI_BLUETOOTH_HC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB393_0571,
    0xBEBA,
    0x4FC5,
    [0x92, 0x03, 0x94, 0x27, 0x24, 0x2E, 0x6A, 0x43],
);

/// EFI Bluetooth I/O Protocol GUID: `{467313de-4e30-43f1-943e-323f89845db5}`.
pub const EFI_BLUETOOTH_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4673_13DE,
    0x4E30,
    0x43F1,
    [0x94, 0x3E, 0x32, 0x3F, 0x89, 0x84, 0x5D, 0xB5],
);

/// EFI Bluetooth Configuration Protocol GUID: `{62960cf3-40ff-4263-a77c-dfdebd191b4b}`.
pub const EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6296_0CF3,
    0x40FF,
    0x4263,
    [0xA7, 0x7C, 0xDF, 0xDE, 0xBD, 0x19, 0x1B, 0x4B],
);
