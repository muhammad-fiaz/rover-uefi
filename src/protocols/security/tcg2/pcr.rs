//! # PCR helpers
//!
//! PCR index and algorithm validation.

/// The SHA-1 trusted platform module algorithm identifier.
pub const TPM_ALG_SHA1: u16 = 0x0004;

/// The SHA-256 trusted platform module algorithm identifier.
pub const TPM_ALG_SHA256: u16 = 0x000B;

/// The SHA-384 trusted platform module algorithm identifier.
pub const TPM_ALG_SHA384: u16 = 0x000C;

/// The SHA-512 trusted platform module algorithm identifier.
pub const TPM_ALG_SHA512: u16 = 0x000D;

/// The SM3-256 trusted platform module algorithm identifier.
pub const TPM_ALG_SM3_256: u16 = 0x0012;

/// Returns whether a PCR index is within the supported range.
#[must_use]
pub fn validate_pcr_index(index: u32) -> bool {
    index <= 23
}

/// Returns the digest size for a trusted platform module
/// algorithm identifier, or `None` for an unknown identifier.
#[must_use]
pub fn digest_size_for_algorithm(algorithm: u16) -> Option<usize> {
    match algorithm {
        TPM_ALG_SHA1 => Some(20),
        TPM_ALG_SHA256 | TPM_ALG_SM3_256 => Some(32),
        TPM_ALG_SHA384 => Some(48),
        TPM_ALG_SHA512 => Some(64),
        _ => None,
    }
}

/// Returns whether an algorithm identifier is known.
#[must_use]
pub fn is_known_algorithm(algorithm: u16) -> bool {
    digest_size_for_algorithm(algorithm).is_some()
}
