//! # IPv6
//!
//! Building IPv6 headers and walking the extension header chain.
//!
//! The base header follows `RFC 8200`. The extension headers that the
//! stack understands are the hop-by-hop options header, the routing
//! header, the fragment header and the destination options header.

use crate::protocols::network::packets::{
    IP_PROTOCOL_UDP, IPV6_FRAGMENT_HEADER_LEN, IPV6_HEADER_LEN, IPV6_NEXT_DEST_OPTS,
    IPV6_NEXT_FRAGMENT, IPV6_NEXT_HOP_BY_HOP, IPV6_NEXT_ROUTING,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::Ipv6Address;

/// The largest number of extension headers that are traversed.
pub const MAX_EXTENSION_HEADERS: usize = 16;

/// The length of an IPv6 options extension header in bytes.
pub const IPV6_OPTIONS_HEADER_LEN: usize = 8;

/// Parameters used to build an IPv6 header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ipv6Params {
    /// The source address.
    pub src: Ipv6Address,
    /// The destination address.
    pub dst: Ipv6Address,
    /// The next-header value of the base header.
    pub next_header: u8,
    /// The hop limit in hops.
    pub hop_limit: u8,
    /// The traffic class.
    pub traffic_class: u8,
    /// The 20-bit flow label.
    pub flow_label: u32,
}

impl Default for Ipv6Params {
    fn default() -> Self {
        Self {
            src: Ipv6Address([0; 16]),
            dst: Ipv6Address([0; 16]),
            next_header: IP_PROTOCOL_UDP,
            hop_limit: 64,
            traffic_class: 0,
            flow_label: 0,
        }
    }
}

/// Writes an IPv6 base header into `out`.
///
/// `payload_len` is the length of everything that follows the base
/// header, including any extension headers.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// 40 bytes or when the resulting payload length does not fit into the
/// 16-bit payload length field.
pub fn write_header(
    out: &mut [u8],
    params: &Ipv6Params,
    payload_len: usize,
) -> Result<usize, StackError> {
    let payload_u16 = u16::try_from(payload_len).map_err(|_| StackError::BufferTooSmall)?;
    let header = out
        .get_mut(..IPV6_HEADER_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    let first =
        (6_u32 << 28) | (u32::from(params.traffic_class) << 20) | (params.flow_label & 0x000F_FFFF);
    header[0..4].copy_from_slice(&first.to_be_bytes());
    header[4..6].copy_from_slice(&payload_u16.to_be_bytes());
    header[6] = params.next_header;
    header[7] = params.hop_limit;
    header[8..24].copy_from_slice(&params.src.0);
    header[24..40].copy_from_slice(&params.dst.0);
    Ok(IPV6_HEADER_LEN)
}

/// Builds a complete IPv6 datagram in `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_datagram(
    out: &mut [u8],
    params: &Ipv6Params,
    payload: &[u8],
) -> Result<usize, StackError> {
    let header_len = write_header(out, params, payload.len())?;
    let body = out
        .get_mut(
            header_len
                ..header_len
                    .checked_add(payload.len())
                    .ok_or(StackError::BufferTooSmall)?,
        )
        .ok_or(StackError::BufferTooSmall)?;
    body.copy_from_slice(payload);
    Ok(header_len + payload.len())
}

/// Writes an IPv6 options extension header into `out`.
///
/// The same layout is used by the hop-by-hop, routing and destination
/// options headers: a next-header field, a length field in eight-byte
/// units and the option body. The body is padded so that the whole
/// header is a multiple of eight bytes long.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// padded header.
pub fn build_extension_header(
    out: &mut [u8],
    next_header: u8,
    options: &[u8],
) -> Result<usize, StackError> {
    let pad = (6 + 8 - (options.len() % 8)) % 8;
    let body_len = options
        .len()
        .checked_add(pad)
        .ok_or(StackError::BufferTooSmall)?;
    let total = body_len.checked_add(2).ok_or(StackError::BufferTooSmall)?;
    let ext_len = u8::try_from(total / 8 - 1).map_err(|_| StackError::BufferTooSmall)?;
    let header = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    header[0] = next_header;
    header[1] = ext_len;
    let body = header.get_mut(2..total).ok_or(StackError::BufferTooSmall)?;
    let written = body
        .get_mut(..options.len())
        .ok_or(StackError::BufferTooSmall)?;
    written.copy_from_slice(options);
    for byte in body
        .get_mut(options.len()..)
        .ok_or(StackError::BufferTooSmall)?
    {
        *byte = 0;
    }
    Ok(total)
}

/// Writes an IPv6 fragment header into `out`.
///
/// The fragment offset is expressed in eight-byte units and the more
/// fragments flag is carried in the low bit of the offset field.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// eight bytes.
pub fn build_fragment_header(
    out: &mut [u8],
    next_header: u8,
    offset_units: u16,
    more_fragments: bool,
    identification: u32,
) -> Result<usize, StackError> {
    let header = out
        .get_mut(..IPV6_FRAGMENT_HEADER_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    header[0] = next_header;
    header[1] = 0;
    let mut offset_flags = (offset_units & 0x1FFF) << 3;
    if more_fragments {
        offset_flags |= 0x0001;
    }
    header[2..4].copy_from_slice(&offset_flags.to_be_bytes());
    header[4..8].copy_from_slice(&identification.to_be_bytes());
    Ok(IPV6_FRAGMENT_HEADER_LEN)
}

/// The end of an IPv6 extension header chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtensionChain {
    /// The upper-layer protocol number found at the end of the chain.
    pub upper_layer: u8,
    /// The offset of the upper-layer payload inside the datagram.
    pub payload_offset: usize,
    /// The number of extension headers that were traversed.
    pub header_count: usize,
}

/// Walks the IPv6 extension header chain.
///
/// The walk starts at `offset` with the next-header value `next_header`
/// and stops at the first value that is not an extension header. At
/// most [`MAX_EXTENSION_HEADERS`] headers are traversed so that a
/// malformed chain cannot loop forever.
///
/// # Errors
///
/// Returns `None` when the chain is truncated, when a header declares
/// an impossible length or when the traversal limit is exceeded.
#[must_use]
pub fn walk_extension_headers(
    data: &[u8],
    offset: usize,
    next_header: u8,
) -> Option<ExtensionChain> {
    let mut position = offset;
    let mut current = next_header;
    let mut count = 0_usize;
    loop {
        match current {
            IPV6_NEXT_HOP_BY_HOP | IPV6_NEXT_ROUTING | IPV6_NEXT_DEST_OPTS => {
                count += 1;
                if count > MAX_EXTENSION_HEADERS {
                    return None;
                }
                let next = *data.get(position)?;
                let length_byte = *data.get(position.checked_add(1)?)?;
                let total = usize::from(length_byte).checked_add(1)?.checked_mul(8)?;
                if total < IPV6_OPTIONS_HEADER_LEN {
                    return None;
                }
                position = position.checked_add(total)?;
                if position > data.len() {
                    return None;
                }
                current = next;
            }
            IPV6_NEXT_FRAGMENT => {
                count += 1;
                if count > MAX_EXTENSION_HEADERS {
                    return None;
                }
                let next = *data.get(position)?;
                position = position.checked_add(IPV6_FRAGMENT_HEADER_LEN)?;
                if position > data.len() {
                    return None;
                }
                current = next;
                // The fragment header is the last extension header of
                // the chain, the upper-layer header follows it.
                break;
            }
            _ => break,
        }
    }
    Some(ExtensionChain {
        upper_layer: current,
        payload_offset: position,
        header_count: count,
    })
}
