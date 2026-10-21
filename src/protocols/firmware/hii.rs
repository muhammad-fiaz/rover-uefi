//! # Human Interface Infrastructure (HII)
//!
//! `HII` protocol identifiers and bounded package validation.
//! `HII` database operations remain firmware-provided; this module
//! describes identifiers and validates package headers.

use crate::base::EfiGuid;

/// The `HII` database protocol GUID.
pub const HII_DATABASE_GUID: EfiGuid = EfiGuid::new(
    0xEF9F_C172,
    0xAF2B,
    0x4C5B,
    [0xB4, 0xD7, 0x49, 0x41, 0x95, 0x44, 0xE6, 0x00],
);

/// The `HII` string package protocol GUID.
pub const HII_STRING_PACKAGE_GUID: EfiGuid = EfiGuid::new(
    0xD7C4_0F9E,
    0x8810,
    0x475F,
    [0x9C, 0x72, 0xCC, 0x55, 0x85, 0x94, 0x1D, 0x00],
);

/// The `HII` forms protocol GUID.
pub const HII_FORMS_GUID: EfiGuid = EfiGuid::new(
    0x92D9_A4FD,
    0xB76D,
    0x4064,
    [0x8A, 0x56, 0x54, 0x6E, 0x94, 0x4E, 0xC3, 0x00],
);

/// The `HII` configuration access protocol GUID.
pub const HII_CONFIG_ACCESS_GUID: EfiGuid = EfiGuid::new(
    0x4D5A_18D2,
    0xD2C9,
    0x4833,
    [0xB6, 0xEF, 0x47, 0x46, 0xBA, 0x0E, 0x0A, 0x00],
);

/// The `HII` font alter protocol GUID.
pub const HII_FONT_ALTER_GUID: EfiGuid = EfiGuid::new(
    0x6194_CBE5,
    0x5A3B,
    0x418E,
    [0x9D, 0x23, 0xC2, 0x5A, 0x0D, 0x21, 0xFC, 0x00],
);

/// A package header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HiiPackageHeader {
    /// The full package length.
    pub package_length: u32,
    /// The package type.
    pub package_type: u8,
}

/// Parses an `HII` package header.
///
/// The header is `PackageListLength(u32)`,
/// `PackageLength(u32)`, `PackageType(u8)`, then a 16 byte `GUID`.
/// Returns `None` when the buffer is shorter than 25 bytes or the
/// declared package length exceeds the buffer.
#[must_use]
pub fn parse_package_header(input: &[u8]) -> Option<HiiPackageHeader> {
    if input.len() < 25 {
        return None;
    }
    let mut len = [0_u8; 4];
    len.copy_from_slice(input.get(4..8)?);
    let package_length = u32::from_le_bytes(len);
    if package_length < 25 || (package_length as usize) > input.len() {
        return None;
    }
    Some(HiiPackageHeader {
        package_length,
        package_type: *input.get(8)?,
    })
}

/// Returns whether a package type is known to the parser.
#[must_use]
pub fn is_known_package_type(package_type: u8) -> bool {
    matches!(package_type, 0x00..=0x07)
}
