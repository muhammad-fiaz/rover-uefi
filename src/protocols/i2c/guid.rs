//! # I2C protocol identifiers
//!
//! UEFI PI 1.3+ I2C Protocol GUIDs.

use crate::base::EfiGuid;

/// The `EFI_I2C_MASTER_PROTOCOL` GUID.
pub const EFI_I2C_MASTER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xCD72_881F,
    0x45B5,
    0x4FEB,
    [0x98, 0xC8, 0x31, 0x3D, 0xA8, 0x11, 0x74, 0x62],
);

/// The `EFI_I2C_IO_PROTOCOL` GUID.
pub const EFI_I2C_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB60A_3E6B,
    0x18C4,
    0x46E5,
    [0xA2, 0x9A, 0xC9, 0xA1, 0x06, 0x65, 0xA2, 0x8E],
);
