//! # TLS record layer
//!
//! Framing for `TLS` records: a one-byte content type, a two-byte
//! legacy version, a two-byte length and the payload. The length field
//! is bounded by [`TLS_MAX_CIPHERTEXT`] so that a hostile peer cannot
//! claim an unbounded record.

use super::TlsError;

/// The length of a record header in bytes.
pub const TLS_RECORD_HEADER_LEN: usize = 5;

/// The largest plaintext a record may carry.
pub const TLS_MAX_PLAINTEXT: usize = 16_384;

/// The largest payload a record may declare, including any `AEAD`
/// expansion and padding.
pub const TLS_MAX_CIPHERTEXT: usize = 16_384 + 256;

/// The legacy record version used by `TLS` 1.3.
pub const TLS_LEGACY_VERSION: u16 = 0x0303;

/// The change-cipher-spec content type.
pub const CONTENT_TYPE_CHANGE_CIPHER_SPEC: u8 = 20;

/// The alert content type.
pub const CONTENT_TYPE_ALERT: u8 = 21;

/// The handshake content type.
pub const CONTENT_TYPE_HANDSHAKE: u8 = 22;

/// The application-data content type.
pub const CONTENT_TYPE_APPLICATION_DATA: u8 = 23;

/// Returns whether `content_type` is one of the four defined types.
#[must_use]
pub const fn is_known_content_type(content_type: u8) -> bool {
    matches!(
        content_type,
        CONTENT_TYPE_CHANGE_CIPHER_SPEC
            | CONTENT_TYPE_ALERT
            | CONTENT_TYPE_HANDSHAKE
            | CONTENT_TYPE_APPLICATION_DATA
    )
}

/// Returns whether `version` is a `TLS` major version 3 record version.
#[must_use]
pub const fn is_supported_version(version: u16) -> bool {
    version >> 8 == 0x03
}

/// A parsed record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record<'a> {
    /// The content type.
    pub content_type: u8,
    /// The legacy record version.
    pub version: u16,
    /// The record payload.
    pub payload: &'a [u8],
}

impl<'a> Record<'a> {
    /// Parses one record from the front of `bytes`.
    ///
    /// Returns the record and the number of bytes it occupies, so that
    /// a caller can walk a stream of records.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Truncated`] when the header or payload is
    /// incomplete and [`TlsError::Malformed`] when the declared length
    /// exceeds [`TLS_MAX_CIPHERTEXT`].
    pub fn parse(bytes: &'a [u8]) -> Result<(Self, usize), TlsError> {
        let header = bytes
            .get(..TLS_RECORD_HEADER_LEN)
            .ok_or(TlsError::Truncated)?;
        let content_type = *header.first().ok_or(TlsError::Truncated)?;
        let version = u16::from_be_bytes([
            *header.get(1).ok_or(TlsError::Truncated)?,
            *header.get(2).ok_or(TlsError::Truncated)?,
        ]);
        let length = usize::from(u16::from_be_bytes([
            *header.get(3).ok_or(TlsError::Truncated)?,
            *header.get(4).ok_or(TlsError::Truncated)?,
        ]));
        if length > TLS_MAX_CIPHERTEXT {
            return Err(TlsError::Malformed);
        }
        let end = TLS_RECORD_HEADER_LEN
            .checked_add(length)
            .ok_or(TlsError::Malformed)?;
        let payload = bytes
            .get(TLS_RECORD_HEADER_LEN..end)
            .ok_or(TlsError::Truncated)?;
        Ok((
            Self {
                content_type,
                version,
                payload,
            },
            end,
        ))
    }

    /// Returns whether the content type and version are valid.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        is_known_content_type(self.content_type) && is_supported_version(self.version)
    }
}

/// Writes one record header and payload into `out`.
///
/// # Errors
///
/// Returns [`TlsError::BufferTooSmall`] when `out` cannot hold the
/// record and [`TlsError::Malformed`] when the payload is larger than
/// [`TLS_MAX_CIPHERTEXT`].
pub fn build_record(
    out: &mut [u8],
    content_type: u8,
    version: u16,
    payload: &[u8],
) -> Result<usize, TlsError> {
    let length = u16::try_from(payload.len()).map_err(|_| TlsError::Malformed)?;
    if payload.len() > TLS_MAX_CIPHERTEXT {
        return Err(TlsError::Malformed);
    }
    let end = TLS_RECORD_HEADER_LEN
        .checked_add(payload.len())
        .ok_or(TlsError::Malformed)?;
    let buffer = out.get_mut(..end).ok_or(TlsError::BufferTooSmall)?;
    buffer[0] = content_type;
    buffer
        .get_mut(1..3)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(&version.to_be_bytes());
    buffer
        .get_mut(3..5)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(&length.to_be_bytes());
    buffer
        .get_mut(TLS_RECORD_HEADER_LEN..end)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(payload);
    Ok(end)
}

/// Parses a sequence of records, returning the first one.
///
/// This is a convenience alias for [`Record::parse`] that documents the
/// walked-stream use case.
///
/// # Errors
///
/// Returns the same errors as [`Record::parse`].
pub fn parse_record(bytes: &[u8]) -> Result<(Record<'_>, usize), TlsError> {
    Record::parse(bytes)
}
