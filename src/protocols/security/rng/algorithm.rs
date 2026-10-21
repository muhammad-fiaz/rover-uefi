//! # RNG algorithms
//!
//! Supported algorithm identifiers defined by the specification.
//! The list is optional and not exhaustive; firmware may expose
//! additional identifiers. Unknown identifiers are preserved by
//! callers and must not be silently replaced with another
//! algorithm.

use crate::base::EfiGuid;

/// Raw entropy provided directly from the entropy source.
pub const ALGORITHM_RAW: EfiGuid = EfiGuid::new(
    0xE431_76D7,
    0xB6E8,
    0x4827,
    [0xB7, 0x84, 0x7F, 0xFD, 0xC4, 0xB6, 0x85, 0x61],
);

/// SP800-90 hash deterministic random bit generator.
pub const ALGORITHM_SP800_90_HASH_256: EfiGuid = EfiGuid::new(
    0xA7AF_67CB,
    0x603B,
    0x4D42,
    [0xBA, 0x21, 0x70, 0xBF, 0xB6, 0x29, 0x3F, 0x96],
);

/// SP800-90 HMAC deterministic random bit generator.
pub const ALGORITHM_SP800_90_HMAC_256: EfiGuid = EfiGuid::new(
    0xC514_9B43,
    0xAE85,
    0x4F53,
    [0x99, 0x82, 0xB9, 0x43, 0x35, 0xD3, 0xA9, 0xE7],
);

/// SP800-90 CTR deterministic random bit generator.
pub const ALGORITHM_SP800_90_CTR_256: EfiGuid = EfiGuid::new(
    0x44F0_DE6E,
    0x4D8C,
    0x4045,
    [0xA8, 0xC7, 0x4D, 0xD1, 0x68, 0x85, 0x6B, 0x9E],
);

/// X9.31 3DES random number generator.
pub const ALGORITHM_X9_31_3DES: EfiGuid = EfiGuid::new(
    0x63C4_785A,
    0xCA34,
    0x4012,
    [0xA3, 0xC8, 0x0B, 0x6A, 0x32, 0x4F, 0x55, 0x46],
);

/// X9.31 AES random number generator.
pub const ALGORITHM_X9_31_AES: EfiGuid = EfiGuid::new(
    0xACD0_3321,
    0x777E,
    0x4D3D,
    [0xB1, 0xC8, 0x20, 0xCF, 0xD8, 0x88, 0x20, 0xC9],
);

/// Architecture random number instruction algorithm.
pub const ALGORITHM_ARM_RNDR: EfiGuid = EfiGuid::new(
    0x43D2_FDE3,
    0x9D4E,
    0x4D79,
    [0x02, 0x96, 0xA8, 0x9B, 0xCA, 0x78, 0x08, 0x41],
);

/// Returns whether the identifier is one of the defined
/// algorithm identifiers.
#[must_use]
pub fn is_defined_algorithm(algorithm: &EfiGuid) -> bool {
    *algorithm == ALGORITHM_RAW
        || *algorithm == ALGORITHM_SP800_90_HASH_256
        || *algorithm == ALGORITHM_SP800_90_HMAC_256
        || *algorithm == ALGORITHM_SP800_90_CTR_256
        || *algorithm == ALGORITHM_X9_31_3DES
        || *algorithm == ALGORITHM_X9_31_AES
        || *algorithm == ALGORITHM_ARM_RNDR
}
