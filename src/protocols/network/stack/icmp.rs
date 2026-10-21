//! # `ICMP` and `ICMPv6`
//!
//! Native encoders and decoders for the Internet Control Message
//! Protocol, versions 4 and 6.
//!
//! The module builds and parses echo messages and exposes the generic
//! message layout used by error reports. `ICMPv4` uses the 16-bit
//! Internet checksum directly; `ICMPv6` requires the IPv6 pseudo
//! header, whose addresses the caller supplies.
//!
//! Nothing here allocates and no function panics. Every length is
//! checked before it is used.

use crate::protocols::network::types::Ipv6Address;

/// The size of the common `ICMP` header in bytes.
pub const ICMP_HEADER_LEN: usize = 8;

/// The `ICMPv4` echo reply type.
pub const ICMPV4_ECHO_REPLY: u8 = 0;

/// The `ICMPv4` destination unreachable type.
pub const ICMPV4_DESTINATION_UNREACHABLE: u8 = 3;

/// The `ICMPv4` echo request type.
pub const ICMPV4_ECHO_REQUEST: u8 = 8;

/// The `ICMPv4` time exceeded type.
pub const ICMPV4_TIME_EXCEEDED: u8 = 11;

/// The `ICMPv4` parameter problem type.
pub const ICMPV4_PARAMETER_PROBLEM: u8 = 12;

/// The `ICMPv6` destination unreachable type.
pub const ICMPV6_DESTINATION_UNREACHABLE: u8 = 1;

/// The `ICMPv6` packet too big type.
pub const ICMPV6_PACKET_TOO_BIG: u8 = 2;

/// The `ICMPv6` time exceeded type.
pub const ICMPV6_TIME_EXCEEDED: u8 = 3;

/// The `ICMPv6` parameter problem type.
pub const ICMPV6_PARAMETER_PROBLEM: u8 = 4;

/// The `ICMPv6` echo request type.
pub const ICMPV6_ECHO_REQUEST: u8 = 128;

/// The `ICMPv6` echo reply type.
pub const ICMPV6_ECHO_REPLY: u8 = 129;

/// The `ICMPv6` next-header value used by the pseudo header.
pub const IPV6_NEXT_HEADER_ICMPV6: u8 = 58;

/// A parsed `ICMP` message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IcmpMessage<'a> {
    /// The message type.
    pub kind: u8,
    /// The message code.
    pub code: u8,
    /// The checksum carried by the message.
    pub checksum: u16,
    /// The echo identifier carried by the header.
    pub identifier: u16,
    /// The echo sequence number carried by the header.
    pub sequence: u16,
    /// The message body after the eight byte header.
    pub body: &'a [u8],
}

impl<'a> IcmpMessage<'a> {
    /// Parses the common `ICMP` header and body.
    ///
    /// Returns `None` for a message shorter than the header.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..ICMP_HEADER_LEN)?;
        Some(Self {
            kind: *header.first()?,
            code: *header.get(1)?,
            checksum: u16::from_be_bytes([*header.get(2)?, *header.get(3)?]),
            identifier: u16::from_be_bytes([*header.get(4)?, *header.get(5)?]),
            sequence: u16::from_be_bytes([*header.get(6)?, *header.get(7)?]),
            body: bytes.get(ICMP_HEADER_LEN..).unwrap_or(&[]),
        })
    }

    /// Returns whether the message is an echo request of `version`.
    ///
    /// `version` is 4 or 6; any other value returns `false`.
    #[must_use]
    pub const fn is_echo_request(&self, version: u8) -> bool {
        match version {
            4 => self.kind == ICMPV4_ECHO_REQUEST,
            6 => self.kind == ICMPV6_ECHO_REQUEST,
            _ => false,
        }
    }

    /// Returns whether the message is an echo reply of `version`.
    #[must_use]
    pub const fn is_echo_reply(&self, version: u8) -> bool {
        match version {
            4 => self.kind == ICMPV4_ECHO_REPLY,
            6 => self.kind == ICMPV6_ECHO_REPLY,
            _ => false,
        }
    }

    /// Returns whether the message reports an error.
    ///
    /// `ICMPv4` errors are types 3, 4, 5, 11 and 12. `ICMPv6` errors are
    /// types below 128.
    #[must_use]
    pub const fn is_error(&self, version: u8) -> bool {
        match version {
            4 => matches!(self.kind, 3..=5 | 11 | 12),
            6 => self.kind < 128,
            _ => false,
        }
    }

    /// Returns the echo identifier and sequence number from the
    /// message header.
    #[must_use]
    pub const fn echo_identity(&self) -> (u16, u16) {
        (self.identifier, self.sequence)
    }
}

/// Returns the 16-bit one's complement checksum of `data`.
///
/// An odd trailing byte is padded with a zero byte. The result is the
/// value to store in the checksum field; computing it over a complete
/// and valid packet returns zero.
#[must_use]
pub fn checksum(data: &[u8]) -> u16 {
    let mut sum = add_words(0, data);
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !u16::try_from(sum & 0xFFFF).unwrap_or(0)
}

/// Returns whether the checksum carried by a complete `ICMPv4` message
/// is correct.
#[must_use]
pub fn verify_checksum(bytes: &[u8]) -> bool {
    checksum(bytes) == 0
}

/// Returns the checksum of an `ICMPv6` message including its pseudo
/// header.
#[must_use]
pub fn checksum_v6(source: Ipv6Address, destination: Ipv6Address, payload: &[u8]) -> u16 {
    let Ok(length) = u32::try_from(payload.len()) else {
        return 0xFFFF;
    };
    let mut sum = add_words(0, &source.0);
    sum = add_words(sum, &destination.0);
    sum = add_words(sum, &length.to_be_bytes());
    sum = add_words(sum, &[0, 0, 0, IPV6_NEXT_HEADER_ICMPV6]);
    sum = add_words(sum, payload);
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !u16::try_from(sum & 0xFFFF).unwrap_or(0)
}

/// Returns whether the checksum of a complete `ICMPv6` message is
/// correct for the given addresses.
#[must_use]
pub fn verify_checksum_v6(source: Ipv6Address, destination: Ipv6Address, bytes: &[u8]) -> bool {
    checksum_v6(source, destination, bytes) == 0
}

/// Builds an `ICMPv4` echo request and returns its length.
///
/// Returns `None` when `out` cannot hold the header and payload.
#[must_use]
pub fn build_echo_request(
    out: &mut [u8],
    identifier: u16,
    sequence: u16,
    payload: &[u8],
) -> Option<usize> {
    build_echo(out, ICMPV4_ECHO_REQUEST, identifier, sequence, payload)
}

/// Builds an `ICMPv4` echo reply and returns its length.
#[must_use]
pub fn build_echo_reply(
    out: &mut [u8],
    identifier: u16,
    sequence: u16,
    payload: &[u8],
) -> Option<usize> {
    build_echo(out, ICMPV4_ECHO_REPLY, identifier, sequence, payload)
}

/// Builds an `ICMPv6` echo request and returns its length.
///
/// The checksum covers the supplied IPv6 pseudo header.
#[must_use]
pub fn build_echo_request_v6(
    out: &mut [u8],
    source: Ipv6Address,
    destination: Ipv6Address,
    identifier: u16,
    sequence: u16,
    payload: &[u8],
) -> Option<usize> {
    let length = build_echo(out, ICMPV6_ECHO_REQUEST, identifier, sequence, payload)?;
    out.get_mut(2..4)?.fill(0);
    let message = out.get(..length)?;
    let value = checksum_v6(source, destination, message);
    let slice = out.get_mut(2..4)?;
    slice.copy_from_slice(&value.to_be_bytes());
    Some(length)
}

/// Builds an `ICMPv6` echo reply and returns its length.
#[must_use]
pub fn build_echo_reply_v6(
    out: &mut [u8],
    source: Ipv6Address,
    destination: Ipv6Address,
    identifier: u16,
    sequence: u16,
    payload: &[u8],
) -> Option<usize> {
    let length = build_echo(out, ICMPV6_ECHO_REPLY, identifier, sequence, payload)?;
    out.get_mut(2..4)?.fill(0);
    let message = out.get(..length)?;
    let value = checksum_v6(source, destination, message);
    let slice = out.get_mut(2..4)?;
    slice.copy_from_slice(&value.to_be_bytes());
    Some(length)
}

/// Writes an echo message of `kind` and returns its length.
fn build_echo(
    out: &mut [u8],
    kind: u8,
    identifier: u16,
    sequence: u16,
    payload: &[u8],
) -> Option<usize> {
    let total = ICMP_HEADER_LEN.checked_add(payload.len())?;
    if out.len() < total {
        return None;
    }
    let header = out.get_mut(..ICMP_HEADER_LEN)?;
    header.fill(0);
    *header.first_mut()? = kind;
    header
        .get_mut(4..6)?
        .copy_from_slice(&identifier.to_be_bytes());
    header
        .get_mut(6..8)?
        .copy_from_slice(&sequence.to_be_bytes());
    let body = out.get_mut(ICMP_HEADER_LEN..total)?;
    body.copy_from_slice(payload);
    let message = out.get(..total)?;
    let value = checksum(message);
    let slice = out.get_mut(2..4)?;
    slice.copy_from_slice(&value.to_be_bytes());
    Some(total)
}

/// Adds 16-bit big-endian words of `bytes` to `sum`.
fn add_words(mut sum: u32, bytes: &[u8]) -> u32 {
    let (chunks, remainder) = bytes.as_chunks::<2>();
    for chunk in chunks {
        sum += u32::from(u16::from_be_bytes(*chunk));
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    if let Some(&last) = remainder.first() {
        sum += u32::from(u16::from_be_bytes([last, 0]));
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    sum
}
