//! # UDP
//!
//! Building and parsing of UDP datagrams.
//!
//! The datagram layout follows `RFC 768`. The optional checksum uses
//! the pseudo-header defined by `RFC 793` for IPv4 and by `RFC 8200`
//! for IPv6. The checksum arithmetic is reused from
//! [`crate::protocols::network::packets`].

use crate::protocols::network::packets::{
    IP_PROTOCOL_UDP, UDP_HEADER_LEN, UdpHeader, ipv4_segment_checksum, ipv6_segment_checksum,
    parse_udp, verify_ipv4_segment_checksum, verify_ipv6_segment_checksum,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::{Ipv4Address, Ipv6Address};

/// The largest UDP datagram that can be built.
pub const UDP_MAX_DATAGRAM_LEN: usize = 65_535;

/// Parameters used to build a UDP datagram.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UdpParams {
    /// The source port.
    pub src_port: u16,
    /// The destination port.
    pub dst_port: u16,
}

/// Builds a UDP datagram without a checksum in `out`.
///
/// A datagram without a checksum is only valid over IPv4.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_datagram(
    out: &mut [u8],
    params: &UdpParams,
    payload: &[u8],
) -> Result<usize, StackError> {
    let total = UDP_HEADER_LEN
        .checked_add(payload.len())
        .ok_or(StackError::BufferTooSmall)?;
    let length = u16::try_from(total).map_err(|_| StackError::BufferTooSmall)?;
    let header = out
        .get_mut(..UDP_HEADER_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    header[0..2].copy_from_slice(&params.src_port.to_be_bytes());
    header[2..4].copy_from_slice(&params.dst_port.to_be_bytes());
    header[4..6].copy_from_slice(&length.to_be_bytes());
    header[6..8].copy_from_slice(&[0, 0]);
    let body = out
        .get_mut(UDP_HEADER_LEN..total)
        .ok_or(StackError::BufferTooSmall)?;
    body.copy_from_slice(payload);
    Ok(total)
}

/// Builds a UDP datagram with an IPv4 checksum in `out`.
///
/// The checksum covers the IPv4 pseudo-header and the datagram. A
/// computed value of zero is transmitted as all ones, because zero
/// means that no checksum was sent.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_datagram_ipv4(
    out: &mut [u8],
    params: &UdpParams,
    payload: &[u8],
    src: &Ipv4Address,
    dst: &Ipv4Address,
) -> Result<usize, StackError> {
    let total = build_datagram(out, params, payload)?;
    let segment = out.get(..total).ok_or(StackError::BufferTooSmall)?;
    let checksum = ipv4_segment_checksum(src, dst, IP_PROTOCOL_UDP, segment);
    let stored = if checksum == 0 { 0xFFFF } else { checksum };
    let field = out.get_mut(6..8).ok_or(StackError::BufferTooSmall)?;
    field.copy_from_slice(&stored.to_be_bytes());
    Ok(total)
}

/// Builds a UDP datagram with an IPv6 checksum in `out`.
///
/// IPv6 requires the checksum, so a computed value of zero is
/// transmitted as all ones.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_datagram_ipv6(
    out: &mut [u8],
    params: &UdpParams,
    payload: &[u8],
    src: &Ipv6Address,
    dst: &Ipv6Address,
) -> Result<usize, StackError> {
    let total = build_datagram(out, params, payload)?;
    let segment = out.get(..total).ok_or(StackError::BufferTooSmall)?;
    let checksum = ipv6_segment_checksum(src, dst, IP_PROTOCOL_UDP, segment);
    let stored = if checksum == 0 { 0xFFFF } else { checksum };
    let field = out.get_mut(6..8).ok_or(StackError::BufferTooSmall)?;
    field.copy_from_slice(&stored.to_be_bytes());
    Ok(total)
}

/// Parses a UDP datagram.
///
/// # Errors
///
/// Returns `None` when the datagram is truncated or declares a length
/// that does not fit the buffer.
#[must_use]
pub fn parse_datagram(data: &[u8]) -> Option<UdpHeader<'_>> {
    parse_udp(data)
}

/// Returns whether a UDP datagram verifies over IPv4.
///
/// # Errors
///
/// Returns `false` when the datagram is truncated or when the stored
/// checksum does not match.
#[must_use]
pub fn verify_ipv4_checksum(datagram: &[u8], src: &Ipv4Address, dst: &Ipv4Address) -> bool {
    let Some(header) = parse_udp(datagram) else {
        return false;
    };
    if header.checksum == 0 {
        return true;
    }
    verify_ipv4_segment_checksum(src, dst, IP_PROTOCOL_UDP, header.datagram)
}

/// Returns whether a UDP datagram verifies over IPv6.
///
/// # Errors
///
/// Returns `false` when the datagram is truncated, when no checksum
/// was transmitted or when the stored checksum does not match.
#[must_use]
pub fn verify_ipv6_checksum(datagram: &[u8], src: &Ipv6Address, dst: &Ipv6Address) -> bool {
    let Some(header) = parse_udp(datagram) else {
        return false;
    };
    if header.checksum == 0 {
        return false;
    }
    verify_ipv6_segment_checksum(src, dst, IP_PROTOCOL_UDP, header.datagram)
}
