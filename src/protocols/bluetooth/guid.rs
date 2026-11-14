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

/// EFI Bluetooth Attribute Service Binding Protocol GUID: `{5639867a-8c8e-408d-ac2f-4b61bdc0bbbb}`.
pub const EFI_BLUETOOTH_ATTRIBUTE_SERVICE_BINDING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5639_867A,
    0x8C8E,
    0x408D,
    [0xAC, 0x2F, 0x4B, 0x61, 0xBD, 0xC0, 0xBB, 0xBB],
);

/// EFI Bluetooth Attribute Protocol GUID: `{898890e9-84b2-4f3a-8c58-d8577813e0ac}`.
pub const EFI_BLUETOOTH_ATTRIBUTE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8988_90E9,
    0x84B2,
    0x4F3A,
    [0x8C, 0x58, 0xD8, 0x57, 0x78, 0x13, 0xE0, 0xAC],
);

/// EFI Bluetooth LE Config Protocol GUID: `{8f76da58-1f99-4275-a4ec-4756515b1ce8}`.
pub const EFI_BLUETOOTH_LE_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8F76_DA58,
    0x1F99,
    0x4275,
    [0xA4, 0xEC, 0x47, 0x56, 0x51, 0x5B, 0x1C, 0xE8],
);
