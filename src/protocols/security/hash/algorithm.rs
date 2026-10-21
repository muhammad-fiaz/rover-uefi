//! # Hash algorithms
//!
//! Supported algorithm identifiers and digest sizes defined by
//! the specification. The no-pad identifiers are not compatible
//! with the second hash protocol.

use crate::base::EfiGuid;

/// SHA-1 algorithm.
pub const SHA1_GUID: EfiGuid = EfiGuid::new(
    0x2AE9_D80F,
    0x3FB2,
    0x4095,
    [0xB7, 0xB1, 0xE9, 0x31, 0x57, 0xB9, 0x46, 0xB6],
);

/// SHA-224 algorithm.
pub const SHA224_GUID: EfiGuid = EfiGuid::new(
    0x8DF0_1A06,
    0x9BD5,
    0x4BF7,
    [0xB0, 0x21, 0xDB, 0x4F, 0xD9, 0xCC, 0xF4, 0x5B],
);

/// SHA-256 algorithm.
pub const SHA256_GUID: EfiGuid = EfiGuid::new(
    0x51AA_59DE,
    0xFDF2,
    0x4EA3,
    [0xBC, 0x63, 0x87, 0x5F, 0xB7, 0x84, 0x2E, 0xE9],
);

/// SHA-384 algorithm.
pub const SHA384_GUID: EfiGuid = EfiGuid::new(
    0xEFA9_6432,
    0xDE33,
    0x4DD2,
    [0xAE, 0xE6, 0x32, 0x8C, 0x33, 0xDF, 0x77, 0x7A],
);

/// SHA-512 algorithm.
pub const SHA512_GUID: EfiGuid = EfiGuid::new(
    0xCAA4_381E,
    0x750C,
    0x4770,
    [0xB8, 0x70, 0x7A, 0x23, 0xB4, 0xE4, 0x21, 0x30],
);

/// MD5 algorithm.
pub const MD5_GUID: EfiGuid = EfiGuid::new(
    0x0AF7_C79C,
    0x65B5,
    0x4319,
    [0xB0, 0xAE, 0x44, 0xEC, 0x48, 0x4E, 0x4A, 0xD7],
);

/// SHA-1 no-pad algorithm, first hash protocol only.
pub const SHA1_NOPAD_GUID: EfiGuid = EfiGuid::new(
    0x24C5_DC2F,
    0x53E2,
    0x40CA,
    [0x9E, 0xD6, 0xA5, 0xD9, 0xA4, 0x9F, 0x46, 0x3B],
);

/// SHA-256 no-pad algorithm, first hash protocol only.
pub const SHA256_NOPAD_GUID: EfiGuid = EfiGuid::new(
    0x8628_752A,
    0x6CB7,
    0x4814,
    [0x96, 0xFC, 0x24, 0xA8, 0x15, 0xAC, 0x22, 0x26],
);

/// SHA-1 digest size in bytes.
pub const SHA1_DIGEST_SIZE: usize = 20;

/// SHA-224 digest size in bytes.
pub const SHA224_DIGEST_SIZE: usize = 28;

/// SHA-256 digest size in bytes.
pub const SHA256_DIGEST_SIZE: usize = 32;

/// SHA-384 digest size in bytes.
pub const SHA384_DIGEST_SIZE: usize = 48;

/// SHA-512 digest size in bytes.
pub const SHA512_DIGEST_SIZE: usize = 64;

/// MD5 digest size in bytes.
pub const MD5_DIGEST_SIZE: usize = 16;

/// Returns the digest size for a defined algorithm, or `None`
/// for an unknown identifier.
#[must_use]
pub fn digest_size(algorithm: &EfiGuid) -> Option<usize> {
    if *algorithm == SHA1_GUID || *algorithm == SHA1_NOPAD_GUID {
        Some(SHA1_DIGEST_SIZE)
    } else if *algorithm == SHA224_GUID {
        Some(SHA224_DIGEST_SIZE)
    } else if *algorithm == SHA256_GUID || *algorithm == SHA256_NOPAD_GUID {
        Some(SHA256_DIGEST_SIZE)
    } else if *algorithm == SHA384_GUID {
        Some(SHA384_DIGEST_SIZE)
    } else if *algorithm == SHA512_GUID {
        Some(SHA512_DIGEST_SIZE)
    } else if *algorithm == MD5_GUID {
        Some(MD5_DIGEST_SIZE)
    } else {
        None
    }
}
