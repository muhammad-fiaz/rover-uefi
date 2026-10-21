//! # TLS 1.3 record protection
//!
//! `AEAD` protection for `TLS` 1.3 records, following `RFC 8446` §5.2.
//!
//! The inner plaintext is the content followed by a content-type byte
//! and, optionally, zero padding. The associated data is the record
//! header of the protected record. The nonce is the static `IV` with
//! the record sequence number exclusive-ored into its low eight bytes.
//!
//! Only `AES-128-GCM` and `AES-256-GCM` are supported. Both operate in
//! place, so a caller can protect a payload without a second buffer.

use super::TlsError;
use super::record::{
    CONTENT_TYPE_APPLICATION_DATA, TLS_LEGACY_VERSION, TLS_MAX_CIPHERTEXT, TLS_MAX_PLAINTEXT,
};
use crate::protocols::security::gcm;

/// The length of the per-record `IV` in bytes.
pub const TLS13_IV_LEN: usize = 12;

/// The length of an `AEAD` tag in bytes.
pub const TLS13_TAG_LEN: usize = gcm::TAG_SIZE;

/// The number of bytes record protection adds to a payload, including
/// the inner content-type byte.
pub const TLS13_OVERHEAD: usize = TLS13_TAG_LEN + 1;

/// The record keys of one direction of a connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordKeys {
    key: [u8; 32],
    key_length: usize,
    iv: [u8; TLS13_IV_LEN],
}

impl RecordKeys {
    /// Creates a key set from a traffic secret's key and `IV`.
    ///
    /// Returns `None` unless the key is 16, 24 or 32 bytes long and the
    /// `IV` is 12 bytes long.
    #[must_use]
    pub fn new(key: &[u8], iv: &[u8]) -> Option<Self> {
        if !matches!(key.len(), 16 | 24 | 32) || iv.len() != TLS13_IV_LEN {
            return None;
        }
        let mut key_bytes = [0_u8; 32];
        key_bytes.get_mut(..key.len())?.copy_from_slice(key);
        let mut iv_bytes = [0_u8; TLS13_IV_LEN];
        iv_bytes.copy_from_slice(iv);
        Some(Self {
            key: key_bytes,
            key_length: key.len(),
            iv: iv_bytes,
        })
    }

    /// Returns the key material actually in use.
    #[must_use]
    pub fn key(&self) -> &[u8] {
        self.key.get(..self.key_length).unwrap_or(&[])
    }

    /// Returns whether the key is usable with the `AES-GCM` backend.
    #[must_use]
    pub fn is_supported(&self) -> bool {
        matches!(self.key_length, 16 | 24 | 32)
    }

    /// Returns the nonce of `sequence`.
    fn nonce(&self, sequence: u64) -> [u8; TLS13_IV_LEN] {
        let mut nonce = self.iv;
        let sequence_bytes = sequence.to_be_bytes();
        let start = TLS13_IV_LEN - sequence_bytes.len();
        for (index, byte) in sequence_bytes.iter().enumerate() {
            if let Some(slot) = nonce.get_mut(start + index) {
                *slot ^= byte;
            }
        }
        nonce
    }
}

/// Protects one record in place.
///
/// On success `out[..written]` holds the ciphertext followed by the
/// tag, where `written` is the returned length. The caller frames it
/// with the application-data content type.
///
/// # Errors
///
/// Returns [`TlsError::BufferTooSmall`] when `out` is shorter than
/// [`TLS13_OVERHEAD`] plus the payload, [`TlsError::Malformed`] when
/// the payload is too large or the content type is not a valid inner
/// type, and [`TlsError::Unsupported`] for an unusable key.
pub fn seal_record(
    keys: &RecordKeys,
    sequence: u64,
    content_type: u8,
    plaintext: &[u8],
    out: &mut [u8],
) -> Result<usize, TlsError> {
    if !keys.is_supported() {
        return Err(TlsError::Unsupported);
    }
    if plaintext.len() > TLS_MAX_PLAINTEXT {
        return Err(TlsError::Malformed);
    }
    if content_type == 0 || content_type >= 0x40 {
        return Err(TlsError::Malformed);
    }
    let inner_length = plaintext.len().checked_add(1).ok_or(TlsError::Malformed)?;
    let total = inner_length
        .checked_add(TLS13_TAG_LEN)
        .ok_or(TlsError::Malformed)?;
    let length_field = u16::try_from(total).map_err(|_| TlsError::Malformed)?;
    let buffer = out.get_mut(..total).ok_or(TlsError::BufferTooSmall)?;
    {
        let (body, tag_slot) = buffer.split_at_mut(inner_length);
        let Some(payload_slot) = body.get_mut(..plaintext.len()) else {
            return Err(TlsError::BufferTooSmall);
        };
        payload_slot.copy_from_slice(plaintext);
        let Some(type_slot) = body.get_mut(plaintext.len()) else {
            return Err(TlsError::BufferTooSmall);
        };
        *type_slot = content_type;

        let aad = associated_data(length_field);
        let mut tag = [0_u8; TLS13_TAG_LEN];
        if !gcm::seal_in_place(keys.key(), &keys.nonce(sequence), &aad, body, &mut tag) {
            return Err(TlsError::Unsupported);
        }
        let Some(tag_destination) = tag_slot.get_mut(..TLS13_TAG_LEN) else {
            return Err(TlsError::BufferTooSmall);
        };
        tag_destination.copy_from_slice(&tag);
    }
    Ok(total)
}

/// Authenticates and decrypts one protected record.
///
/// `ciphertext` includes the tag. On success the returned pair is the
/// plaintext length and the inner content type, and
/// `plaintext[..length]` holds the payload.
///
/// # Errors
///
/// Returns [`TlsError::Truncated`] when the record is shorter than a
/// tag, [`TlsError::DecryptionFailed`] when authentication fails, and
/// [`TlsError::Malformed`] when the inner payload has no content type.
pub fn open_record(
    keys: &RecordKeys,
    sequence: u64,
    ciphertext: &[u8],
    plaintext: &mut [u8],
) -> Result<(usize, u8), TlsError> {
    if !keys.is_supported() {
        return Err(TlsError::Unsupported);
    }
    if ciphertext.len() < TLS13_TAG_LEN + 1 || ciphertext.len() > TLS_MAX_CIPHERTEXT {
        return Err(TlsError::Truncated);
    }
    let inner_length = ciphertext.len() - TLS13_TAG_LEN;
    let Ok(length_field) = u16::try_from(ciphertext.len()) else {
        return Err(TlsError::Malformed);
    };
    let buffer = plaintext
        .get_mut(..inner_length)
        .ok_or(TlsError::BufferTooSmall)?;
    buffer.copy_from_slice(&ciphertext[..inner_length]);
    let Some(tag_bytes) = ciphertext.get(inner_length..) else {
        return Err(TlsError::Truncated);
    };
    let mut tag = [0_u8; TLS13_TAG_LEN];
    tag.copy_from_slice(tag_bytes);
    let aad = associated_data(length_field);
    if !gcm::open_in_place(keys.key(), &keys.nonce(sequence), &aad, buffer, &tag) {
        buffer.fill(0);
        return Err(TlsError::DecryptionFailed);
    }
    let Some(end) = buffer.iter().rposition(|byte| *byte != 0) else {
        buffer.fill(0);
        return Err(TlsError::Malformed);
    };
    let content_type = *buffer.get(end).ok_or(TlsError::Malformed)?;
    if content_type == 0 || content_type >= 0x40 {
        buffer.fill(0);
        return Err(TlsError::Malformed);
    }
    Ok((end, content_type))
}

/// Returns the record header used as associated data.
fn associated_data(length: u16) -> [u8; 5] {
    let version = TLS_LEGACY_VERSION.to_be_bytes();
    let length = length.to_be_bytes();
    [
        CONTENT_TYPE_APPLICATION_DATA,
        version[0],
        version[1],
        length[1],
        length[0],
    ]
}
