//! # Security identifiers
//!
//! Official protocol `GUID` constants for random number
//! generation, hashing, image authentication and platform
//! measurement. Each value is the mixed-endian `GUID`
//! defined by the applicable specification revision.

use crate::base::EfiGuid;

/// The RNG protocol `GUID`.
pub const RNG_GUID: EfiGuid = EfiGuid::new(
    0x3152_BCA5,
    0xEADE,
    0x433D,
    [0x86, 0x2E, 0xC0, 0x1C, 0xDC, 0x29, 0x1F, 0x44],
);

/// The hash service binding protocol `GUID`.
pub const HASH_SERVICE_BINDING_GUID: EfiGuid = EfiGuid::new(
    0x4288_1C98,
    0xA4F3,
    0x44B0,
    [0xA3, 0x9D, 0xDF, 0xA1, 0x86, 0x67, 0xD8, 0xCD],
);

/// The hash protocol `GUID`.
pub const HASH_GUID: EfiGuid = EfiGuid::new(
    0xC518_4932,
    0xDBA5,
    0x46DB,
    [0xA5, 0xBA, 0xCC, 0x0B, 0xDA, 0x9C, 0x14, 0x35],
);

/// The hash2 service binding protocol `GUID`.
pub const HASH2_SERVICE_BINDING_GUID: EfiGuid = EfiGuid::new(
    0xDA83_6F8D,
    0x217F,
    0x4CA0,
    [0x99, 0xC2, 0x1C, 0xA4, 0xE1, 0x60, 0x77, 0xEA],
);

/// The hash2 protocol `GUID`.
pub const HASH2_GUID: EfiGuid = EfiGuid::new(
    0x55B1_D734,
    0xC5E1,
    0x49DB,
    [0x96, 0x47, 0xB1, 0x6A, 0xFB, 0x0E, 0x30, 0x5B],
);

/// The security architecture protocol `GUID`.
pub const SECURITY_ARCH_GUID: EfiGuid = EfiGuid::new(
    0xA464_23E3,
    0x4617,
    0x49F1,
    [0xB9, 0xFF, 0xD1, 0xBF, 0xA9, 0x11, 0x58, 0x39],
);

/// The Security2 architecture protocol `GUID`.
pub const SECURITY2_ARCH_GUID: EfiGuid = EfiGuid::new(
    0x94AB_2F58,
    0x1438,
    0x4EF1,
    [0x91, 0x52, 0x18, 0x94, 0x1A, 0x3A, 0x0E, 0x68],
);

/// The TCG2 protocol `GUID`.
pub const TCG2_GUID: EfiGuid = EfiGuid::new(
    0x607F_766C,
    0x7455,
    0x42BE,
    [0x93, 0x0B, 0xE4, 0xD7, 0x6D, 0xB2, 0x72, 0x0F],
);

/// The TCG2 final events table `GUID`.
pub const TCG2_FINAL_EVENTS_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x1E2E_D096,
    0x30E2,
    0x4254,
    [0xBD, 0x89, 0x86, 0x3B, 0xBE, 0xF8, 0x23, 0x25],
);
