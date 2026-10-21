//! # TLS handshake messages
//!
//! The handshake framing shared by all `TLS` versions (a one-byte type
//! and a three-byte length) plus typed decoders for the two messages a
//! client and a server exchange in the clear: `ClientHello` and
//! `ServerHello`.
//!
//! Extensions are decoded generically, so a caller can find the
//! extensions it understands and skip the rest. The typed helpers for
//! `supported_versions` and `key_share` follow `RFC 8446`.

use super::{Reader, TlsError};

/// The length of a handshake message header in bytes.
pub const HANDSHAKE_HEADER_LEN: usize = 4;

/// The `ClientHello` handshake type.
pub const HS_CLIENT_HELLO: u8 = 1;

/// The `ServerHello` handshake type.
pub const HS_SERVER_HELLO: u8 = 2;

/// The `NewSessionTicket` handshake type.
pub const HS_NEW_SESSION_TICKET: u8 = 4;

/// The `EndOfEarlyData` handshake type.
pub const HS_END_OF_EARLY_DATA: u8 = 5;

/// The `EncryptedExtensions` handshake type.
pub const HS_ENCRYPTED_EXTENSIONS: u8 = 8;

/// The `Certificate` handshake type.
pub const HS_CERTIFICATE: u8 = 11;

/// The `CertificateRequest` handshake type.
pub const HS_CERTIFICATE_REQUEST: u8 = 13;

/// The `CertificateVerify` handshake type.
pub const HS_CERTIFICATE_VERIFY: u8 = 15;

/// The `Finished` handshake type.
pub const HS_FINISHED: u8 = 20;

/// The `KeyUpdate` handshake type.
pub const HS_KEY_UPDATE: u8 = 24;

/// The `MessageHash` handshake type.
pub const HS_MESSAGE_HASH: u8 = 254;

/// The `server_name` extension.
pub const EXT_SERVER_NAME: u16 = 0;

/// The `supported_groups` extension.
pub const EXT_SUPPORTED_GROUPS: u16 = 10;

/// The `signature_algorithms` extension.
pub const EXT_SIGNATURE_ALGORITHMS: u16 = 13;

/// The `application_layer_protocol_negotiation` extension.
pub const EXT_ALPN: u16 = 16;

/// The `supported_versions` extension.
pub const EXT_SUPPORTED_VERSIONS: u16 = 43;

/// The `psk_key_exchange_modes` extension.
pub const EXT_PSK_KEY_EXCHANGE_MODES: u16 = 45;

/// The `key_share` extension.
pub const EXT_KEY_SHARE: u16 = 51;

/// The `x25519` named group.
pub const GROUP_X25519: u16 = 0x001D;

/// The `secp256r1` named group.
pub const GROUP_SECP256R1: u16 = 0x0017;

/// The `secp384r1` named group.
pub const GROUP_SECP384R1: u16 = 0x0018;

/// The `TLS_AES_128_GCM_SHA256` cipher suite.
pub const CIPHER_TLS_AES_128_GCM_SHA256: u16 = 0x1301;

/// The `TLS_AES_256_GCM_SHA384` cipher suite.
pub const CIPHER_TLS_AES_256_GCM_SHA384: u16 = 0x1302;

/// The `TLS_CHACHA20_POLY1305_SHA256` cipher suite.
pub const CIPHER_TLS_CHACHA20_POLY1305_SHA256: u16 = 0x1303;

/// The `TLS_AES_128_CCM_SHA256` cipher suite.
pub const CIPHER_TLS_AES_128_CCM_SHA256: u16 = 0x1304;

/// The `TLS` 1.2 protocol version.
pub const TLS_VERSION_1_2: u16 = 0x0303;

/// The `TLS` 1.3 protocol version.
pub const TLS_VERSION_1_3: u16 = 0x0304;

/// A handshake message header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandshakeHeader {
    /// The handshake message type.
    pub kind: u8,
    /// The length of the body that follows the header.
    pub length: usize,
}

impl HandshakeHeader {
    /// Parses a handshake header from the front of `bytes`.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Truncated`] when fewer than four bytes are
    /// available.
    pub fn parse(bytes: &[u8]) -> Result<Self, TlsError> {
        let header = bytes
            .get(..HANDSHAKE_HEADER_LEN)
            .ok_or(TlsError::Truncated)?;
        let kind = *header.first().ok_or(TlsError::Truncated)?;
        let length = (usize::from(*header.get(1).ok_or(TlsError::Truncated)?) << 16)
            | (usize::from(*header.get(2).ok_or(TlsError::Truncated)?) << 8)
            | usize::from(*header.get(3).ok_or(TlsError::Truncated)?);
        Ok(Self { kind, length })
    }
}

/// A handshake message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandshakeMessage<'a> {
    /// The handshake message type.
    pub kind: u8,
    /// The message body.
    pub body: &'a [u8],
}

/// An iterator over the handshake messages of a buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandshakeIter<'a> {
    rest: &'a [u8],
}

impl<'a> Iterator for HandshakeIter<'a> {
    type Item = HandshakeMessage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let header = HandshakeHeader::parse(self.rest).ok()?;
        let start = HANDSHAKE_HEADER_LEN;
        let end = start.checked_add(header.length)?;
        let body = self.rest.get(start..end)?;
        self.rest = self.rest.get(end..).unwrap_or(&[]);
        Some(HandshakeMessage {
            kind: header.kind,
            body,
        })
    }
}

/// Parses a buffer of consecutive handshake messages.
///
/// The whole buffer is validated up front, so the iterator never ends
/// silently in the middle of a message.
///
/// # Errors
///
/// Returns [`TlsError::Truncated`] when a message body is cut short.
pub fn parse_messages(bytes: &[u8]) -> Result<HandshakeIter<'_>, TlsError> {
    let mut rest = bytes;
    while !rest.is_empty() {
        let header = HandshakeHeader::parse(rest)?;
        let end = HANDSHAKE_HEADER_LEN
            .checked_add(header.length)
            .ok_or(TlsError::Malformed)?;
        rest = rest.get(end..).ok_or(TlsError::Truncated)?;
    }
    Ok(HandshakeIter { rest: bytes })
}

/// Writes a handshake message into `out`.
///
/// # Errors
///
/// Returns [`TlsError::BufferTooSmall`] when `out` cannot hold the
/// message and [`TlsError::Malformed`] when the body is longer than
/// the 24-bit length field allows.
pub fn encode(out: &mut [u8], kind: u8, body: &[u8]) -> Result<usize, TlsError> {
    if body.len() > 0x00FF_FFFF {
        return Err(TlsError::Malformed);
    }
    let end = HANDSHAKE_HEADER_LEN
        .checked_add(body.len())
        .ok_or(TlsError::Malformed)?;
    let buffer = out.get_mut(..end).ok_or(TlsError::BufferTooSmall)?;
    buffer[0] = kind;
    let length = u32::try_from(body.len()).map_err(|_| TlsError::Malformed)?;
    buffer
        .get_mut(1..4)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(&length.to_be_bytes()[1..4]);
    buffer
        .get_mut(HANDSHAKE_HEADER_LEN..end)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(body);
    Ok(end)
}

/// A `TLS` extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extension<'a> {
    /// The extension type.
    pub kind: u16,
    /// The extension body.
    pub data: &'a [u8],
}

impl<'a> Extension<'a> {
    /// Returns the list of supported versions.
    ///
    /// The extension body is a one-byte length followed by 16-bit
    /// version values.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Malformed`] when the list is inconsistent.
    pub fn supported_versions(&self) -> Result<VersionIter<'a>, TlsError> {
        if self.kind != EXT_SUPPORTED_VERSIONS {
            return Err(TlsError::Malformed);
        }
        let mut reader = Reader::new(self.data);
        let list = reader.vec8()?;
        reader.finish()?;
        if list.len() % 2 != 0 {
            return Err(TlsError::Malformed);
        }
        Ok(VersionIter { rest: list })
    }

    /// Returns the key share entries of a `key_share` extension.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Malformed`] when the vector is inconsistent.
    pub fn key_shares(&self) -> Result<KeyShareIter<'a>, TlsError> {
        if self.kind != EXT_KEY_SHARE {
            return Err(TlsError::Malformed);
        }
        let shares = key_share_vector(self.data)?;
        Ok(KeyShareIter { rest: shares })
    }
}

/// An iterator over the extensions of a hello message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionIter<'a> {
    rest: &'a [u8],
}

impl<'a> Iterator for ExtensionIter<'a> {
    type Item = Extension<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut reader = Reader::new(self.rest);
        let kind = reader.u16().ok()?;
        let data = reader.vec16().ok()?;
        let consumed = self.rest.len() - reader.remaining().len();
        self.rest = self.rest.get(consumed..).unwrap_or(&[]);
        Some(Extension { kind, data })
    }
}

/// Parses an extension block.
///
/// # Errors
///
/// Returns [`TlsError::Malformed`] when an extension runs past the end
/// of the block.
pub fn parse_extensions(bytes: &[u8]) -> Result<ExtensionIter<'_>, TlsError> {
    let mut rest = bytes;
    while !rest.is_empty() {
        let mut reader = Reader::new(rest);
        let _kind = reader.u16()?;
        let _data = reader.vec16()?;
        rest = reader.remaining();
    }
    Ok(ExtensionIter { rest: bytes })
}

/// Writes one extension into `out`.
///
/// # Errors
///
/// Returns [`TlsError::BufferTooSmall`] when `out` cannot hold the
/// extension and [`TlsError::Malformed`] when the body is too long.
pub fn write_extension(out: &mut [u8], kind: u16, data: &[u8]) -> Result<usize, TlsError> {
    let length = u16::try_from(data.len()).map_err(|_| TlsError::Malformed)?;
    let end = 4_usize.checked_add(data.len()).ok_or(TlsError::Malformed)?;
    let buffer = out.get_mut(..end).ok_or(TlsError::BufferTooSmall)?;
    buffer
        .get_mut(..2)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(&kind.to_be_bytes());
    buffer
        .get_mut(2..4)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(&length.to_be_bytes());
    buffer
        .get_mut(4..end)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(data);
    Ok(end)
}

/// An advertised protocol version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionIter<'a> {
    rest: &'a [u8],
}

impl Iterator for VersionIter<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let first = *self.rest.first()?;
        let second = *self.rest.get(1)?;
        self.rest = self.rest.get(2..).unwrap_or(&[]);
        Some(u16::from_be_bytes([first, second]))
    }
}

/// One `key_share` entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyShareEntry<'a> {
    /// The named group of the key exchange.
    pub group: u16,
    /// The encoded public key.
    pub key_exchange: &'a [u8],
}

impl<'a> KeyShareEntry<'a> {
    /// Parses a single entry, as carried by a `ServerHello`.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Truncated`] when the entry is incomplete.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, TlsError> {
        let mut reader = Reader::new(bytes);
        let group = reader.u16()?;
        let key_exchange = reader.vec16()?;
        reader.finish()?;
        Ok(Self {
            group,
            key_exchange,
        })
    }
}

/// An iterator over the `key_share` entries of a `ClientHello`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyShareIter<'a> {
    rest: &'a [u8],
}

impl<'a> Iterator for KeyShareIter<'a> {
    type Item = KeyShareEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut reader = Reader::new(self.rest);
        let entry = KeyShareEntry::parse(self.rest).ok()?;
        let consumed = self.rest.len() - reader.remaining().len();
        let _ = consumed;
        // Advance past the entry that was just decoded.
        let entry_len = 4_usize.checked_add(entry.key_exchange.len())?;
        reader.take(entry_len).ok()?;
        self.rest = reader.remaining();
        Some(entry)
    }
}

/// Returns the entries of a `key_share` vector.
fn key_share_vector(data: &[u8]) -> Result<&[u8], TlsError> {
    let mut reader = Reader::new(data);
    let shares = reader.vec16()?;
    reader.finish()?;
    Ok(shares)
}

/// A parsed `ClientHello`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientHello<'a> {
    /// The legacy version field.
    pub legacy_version: u16,
    /// The 32-byte client random.
    pub random: [u8; 32],
    /// The legacy session identifier.
    pub session_id: &'a [u8],
    /// The advertised cipher suites, two bytes each.
    pub cipher_suites: &'a [u8],
    /// The advertised compression methods.
    pub compression_methods: &'a [u8],
    /// The extensions.
    pub extensions: ExtensionIter<'a>,
}

impl<'a> ClientHello<'a> {
    /// Parses a `ClientHello` body.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Truncated`] or [`TlsError::Malformed`] when
    /// the body is inconsistent.
    pub fn parse(body: &'a [u8]) -> Result<Self, TlsError> {
        let mut reader = Reader::new(body);
        let legacy_version = reader.u16()?;
        let random_bytes = reader.take(32)?;
        let mut random = [0_u8; 32];
        random.copy_from_slice(random_bytes);
        let session_id = reader.vec8()?;
        let cipher_suites = reader.vec16()?;
        if cipher_suites.len() % 2 != 0 {
            return Err(TlsError::Malformed);
        }
        let compression_methods = reader.vec8()?;
        let extensions = if reader.remaining().is_empty() {
            parse_extensions(&[])?
        } else {
            let block = reader.vec16()?;
            parse_extensions(block)?
        };
        reader.finish()?;
        Ok(Self {
            legacy_version,
            random,
            session_id,
            cipher_suites,
            compression_methods,
            extensions,
        })
    }

    /// Returns the first extension of `kind`.
    #[must_use]
    pub fn find_extension(&self, kind: u16) -> Option<Extension<'a>> {
        self.extensions.clone().find(|entry| entry.kind == kind)
    }
}

/// A parsed `ServerHello`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerHello<'a> {
    /// The legacy version field.
    pub legacy_version: u16,
    /// The 32-byte server random.
    pub random: [u8; 32],
    /// The echoed session identifier.
    pub session_id: &'a [u8],
    /// The selected cipher suite.
    pub cipher_suite: u16,
    /// The selected compression method.
    pub compression_method: u8,
    /// The extensions.
    pub extensions: ExtensionIter<'a>,
}

impl<'a> ServerHello<'a> {
    /// Parses a `ServerHello` body.
    ///
    /// # Errors
    ///
    /// Returns [`TlsError::Truncated`] or [`TlsError::Malformed`] when
    /// the body is inconsistent.
    pub fn parse(body: &'a [u8]) -> Result<Self, TlsError> {
        let mut reader = Reader::new(body);
        let legacy_version = reader.u16()?;
        let random_bytes = reader.take(32)?;
        let mut random = [0_u8; 32];
        random.copy_from_slice(random_bytes);
        let session_id = reader.vec8()?;
        let cipher_suite = reader.u16()?;
        let compression_method = reader.byte()?;
        let extensions = if reader.remaining().is_empty() {
            parse_extensions(&[])?
        } else {
            let block = reader.vec16()?;
            parse_extensions(block)?
        };
        reader.finish()?;
        Ok(Self {
            legacy_version,
            random,
            session_id,
            cipher_suite,
            compression_method,
            extensions,
        })
    }

    /// Returns the first extension of `kind`.
    #[must_use]
    pub fn find_extension(&self, kind: u16) -> Option<Extension<'a>> {
        self.extensions.clone().find(|entry| entry.kind == kind)
    }
}

/// The inputs of a `ClientHello` encoder.
#[derive(Clone, Copy, Debug)]
pub struct ClientHelloParams<'a> {
    /// The legacy version sent in the clear.
    pub legacy_version: u16,
    /// The 32-byte client random.
    pub random: [u8; 32],
    /// The legacy session identifier.
    pub session_id: &'a [u8],
    /// The cipher suites to advertise, two bytes each.
    pub cipher_suites: &'a [u8],
    /// The compression methods to advertise.
    pub compression_methods: &'a [u8],
    /// The already encoded extension block.
    pub extensions: &'a [u8],
}

impl Default for ClientHelloParams<'_> {
    fn default() -> Self {
        Self {
            legacy_version: TLS_VERSION_1_2,
            random: [0; 32],
            session_id: &[],
            cipher_suites: &[],
            compression_methods: &[0],
            extensions: &[],
        }
    }
}

/// Builds a complete `ClientHello` handshake message.
///
/// # Errors
///
/// Returns [`TlsError::BufferTooSmall`] when `out` cannot hold the
/// message and [`TlsError::Malformed`] when a length field overflows.
pub fn build_client_hello(
    out: &mut [u8],
    params: &ClientHelloParams<'_>,
) -> Result<usize, TlsError> {
    let session_len = u8::try_from(params.session_id.len()).map_err(|_| TlsError::Malformed)?;
    let suites_len = u16::try_from(params.cipher_suites.len()).map_err(|_| TlsError::Malformed)?;
    let compression_len =
        u8::try_from(params.compression_methods.len()).map_err(|_| TlsError::Malformed)?;
    let extensions_len = u16::try_from(params.extensions.len()).map_err(|_| TlsError::Malformed)?;

    let body_len = 2_usize
        .checked_add(32)
        .and_then(|value| value.checked_add(1 + params.session_id.len()))
        .and_then(|value| value.checked_add(2 + params.cipher_suites.len()))
        .and_then(|value| value.checked_add(1 + params.compression_methods.len()))
        .and_then(|value| value.checked_add(2 + params.extensions.len()))
        .ok_or(TlsError::Malformed)?;
    let total = HANDSHAKE_HEADER_LEN
        .checked_add(body_len)
        .ok_or(TlsError::Malformed)?;
    let buffer = out.get_mut(..total).ok_or(TlsError::BufferTooSmall)?;
    buffer[0] = HS_CLIENT_HELLO;
    let body_len_u32 = u32::try_from(body_len).map_err(|_| TlsError::Malformed)?;
    buffer
        .get_mut(1..4)
        .ok_or(TlsError::BufferTooSmall)?
        .copy_from_slice(&body_len_u32.to_be_bytes()[1..4]);

    let mut position = HANDSHAKE_HEADER_LEN;
    let write = |buffer: &mut [u8], at: usize, data: &[u8]| -> Result<usize, TlsError> {
        let end = at.checked_add(data.len()).ok_or(TlsError::Malformed)?;
        buffer
            .get_mut(at..end)
            .ok_or(TlsError::BufferTooSmall)?
            .copy_from_slice(data);
        Ok(end)
    };

    position = write(buffer, position, &params.legacy_version.to_be_bytes())?;
    position = write(buffer, position, &params.random)?;
    position = write(buffer, position, &[session_len])?;
    position = write(buffer, position, params.session_id)?;
    position = write(buffer, position, &suites_len.to_be_bytes())?;
    position = write(buffer, position, params.cipher_suites)?;
    position = write(buffer, position, &[compression_len])?;
    position = write(buffer, position, params.compression_methods)?;
    position = write(buffer, position, &extensions_len.to_be_bytes())?;
    position = write(buffer, position, params.extensions)?;
    Ok(position)
}
