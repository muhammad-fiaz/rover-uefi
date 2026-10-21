//! # PCI identifiers
//!
//! Official PCI protocol `GUID` constants.

use crate::base::EfiGuid;

/// The PCI root bridge I/O protocol `GUID`.
pub const PCI_ROOT_BRIDGE_IO_GUID: EfiGuid = EfiGuid::new(
    0x2F70_7EBB,
    0x4A1A,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// The PCI I/O protocol `GUID`.
pub const PCI_IO_GUID: EfiGuid = EfiGuid::new(
    0x4CF5_B200,
    0x68B8,
    0x4CA5,
    [0x9E, 0xEC, 0xB2, 0x3E, 0x3F, 0x50, 0x02, 0x9A],
);
