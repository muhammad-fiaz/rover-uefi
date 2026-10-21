//! # Network packets
//!
//! Bounded, heapless parsers for Ethernet, ARP, IPv4, IPv6, ICMP, UDP
//! and TCP headers.
//!
//! All parsers are `no_std`, zero-allocation and operate on borrowed
//! slices. They return `Option` on truncated or malformed input and
//! never panic or overflow. Length fields are always checked against
//! the available buffer before slicing.
//!
//! Internet checksums follow `RFC 1071`: 16-bit big-endian words are
//! summed with end-around carry and complemented. Pseudo-headers for
//! TCP, UDP and `ICMPv6` are built on the stack without allocation.

use crate::base::bytes::{read_u16_be, read_u32_be};

use crate::protocols::network::types::{Ipv4Address, Ipv6Address, MacAddress};

/// `EtherType` for IPv4.
pub const ETHERTYPE_IPV4: u16 = 0x0800;
/// `EtherType` for ARP.
pub const ETHERTYPE_ARP: u16 = 0x0806;
/// `EtherType` for IPv6.
pub const ETHERTYPE_IPV6: u16 = 0x86DD;
/// `EtherType` for a single 802.1Q VLAN tag.
pub const ETHERTYPE_VLAN: u16 = 0x8100;

/// Length of the Ethernet header without a VLAN tag in bytes.
pub const ETHERNET_HEADER_LEN: usize = 14;
/// Additional bytes consumed by a single 802.1Q VLAN tag.
pub const ETHERNET_VLAN_TAG_LEN: usize = 4;
/// Length of a fixed Ethernet / IPv4 ARP frame payload in bytes.
pub const ARP_FRAME_LEN: usize = 28;
/// Length of the IPv4 header without options in bytes.
pub const IPV4_MIN_HEADER_LEN: usize = 20;
/// Length of the IPv6 header in bytes.
pub const IPV6_HEADER_LEN: usize = 40;
/// Length of an IPv6 fragment header in bytes.
pub const IPV6_FRAGMENT_HEADER_LEN: usize = 8;
/// Length of the UDP header in bytes.
pub const UDP_HEADER_LEN: usize = 8;
/// Length of the TCP header without options in bytes.
pub const TCP_MIN_HEADER_LEN: usize = 20;

/// IP protocol number for ICMP.
pub const IP_PROTOCOL_ICMP: u8 = 1;
/// IP protocol number for TCP.
pub const IP_PROTOCOL_TCP: u8 = 6;
/// IP protocol number for UDP.
pub const IP_PROTOCOL_UDP: u8 = 17;

/// IPv6 next-header value for `ICMPv6`.
pub const IPV6_NEXT_ICMP: u8 = 58;
/// IPv6 next-header value for hop-by-hop options.
pub const IPV6_NEXT_HOP_BY_HOP: u8 = 0;
/// IPv6 next-header value for routing.
pub const IPV6_NEXT_ROUTING: u8 = 43;
/// IPv6 next-header value for fragment.
pub const IPV6_NEXT_FRAGMENT: u8 = 44;
/// IPv6 next-header value for destination options.
pub const IPV6_NEXT_DEST_OPTS: u8 = 60;

/// ICMP type for echo reply.
pub const ICMPV4_ECHO_REPLY: u8 = 0;
/// ICMP type for destination unreachable.
pub const ICMPV4_DEST_UNREACHABLE: u8 = 3;
/// ICMP type for echo request.
pub const ICMPV4_ECHO_REQUEST: u8 = 8;
/// ICMP type for time exceeded.
pub const ICMPV4_TIME_EXCEEDED: u8 = 11;

/// `ICMPv6` type for echo reply.
pub const ICMPV6_ECHO_REPLY: u8 = 129;
/// `ICMPv6` type for echo request.
pub const ICMPV6_ECHO_REQUEST: u8 = 128;

/// ARP hardware type for Ethernet.
pub const ARP_HTYPE_ETHERNET: u16 = 1;
/// ARP protocol type for IPv4.
pub const ARP_PTYPE_IPV4: u16 = 0x0800;
/// ARP operation for request.
pub const ARP_OPER_REQUEST: u16 = 1;
/// ARP operation for reply.
pub const ARP_OPER_REPLY: u16 = 2;

/// TCP flag for finish.
pub const TCP_FLAG_FIN: u8 = 0x01;
/// TCP flag for synchronize.
pub const TCP_FLAG_SYN: u8 = 0x02;
/// TCP flag for reset.
pub const TCP_FLAG_RST: u8 = 0x04;
/// TCP flag for push.
pub const TCP_FLAG_PSH: u8 = 0x08;
/// TCP flag for acknowledgment.
pub const TCP_FLAG_ACK: u8 = 0x10;
/// TCP flag for urgent.
pub const TCP_FLAG_URG: u8 = 0x20;

// ---------------------------------------------------------------------------
// Internet checksum (RFC 1071)
// ---------------------------------------------------------------------------

/// Adds big-endian 16-bit words of `data` to `sum`.
///
/// Folding is performed after every addition so the accumulator stays far
/// from overflow for any realistic packet size.
fn sum_words(mut sum: u32, data: &[u8]) -> u32 {
    let (chunks, remainder) = data.as_chunks::<2>();
    for chunk in chunks {
        let word = u32::from(u16::from_be_bytes([chunk[0], chunk[1]]));
        sum = sum.saturating_add(word);
        // Fold carry back into the low 16 bits immediately.
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    if !remainder.is_empty() {
        let word = u32::from(u16::from_be_bytes([remainder[0], 0]));
        sum = sum.saturating_add(word);
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    // One final fold so the result is always a 16-bit value.
    if sum > 0xFFFF {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    sum
}

/// Folds a 32-bit accumulator down to a 16-bit ones-complement sum.
fn fold_sum(sum: u32) -> u16 {
    let folded = (sum & 0xFFFF) + (sum >> 16);
    let folded = (folded & 0xFFFF) + (folded >> 16);
    u16::try_from(folded).unwrap_or(0xFFFF)
}

/// Computes the folded ones-complement sum over `data`.
///
/// The returned value is the checksum complement: the stored checksum
/// field makes this sum equal to `0xFFFF` when the checksum is included.
fn folded_sum(data: &[u8]) -> u16 {
    fold_sum(sum_words(0, data))
}

/// Computes the folded ones-complement sum over two consecutive buffers
/// whose boundary is word-aligned.
fn folded_sum_two(first: &[u8], second: &[u8]) -> u16 {
    fold_sum(sum_words(sum_words(0, first), second))
}

/// Returns the `RFC 1071` internet checksum for `data`.
#[must_use]
pub fn internet_checksum(data: &[u8]) -> u16 {
    !folded_sum(data)
}

/// Returns whether `data`, which includes a stored checksum field,
/// verifies to zero under the internet checksum.
#[must_use]
pub fn verify_checksum(data: &[u8]) -> bool {
    folded_sum(data) == 0xFFFF
}

/// Builds an IPv4 pseudo-header for TCP, UDP or ICMP checksums.
fn ipv4_pseudo_header(src: Ipv4Address, dst: Ipv4Address, protocol: u8, len: usize) -> [u8; 12] {
    let len_u16 = u16::try_from(len).unwrap_or(u16::MAX);
    let mut pseudo = [0_u8; 12];
    pseudo[0..4].copy_from_slice(&src.0);
    pseudo[4..8].copy_from_slice(&dst.0);
    pseudo[8] = 0;
    pseudo[9] = protocol;
    pseudo[10..12].copy_from_slice(&len_u16.to_be_bytes());
    pseudo
}

/// Builds an IPv6 pseudo-header for TCP, UDP or `ICMPv6` checksums.
fn ipv6_pseudo_header(
    src: &Ipv6Address,
    dst: &Ipv6Address,
    next_header: u8,
    len: usize,
) -> [u8; 40] {
    let len_u32 = u32::try_from(len).unwrap_or(u32::MAX);
    let mut pseudo = [0_u8; 40];
    pseudo[0..16].copy_from_slice(&src.0);
    pseudo[16..32].copy_from_slice(&dst.0);
    pseudo[32..36].copy_from_slice(&len_u32.to_be_bytes());
    pseudo[36] = 0;
    pseudo[37] = 0;
    pseudo[38] = 0;
    pseudo[39] = next_header;
    pseudo
}

/// Returns the checksum value for an IPv4 upper-layer segment.
///
/// `segment_with_zero_checksum` must be the segment with its checksum
/// field cleared. Callers insert the returned value into that field.
#[must_use]
pub fn ipv4_segment_checksum(
    src: &Ipv4Address,
    dst: &Ipv4Address,
    protocol: u8,
    segment_with_zero_checksum: &[u8],
) -> u16 {
    let pseudo = ipv4_pseudo_header(*src, *dst, protocol, segment_with_zero_checksum.len());
    !folded_sum_two(&pseudo, segment_with_zero_checksum)
}

/// Returns the checksum value for an IPv6 upper-layer segment.
///
/// `segment_with_zero_checksum` must be the segment with its checksum
/// field cleared. Callers insert the returned value into that field.
#[must_use]
pub fn ipv6_segment_checksum(
    src: &Ipv6Address,
    dst: &Ipv6Address,
    next_header: u8,
    segment_with_zero_checksum: &[u8],
) -> u16 {
    let pseudo = ipv6_pseudo_header(src, dst, next_header, segment_with_zero_checksum.len());
    !folded_sum_two(&pseudo, segment_with_zero_checksum)
}

/// Returns whether an IPv4 upper-layer segment verifies.
///
/// `segment` must include the stored checksum field.
#[must_use]
pub fn verify_ipv4_segment_checksum(
    src: &Ipv4Address,
    dst: &Ipv4Address,
    protocol: u8,
    segment: &[u8],
) -> bool {
    let pseudo = ipv4_pseudo_header(*src, *dst, protocol, segment.len());
    folded_sum_two(&pseudo, segment) == 0xFFFF
}

/// Returns whether an IPv6 upper-layer segment verifies.
///
/// `segment` must include the stored checksum field.
#[must_use]
pub fn verify_ipv6_segment_checksum(
    src: &Ipv6Address,
    dst: &Ipv6Address,
    next_header: u8,
    segment: &[u8],
) -> bool {
    let pseudo = ipv6_pseudo_header(src, dst, next_header, segment.len());
    folded_sum_two(&pseudo, segment) == 0xFFFF
}

/// Copies six bytes at `offset` into a `MacAddress`.
fn read_mac(data: &[u8], offset: usize) -> Option<MacAddress> {
    let bytes = data.get(offset..offset.checked_add(6)?)?;
    let mut addr = [0_u8; 6];
    addr.copy_from_slice(bytes);
    Some(MacAddress(addr))
}

/// Copies four bytes at `offset` into an `Ipv4Address`.
fn read_ipv4_addr(data: &[u8], offset: usize) -> Option<Ipv4Address> {
    let bytes = data.get(offset..offset.checked_add(4)?)?;
    let mut addr = [0_u8; 4];
    addr.copy_from_slice(bytes);
    Some(Ipv4Address(addr))
}

/// Copies sixteen bytes at `offset` into an `Ipv6Address`.
fn read_ipv6_addr(data: &[u8], offset: usize) -> Option<Ipv6Address> {
    let bytes = data.get(offset..offset.checked_add(16)?)?;
    let mut addr = [0_u8; 16];
    addr.copy_from_slice(bytes);
    Some(Ipv6Address(addr))
}

// ---------------------------------------------------------------------------
// Ethernet
// ---------------------------------------------------------------------------

/// An 802.1Q VLAN tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VlanTag {
    /// The tag control information field.
    pub tci: u16,
    /// The 12-bit VLAN identifier.
    pub id: u16,
    /// The 3-bit priority code point.
    pub pcp: u8,
    /// The drop eligible indicator.
    pub dei: bool,
}

/// A parsed Ethernet II header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EthernetHeader<'a> {
    /// Destination MAC address.
    pub dst: MacAddress,
    /// Source MAC address.
    pub src: MacAddress,
    /// The (inner) `EtherType`, after a VLAN tag when present.
    pub ethertype: u16,
    /// The VLAN tag when the outer `EtherType` was `0x8100`.
    pub vlan: Option<VlanTag>,
    /// Header length in bytes including any VLAN tag.
    pub header_len: usize,
    /// Bytes after the header.
    pub payload: &'a [u8],
}

impl<'a> EthernetHeader<'a> {
    /// Parses an Ethernet header from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_header(data)
    }

    /// Returns the bytes after the header.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }

    /// Returns whether a VLAN tag was present.
    #[must_use]
    pub const fn is_vlan(&self) -> bool {
        self.vlan.is_some()
    }

    /// Returns whether the frame carries IPv4.
    #[must_use]
    pub const fn is_ipv4(&self) -> bool {
        self.ethertype == ETHERTYPE_IPV4
    }

    /// Returns whether the frame carries IPv6.
    #[must_use]
    pub const fn is_ipv6(&self) -> bool {
        self.ethertype == ETHERTYPE_IPV6
    }

    /// Returns whether the frame carries ARP.
    #[must_use]
    pub const fn is_arp(&self) -> bool {
        self.ethertype == ETHERTYPE_ARP
    }
}

/// Parses an Ethernet II header, supporting one 802.1Q VLAN tag.
///
/// Returns `None` when fewer than 14 bytes are present, when a VLAN
/// tag is declared but truncated, or when nested VLAN tags are seen.
#[must_use]
pub fn parse_header(data: &[u8]) -> Option<EthernetHeader<'_>> {
    if data.len() < ETHERNET_HEADER_LEN {
        return None;
    }
    let dst = read_mac(data, 0)?;
    let src = read_mac(data, 6)?;
    let outer = read_u16_be(data, 12)?;
    if outer != ETHERTYPE_VLAN {
        let payload = data.get(ETHERNET_HEADER_LEN..)?;
        return Some(EthernetHeader {
            dst,
            src,
            ethertype: outer,
            vlan: None,
            header_len: ETHERNET_HEADER_LEN,
            payload,
        });
    }
    if data.len() < ETHERNET_HEADER_LEN + ETHERNET_VLAN_TAG_LEN {
        return None;
    }
    let tci = read_u16_be(data, 14)?;
    let inner = read_u16_be(data, 16)?;
    if inner == ETHERTYPE_VLAN {
        // Stacked tags are outside the single-tag contract.
        return None;
    }
    let header_len = ETHERNET_HEADER_LEN + ETHERNET_VLAN_TAG_LEN;
    let payload = data.get(header_len..)?;
    let id = tci & 0x0FFF;
    let pcp = u8::try_from((tci >> 13) & 0x07).unwrap_or(0);
    let dei = (tci & 0x1000) != 0;
    Some(EthernetHeader {
        dst,
        src,
        ethertype: inner,
        vlan: Some(VlanTag { tci, id, pcp, dei }),
        header_len,
        payload,
    })
}

// ---------------------------------------------------------------------------
// ARP
// ---------------------------------------------------------------------------

/// A parsed Ethernet / IPv4 ARP packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArpPacket<'a> {
    /// Hardware type.
    pub hardware_type: u16,
    /// Protocol type.
    pub protocol_type: u16,
    /// Hardware address length.
    pub hardware_len: u8,
    /// Protocol address length.
    pub protocol_len: u8,
    /// Operation: 1 is request, 2 is reply.
    pub oper: u16,
    /// Sender hardware address.
    pub sender_mac: MacAddress,
    /// Sender protocol address.
    pub sender_ip: Ipv4Address,
    /// Target hardware address.
    pub target_mac: MacAddress,
    /// Target protocol address.
    pub target_ip: Ipv4Address,
    /// Fixed header length in bytes, always 28 here.
    pub header_len: usize,
    /// Bytes after the fixed 28-byte body, for example Ethernet padding.
    pub payload: &'a [u8],
}

impl<'a> ArpPacket<'a> {
    /// Parses an ARP packet from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_arp(data)
    }

    /// Returns whether the operation is a request.
    #[must_use]
    pub const fn is_request(&self) -> bool {
        self.oper == ARP_OPER_REQUEST
    }

    /// Returns whether the operation is a reply.
    #[must_use]
    pub const fn is_reply(&self) -> bool {
        self.oper == ARP_OPER_REPLY
    }

    /// Returns the bytes after the ARP body.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }
}

/// Parses an Ethernet / IPv4 ARP packet.
///
/// Only the fixed 28-byte Ethernet / IPv4 shape is accepted: hardware
/// type 1, protocol type `0x0800`, hardware length 6 and protocol
/// length 4. Anything else returns `None`.
#[must_use]
pub fn parse_arp(data: &[u8]) -> Option<ArpPacket<'_>> {
    if data.len() < ARP_FRAME_LEN {
        return None;
    }
    let hardware_type = read_u16_be(data, 0)?;
    let protocol_type = read_u16_be(data, 2)?;
    let hardware_len = *data.get(4)?;
    let protocol_len = *data.get(5)?;
    let oper = read_u16_be(data, 6)?;
    if hardware_type != ARP_HTYPE_ETHERNET
        || protocol_type != ARP_PTYPE_IPV4
        || hardware_len != 6
        || protocol_len != 4
    {
        return None;
    }
    let sender_mac = read_mac(data, 8)?;
    let sender_ip = read_ipv4_addr(data, 14)?;
    let target_mac = read_mac(data, 18)?;
    let target_ip = read_ipv4_addr(data, 24)?;
    let payload = data.get(ARP_FRAME_LEN..).unwrap_or(&[]);
    Some(ArpPacket {
        hardware_type,
        protocol_type,
        hardware_len,
        protocol_len,
        oper,
        sender_mac,
        sender_ip,
        target_mac,
        target_ip,
        header_len: ARP_FRAME_LEN,
        payload,
    })
}

// ---------------------------------------------------------------------------
// IPv4
// ---------------------------------------------------------------------------

/// A parsed IPv4 header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ipv4Header<'a> {
    /// Version, always 4 for a successful parse.
    pub version: u8,
    /// Internet header length field.
    pub ihl: u8,
    /// Header length in bytes.
    pub header_len: usize,
    /// Total datagram length declared in the header.
    pub total_len: u16,
    /// Identification field.
    pub identification: u16,
    /// Don't-fragment flag.
    pub dont_fragment: bool,
    /// More-fragments flag.
    pub more_fragments: bool,
    /// Fragment offset in 8-byte units.
    pub fragment_offset: u16,
    /// Time to live.
    pub ttl: u8,
    /// Upper-layer protocol number.
    pub protocol: u8,
    /// Stored header checksum.
    pub checksum: u16,
    /// Source address.
    pub src: Ipv4Address,
    /// Destination address.
    pub dst: Ipv4Address,
    /// Header bytes including options.
    pub raw_header: &'a [u8],
    /// Payload after the header, bounded by `total_len`.
    pub payload: &'a [u8],
}

impl<'a> Ipv4Header<'a> {
    /// Parses an IPv4 datagram from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_ipv4(data)
    }

    /// Returns whether the datagram is fragmented.
    #[must_use]
    pub const fn is_fragmented(&self) -> bool {
        self.more_fragments || self.fragment_offset != 0
    }

    /// Returns the fragment offset in bytes.
    #[must_use]
    pub fn fragment_offset_bytes(&self) -> u32 {
        u32::from(self.fragment_offset).saturating_mul(8)
    }

    /// Returns the payload after the header.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }

    /// Returns whether the header checksum verifies.
    #[must_use]
    pub fn validate_checksum(&self) -> bool {
        verify_checksum(self.raw_header)
    }
}

/// Parses an IPv4 datagram.
///
/// Enforces version 4, `IHL >= 5`, header length within the buffer and
/// `total_len` covering the header and fitting in the buffer. Trailing
/// bytes beyond `total_len`, such as Ethernet padding, are ignored.
#[must_use]
pub fn parse_ipv4(data: &[u8]) -> Option<Ipv4Header<'_>> {
    if data.len() < IPV4_MIN_HEADER_LEN {
        return None;
    }
    let first = *data.first()?;
    let version = first >> 4;
    let ihl = first & 0x0F;
    if version != 4 || ihl < 5 {
        return None;
    }
    let header_len = usize::from(ihl).checked_mul(4)?;
    if header_len > data.len() || header_len > 60 {
        return None;
    }
    let total_len = read_u16_be(data, 2)?;
    let total = usize::from(total_len);
    if total < header_len || total > data.len() {
        return None;
    }
    let identification = read_u16_be(data, 4)?;
    let flags_fragment = read_u16_be(data, 6)?;
    let flags = (flags_fragment >> 13) & 0x07;
    let dont_fragment = (flags & 0x02) != 0;
    let more_fragments = (flags & 0x01) != 0;
    let fragment_offset = flags_fragment & 0x1FFF;
    let ttl = *data.get(8)?;
    let protocol = *data.get(9)?;
    let checksum = read_u16_be(data, 10)?;
    let src = read_ipv4_addr(data, 12)?;
    let dst = read_ipv4_addr(data, 16)?;
    let raw_header = data.get(..header_len)?;
    let payload = data.get(header_len..total)?;
    Some(Ipv4Header {
        version,
        ihl,
        header_len,
        total_len,
        identification,
        dont_fragment,
        more_fragments,
        fragment_offset,
        ttl,
        protocol,
        checksum,
        src,
        dst,
        raw_header,
        payload,
    })
}

/// Returns whether the IPv4 datagram in `data` is fragmented.
#[must_use]
pub fn is_fragmented(data: &[u8]) -> bool {
    parse_ipv4(data).is_some_and(|header| header.is_fragmented())
}

/// Returns the IPv4 fragment offset in bytes for `data`, or `None` when
/// `data` does not contain a valid IPv4 header.
#[must_use]
pub fn fragment_offset_bytes(data: &[u8]) -> Option<u32> {
    parse_ipv4(data).map(|header| header.fragment_offset_bytes())
}

// ---------------------------------------------------------------------------
// IPv6
// ---------------------------------------------------------------------------

/// A parsed IPv6 header with extension headers skipped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ipv6Header<'a> {
    /// Version, always 6 for a successful parse.
    pub version: u8,
    /// Traffic class.
    pub traffic_class: u8,
    /// Flow label.
    pub flow_label: u32,
    /// Payload length in bytes after the 40-byte header.
    pub payload_len: u16,
    /// Original next-header field.
    pub next_header: u8,
    /// Next-header value after skipping extension headers.
    pub upper_next_header: u8,
    /// Hop limit.
    pub hop_limit: u8,
    /// Source address.
    pub src: Ipv6Address,
    /// Destination address.
    pub dst: Ipv6Address,
    /// Header length in bytes including all extension headers.
    pub header_len: usize,
    /// Whether a fragment header was present.
    pub has_fragment: bool,
    /// Fragment offset in 8-byte units, zero when unfragmented.
    pub fragment_offset: u16,
    /// More-fragments flag from the fragment header.
    pub more_fragments: bool,
    /// Fragment identification field.
    pub fragment_id: u32,
    /// Payload after all extension headers.
    pub payload: &'a [u8],
    /// The first 40 header bytes.
    pub raw_header: &'a [u8],
}

impl<'a> Ipv6Header<'a> {
    /// Parses an IPv6 datagram from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_ipv6(data)
    }

    /// Returns whether a fragment header was present or the offset or
    /// more-fragments flag is set.
    #[must_use]
    pub const fn is_fragmented(&self) -> bool {
        self.has_fragment || self.more_fragments || self.fragment_offset != 0
    }

    /// Returns the fragment offset in bytes.
    #[must_use]
    pub fn fragment_offset_bytes(&self) -> u32 {
        u32::from(self.fragment_offset).saturating_mul(8)
    }

    /// Returns the payload after all headers.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }
}

/// Parses an IPv6 datagram, skipping extension headers.
///
/// Handles hop-by-hop (0), routing (43), fragment (44) and destination
/// options (60). The traversal advances at least eight bytes per step,
/// is bounded by `payload_len`, and rejects truncated extension
/// headers. Unknown next-header values terminate the walk and become
/// the upper-layer protocol number.
#[must_use]
pub fn parse_ipv6(data: &[u8]) -> Option<Ipv6Header<'_>> {
    if data.len() < IPV6_HEADER_LEN {
        return None;
    }
    let first = read_u32_be(data, 0)?;
    let version = u8::try_from(first >> 28).unwrap_or(0);
    if version != 6 {
        return None;
    }
    let traffic_class = u8::try_from((first >> 20) & 0xFF).unwrap_or(0);
    let flow_label = first & 0x000F_FFFF;
    let payload_len = read_u16_be(data, 4)?;
    let total = usize::from(payload_len).checked_add(IPV6_HEADER_LEN)?;
    if total > data.len() {
        return None;
    }
    let next_header = *data.get(6)?;
    let hop_limit = *data.get(7)?;
    let src = read_ipv6_addr(data, 8)?;
    let dst = read_ipv6_addr(data, 24)?;
    let raw_header = data.get(..IPV6_HEADER_LEN)?;

    let mut offset = IPV6_HEADER_LEN;
    let mut upper = next_header;
    let mut has_fragment = false;
    let mut fragment_offset: u16 = 0;
    let mut more_fragments = false;
    let mut fragment_id: u32 = 0;
    // Each extension is at least 8 bytes; cap iterations defensively.
    let mut iterations = 0_u32;
    loop {
        iterations = iterations.checked_add(1)?;
        if iterations > 32 {
            return None;
        }
        match upper {
            IPV6_NEXT_HOP_BY_HOP | IPV6_NEXT_ROUTING | IPV6_NEXT_DEST_OPTS => {
                let next = *data.get(offset)?;
                let len_byte = *data.get(offset.checked_add(1)?)?;
                let ext_len = usize::from(len_byte).checked_add(1)?.checked_mul(8)?;
                let end = offset.checked_add(ext_len)?;
                if end > total || ext_len < 8 {
                    return None;
                }
                offset = end;
                upper = next;
            }
            IPV6_NEXT_FRAGMENT => {
                let next = *data.get(offset)?;
                // Reserved byte is skipped.
                let _reserved = *data.get(offset.checked_add(1)?)?;
                let offset_flags = read_u16_be(data, offset.checked_add(2)?)?;
                let ident = read_u32_be(data, offset.checked_add(4)?)?;
                fragment_offset = (offset_flags >> 3) & 0x1FFF;
                more_fragments = (offset_flags & 0x01) != 0;
                fragment_id = ident;
                has_fragment = true;
                let end = offset.checked_add(IPV6_FRAGMENT_HEADER_LEN)?;
                if end > total {
                    return None;
                }
                offset = end;
                upper = next;
                // Fragment payload follows; no further extensions are
                // interpreted after the fragment header.
                break;
            }
            _ => break,
        }
        if offset >= total {
            break;
        }
    }
    if offset > total {
        return None;
    }
    let payload = data.get(offset..total)?;
    Some(Ipv6Header {
        version,
        traffic_class,
        flow_label,
        payload_len,
        next_header,
        upper_next_header: upper,
        hop_limit,
        src,
        dst,
        header_len: offset,
        has_fragment,
        fragment_offset,
        more_fragments,
        fragment_id,
        payload,
        raw_header,
    })
}

// ---------------------------------------------------------------------------
// ICMPv4
// ---------------------------------------------------------------------------

/// A parsed `ICMPv4` message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Icmpv4Message<'a> {
    /// Message type.
    pub msg_type: u8,
    /// Message code.
    pub code: u8,
    /// Stored checksum.
    pub checksum: u16,
    /// Header length before the payload; `ICMPv4` has a 4-byte base header.
    pub header_len: usize,
    /// Payload after the 4-byte base header.
    pub payload: &'a [u8],
    /// The entire `ICMPv4` message as supplied.
    pub message: &'a [u8],
}

impl<'a> Icmpv4Message<'a> {
    /// Parses an `ICMPv4` message from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_icmpv4(data)
    }

    /// Returns the payload after the header.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }

    /// Returns whether the checksum over the message verifies.
    #[must_use]
    pub fn validate_checksum(&self) -> bool {
        verify_checksum(self.message)
    }
}

/// Parses an `ICMPv4` message.
///
/// The base header is four bytes; everything after it is the payload.
/// Shorter inputs are rejected. Checksum validation is available via
/// the returned value.
#[must_use]
pub fn parse_icmpv4(data: &[u8]) -> Option<Icmpv4Message<'_>> {
    if data.len() < 4 {
        return None;
    }
    let msg_type = *data.first()?;
    let code = *data.get(1)?;
    let checksum = read_u16_be(data, 2)?;
    let payload = data.get(4..)?;
    Some(Icmpv4Message {
        msg_type,
        code,
        checksum,
        header_len: 4,
        payload,
        message: data,
    })
}

// ---------------------------------------------------------------------------
// ICMPv6
// ---------------------------------------------------------------------------

/// A parsed `ICMPv6` message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Icmpv6Message<'a> {
    /// Message type.
    pub msg_type: u8,
    /// Message code.
    pub code: u8,
    /// Stored checksum.
    pub checksum: u16,
    /// Header length before the payload; `ICMPv6` has a 4-byte base header.
    pub header_len: usize,
    /// Payload after the 4-byte base header.
    pub payload: &'a [u8],
    /// The entire `ICMPv6` message as supplied.
    pub message: &'a [u8],
}

impl<'a> Icmpv6Message<'a> {
    /// Parses an `ICMPv6` message from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_icmpv6(data)
    }

    /// Returns the payload after the header.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }

    /// Returns whether the checksum with the IPv6 pseudo-header verifies.
    #[must_use]
    pub fn validate_checksum(&self, src: &Ipv6Address, dst: &Ipv6Address) -> bool {
        verify_ipv6_segment_checksum(src, dst, IPV6_NEXT_ICMP, self.message)
    }
}

/// Parses an `ICMPv6` message.
///
/// The base header is four bytes; everything after it is the payload.
#[must_use]
pub fn parse_icmpv6(data: &[u8]) -> Option<Icmpv6Message<'_>> {
    if data.len() < 4 {
        return None;
    }
    let msg_type = *data.first()?;
    let code = *data.get(1)?;
    let checksum = read_u16_be(data, 2)?;
    let payload = data.get(4..)?;
    Some(Icmpv6Message {
        msg_type,
        code,
        checksum,
        header_len: 4,
        payload,
        message: data,
    })
}

// ---------------------------------------------------------------------------
// UDP
// ---------------------------------------------------------------------------

/// A parsed UDP datagram.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UdpHeader<'a> {
    /// Source port.
    pub src_port: u16,
    /// Destination port.
    pub dst_port: u16,
    /// Datagram length including the header.
    pub length: u16,
    /// Stored checksum.
    pub checksum: u16,
    /// Header length in bytes, always 8 here.
    pub header_len: usize,
    /// Payload after the header.
    pub payload: &'a [u8],
    /// The UDP datagram bounded by `length`.
    pub datagram: &'a [u8],
}

impl<'a> UdpHeader<'a> {
    /// Parses a UDP datagram from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_udp(data)
    }

    /// Returns the payload after the header.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }

    /// Returns whether a checksum was transmitted.
    #[must_use]
    pub const fn has_checksum(&self) -> bool {
        self.checksum != 0
    }

    /// Validates the checksum over IPv4.
    ///
    /// A zero checksum means no checksum was transmitted and counts
    /// as valid for IPv4.
    #[must_use]
    pub fn validate_ipv4_checksum(&self, src: &Ipv4Address, dst: &Ipv4Address) -> bool {
        if self.checksum == 0 {
            return true;
        }
        verify_ipv4_segment_checksum(src, dst, IP_PROTOCOL_UDP, self.datagram)
    }

    /// Validates the checksum over IPv6.
    ///
    /// IPv6 requires a checksum, so zero is invalid.
    #[must_use]
    pub fn validate_ipv6_checksum(&self, src: &Ipv6Address, dst: &Ipv6Address) -> bool {
        if self.checksum == 0 {
            return false;
        }
        verify_ipv6_segment_checksum(src, dst, IP_PROTOCOL_UDP, self.datagram)
    }
}

/// Parses a UDP datagram.
///
/// Enforces a minimum length of 8, a declared length that covers the
/// header and fits in the buffer. Bytes beyond the declared length are
/// ignored, allowing Ethernet padding to be present.
#[must_use]
pub fn parse_udp(data: &[u8]) -> Option<UdpHeader<'_>> {
    if data.len() < UDP_HEADER_LEN {
        return None;
    }
    let src_port = read_u16_be(data, 0)?;
    let dst_port = read_u16_be(data, 2)?;
    let length = read_u16_be(data, 4)?;
    let checksum = read_u16_be(data, 6)?;
    let len = usize::from(length);
    if len < UDP_HEADER_LEN || len > data.len() {
        return None;
    }
    let datagram = data.get(..len)?;
    let payload = datagram.get(UDP_HEADER_LEN..)?;
    Some(UdpHeader {
        src_port,
        dst_port,
        length,
        checksum,
        header_len: UDP_HEADER_LEN,
        payload,
        datagram,
    })
}

// ---------------------------------------------------------------------------
// TCP
// ---------------------------------------------------------------------------

/// A parsed TCP segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcpHeader<'a> {
    /// Source port.
    pub src_port: u16,
    /// Destination port.
    pub dst_port: u16,
    /// Sequence number.
    pub seq: u32,
    /// Acknowledgment number.
    pub ack: u32,
    /// Data offset field in 4-byte units.
    pub data_offset: u8,
    /// Header length in bytes.
    pub header_len: usize,
    /// Flag bits.
    pub flags: u8,
    /// Window size.
    pub window: u16,
    /// Stored checksum.
    pub checksum: u16,
    /// Urgent pointer.
    pub urgent_pointer: u16,
    /// Payload after the header, including options in the header length.
    pub payload: &'a [u8],
    /// The TCP segment as supplied.
    pub segment: &'a [u8],
}

impl<'a> TcpHeader<'a> {
    /// Parses a TCP segment from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_tcp(data)
    }

    /// Returns the payload after the header.
    #[must_use]
    pub const fn payload_after_header(&self) -> &'a [u8] {
        self.payload
    }

    /// Returns whether the SYN flag is set.
    #[must_use]
    pub const fn is_syn(&self) -> bool {
        (self.flags & TCP_FLAG_SYN) != 0
    }

    /// Returns whether the ACK flag is set.
    #[must_use]
    pub const fn is_ack(&self) -> bool {
        (self.flags & TCP_FLAG_ACK) != 0
    }

    /// Returns whether the FIN flag is set.
    #[must_use]
    pub const fn is_fin(&self) -> bool {
        (self.flags & TCP_FLAG_FIN) != 0
    }

    /// Returns whether the RST flag is set.
    #[must_use]
    pub const fn is_rst(&self) -> bool {
        (self.flags & TCP_FLAG_RST) != 0
    }

    /// Returns whether the urgent flag is set.
    #[must_use]
    pub const fn is_urg(&self) -> bool {
        (self.flags & TCP_FLAG_URG) != 0
    }

    /// Returns whether the push flag is set.
    #[must_use]
    pub const fn is_psh(&self) -> bool {
        (self.flags & TCP_FLAG_PSH) != 0
    }

    /// Validates the checksum over IPv4.
    #[must_use]
    pub fn validate_ipv4_checksum(&self, src: &Ipv4Address, dst: &Ipv4Address) -> bool {
        verify_ipv4_segment_checksum(src, dst, IP_PROTOCOL_TCP, self.segment)
    }

    /// Validates the checksum over IPv6.
    #[must_use]
    pub fn validate_ipv6_checksum(&self, src: &Ipv6Address, dst: &Ipv6Address) -> bool {
        verify_ipv6_segment_checksum(src, dst, IP_PROTOCOL_TCP, self.segment)
    }
}

/// Parses a TCP segment.
///
/// Enforces at least 20 bytes, a data offset of at least five words,
/// and a header length within the supplied buffer. Trailing bytes are
/// the payload.
#[must_use]
pub fn parse_tcp(data: &[u8]) -> Option<TcpHeader<'_>> {
    if data.len() < TCP_MIN_HEADER_LEN {
        return None;
    }
    let src_port = read_u16_be(data, 0)?;
    let dst_port = read_u16_be(data, 2)?;
    let seq = read_u32_be(data, 4)?;
    let ack = read_u32_be(data, 8)?;
    let offset_byte = *data.get(12)?;
    let data_offset = offset_byte >> 4;
    if data_offset < 5 {
        return None;
    }
    let header_len = usize::from(data_offset).checked_mul(4)?;
    if header_len > data.len() || header_len > 60 {
        return None;
    }
    let flags = *data.get(13)?;
    let window = read_u16_be(data, 14)?;
    let checksum = read_u16_be(data, 16)?;
    let urgent_pointer = read_u16_be(data, 18)?;
    let payload = data.get(header_len..)?;
    let segment = data;
    Some(TcpHeader {
        src_port,
        dst_port,
        seq,
        ack,
        data_offset,
        header_len,
        flags,
        window,
        checksum,
        urgent_pointer,
        payload,
        segment,
    })
}
