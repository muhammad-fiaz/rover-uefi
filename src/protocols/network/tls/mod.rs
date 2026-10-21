//! # TLS
//!
//! Native building blocks for Transport Layer Security 1.3.
//!
//! The module provides the record layer framing, the handshake message
//! codec with typed `ClientHello` and `ServerHello` decoders, the
//! `HKDF`-based key schedule of `RFC 8446` and the authenticated record
//! protection that wraps handshake and application data.
//!
//! What is implemented is the wire format and the cryptography around
//! it: nothing here performs network I/O, certificate verification of
//! a peer, or a complete handshake state machine. A caller drives the
//! exchange and supplies the ephemeral secrets. `AES-128-GCM` and
//! `AES-256-GCM` record protection are supported; the `ChaCha20`
//! suites are recognised by name but not implemented.
//!
//! Nothing in this module allocates and no function panics.

pub mod handshake;
pub mod key_schedule;
pub mod protection;
pub mod record;

pub use handshake::{
    ClientHello, ClientHelloParams, Extension, ExtensionIter, HandshakeHeader, HandshakeIter,
    HandshakeMessage, KeyShareEntry, ServerHello, build_client_hello, encode as encode_handshake,
    parse_extensions, parse_messages, write_extension,
};
pub use key_schedule::{
    derive_secret, expand_label, extract_secret, finished_key, traffic_iv, traffic_key,
};
pub use protection::{RecordKeys, open_record, seal_record};
pub use record::{Record, build_record, parse_record};

/// Errors reported by the `TLS` codecs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TlsError {
    /// The output buffer cannot hold the encoding.
    BufferTooSmall,
    /// The input ended before a structure was complete.
    Truncated,
    /// A length field or identifier is not valid.
    Malformed,
    /// The construct is valid but not implemented here.
    Unsupported,
    /// The record failed authentication.
    DecryptionFailed,
}

impl core::fmt::Display for TlsError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::BufferTooSmall => "the output buffer is too small",
            Self::Truncated => "the input is truncated",
            Self::Malformed => "the input is malformed",
            Self::Unsupported => "the construct is not supported",
            Self::DecryptionFailed => "record authentication failed",
        };
        formatter.write_str(text)
    }
}

impl core::error::Error for TlsError {}

/// A cursor over a length-prefixed byte string.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Reader<'a> {
    rest: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Creates a cursor over `bytes`.
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { rest: bytes }
    }

    /// Returns the bytes that have not been consumed.
    pub(crate) const fn remaining(&self) -> &'a [u8] {
        self.rest
    }

    /// Consumes `count` bytes.
    pub(crate) fn take(&mut self, count: usize) -> Result<&'a [u8], TlsError> {
        let head = self.rest.get(..count).ok_or(TlsError::Truncated)?;
        self.rest = self.rest.get(count..).unwrap_or(&[]);
        Ok(head)
    }

    /// Consumes one byte.
    pub(crate) fn byte(&mut self) -> Result<u8, TlsError> {
        let value = *self.rest.first().ok_or(TlsError::Truncated)?;
        self.rest = self.rest.get(1..).unwrap_or(&[]);
        Ok(value)
    }

    /// Consumes a big-endian 16-bit value.
    pub(crate) fn u16(&mut self) -> Result<u16, TlsError> {
        let head = self.take(2)?;
        Ok(u16::from_be_bytes([
            *head.first().ok_or(TlsError::Truncated)?,
            *head.get(1).ok_or(TlsError::Truncated)?,
        ]))
    }

    /// Consumes a length-prefixed byte string whose length fits in
    /// one byte.
    pub(crate) fn vec8(&mut self) -> Result<&'a [u8], TlsError> {
        let length = usize::from(self.byte()?);
        self.take(length)
    }

    /// Consumes a length-prefixed byte string whose length fits in
    /// two bytes.
    pub(crate) fn vec16(&mut self) -> Result<&'a [u8], TlsError> {
        let length = usize::from(self.u16()?);
        self.take(length)
    }

    /// Fails unless every byte has been consumed.
    pub(crate) fn finish(self) -> Result<(), TlsError> {
        if self.rest.is_empty() {
            Ok(())
        } else {
            Err(TlsError::Malformed)
        }
    }
}
