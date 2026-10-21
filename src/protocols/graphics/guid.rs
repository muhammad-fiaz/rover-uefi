//! # Graphics identifiers
//!
//! Official graphics protocol `GUID` constants.

use crate::base::EfiGuid;

/// The graphics output protocol `GUID`.
pub const GRAPHICS_OUTPUT_GUID: EfiGuid = EfiGuid::new(
    0x9042_A9DE,
    0x23DC,
    0x4A38,
    [0x96, 0xFB, 0x7A, 0xDE, 0xD0, 0x80, 0x51, 0x6A],
);

/// The EDID discovered protocol `GUID`.
pub const EDID_DISCOVERED_GUID: EfiGuid = EfiGuid::new(
    0x1C0C_34F6,
    0xD380,
    0x41FA,
    [0xA0, 0x49, 0x8A, 0xD0, 0x6C, 0x1A, 0x66, 0xAA],
);

/// The EDID active protocol `GUID`.
pub const EDID_ACTIVE_GUID: EfiGuid = EfiGuid::new(
    0xBD8C_1056,
    0x9F36,
    0x44EC,
    [0x92, 0xA8, 0xA6, 0x33, 0x7F, 0x81, 0x79, 0x86],
);
