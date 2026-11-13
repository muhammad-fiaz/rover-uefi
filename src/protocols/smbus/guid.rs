//! # `SMBus` Protocol GUIDs
//!
//! Defined in UEFI Platform Initialization (PI) Specification Volume 5.

use crate::base::EfiGuid;

/// EFI `SMBus` Host Controller Protocol GUID: `{e49d33ed-513d-4634-b698-6f55aa751c1b}`.
pub const EFI_SMBUS_HC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE49D_33ED,
    0x513D,
    0x4634,
    [0xB6, 0x98, 0x6F, 0x55, 0xAA, 0x75, 0x1C, 0x1B],
);
