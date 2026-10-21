//! # TLS 1.3 key schedule
//!
//! The `HKDF`-based derivation of `RFC 8446` §7.1, built on the native
//! `HKDF` implementation.
//!
//! The functions are generic over the hash so that both the
//! `SHA-256` and `SHA-384` suites can be driven from one
//! implementation. Every function writes into a caller-supplied buffer
//! and reports failure instead of panicking.

use crate::protocols::security::hkdf;
use crate::protocols::security::hmac::HashFunction;

/// The prefix that every `TLS` 1.3 label carries.
pub const LABEL_PREFIX: &[u8] = b"tls13 ";

/// The longest label the encoding accepts, excluding the prefix.
pub const MAX_LABEL_LEN: usize = 249;

/// The longest context the encoding accepts.
pub const MAX_CONTEXT_LEN: usize = 255;

/// The longest encoded `HkdfLabel` structure.
pub const MAX_INFO_LEN: usize = 2 + 1 + 255 + 1 + 255;

/// Performs `HKDF-Expand-Label`.
///
/// The label is used verbatim after the `"tls13 "` prefix; the context
/// is the transcript hash for `Derive-Secret` and empty otherwise.
///
/// Returns `false` when the label, context or output length cannot be
/// encoded, or when the underlying `HKDF` fails.
pub fn expand_label<H: HashFunction>(
    secret: &[u8],
    label: &[u8],
    context: &[u8],
    out: &mut [u8],
) -> bool {
    if label.len() > MAX_LABEL_LEN || context.len() > MAX_CONTEXT_LEN {
        return false;
    }
    let Ok(length_field) = u16::try_from(out.len()) else {
        return false;
    };
    let mut info = [0_u8; MAX_INFO_LEN];
    info[..2].copy_from_slice(&length_field.to_be_bytes());
    let Ok(label_length) = u8::try_from(LABEL_PREFIX.len() + label.len()) else {
        return false;
    };
    info[2] = label_length;
    let prefix_end = 3 + LABEL_PREFIX.len();
    info[3..prefix_end].copy_from_slice(LABEL_PREFIX);
    let label_end = prefix_end + label.len();
    info[prefix_end..label_end].copy_from_slice(label);
    let Ok(context_length) = u8::try_from(context.len()) else {
        return false;
    };
    info[label_end] = context_length;
    let context_start = label_end + 1;
    let context_end = context_start + context.len();
    info[context_start..context_end].copy_from_slice(context);
    hkdf::expand::<H>(secret, &info[..context_end], out)
}

/// Derives a secret from `secret` and a transcript hash.
///
/// This is `Derive-Secret(Secret, Label, Messages)`.
pub fn derive_secret<H: HashFunction>(
    secret: &[u8],
    label: &[u8],
    transcript_hash: &[u8],
    out: &mut [u8],
) -> bool {
    expand_label::<H>(secret, label, transcript_hash, out)
}

/// Derives the `finished` key from a traffic secret.
pub fn finished_key<H: HashFunction>(base_key: &[u8], out: &mut [u8]) -> bool {
    expand_label::<H>(base_key, b"finished", &[], out)
}

/// Derives the record protection key from a traffic secret.
pub fn traffic_key<H: HashFunction>(secret: &[u8], out: &mut [u8]) -> bool {
    expand_label::<H>(secret, b"key", &[], out)
}

/// Derives the record protection `IV` from a traffic secret.
pub fn traffic_iv<H: HashFunction>(secret: &[u8], out: &mut [u8]) -> bool {
    expand_label::<H>(secret, b"iv", &[], out)
}

/// Performs `HKDF-Extract` for a schedule stage.
///
/// An empty salt is equivalent to a salt of `HashLen` zero bytes, which
/// is what the schedule uses for the early and master stages.
pub fn extract_secret<H: HashFunction>(salt: &[u8], input: &[u8], out: &mut [u8]) -> bool {
    hkdf::extract::<H>(salt, input, out)
}
