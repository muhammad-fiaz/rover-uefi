//! # Ethernet framing
//!
//! Building and parsing of Ethernet II frames.
//!
//! The builder side of this module writes the 14-byte Ethernet II
//! header described by `IEEE 802.3` and `IEEE 802.1Q` into a caller
//! supplied buffer. The parser side reuses the header parser in
//! [`crate::protocols::network::packets`] so that both directions
//! agree on the layout.

use crate::protocols::network::packets::{
    ETHERNET_HEADER_LEN, ETHERNET_VLAN_TAG_LEN, ETHERTYPE_ARP, ETHERTYPE_IPV4, ETHERTYPE_IPV6,
    ETHERTYPE_VLAN, EthernetHeader, VlanTag,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::MacAddress;

pub use crate::protocols::network::packets::{
    ETHERNET_HEADER_LEN as HEADER_LEN, ETHERNET_VLAN_TAG_LEN as VLAN_TAG_LEN,
};

/// The broadcast link-layer address.
pub const BROADCAST_MAC: MacAddress = MacAddress([0xFF; 6]);

/// The all-zero link-layer address.
pub const ZERO_MAC: MacAddress = MacAddress([0x00; 6]);

/// A parsed Ethernet frame.
///
/// This is the header type produced by
/// [`crate::protocols::network::packets::parse_header`], re-exported so
/// that stack users do not have to reach into the parser module.
pub type EthernetFrame<'a> = EthernetHeader<'a>;

/// Returns the number of bytes a frame with `vlan` occupies before the
/// payload.
#[must_use]
pub const fn frame_overhead(vlan: bool) -> usize {
    if vlan {
        ETHERNET_HEADER_LEN + ETHERNET_VLAN_TAG_LEN
    } else {
        ETHERNET_HEADER_LEN
    }
}

/// Builds an Ethernet II frame in `out`.
///
/// The frame contains the destination address, the source address, the
/// `EtherType` and the payload. The returned value is the number of
/// bytes written.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_frame(
    out: &mut [u8],
    dst: MacAddress,
    src: MacAddress,
    ethertype: u16,
    payload: &[u8],
) -> Result<usize, StackError> {
    build_frame_vlan(out, dst, src, None, ethertype, payload)
}

/// Builds an Ethernet II frame with an optional 802.1Q tag.
///
/// When `vlan` is `Some`, the tag control information and the inner
/// `EtherType` follow the source address.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_frame_vlan(
    out: &mut [u8],
    dst: MacAddress,
    src: MacAddress,
    vlan: Option<VlanTag>,
    ethertype: u16,
    payload: &[u8],
) -> Result<usize, StackError> {
    let overhead = frame_overhead(vlan.is_some());
    let total = overhead
        .checked_add(payload.len())
        .ok_or(StackError::BufferTooSmall)?;
    if out.len() < total {
        return Err(StackError::BufferTooSmall);
    }
    let dst_bytes = out.get_mut(..6).ok_or(StackError::BufferTooSmall)?;
    dst_bytes.copy_from_slice(&dst.0);
    let src_bytes = out.get_mut(6..12).ok_or(StackError::BufferTooSmall)?;
    src_bytes.copy_from_slice(&src.0);
    match vlan {
        None => {
            let type_bytes = out.get_mut(12..14).ok_or(StackError::BufferTooSmall)?;
            type_bytes.copy_from_slice(&ethertype.to_be_bytes());
        }
        Some(tag) => {
            let type_bytes = out.get_mut(12..14).ok_or(StackError::BufferTooSmall)?;
            type_bytes.copy_from_slice(&ETHERTYPE_VLAN.to_be_bytes());
            let tag_bytes = out.get_mut(14..16).ok_or(StackError::BufferTooSmall)?;
            tag_bytes.copy_from_slice(&tag.tci.to_be_bytes());
            let inner = out.get_mut(16..18).ok_or(StackError::BufferTooSmall)?;
            inner.copy_from_slice(&ethertype.to_be_bytes());
        }
    }
    let body = out
        .get_mut(overhead..total)
        .ok_or(StackError::BufferTooSmall)?;
    body.copy_from_slice(payload);
    Ok(total)
}

/// Parses an Ethernet II frame.
///
/// A single 802.1Q tag is recognised and reported through the `vlan`
/// field of the returned header.
///
/// # Errors
///
/// Returns `None` when the frame is truncated, when a declared tag is
/// truncated or when nested tags are present.
#[must_use]
pub fn parse_frame(data: &[u8]) -> Option<EthernetFrame<'_>> {
    crate::protocols::network::packets::parse_header(data)
}

/// Builds a tag control information field from its parts.
///
/// The priority code point occupies the three most significant bits,
/// the drop eligible indicator the fourth and the identifier the
/// twelve least significant bits.
#[must_use]
pub fn vlan_tci(vid: u16, pcp: u8, dei: bool) -> u16 {
    let vid = vid & 0x0FFF;
    let pcp = u16::from(pcp & 0x07);
    let dei_bit = if dei { 0x1000_u16 } else { 0 };
    (pcp << 13) | dei_bit | vid
}

/// Returns whether the `EtherType` denotes IPv4.
#[must_use]
pub const fn is_ipv4(ethertype: u16) -> bool {
    ethertype == ETHERTYPE_IPV4
}

/// Returns whether the `EtherType` denotes IPv6.
#[must_use]
pub const fn is_ipv6(ethertype: u16) -> bool {
    ethertype == ETHERTYPE_IPV6
}

/// Returns whether the `EtherType` denotes ARP.
#[must_use]
pub const fn is_arp(ethertype: u16) -> bool {
    ethertype == ETHERTYPE_ARP
}
