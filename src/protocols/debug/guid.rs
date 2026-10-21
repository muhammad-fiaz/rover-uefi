//! # Debug identifiers
//!
//! Official debug protocol `GUID` constants.

use crate::base::EfiGuid;

/// The debug support protocol `GUID`.
pub const DEBUG_SUPPORT_GUID: EfiGuid = EfiGuid::new(
    0x2755_590C,
    0x6F3C,
    0x42FA,
    [0x9E, 0xA4, 0xA3, 0xBA, 0x54, 0x3C, 0xDA, 0x25],
);

/// The debug port protocol `GUID`.
pub const DEBUG_PORT_GUID: EfiGuid = EfiGuid::new(
    0xEBA4_E8D2,
    0x3858,
    0x41EC,
    [0xA2, 0x81, 0x26, 0x47, 0xBA, 0x96, 0x60, 0xD0],
);
