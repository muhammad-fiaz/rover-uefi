//! # Protocol identifiers
//!
//! Official protocol `GUID` constants.
//!
//! Each constant matches the specification
//! value. The mixed-endian byte form is
//! provided by [`EfiGuid::to_bytes`].

use crate::base::EfiGuid;

/// The simple text input protocol `GUID`.
pub const SIMPLE_TEXT_INPUT_GUID: EfiGuid = EfiGuid::new(
    0x3874_77C1,
    0x69C7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// The simple text output protocol `GUID`.
pub const SIMPLE_TEXT_OUTPUT_GUID: EfiGuid = EfiGuid::new(
    0x3874_77C2,
    0x69C7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// The device path protocol `GUID`.
pub const DEVICE_PATH_GUID: EfiGuid = EfiGuid::new(
    0x0957_6E91,
    0x6D3F,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);
