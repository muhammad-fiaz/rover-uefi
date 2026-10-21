//! # USB identifiers
//!
//! Official USB protocol `GUID` constants.

use crate::base::EfiGuid;

/// The USB I/O protocol `GUID`.
pub const USB_IO_GUID: EfiGuid = EfiGuid::new(
    0x2B2F_68D6,
    0x0CD2,
    0x44CF,
    [0x8E, 0x8B, 0xBB, 0xA2, 0x0B, 0x1B, 0x5B, 0x75],
);

/// The USB 2.0 host controller `GUID`.
pub const USB2_HC_GUID: EfiGuid = EfiGuid::new(
    0x3E74_5226,
    0x9818,
    0x45B6,
    [0xA2, 0xAC, 0xD7, 0xCD, 0x0E, 0x8B, 0xA2, 0xBC],
);
