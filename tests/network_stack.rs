//! Behavioural tests for the hardware-independent network stack.

use rover_uefi::protocols::network::packets;
use rover_uefi::protocols::network::stack::arp;
use rover_uefi::protocols::network::stack::dhcp;
use rover_uefi::protocols::network::stack::dns_client;
use rover_uefi::protocols::network::stack::ethernet;
use rover_uefi::protocols::network::stack::ipv4;
use rover_uefi::protocols::network::stack::ipv6;
use rover_uefi::protocols::network::stack::route;
use rover_uefi::protocols::network::stack::tcp;
use rover_uefi::protocols::network::stack::udp;
use rover_uefi::protocols::network::stack::{IpAddress, StackError};
use rover_uefi::protocols::network::types::{Ipv4Address, Ipv6Address, MacAddress};

// ---------------------------------------------------------------------------
// Independent checksum arithmetic
// ---------------------------------------------------------------------------

fn checksum_sum(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let (chunks, remainder) = data.as_chunks::<2>();
    for chunk in chunks {
        sum += u32::from(u16::from_be_bytes([chunk[0], chunk[1]]));
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    if !remainder.is_empty() {
        sum += u32::from(u16::from_be_bytes([remainder[0], 0]));
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    if sum > 0xFFFF {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    sum
}

fn folded_sum(first: &[u8], second: &[u8]) -> u16 {
    let sum = checksum_sum(first) + checksum_sum(second);
    let folded = (sum & 0xFFFF) + (sum >> 16);
    let folded = (folded & 0xFFFF) + (folded >> 16);
    u16::try_from(folded).unwrap_or(0xFFFF)
}

fn ipv4_pseudo(src: [u8; 4], dst: [u8; 4], protocol: u8, len: usize) -> [u8; 12] {
    let len_u16 = u16::try_from(len).unwrap_or(u16::MAX);
    let mut pseudo = [0_u8; 12];
    pseudo[0..4].copy_from_slice(&src);
    pseudo[4..8].copy_from_slice(&dst);
    pseudo[9] = protocol;
    pseudo[10..12].copy_from_slice(&len_u16.to_be_bytes());
    pseudo
}

fn ipv6_pseudo(src: &[u8; 16], dst: &[u8; 16], next: u8, len: usize) -> [u8; 40] {
    let len_u32 = u32::try_from(len).unwrap_or(u32::MAX);
    let mut pseudo = [0_u8; 40];
    pseudo[0..16].copy_from_slice(src);
    pseudo[16..32].copy_from_slice(dst);
    pseudo[32..36].copy_from_slice(&len_u32.to_be_bytes());
    pseudo[39] = next;
    pseudo
}

fn fragment_slice(datagrams: &[[u8; 64]], index: usize) -> &[u8] {
    datagrams
        .get(index)
        .map_or(&[], |datagram| datagram.as_slice())
}

// ---------------------------------------------------------------------------
// Ethernet
// ---------------------------------------------------------------------------

#[test]
fn ethernet_frame_round_trip() {
    let dst = MacAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    let src = MacAddress([0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB]);
    let payload = [0xDE, 0xAD, 0xBE, 0xEF];
    let mut frame = [0_u8; 64];
    let written =
        ethernet::build_frame(&mut frame, dst, src, packets::ETHERTYPE_IPV4, &payload).unwrap_or(0);
    assert_eq!(written, 18);
    let parsed = ethernet::parse_frame(&frame[..written]);
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        assert_eq!(header.dst, dst);
        assert_eq!(header.src, src);
        assert_eq!(header.ethertype, packets::ETHERTYPE_IPV4);
        assert!(header.vlan.is_none());
        assert_eq!(header.payload, &payload);
        assert!(ethernet::is_ipv4(header.ethertype));
        assert!(!ethernet::is_arp(header.ethertype));
    }
}

#[test]
fn ethernet_vlan_tag_is_encoded_and_parsed() {
    let dst = MacAddress([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    let src = MacAddress([0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
    let tag = packets::VlanTag {
        tci: ethernet::vlan_tci(42, 5, true),
        id: 42,
        pcp: 5,
        dei: true,
    };
    let payload = [0x01, 0x02];
    let mut frame = [0_u8; 64];
    let written = ethernet::build_frame_vlan(
        &mut frame,
        dst,
        src,
        Some(tag),
        packets::ETHERTYPE_IPV6,
        &payload,
    )
    .unwrap_or(0);
    assert_eq!(written, 20);
    assert_eq!(frame.get(12..14).unwrap_or(&[]), &[0x81, 0x00]);
    assert_eq!(frame.get(16..18).unwrap_or(&[]), &[0x86, 0xDD]);
    let parsed = ethernet::parse_frame(&frame[..written]);
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        assert!(header.is_vlan());
        assert_eq!(header.ethertype, packets::ETHERTYPE_IPV6);
        assert_eq!(header.payload, &payload);
        assert!(ethernet::is_ipv6(header.ethertype));
        if let Some(parsed_tag) = header.vlan {
            assert_eq!(parsed_tag.id, 42);
            assert_eq!(parsed_tag.pcp, 5);
            assert!(parsed_tag.dei);
        }
    }
}

#[test]
fn ethernet_rejects_a_truncated_frame() {
    assert!(ethernet::parse_frame(&[0_u8; 10]).is_none());
    let mut frame = [0_u8; 64];
    let written = ethernet::build_frame(
        &mut frame,
        MacAddress([0; 6]),
        MacAddress([0; 6]),
        packets::ETHERTYPE_ARP,
        &[0; 8],
    )
    .unwrap_or(0);
    assert!(ethernet::parse_frame(frame.get(..10).unwrap_or(&[])).is_none());
    // A frame with a complete header but a short payload still parses.
    assert!(ethernet::parse_frame(frame.get(..written).unwrap_or(&[])).is_some());
}

// ---------------------------------------------------------------------------
// ARP
// ---------------------------------------------------------------------------

#[test]
fn arp_request_and_reply_are_built_correctly() {
    let sender_mac = MacAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let sender_ip = Ipv4Address([192, 168, 1, 10]);
    let target_ip = Ipv4Address([192, 168, 1, 1]);
    let mut buffer = [0_u8; 32];
    let written = arp::build_request(&mut buffer, sender_mac, sender_ip, target_ip).unwrap_or(0);
    assert_eq!(written, 28);
    let parsed = arp::parse_arp(&buffer[..written]);
    assert!(parsed.is_some());
    if let Some(request) = parsed {
        assert!(request.is_request());
        assert!(!request.is_reply());
        assert_eq!(request.sender_mac, sender_mac);
        assert_eq!(request.sender_ip, sender_ip);
        assert_eq!(request.target_ip, target_ip);
    }

    let target_mac = MacAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]);
    let written =
        arp::build_reply(&mut buffer, target_mac, target_ip, sender_mac, sender_ip).unwrap_or(0);
    let parsed = arp::parse_arp(&buffer[..written]);
    assert!(parsed.is_some());
    if let Some(reply) = parsed {
        assert!(reply.is_reply());
        assert_eq!(reply.sender_mac, target_mac);
        assert_eq!(reply.sender_ip, target_ip);
        assert_eq!(reply.target_mac, sender_mac);
        assert_eq!(reply.target_ip, sender_ip);
    }
}

#[test]
fn arp_cache_insert_lookup_and_age_out() {
    let mut cache = arp::ArpCache::new();
    assert!(cache.is_empty());
    let first_ip = Ipv4Address([10, 0, 0, 1]);
    let first_mac = MacAddress([0x02, 0, 0, 0, 0, 1]);
    assert!(cache.insert(first_ip, first_mac, 1_000));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.lookup(&first_ip), Some(first_mac));

    let second_ip = Ipv4Address([10, 0, 0, 2]);
    let second_mac = MacAddress([0x02, 0, 0, 0, 0, 2]);
    assert!(cache.insert(second_ip, second_mac, 2_000));
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.lookup(&second_ip), Some(second_mac));

    // A repeated address updates the entry in place.
    let updated_mac = MacAddress([0x02, 0, 0, 0, 0, 9]);
    assert!(cache.insert(first_ip, updated_mac, 3_000));
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.lookup(&first_ip), Some(updated_mac));

    // The second entry is the older one and expires first.
    let removed = cache.age_out(3_000, 500);
    assert_eq!(removed, 1);
    assert_eq!(cache.len(), 1);
    assert!(cache.lookup(&second_ip).is_none());
    assert_eq!(cache.lookup(&first_ip), Some(updated_mac));

    // The remaining entry expires later.
    assert_eq!(cache.age_out(10_000, 500), 1);
    assert!(cache.is_empty());
}

#[test]
fn arp_cache_is_bounded() {
    let mut cache = arp::ArpCache::new();
    for index in 0..arp::ARP_CACHE_CAPACITY {
        let last = u8::try_from(index).unwrap_or(0);
        let ip = Ipv4Address([10, 0, 1, last]);
        let mac = MacAddress([0x02, 0, 0, 0, 0, last]);
        assert!(cache.insert(ip, mac, u64::try_from(index).unwrap_or(0)));
    }
    assert_eq!(cache.len(), arp::ARP_CACHE_CAPACITY);
    let overflow_ip = Ipv4Address([10, 9, 9, 9]);
    let overflow_mac = MacAddress([0x02, 0, 0, 0, 0, 9]);
    assert!(cache.insert(overflow_ip, overflow_mac, 9_999));
    assert_eq!(cache.len(), arp::ARP_CACHE_CAPACITY);
    assert_eq!(cache.lookup(&overflow_ip), Some(overflow_mac));
}

// ---------------------------------------------------------------------------
// IPv4
// ---------------------------------------------------------------------------

fn ipv4_params() -> ipv4::Ipv4Params {
    ipv4::Ipv4Params {
        src: Ipv4Address([192, 168, 1, 10]),
        dst: Ipv4Address([192, 168, 1, 1]),
        protocol: packets::IP_PROTOCOL_UDP,
        ttl: 64,
        identification: 0x1234_u16,
        dscp: 0,
        ecn: 0,
        dont_fragment: false,
        more_fragments: false,
        fragment_offset: 0,
    }
}

#[test]
fn ipv4_header_checksum_matches_an_independent_computation() {
    let params = ipv4_params();
    let payload = [0x11_u8; 16];
    let mut datagram = [0_u8; 64];
    let written = ipv4::build_datagram(&mut datagram, &params, &payload).unwrap_or(0);
    assert_eq!(written, 36);

    let header = datagram.get(..20).unwrap_or(&[]);
    let stored = u16::from_be_bytes([header[10], header[11]]);
    let mut zeroed = [0_u8; 20];
    zeroed.copy_from_slice(header);
    zeroed[10] = 0;
    zeroed[11] = 0;
    let complement = !folded_sum(&zeroed, &[]);
    assert_eq!(stored, complement);
    assert!(ipv4::validate_checksum(header));

    // The parser sees the same values.
    let parsed = packets::parse_ipv4(&datagram[..written]);
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        assert_eq!(header.version, 4);
        assert_eq!(header.ihl, 5);
        assert_eq!(header.total_len, 36);
        assert_eq!(header.identification, params.identification);
        assert_eq!(header.ttl, 64);
        assert_eq!(header.protocol, packets::IP_PROTOCOL_UDP);
        assert_eq!(header.src, params.src);
        assert_eq!(header.dst, params.dst);
        assert_eq!(header.payload, &payload);
        assert!(header.validate_checksum());
    }
}

#[test]
fn ipv4_fragments_are_reassembled_out_of_order() {
    let params = ipv4_params();
    let payload = [0x5A_u8; 100];
    let mut layouts = [ipv4::FragmentLayout {
        offset_units: 0,
        byte_offset: 0,
        length: 0,
        more_fragments: false,
    }; 8];
    let count = ipv4::layout_fragments(payload.len(), 32, &mut layouts).unwrap_or(0);
    assert_eq!(count, 4);
    let first = layouts.first().copied().unwrap_or(layouts[0]);
    assert_eq!(first.byte_offset, 0);
    assert_eq!(first.length, 32);
    assert!(first.more_fragments);
    let last = layouts.get(3).copied().unwrap_or(first);
    assert_eq!(last.byte_offset, 96);
    assert_eq!(last.length, 4);
    assert!(!last.more_fragments);
    let second = layouts.get(1).copied().unwrap_or(first);
    assert_eq!(second.offset_units, 4);
    assert_eq!(last.offset_units, 12);

    let mut datagrams = [[0_u8; 64]; 4];
    for (datagram, layout) in datagrams.iter_mut().zip(layouts.iter()).take(count) {
        let written = ipv4::write_fragment(datagram, &params, &payload, layout).unwrap_or(0);
        assert!(written > 0);
        let parsed = packets::parse_ipv4(datagram.get(..written).unwrap_or(&[]));
        assert!(parsed.is_some());
        if let Some(header) = parsed {
            assert!(header.is_fragmented());
            assert_eq!(header.identification, params.identification);
            assert_eq!(header.fragment_offset, layout.offset_units);
            assert_eq!(header.more_fragments, layout.more_fragments);
        }
    }

    let mut reassembler = ipv4::Ipv4Reassembler::new();
    let status = reassembler
        .push(fragment_slice(&datagrams, 1))
        .unwrap_or(ipv4::ReassemblyStatus::Rejected);
    assert_eq!(status, ipv4::ReassemblyStatus::Incomplete);
    let status = reassembler
        .push(fragment_slice(&datagrams, 3))
        .unwrap_or(ipv4::ReassemblyStatus::Rejected);
    assert_eq!(status, ipv4::ReassemblyStatus::Incomplete);
    let status = reassembler
        .push(fragment_slice(&datagrams, 0))
        .unwrap_or(ipv4::ReassemblyStatus::Rejected);
    assert_eq!(status, ipv4::ReassemblyStatus::Incomplete);
    let status = reassembler
        .push(fragment_slice(&datagrams, 2))
        .unwrap_or(ipv4::ReassemblyStatus::Rejected);
    assert_eq!(status, ipv4::ReassemblyStatus::Complete);
    assert_eq!(reassembler.payload().unwrap_or(&[]), &payload);
}

#[test]
fn ipv4_rejects_an_overlapping_fragment() {
    let params = ipv4_params();
    let payload = [0x5A_u8; 100];
    let layout = ipv4::FragmentLayout {
        offset_units: 0,
        byte_offset: 0,
        length: 32,
        more_fragments: true,
    };
    let mut datagram = [0_u8; 64];
    let written = ipv4::write_fragment(&mut datagram, &params, &payload, &layout).unwrap_or(0);
    let overlapping = ipv4::FragmentLayout {
        offset_units: 2,
        byte_offset: 16,
        length: 32,
        more_fragments: true,
    };
    let mut second = [0_u8; 64];
    let second_written =
        ipv4::write_fragment(&mut second, &params, &payload, &overlapping).unwrap_or(0);

    let mut reassembler = ipv4::Ipv4Reassembler::new();
    let status = reassembler
        .push(datagram.get(..written).unwrap_or(&[]))
        .unwrap_or(ipv4::ReassemblyStatus::Rejected);
    assert_eq!(status, ipv4::ReassemblyStatus::Incomplete);
    let status = reassembler
        .push(second.get(..second_written).unwrap_or(&[]))
        .unwrap_or(ipv4::ReassemblyStatus::Rejected);
    assert_eq!(status, ipv4::ReassemblyStatus::Rejected);
}

#[test]
fn ipv4_fragmentation_rejects_an_invalid_unit() {
    let mut layouts = [ipv4::FragmentLayout {
        offset_units: 0,
        byte_offset: 0,
        length: 0,
        more_fragments: false,
    }; 2];
    assert_eq!(
        ipv4::layout_fragments(10, 7, &mut layouts),
        Err(StackError::InvalidState)
    );
    assert_eq!(
        ipv4::layout_fragments(10, 0, &mut layouts),
        Err(StackError::InvalidState)
    );
}

// ---------------------------------------------------------------------------
// IPv6
// ---------------------------------------------------------------------------

#[test]
fn ipv6_header_with_a_hop_by_hop_extension_header() {
    let params = ipv6::Ipv6Params {
        src: Ipv6Address([0x20; 16]),
        dst: Ipv6Address([0x30; 16]),
        next_header: packets::IPV6_NEXT_HOP_BY_HOP,
        hop_limit: 42,
        traffic_class: 0x28,
        flow_label: 0x12345,
    };
    let mut extension = [0_u8; 32];
    let option_bytes = [1_u8, 4, 0, 0, 0, 0];
    let extension_len =
        ipv6::build_extension_header(&mut extension, packets::IP_PROTOCOL_TCP, &option_bytes)
            .unwrap_or(0);
    assert_eq!(extension_len, 8);
    assert_eq!(
        extension.first().copied().unwrap_or(0),
        packets::IP_PROTOCOL_TCP
    );
    assert_eq!(extension.get(1).copied().unwrap_or(0), 0);

    let mut payload = [0_u8; 32];
    payload
        .get_mut(..extension_len)
        .unwrap_or(&mut [])
        .copy_from_slice(extension.get(..extension_len).unwrap_or(&[]));
    payload
        .get_mut(extension_len..extension_len + 4)
        .unwrap_or(&mut [])
        .copy_from_slice(&[1, 2, 3, 4]);

    let mut datagram = [0_u8; 64];
    let written = ipv6::build_datagram(
        &mut datagram,
        &params,
        payload.get(..extension_len + 4).unwrap_or(&[]),
    )
    .unwrap_or(0);
    assert_eq!(written, 52);

    let parsed = packets::parse_ipv6(datagram.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        assert_eq!(header.version, 6);
        assert_eq!(header.traffic_class, 0x28);
        assert_eq!(header.flow_label, 0x12345);
        assert_eq!(header.hop_limit, 42);
        assert_eq!(header.src, params.src);
        assert_eq!(header.dst, params.dst);
        assert_eq!(header.next_header, packets::IPV6_NEXT_HOP_BY_HOP);
        assert_eq!(header.upper_next_header, packets::IP_PROTOCOL_TCP);
        assert_eq!(header.header_len, 48);
        assert_eq!(header.payload, &[1, 2, 3, 4]);
    }

    let chain = ipv6::walk_extension_headers(
        datagram.get(..written).unwrap_or(&[]),
        40,
        packets::IPV6_NEXT_HOP_BY_HOP,
    );
    assert!(chain.is_some());
    if let Some(chain) = chain {
        assert_eq!(chain.upper_layer, packets::IP_PROTOCOL_TCP);
        assert_eq!(chain.payload_offset, 48);
        assert_eq!(chain.header_count, 1);
    }
}

// ---------------------------------------------------------------------------
// UDP
// ---------------------------------------------------------------------------

#[test]
fn udp_checksum_is_correct_for_ipv4() {
    let params = udp::UdpParams {
        src_port: 1234,
        dst_port: 53,
    };
    let src = Ipv4Address([192, 168, 1, 10]);
    let dst = Ipv4Address([192, 168, 1, 1]);
    let payload = [0x42_u8; 20];
    let mut datagram = [0_u8; 64];
    let written =
        udp::build_datagram_ipv4(&mut datagram, &params, &payload, &src, &dst).unwrap_or(0);
    assert_eq!(written, 28);

    let pseudo = ipv4_pseudo(src.0, dst.0, packets::IP_PROTOCOL_UDP, written);
    assert_eq!(
        folded_sum(&pseudo, datagram.get(..written).unwrap_or(&[])),
        0xFFFF
    );

    let parsed = udp::parse_datagram(datagram.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        assert_eq!(header.src_port, 1234);
        assert_eq!(header.dst_port, 53);
        assert_eq!(header.length, 28);
        assert_eq!(header.payload, &payload);
        assert!(header.has_checksum());
    }
    assert!(udp::verify_ipv4_checksum(
        datagram.get(..written).unwrap_or(&[]),
        &src,
        &dst
    ));
}

#[test]
fn udp_checksum_is_correct_for_ipv6() {
    let params = udp::UdpParams {
        src_port: 53,
        dst_port: 5353,
    };
    let src = Ipv6Address([0x20; 16]);
    let dst = Ipv6Address([0x30; 16]);
    let payload = [0x24_u8; 8];
    let mut datagram = [0_u8; 64];
    let written =
        udp::build_datagram_ipv6(&mut datagram, &params, &payload, &src, &dst).unwrap_or(0);
    assert_eq!(written, 16);

    let pseudo = ipv6_pseudo(&src.0, &dst.0, packets::IP_PROTOCOL_UDP, written);
    assert_eq!(
        folded_sum(&pseudo, datagram.get(..written).unwrap_or(&[])),
        0xFFFF
    );
    assert!(udp::verify_ipv6_checksum(
        datagram.get(..written).unwrap_or(&[]),
        &src,
        &dst
    ));
    assert!(!udp::verify_ipv6_checksum(
        datagram.get(..written - 1).unwrap_or(&[]),
        &src,
        &dst
    ));
}

// ---------------------------------------------------------------------------
// TCP
// ---------------------------------------------------------------------------

const TCP_SYN: u8 = packets::TCP_FLAG_SYN;
const TCP_ACK: u8 = packets::TCP_FLAG_ACK;
const TCP_FIN: u8 = packets::TCP_FLAG_FIN;
const TCP_RST: u8 = packets::TCP_FLAG_RST;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SegmentInfo {
    flags: u8,
    seq: u32,
    ack: u32,
    window: u16,
    payload_len: usize,
    options_len: usize,
    payload_head: [u8; 16],
}

fn segment_info(segment: Option<tcp::TcpSegment<'_>>) -> SegmentInfo {
    let mut info = SegmentInfo {
        flags: 0,
        seq: 0,
        ack: 0,
        window: 0,
        payload_len: 0,
        options_len: 0,
        payload_head: [0; 16],
    };
    if let Some(segment) = segment {
        info.flags = segment.flags.bits();
        info.seq = segment.seq;
        info.ack = segment.ack;
        info.window = segment.window;
        info.payload_len = segment.payload.len();
        info.options_len = segment.options.len();
        let head = segment.payload.get(..16).unwrap_or(&[]);
        if let Some(target) = info.payload_head.get_mut(..head.len()) {
            target.copy_from_slice(head);
        }
    }
    info
}

fn peer_segment(
    buffer: &mut [u8],
    seq: u32,
    ack: u32,
    flags: u8,
    window: u16,
    options: &[u8],
    payload: &[u8],
) -> usize {
    let segment = tcp::TcpSegment {
        flags: tcp::TcpFlags(flags),
        seq,
        ack,
        window,
        urgent_pointer: 0,
        payload,
        options,
    };
    let src = IpAddress::V4(Ipv4Address([192, 168, 1, 1]));
    let dst = IpAddress::V4(Ipv4Address([192, 168, 1, 10]));
    tcp::write_segment(buffer, 80, 49_152, &segment, &src, &dst).unwrap_or(0)
}

fn client_connection() -> tcp::TcpConnection {
    let local = tcp::Endpoint {
        address: IpAddress::V4(Ipv4Address([192, 168, 1, 10])),
        port: 49_152,
    };
    let remote = tcp::Endpoint {
        address: IpAddress::V4(Ipv4Address([192, 168, 1, 1])),
        port: 80,
    };
    tcp::TcpConnection::new_client(local, remote, 1_000, tcp::TcpConfig::default())
}

fn complete_handshake(connection: &mut tcp::TcpConnection, peer_mss: u16) {
    let mut options = [0_u8; 4];
    let option_len = tcp::write_mss(&mut options, peer_mss).unwrap_or(0);
    let mut buffer = [0_u8; 128];
    let written = peer_segment(
        &mut buffer,
        5_000,
        1_001,
        TCP_SYN | TCP_ACK,
        65_535,
        options.get(..option_len).unwrap_or(&[]),
        &[],
    );
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        connection.on_segment(&header, 10);
    }
}

#[test]
fn tcp_handshake_reaches_established() {
    let mut connection = client_connection();
    assert_eq!(connection.state(), tcp::TcpState::Closed);
    assert!(connection.open());
    assert_eq!(connection.state(), tcp::TcpState::SynSent);

    let syn = segment_info(connection.poll(0));
    assert_eq!(syn.flags, TCP_SYN);
    assert_eq!(syn.seq, 1_000);
    assert_eq!(syn.ack, 0);
    assert_eq!(syn.options_len, 4);

    complete_handshake(&mut connection, 1_000);
    assert_eq!(connection.state(), tcp::TcpState::SynReceived);
    assert_eq!(connection.initial_receive_sequence(), 5_000);
    assert_eq!(connection.receive_next(), 5_001);
    assert_eq!(connection.send_unacknowledged(), 1_001);

    let ack = segment_info(connection.poll(10));
    assert_eq!(ack.flags, TCP_ACK);
    assert_eq!(ack.seq, 1_001);
    assert_eq!(ack.ack, 5_001);
    assert_eq!(connection.state(), tcp::TcpState::Established);
}

#[test]
fn tcp_data_is_segmented_by_the_maximum_segment_size() {
    let mut connection = client_connection();
    connection.open();
    let _ = segment_info(connection.poll(0));
    complete_handshake(&mut connection, 1_000);
    let _ = segment_info(connection.poll(10));

    let payload = [0xA5_u8; 2_500];
    assert_eq!(connection.send_data(&payload), 2_500);
    assert_eq!(connection.queued_data(), 2_500);

    let first = segment_info(connection.poll(20));
    assert_eq!(first.payload_len, 1_000);
    assert_eq!(first.seq, 1_001);
    assert_eq!(first.payload_head.first().copied().unwrap_or(0), 0xA5);

    let second = segment_info(connection.poll(20));
    assert_eq!(second.payload_len, 1_000);
    assert_eq!(second.seq, 2_001);

    let third = segment_info(connection.poll(20));
    assert_eq!(third.payload_len, 500);
    assert_eq!(third.seq, 3_001);
    assert_eq!(connection.queued_data(), 0);
    assert_eq!(connection.send_next(), 3_501);
}

#[test]
fn tcp_data_is_received_and_the_window_is_updated() {
    let mut connection = client_connection();
    connection.open();
    let _ = segment_info(connection.poll(0));
    complete_handshake(&mut connection, 1_000);
    let _ = segment_info(connection.poll(10));

    let window_before = connection.advertised_window();
    let expected = connection.receive_next();
    let payload = [0x77_u8; 100];
    let mut buffer = [0_u8; 256];
    let written = peer_segment(&mut buffer, expected, 1_001, TCP_ACK, 65_535, &[], &payload);
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        let outcome = connection.on_segment(&header, 20);
        assert_eq!(outcome, tcp::TcpOutcome::Response);
    }
    assert_eq!(connection.available_data(), 100);
    assert_eq!(connection.received_data(), &payload);
    assert_eq!(connection.receive_next(), expected + 100);

    let ack = segment_info(connection.poll(20));
    assert_eq!(ack.flags, TCP_ACK);
    assert_eq!(ack.ack, expected + 100);
    assert!(ack.window < window_before);

    let mut drained = [0_u8; 256];
    let count = connection.drain(&mut drained);
    assert_eq!(count, 100);
    assert_eq!(drained.get(..count).unwrap_or(&[]), &payload);
    assert_eq!(connection.available_data(), 0);
    assert_eq!(connection.advertised_window(), window_before);
}

#[test]
fn tcp_fin_teardown_reaches_closed() {
    let mut connection = client_connection();
    connection.open();
    let _ = segment_info(connection.poll(0));
    complete_handshake(&mut connection, 1_000);
    let _ = segment_info(connection.poll(10));

    assert!(connection.close());
    let fin = segment_info(connection.poll(20));
    assert_eq!(fin.flags, TCP_FIN | TCP_ACK);
    assert_eq!(fin.seq, 1_001);
    assert_eq!(connection.state(), tcp::TcpState::FinWait1);

    let mut buffer = [0_u8; 128];
    let written = peer_segment(&mut buffer, 5_001, fin.seq + 1, TCP_ACK, 65_535, &[], &[]);
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        connection.on_segment(&header, 30);
    }
    assert_eq!(connection.state(), tcp::TcpState::FinWait2);

    let written = peer_segment(
        &mut buffer,
        5_001,
        fin.seq + 1,
        TCP_FIN | TCP_ACK,
        65_535,
        &[],
        &[],
    );
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        connection.on_segment(&header, 40);
    }
    assert_eq!(connection.state(), tcp::TcpState::TimeWait);

    let ack = segment_info(connection.poll(40));
    assert_eq!(ack.flags, TCP_ACK);
    assert_eq!(ack.ack, 5_002);

    assert!(connection.poll(40 + tcp::TIME_WAIT_MILLIS).is_none());
    assert_eq!(connection.state(), tcp::TcpState::Closed);
}

#[test]
fn tcp_reset_aborts_the_connection() {
    let mut connection = client_connection();
    connection.open();
    let _ = segment_info(connection.poll(0));
    complete_handshake(&mut connection, 1_000);
    let _ = segment_info(connection.poll(10));
    assert_eq!(connection.state(), tcp::TcpState::Established);

    let mut buffer = [0_u8; 128];
    let written = peer_segment(&mut buffer, 0, 0, TCP_RST, 0, &[], &[]);
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        let outcome = connection.on_segment(&header, 20);
        assert_eq!(outcome, tcp::TcpOutcome::Reset);
    }
    assert_eq!(connection.state(), tcp::TcpState::Closed);
    assert_eq!(connection.error(), Some(tcp::TcpError::ConnectionReset));
}

#[test]
fn tcp_retransmission_retries_and_then_aborts() {
    let mut connection = client_connection();
    connection.open();
    let syn = segment_info(connection.poll(0));
    assert_eq!(syn.flags, TCP_SYN);
    assert_eq!(syn.seq, 1_000);

    let retry = segment_info(connection.poll(2_000));
    assert_eq!(retry.flags, TCP_SYN);
    assert_eq!(retry.seq, 1_000);

    let mut now = 2_000_u64;
    for _ in 0..12 {
        now += 100_000;
        if connection.poll(now).is_none() {
            break;
        }
    }
    assert_eq!(connection.state(), tcp::TcpState::Closed);
    assert_eq!(connection.error(), Some(tcp::TcpError::Timeout));
}

#[test]
fn tcp_options_round_trip() {
    let mut options = [0_u8; 40];
    let mut length = 0_usize;
    length += tcp::write_mss(options.get_mut(length..).unwrap_or(&mut []), 1_460).unwrap_or(0);
    length += tcp::write_window_scale(options.get_mut(length..).unwrap_or(&mut []), 7).unwrap_or(0);
    length += tcp::write_timestamp(options.get_mut(length..).unwrap_or(&mut []), 12_345, 67_890)
        .unwrap_or(0);
    assert_eq!(length, 20);

    let parsed = tcp::TcpOptions::new(options.get(..length).unwrap_or(&[]));
    assert_eq!(parsed.mss(), Some(1_460));
    assert_eq!(parsed.window_scale(), Some(7));
    assert_eq!(parsed.timestamps(), Some((12_345, 67_890)));
    assert!(!parsed.sack_permitted());

    // The same options survive a round trip through the wire format.
    let mut buffer = [0_u8; 128];
    let written = peer_segment(
        &mut buffer,
        1,
        2,
        TCP_SYN | TCP_ACK,
        1_024,
        options.get(..length).unwrap_or(&[]),
        &[],
    );
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        let wire_options = tcp::TcpOptions::new(tcp::segment_options(&header));
        assert_eq!(wire_options.mss(), Some(1_460));
        assert_eq!(wire_options.window_scale(), Some(7));
        assert_eq!(wire_options.timestamps(), Some((12_345, 67_890)));
        assert_eq!(header.header_len, 40);
    }
}

#[test]
fn tcp_selective_acknowledgement_blocks_are_decoded() {
    let mut options = [0_u8; 40];
    let mut length = 0_usize;
    length += tcp::write_sack_permitted(options.get_mut(length..).unwrap_or(&mut [])).unwrap_or(0);
    length += tcp::write_sack_blocks(
        options.get_mut(length..).unwrap_or(&mut []),
        100,
        200,
        300,
        400,
    )
    .unwrap_or(0);
    let parsed = tcp::TcpOptions::new(options.get(..length).unwrap_or(&[]));
    assert!(parsed.sack_permitted());
    let mut blocks = [tcp::SackBlock::default(); 2];
    let mut count = 0_usize;
    for block in parsed.sack_blocks() {
        if let Some(slot) = blocks.get_mut(count) {
            *slot = block;
        }
        count += 1;
    }
    assert_eq!(count, 2);
    assert_eq!(blocks.first().copied().unwrap_or_default().left, 100);
    assert_eq!(blocks.first().copied().unwrap_or_default().right, 200);
    assert_eq!(blocks.get(1).copied().unwrap_or_default().left, 300);
    assert_eq!(blocks.get(1).copied().unwrap_or_default().right, 400);
}

#[test]
fn tcp_sequence_arithmetic_wraps_around() {
    assert_eq!(tcp::seq_add(u32::MAX, 2), 1);
    assert!(tcp::seq_lt(3, 5));
    assert!(!tcp::seq_lt(5, 3));
    assert!(tcp::seq_lt(u32::MAX, 0));
    assert!(!tcp::seq_lt(0, u32::MAX));
    assert!(tcp::seq_gt(5, 3));
    assert!(tcp::seq_ge(5, 5));
    assert_eq!(tcp::seq_distance(10, 20), 10);
    assert_eq!(tcp::seq_distance(u32::MAX, 1), 2);
}

#[test]
fn tcp_segment_checksum_is_correct() {
    let src = IpAddress::V4(Ipv4Address([192, 168, 1, 10]));
    let dst = IpAddress::V4(Ipv4Address([192, 168, 1, 1]));
    let payload = [0x33_u8; 12];
    let segment = tcp::TcpSegment {
        flags: tcp::TcpFlags::ACK,
        seq: 1,
        ack: 2,
        window: 4_096,
        urgent_pointer: 0,
        payload: &payload,
        options: &[],
    };
    let mut buffer = [0_u8; 128];
    let written = tcp::write_segment(&mut buffer, 49_152, 80, &segment, &src, &dst).unwrap_or(0);
    let pseudo = ipv4_pseudo(
        [192, 168, 1, 10],
        [192, 168, 1, 1],
        packets::IP_PROTOCOL_TCP,
        written,
    );
    assert_eq!(
        folded_sum(&pseudo, buffer.get(..written).unwrap_or(&[])),
        0xFFFF
    );
    let parsed = tcp::parse_segment(buffer.get(..written).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(header) = parsed {
        assert!(header.validate_ipv4_checksum(
            &Ipv4Address([192, 168, 1, 10]),
            &Ipv4Address([192, 168, 1, 1])
        ));
    }
}

// ---------------------------------------------------------------------------
// DHCP
// ---------------------------------------------------------------------------

fn dhcp_reply(
    buffer: &mut [u8],
    xid: u32,
    yiaddr: Ipv4Address,
    server_id: Ipv4Address,
    lease_seconds: u32,
    message_type: u8,
) {
    for byte in buffer.iter_mut() {
        *byte = 0;
    }
    buffer[0] = 2;
    buffer[1] = 1;
    buffer[2] = 6;
    buffer[4..8].copy_from_slice(&xid.to_be_bytes());
    buffer[16..20].copy_from_slice(&yiaddr.0);
    buffer[28..34].copy_from_slice(&[0x02, 0, 0, 0, 0, 1]);
    buffer[236..240].copy_from_slice(&[0x63, 0x53, 0x82, 0x63]);
    let mut offset = 240_usize;
    buffer[offset] = dhcp::DHCP_OPT_MESSAGE_TYPE;
    buffer[offset + 1] = 1;
    buffer[offset + 2] = message_type;
    offset += 3;
    buffer[offset] = dhcp::DHCP_OPT_SERVER_IDENTIFIER;
    buffer[offset + 1] = 4;
    buffer[offset + 2..offset + 6].copy_from_slice(&server_id.0);
    offset += 6;
    buffer[offset] = dhcp::DHCP_OPT_SUBNET_MASK;
    buffer[offset + 1] = 4;
    buffer[offset + 2..offset + 6].copy_from_slice(&[255, 255, 255, 0]);
    offset += 6;
    buffer[offset] = dhcp::DHCP_OPT_ROUTER;
    buffer[offset + 1] = 4;
    buffer[offset + 2..offset + 6].copy_from_slice(&[192, 168, 1, 1]);
    offset += 6;
    buffer[offset] = dhcp::DHCP_OPT_DNS_SERVERS;
    buffer[offset + 1] = 4;
    buffer[offset + 2..offset + 6].copy_from_slice(&[192, 168, 1, 53]);
    offset += 6;
    buffer[offset] = dhcp::DHCP_OPT_LEASE_TIME;
    buffer[offset + 1] = 4;
    buffer[offset + 2..offset + 6].copy_from_slice(&lease_seconds.to_be_bytes());
    offset += 6;
    buffer[offset] = 255;
}

#[test]
fn dhcp_client_completes_the_dora_exchange() {
    let mac = MacAddress([0x02, 0, 0, 0, 0, 1]);
    let xid = 0x1234_5678_u32;
    let mut client = dhcp::DhcpClient::new(mac, xid);
    assert_eq!(client.state(), dhcp::DhcpState::Init);

    let mut message = [0_u8; 512];
    let written = client.build_discover(&mut message).unwrap_or(0);
    assert!(written > 240);
    assert_eq!(client.state(), dhcp::DhcpState::Select);
    assert_eq!(
        message.get(236..240).unwrap_or(&[]),
        &[0x63, 0x53, 0x82, 0x63]
    );
    assert_eq!(
        dhcp::option_message_type(message.get(240..written).unwrap_or(&[])),
        Some(1)
    );
    assert_eq!(message.get(4..8).unwrap_or(&[]), xid.to_be_bytes());

    let server = Ipv4Address([192, 168, 1, 1]);
    let offered = Ipv4Address([192, 168, 1, 77]);
    let mut reply = [0_u8; 512];
    dhcp_reply(&mut reply, xid, offered, server, 3_600, 2);
    let parsed = dhcp::DhcpClient::parse_message(&reply);
    assert!(parsed.is_some());
    if let Some(packet) = parsed {
        let event = client.on_message(&packet, 1_000);
        if let dhcp::DhcpEvent::Offer(offer) = event {
            assert_eq!(offer.address, offered);
            assert_eq!(offer.server_identifier, server);
            assert_eq!(offer.lease_seconds, 3_600);
            assert_eq!(offer.subnet_mask, Some(Ipv4Address([255, 255, 255, 0])));
            assert_eq!(offer.router, Some(server));
            assert_eq!(
                offer.dns_servers.first(),
                Some(Ipv4Address([192, 168, 1, 53]))
            );
        }
    }
    assert_eq!(client.state(), dhcp::DhcpState::Request);

    let written = client.build_request(&mut message).unwrap_or(0);
    assert!(written > 240);
    assert_eq!(
        message.get(236..240).unwrap_or(&[]),
        &[0x63, 0x53, 0x82, 0x63]
    );
    assert_eq!(
        dhcp::option_message_type(message.get(240..written).unwrap_or(&[])),
        Some(3)
    );
    assert_eq!(
        dhcp::option_ipv4(
            message.get(240..written).unwrap_or(&[]),
            dhcp::DHCP_OPT_REQUESTED_IP
        ),
        Some(offered)
    );
    assert_eq!(
        dhcp::option_ipv4(
            message.get(240..written).unwrap_or(&[]),
            dhcp::DHCP_OPT_SERVER_IDENTIFIER
        ),
        Some(server)
    );

    dhcp_reply(&mut reply, xid, offered, server, 3_600, 5);
    let parsed = dhcp::DhcpClient::parse_message(&reply);
    assert!(parsed.is_some());
    if let Some(packet) = parsed {
        let event = client.on_message(&packet, 2_000);
        if let dhcp::DhcpEvent::Bound(lease) = event {
            assert_eq!(lease.address, offered);
            assert_eq!(lease.server_identifier, server);
            assert_eq!(lease.lease_seconds, 3_600);
            assert_eq!(lease.acquired_at_millis, 2_000);
            assert_eq!(lease.expires_at_millis(), 2_000 + 3_600_000);
            assert!(!lease.is_expired(3_000));
            assert!(lease.is_expired(2_000 + 3_600_000));
        }
    }
    assert_eq!(client.state(), dhcp::DhcpState::Bound);
    let stored = client.lease().copied().unwrap_or_default();
    assert_eq!(stored.address, offered);
}

// ---------------------------------------------------------------------------
// DNS
// ---------------------------------------------------------------------------

#[test]
fn dns_query_is_built_correctly() {
    let mut buffer = [0_u8; 64];
    let written = dns_client::build_query(
        &mut buffer,
        0x1234,
        b"www.example.com",
        dns_client::address_record_type(),
        dns_client::internet_class(),
        true,
    )
    .unwrap_or(0);
    assert_eq!(written, 12 + 17 + 4);
    assert_eq!(buffer.get(0..2).unwrap_or(&[]), &[0x12, 0x34]);
    assert_eq!(buffer.get(2..4).unwrap_or(&[]), &[0x01, 0x00]);
    assert_eq!(buffer.get(4..6).unwrap_or(&[]), &[0x00, 0x01]);
    assert_eq!(buffer.get(6..8).unwrap_or(&[]), &[0x00, 0x00]);
    assert_eq!(buffer.get(12).copied().unwrap_or(0), 3);
    assert_eq!(buffer.get(13..16).unwrap_or(&[]), b"www");
    assert_eq!(buffer.get(16).copied().unwrap_or(0), 7);
    assert_eq!(buffer.get(17..24).unwrap_or(&[]), b"example");
    assert_eq!(buffer.get(24).copied().unwrap_or(0), 3);
    assert_eq!(buffer.get(25..28).unwrap_or(&[]), b"com");
    assert_eq!(buffer.get(28).copied().unwrap_or(0), 0);
    assert_eq!(buffer.get(29..31).unwrap_or(&[]), &[0x00, 0x01]);
    assert_eq!(buffer.get(31..33).unwrap_or(&[]), &[0x00, 0x01]);
}

fn dns_response_with_compressed_name() -> [u8; 64] {
    let mut buffer = [0_u8; 64];
    buffer[0..2].copy_from_slice(&[0x12, 0x34]);
    buffer[2..4].copy_from_slice(&[0x81, 0x80]);
    buffer[4..6].copy_from_slice(&[0x00, 0x01]);
    buffer[6..8].copy_from_slice(&[0x00, 0x01]);
    buffer[12] = 3;
    buffer[13..16].copy_from_slice(b"www");
    buffer[16] = 7;
    buffer[17..24].copy_from_slice(b"example");
    buffer[24] = 3;
    buffer[25..28].copy_from_slice(b"com");
    buffer[28] = 0;
    buffer[29..31].copy_from_slice(&[0x00, 0x01]);
    buffer[31..33].copy_from_slice(&[0x00, 0x01]);
    buffer[33] = 0xC0;
    buffer[34] = 0x0C;
    buffer[35..37].copy_from_slice(&[0x00, 0x01]);
    buffer[37..39].copy_from_slice(&[0x00, 0x01]);
    buffer[39..43].copy_from_slice(&[0x00, 0x00, 0x00, 0x3C]);
    buffer[43..45].copy_from_slice(&[0x00, 0x04]);
    buffer[45..49].copy_from_slice(&[93, 184, 216, 34]);
    buffer
}

#[test]
fn dns_response_with_a_compressed_name_is_parsed() {
    let message = dns_response_with_compressed_name();
    let parsed = dns_client::DnsResponse::parse(message.get(..49).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(response) = parsed {
        assert_eq!(response.rcode(), 0);
        assert!(response.header.is_response());
        assert!(response.header.is_recursion_desired());
        assert_eq!(response.header.qd_count, 1);
        assert_eq!(response.header.an_count, 1);

        let question = response.question();
        assert!(question.is_some());
        if let Some(question) = question {
            let mut name = [0_u8; 64];
            let length = question.decoded_name(&mut name).unwrap_or(0);
            assert_eq!(name.get(..length).unwrap_or(&[]), b"www.example.com");
        }

        let answer = response.answer(0);
        assert!(answer.is_some());
        if let Some(answer) = answer {
            assert!(answer.is_a());
            assert_eq!(answer.a_address(), Some(Ipv4Address([93, 184, 216, 34])));
            assert_eq!(answer.ttl, 60);
        }
        assert_eq!(response.first_ipv4(), Some(Ipv4Address([93, 184, 216, 34])));
        assert!(response.first_ipv6().is_none());
    }

    // The owner name of the answer uses a compression pointer.
    let mut owner = [0_u8; 64];
    let decoded = dns_client::decode_name(message.get(..49).unwrap_or(&[]), 33, &mut owner);
    assert!(decoded.is_some());
    if let Some((length, next)) = decoded {
        assert_eq!(owner.get(..length).unwrap_or(&[]), b"www.example.com");
        assert_eq!(next, 35);
    }
}

#[test]
fn dns_canonical_name_and_ipv6_records_are_parsed() {
    let mut message = [0_u8; 96];
    message[0..2].copy_from_slice(&[0x12, 0x34]);
    message[2..4].copy_from_slice(&[0x81, 0x80]);
    message[4..6].copy_from_slice(&[0x00, 0x01]);
    message[6..8].copy_from_slice(&[0x00, 0x02]);
    message[12] = 3;
    message[13..16].copy_from_slice(b"www");
    message[16] = 7;
    message[17..24].copy_from_slice(b"example");
    message[24] = 3;
    message[25..28].copy_from_slice(b"com");
    message[28] = 0;
    message[29..31].copy_from_slice(&[0x00, 0x01]);
    message[31..33].copy_from_slice(&[0x00, 0x01]);
    // The first answer is a canonical name that points at the question.
    message[33] = 0xC0;
    message[34] = 0x0C;
    message[35..37].copy_from_slice(&[0x00, 0x05]);
    message[37..39].copy_from_slice(&[0x00, 0x01]);
    message[39..43].copy_from_slice(&[0x00, 0x00, 0x00, 0x3C]);
    message[43..45].copy_from_slice(&[0x00, 0x02]);
    message[45] = 0xC0;
    message[46] = 0x0C;
    // The second answer carries an IPv6 address.
    message[47] = 0xC0;
    message[48] = 0x0C;
    message[49..51].copy_from_slice(&[0x00, 0x1C]);
    message[51..53].copy_from_slice(&[0x00, 0x01]);
    message[53..57].copy_from_slice(&[0x00, 0x00, 0x00, 0x3C]);
    message[57..59].copy_from_slice(&[0x00, 0x10]);
    message[59..75].copy_from_slice(&[0x20; 16]);

    let parsed = dns_client::DnsResponse::parse(message.get(..75).unwrap_or(&[]));
    assert!(parsed.is_some());
    if let Some(response) = parsed {
        assert_eq!(response.header.an_count, 2);
        let cname = response.answer(0);
        assert!(cname.is_some());
        if let Some(cname) = cname {
            assert!(cname.is_cname());
            let mut target = [0_u8; 64];
            let length = cname.target_name(&mut target).unwrap_or(0);
            assert_eq!(target.get(..length).unwrap_or(&[]), b"www.example.com");
        }
        let aaaa = response.answer(1);
        assert!(aaaa.is_some());
        if let Some(aaaa) = aaaa {
            assert!(aaaa.is_aaaa());
            assert_eq!(aaaa.aaaa_address(), Some(Ipv6Address([0x20; 16])));
        }
        assert_eq!(response.first_ipv6(), Some(Ipv6Address([0x20; 16])));
        assert!(response.first_ipv4().is_none());
    }
}

#[test]
fn dns_rejects_a_compression_loop() {
    let mut message = [0_u8; 32];
    message[0..2].copy_from_slice(&[0x12, 0x34]);
    message[4..6].copy_from_slice(&[0x00, 0x01]);
    // The name at offset twelve points at itself.
    message[12] = 0xC0;
    message[13] = 0x0C;
    let mut name = [0_u8; 64];
    assert!(dns_client::decode_name(message.get(..14).unwrap_or(&[]), 12, &mut name).is_none());
}

#[test]
fn dns_query_rejects_a_malformed_name() {
    let mut buffer = [0_u8; 64];
    assert_eq!(
        dns_client::build_query(
            &mut buffer,
            1,
            b"bad..name",
            dns_client::address_record_type(),
            dns_client::internet_class(),
            false
        ),
        Err(StackError::Malformed)
    );
    assert_eq!(
        dns_client::build_query(
            &mut buffer,
            1,
            b"",
            dns_client::address_record_type(),
            dns_client::internet_class(),
            false
        ),
        Err(StackError::Malformed)
    );
}

// ---------------------------------------------------------------------------
// Routing
// ---------------------------------------------------------------------------

#[test]
fn routing_table_uses_the_longest_prefix_match() {
    let mut table = route::RoutingTable::new();
    assert!(table.is_empty());
    let gateway = Ipv4Address([192, 168, 1, 1]);
    assert!(
        table
            .insert(route::RouteEntry::default_route(gateway, 1))
            .is_ok()
    );
    let host = route::RouteEntry {
        destination: Ipv4Address([192, 168, 1, 55]),
        prefix_length: 32,
        next_hop: gateway,
        interface: 1,
        metric: 0,
    };
    assert!(table.insert(host).is_ok());
    let network = route::RouteEntry {
        destination: Ipv4Address([192, 168, 1, 0]),
        prefix_length: 24,
        next_hop: gateway,
        interface: 1,
        metric: 0,
    };
    assert!(table.insert(network).is_ok());

    let matched = table.lookup(&Ipv4Address([192, 168, 1, 55])).copied();
    assert_eq!(matched.map(|entry| entry.prefix_length), Some(32));

    let matched = table.lookup(&Ipv4Address([192, 168, 1, 99])).copied();
    assert_eq!(matched.map(|entry| entry.prefix_length), Some(24));

    let matched = table.lookup(&Ipv4Address([10, 0, 0, 1])).copied();
    assert_eq!(matched.map(|entry| entry.prefix_length), Some(0));
    assert!(matched.is_some_and(|entry| entry.is_default()));
    assert_eq!(table.len(), 3);

    let fallback = table.default_route().copied();
    assert_eq!(fallback.map(|entry| entry.next_hop), Some(gateway));

    assert!(table.remove(&Ipv4Address([192, 168, 1, 0]), 24));
    assert_eq!(table.len(), 2);
    assert!(!table.remove(&Ipv4Address([192, 168, 1, 0]), 24));
}
