//! # SMBIOS identifiers
//!
//! Configuration table `GUID` constants.

use crate::base::EfiGuid;

/// The SMBIOS 2.x `GUID`.
pub const SMBIOS_2X_GUID: EfiGuid = EfiGuid::new(
    0xEB9D_2D31,
    0x2D88,
    0x11D3,
    [0x9A, 0x16, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// The SMBIOS 3.x `GUID`.
pub const SMBIOS_3X_GUID: EfiGuid = EfiGuid::new(
    0xF2FD_1544,
    0x9794,
    0x4A2C,
    [0x99, 0x2E, 0xE5, 0xBB, 0xCF, 0x20, 0xE3, 0x94],
);
