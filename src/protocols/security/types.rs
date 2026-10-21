//! # Security types
//!
//! Common security identifiers and variable attributes.

use crate::base::EfiGuid;

/// The global variable `GUID` for platform variables including
/// the platform key, key-exchange database and Secure Boot
/// mode variables.
pub const GLOBAL_VARIABLE_GUID: EfiGuid = EfiGuid::new(
    0x8BE4_DF61,
    0x93CA,
    0x11D2,
    [0xAA, 0x0D, 0x00, 0xE0, 0x98, 0x03, 0x2B, 0x8C],
);

/// The image security database `GUID` for the authorized,
/// forbidden and timestamp signature databases.
pub const IMAGE_SECURITY_DATABASE_GUID: EfiGuid = EfiGuid::new(
    0xD719_B2CB,
    0x3D3A,
    0x4596,
    [0xA3, 0xBC, 0xDA, 0xD0, 0x0E, 0x67, 0x65, 0x6F],
);

/// The non-volatile variable attribute.
pub const VARIABLE_NON_VOLATILE: u32 = 0x0000_0001;

/// The boot-service access attribute.
pub const VARIABLE_BOOT_SERVICE_ACCESS: u32 = 0x0000_0002;

/// The runtime access attribute.
pub const VARIABLE_RUNTIME_ACCESS: u32 = 0x0000_0004;

/// The authenticated-write-access attribute.
pub const VARIABLE_AUTHENTICATED_WRITE_ACCESS: u32 = 0x0000_0010;

/// The time-based authenticated-write-access attribute.
pub const VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: u32 = 0x0000_0020;

/// The append-write attribute.
pub const VARIABLE_APPEND_WRITE: u32 = 0x0000_0040;
