//! Network packet parser tests.

use rover_uefi::protocols::network::application::{
    DNS_HEADER_LEN, DhcpOptions, decode_dns_name, dhcp_message_type, parse_answer, parse_dhcp,
    parse_dns_header, parse_dns_message, parse_http_request, parse_http_response, parse_question,
    parse_request_line, parse_status_line,
};
use rover_uefi::protocols::network::packets::{
    ETHERTYPE_IPV4, ETHERTYPE_VLAN, parse_arp, parse_header, parse_icmpv4, parse_icmpv6,
    parse_ipv4, parse_ipv6, parse_tcp, parse_udp,
};
use rover_uefi::protocols::network::types::{Ipv4Address, Ipv6Address, MacAddress};

fn read_be_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn read_be_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

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

fn internet_checksum(data: &[u8]) -> u16 {
    !u16::try_from(checksum_sum(data)).unwrap_or(0xFFFF)
}

fn checksum_two(first: &[u8], second: &[u8]) -> u16 {
    let sum = checksum_sum(first) + checksum_sum(second);
    let folded = (sum & 0xFFFF) + (sum >> 16);
    let folded = (folded & 0xFFFF) + (folded >> 16);
    !u16::try_from(folded).unwrap_or(0xFFFF)
}

fn ipv4_pseudo(src: [u8; 4], dst: [u8; 4], protocol: u8, len: usize) -> [u8; 12] {
    let len_u16 = u16::try_from(len).unwrap_or(0xFFFF);
    let mut pseudo = [0_u8; 12];
    pseudo[0..4].copy_from_slice(&src);
    pseudo[4..8].copy_from_slice(&dst);
    pseudo[8] = 0;
    pseudo[9] = protocol;
    pseudo[10..12].copy_from_slice(&len_u16.to_be_bytes());
    pseudo
}

fn ipv6_pseudo(src: &[u8; 16], dst: &[u8; 16], next: u8, len: usize) -> [u8; 40] {
    let len_u32 = u32::try_from(len).unwrap_or(0xFFFF_FFFF);
    let mut pseudo = [0_u8; 40];
    pseudo[0..16].copy_from_slice(src);
    pseudo[16..32].copy_from_slice(dst);
    pseudo[32..36].copy_from_slice(&len_u32.to_be_bytes());
    pseudo[39] = next;
    pseudo
}

fn build_ipv4_header(src: [u8; 4], dst: [u8; 4], protocol: u8, payload_len: usize) -> [u8; 20] {
    let mut header = [0_u8; 20];
    header[0] = 0x45;
    let total = u16::try_from(20 + payload_len).unwrap_or(u16::MAX);
    header[2..4].copy_from_slice(&total.to_be_bytes());
    header[4] = 0x12;
    header[5] = 0x34;
    // Flags: DF set.
    header[6] = 0x40;
    header[7] = 0x00;
    header[8] = 64;
    header[9] = protocol;
    header[10] = 0;
    header[11] = 0;
    header[12..16].copy_from_slice(&src);
    header[16..20].copy_from_slice(&dst);
    let checksum = internet_checksum(&header);
    header[10..12].copy_from_slice(&checksum.to_be_bytes());
    header
}

fn build_udp_datagram(
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
    src_port: u16,
    dst_port: u16,
    payload: &[u8],
) -> Vec<u8> {
    let mut segment = Vec::new();
    segment.extend_from_slice(&src_port.to_be_bytes());
    segment.extend_from_slice(&dst_port.to_be_bytes());
    let length = u16::try_from(8 + payload.len()).unwrap_or(u16::MAX);
    segment.extend_from_slice(&length.to_be_bytes());
    segment.extend_from_slice(&[0_u8, 0_u8]);
    segment.extend_from_slice(payload);
    let pseudo = ipv4_pseudo(src_ip, dst_ip, 17, segment.len());
    let checksum = checksum_two(&pseudo, &segment);
    let checksum = if checksum == 0 { 0xFFFF } else { checksum };
    segment[6..8].copy_from_slice(&checksum.to_be_bytes());
    segment
}

fn build_tcp_segment(
    src_ip: Option<&[u8; 4]>,
    dst_ip: Option<&[u8; 4]>,
    src6: Option<&[u8; 16]>,
    dst6: Option<&[u8; 16]>,
    src_port: u16,
    dst_port: u16,
    payload: &[u8],
) -> Vec<u8> {
    let mut segment = Vec::new();
    segment.extend_from_slice(&src_port.to_be_bytes());
    segment.extend_from_slice(&dst_port.to_be_bytes());
    segment.extend_from_slice(&0x0102_0304_u32.to_be_bytes());
    segment.extend_from_slice(&0x0506_0708_u32.to_be_bytes());
    segment.extend_from_slice(&[0x50, 0x02]);
    segment.extend_from_slice(&0x2000_u16.to_be_bytes());
    segment.extend_from_slice(&[0_u8, 0_u8]);
    segment.extend_from_slice(&[0_u8, 0_u8]);
    segment.extend_from_slice(payload);
    let checksum = if let (Some(src), Some(dst)) = (src_ip, dst_ip) {
        let pseudo = ipv4_pseudo(*src, *dst, 6, segment.len());
        checksum_two(&pseudo, &segment)
    } else if let (Some(src), Some(dst)) = (src6, dst6) {
        let pseudo = ipv6_pseudo(src, dst, 6, segment.len());
        checksum_two(&pseudo, &segment)
    } else {
        0_u16
    };
    let checksum = if checksum == 0 { 0xFFFF } else { checksum };
    segment[16..18].copy_from_slice(&checksum.to_be_bytes());
    segment
}

#[test]
fn ethernet_ipv4_udp_valid() {
    let src_ip = [192_u8, 168, 1, 1];
    let dst_ip = [192_u8, 168, 1, 2];
    let payload = b"hello";
    let udp = build_udp_datagram(src_ip, dst_ip, 1234, 53, payload);
    let ip_header = build_ipv4_header(src_ip, dst_ip, 17, udp.len());
    let mut frame = Vec::new();
    frame.extend_from_slice(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    frame.extend_from_slice(&[0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
    frame.extend_from_slice(&ETHERTYPE_IPV4.to_be_bytes());
    frame.extend_from_slice(&ip_header);
    frame.extend_from_slice(&udp);

    let Some(eth) = parse_header(&frame) else {
        panic!("ethernet parse failed");
    };
    assert_eq!(eth.dst, MacAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]));
    assert_eq!(eth.src, MacAddress([0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]));
    assert_eq!(eth.ethertype, ETHERTYPE_IPV4);
    assert_eq!(eth.header_len, 14);
    assert!(eth.vlan.is_none());

    let Some(ip) = parse_ipv4(eth.payload_after_header()) else {
        panic!("ipv4 parse failed");
    };
    assert_eq!(ip.version, 4);
    assert_eq!(ip.src, Ipv4Address(src_ip));
    assert_eq!(ip.dst, Ipv4Address(dst_ip));
    assert_eq!(ip.protocol, 17);
    assert!(ip.validate_checksum());

    let Some(udp_header) = parse_udp(ip.payload_after_header()) else {
        panic!("udp parse failed");
    };
    assert_eq!(udp_header.src_port, 1234);
    assert_eq!(udp_header.dst_port, 53);
    assert_eq!(udp_header.payload_after_header(), payload);
    assert!(udp_header.validate_ipv4_checksum(&Ipv4Address(src_ip), &Ipv4Address(dst_ip)));
}

#[test]
fn ethernet_truncated_and_vlan() {
    assert!(parse_header(&[0_u8; 13]).is_none());
    assert!(parse_header(&[]).is_none());

    // VLAN frame.
    let src_ip = [10_u8, 0, 0, 1];
    let dst_ip = [10_u8, 0, 0, 2];
    let udp = build_udp_datagram(src_ip, dst_ip, 80, 80, b"x");
    let ip_header = build_ipv4_header(src_ip, dst_ip, 17, udp.len());
    let mut frame = Vec::new();
    frame.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    frame.extend_from_slice(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
    frame.extend_from_slice(&ETHERTYPE_VLAN.to_be_bytes());
    // TCI: priority 5, VLAN 101.
    let tci: u16 = 0xA065;
    frame.extend_from_slice(&tci.to_be_bytes());
    frame.extend_from_slice(&ETHERTYPE_IPV4.to_be_bytes());
    frame.extend_from_slice(&ip_header);
    frame.extend_from_slice(&udp);

    let Some(eth) = parse_header(&frame) else {
        panic!("vlan parse failed");
    };
    assert_eq!(eth.ethertype, ETHERTYPE_IPV4);
    assert_eq!(eth.header_len, 18);
    let Some(tag) = eth.vlan else {
        panic!("vlan tag missing");
    };
    assert_eq!(tag.id, 101);
    assert_eq!(tag.pcp, 5);
    assert!(eth.is_ipv4());
    assert!(eth.is_vlan());

    // Truncated VLAN tag must be rejected.
    let truncated = &frame[..16];
    assert!(parse_header(truncated).is_none());
}

#[test]
fn arp_valid_and_truncated() {
    let mut arp = [0_u8; 28];
    arp[0..2].copy_from_slice(&1_u16.to_be_bytes());
    arp[2..4].copy_from_slice(&0x0800_u16.to_be_bytes());
    arp[4] = 6;
    arp[5] = 4;
    arp[6..8].copy_from_slice(&1_u16.to_be_bytes());
    arp[8..14].copy_from_slice(&[0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
    arp[14..18].copy_from_slice(&[192, 168, 1, 1]);
    arp[18..24].copy_from_slice(&[0, 0, 0, 0, 0, 0]);
    arp[24..28].copy_from_slice(&[192, 168, 1, 2]);

    let Some(packet) = parse_arp(&arp) else {
        panic!("arp parse failed");
    };
    assert_eq!(packet.oper, 1);
    assert!(packet.is_request());
    assert!(!packet.is_reply());
    assert_eq!(packet.sender_ip, Ipv4Address([192, 168, 1, 1]));
    assert_eq!(packet.target_ip, Ipv4Address([192, 168, 1, 2]));
    assert_eq!(
        packet.sender_mac,
        MacAddress([0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE])
    );

    assert!(parse_arp(&arp[..20]).is_none());

    // Wrong hardware length must be rejected.
    let mut bad = arp;
    bad[4] = 7;
    assert!(parse_arp(&bad).is_none());
}

#[test]
fn ipv4_bad_checksum_and_truncated() {
    let src = [192_u8, 168, 0, 10];
    let dst = [192_u8, 168, 0, 11];
    let header = build_ipv4_header(src, dst, 6, 4);
    let mut datagram = Vec::new();
    datagram.extend_from_slice(&header);
    datagram.extend_from_slice(&[0x00, 0x01, 0x00, 0x02]);

    let Some(ip) = parse_ipv4(&datagram) else {
        panic!("ipv4 parse failed");
    };
    assert!(ip.validate_checksum());

    // Corrupt the checksum.
    let mut bad = datagram.clone();
    bad[10] ^= 0xFF;
    let Some(bad_ip) = parse_ipv4(&bad) else {
        panic!("ipv4 bad parse failed");
    };
    assert!(!bad_ip.validate_checksum());

    // Truncated header.
    assert!(parse_ipv4(&datagram[..19]).is_none());

    // Oversized total length must be rejected.
    let mut oversized = datagram.clone();
    oversized[2] = 0xFF;
    oversized[3] = 0xFF;
    assert!(parse_ipv4(&oversized).is_none());

    // Invalid version.
    let mut wrong_version = datagram.clone();
    wrong_version[0] = 0x65;
    assert!(parse_ipv4(&wrong_version).is_none());

    // Invalid IHL.
    let mut bad_ihl = datagram.clone();
    bad_ihl[0] = 0x44;
    assert!(parse_ipv4(&bad_ihl).is_none());
}

#[test]
fn ipv4_fragment_helpers() {
    let src = [10_u8, 0, 0, 1];
    let dst = [10_u8, 0, 0, 2];
    let mut header = build_ipv4_header(src, dst, 17, 0);
    // Set MF and offset 2 (16 bytes).
    header[6] = 0x20;
    header[7] = 0x02;
    header[10] = 0;
    header[11] = 0;
    let checksum = internet_checksum(&header);
    header[10..12].copy_from_slice(&checksum.to_be_bytes());

    let Some(ip) = parse_ipv4(&header) else {
        panic!("fragment ipv4 parse failed");
    };
    assert!(ip.is_fragmented());
    assert_eq!(ip.fragment_offset_bytes(), 16);
    assert!(!ip.dont_fragment || ip.more_fragments);
    assert!(rover_uefi::protocols::network::packets::is_fragmented(
        &header
    ));
    assert_eq!(
        rover_uefi::protocols::network::packets::fragment_offset_bytes(&header),
        Some(16)
    );
}

#[test]
fn ipv6_with_extension_headers() {
    let src = [0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let dst = [0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
    let tcp = build_tcp_segment(None, None, Some(&src), Some(&dst), 443, 1234, b"hi");
    // Hop-by-hop (8) -> dest opts (8) -> TCP.
    let hop = [60_u8, 0_u8, 0, 0, 0, 0, 0, 0];
    let dest_opts = [6_u8, 0_u8, 0, 0, 0, 0, 0, 0];
    let payload_len = u16::try_from(hop.len() + dest_opts.len() + tcp.len()).unwrap_or(u16::MAX);
    let mut datagram = Vec::new();
    datagram.extend_from_slice(&[0x60, 0, 0, 0]);
    datagram.extend_from_slice(&payload_len.to_be_bytes());
    datagram.extend_from_slice(&[0_u8, 64_u8]);
    datagram.extend_from_slice(&src);
    datagram.extend_from_slice(&dst);
    datagram.extend_from_slice(&hop);
    datagram.extend_from_slice(&dest_opts);
    datagram.extend_from_slice(&tcp);

    let Some(ip) = parse_ipv6(&datagram) else {
        panic!("ipv6 parse failed");
    };
    assert_eq!(ip.version, 6);
    assert_eq!(ip.next_header, 0);
    assert_eq!(ip.upper_next_header, 6);
    assert_eq!(ip.header_len, 56);
    assert_eq!(ip.payload_after_header(), tcp.as_slice());
    assert!(!ip.is_fragmented());

    let Some(header) = parse_tcp(ip.payload_after_header()) else {
        panic!("tcp after ipv6 parse failed");
    };
    assert_eq!(header.src_port, 443);
    assert!(header.validate_ipv6_checksum(&Ipv6Address(src), &Ipv6Address(dst)));

    // Truncated extension header must be rejected.
    let mut truncated = datagram.clone();
    // Cut in the middle of the second extension.
    truncated.truncate(50);
    // Shrink payload length to match truncated buffer so the only error
    // is the extension walk; parse must still reject for size mismatch.
    assert!(parse_ipv6(&truncated).is_none());

    // Fragment header walk: routing -> fragment -> TCP.
    let routing = [44_u8, 0_u8, 0, 0, 0, 0, 0, 0];
    let fragment = [6_u8, 0_u8, 0x00, 0x09, 0x00, 0x00, 0x00, 0x01];
    let payload_len2 =
        u16::try_from(routing.len() + fragment.len() + tcp.len()).unwrap_or(u16::MAX);
    let mut frag_datagram = Vec::new();
    frag_datagram.extend_from_slice(&[0x60, 0, 0, 0]);
    frag_datagram.extend_from_slice(&payload_len2.to_be_bytes());
    frag_datagram.extend_from_slice(&[43_u8, 64_u8]);
    frag_datagram.extend_from_slice(&src);
    frag_datagram.extend_from_slice(&dst);
    frag_datagram.extend_from_slice(&routing);
    frag_datagram.extend_from_slice(&fragment);
    frag_datagram.extend_from_slice(&tcp);

    let Some(frag_ip) = parse_ipv6(&frag_datagram) else {
        panic!("ipv6 fragment parse failed");
    };
    assert_eq!(frag_ip.upper_next_header, 6);
    assert!(frag_ip.is_fragmented());
    assert_eq!(frag_ip.fragment_offset_bytes(), 8);
}

#[test]
fn icmpv4_and_icmpv6_checksums() {
    // ICMPv4 echo request: type 8, code 0, id 0x1234, seq 1, data.
    let mut message = Vec::new();
    message.extend_from_slice(&[8_u8, 0_u8, 0_u8, 0_u8]);
    message.extend_from_slice(&0x1234_u16.to_be_bytes());
    message.extend_from_slice(&0x0001_u16.to_be_bytes());
    message.extend_from_slice(b"data");
    let checksum = internet_checksum(&message);
    message[2..4].copy_from_slice(&checksum.to_be_bytes());

    let Some(icmp) = parse_icmpv4(&message) else {
        panic!("icmpv4 parse failed");
    };
    assert_eq!(icmp.msg_type, 8);
    assert_eq!(icmp.code, 0);
    assert!(icmp.validate_checksum());

    let mut bad = message.clone();
    bad[6] ^= 0xFF;
    let Some(bad_icmp) = parse_icmpv4(&bad) else {
        panic!("icmpv4 bad parse failed");
    };
    assert!(!bad_icmp.validate_checksum());
    assert!(parse_icmpv4(&[0_u8; 3]).is_none());

    // ICMPv6 echo request with pseudo-header.
    let src = [0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let dst = [0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
    let mut message6 = Vec::new();
    message6.extend_from_slice(&[128_u8, 0_u8, 0_u8, 0_u8]);
    message6.extend_from_slice(&0x00AB_u16.to_be_bytes());
    message6.extend_from_slice(&[0_u8, 0_u8]);
    message6.extend_from_slice(b"ping");
    let pseudo = ipv6_pseudo(&src, &dst, 58, message6.len());
    let checksum6 = checksum_two(&pseudo, &message6);
    message6[2..4].copy_from_slice(&checksum6.to_be_bytes());

    let Some(icmp6) = parse_icmpv6(&message6) else {
        panic!("icmpv6 parse failed");
    };
    assert_eq!(icmp6.msg_type, 128);
    assert!(icmp6.validate_checksum(&Ipv6Address(src), &Ipv6Address(dst)));

    let mut bad6 = message6.clone();
    bad6[7] ^= 0x01;
    let Some(bad_icmp6) = parse_icmpv6(&bad6) else {
        panic!("icmpv6 bad parse failed");
    };
    assert!(!bad_icmp6.validate_checksum(&Ipv6Address(src), &Ipv6Address(dst)));
    assert!(parse_icmpv6(&[0_u8; 3]).is_none());
}

#[test]
fn udp_truncated_oversized_and_zero_checksum() {
    let src = [192_u8, 168, 1, 10];
    let dst = [192_u8, 168, 1, 11];
    let segment = build_udp_datagram(src, dst, 9999, 53, b"abc");

    let Some(udp) = parse_udp(&segment) else {
        panic!("udp parse failed");
    };
    assert!(udp.validate_ipv4_checksum(&Ipv4Address(src), &Ipv4Address(dst)));

    // Truncated.
    assert!(parse_udp(&segment[..7]).is_none());

    // Oversized length field.
    let mut oversized = segment.clone();
    oversized[4] = 0xFF;
    oversized[5] = 0xFF;
    assert!(parse_udp(&oversized).is_none());

    // Bad checksum.
    let mut bad = segment.clone();
    bad[7] ^= 0xFF;
    let Some(bad_udp) = parse_udp(&bad) else {
        panic!("udp bad parse failed");
    };
    assert!(!bad_udp.validate_ipv4_checksum(&Ipv4Address(src), &Ipv4Address(dst)));

    // Zero checksum is allowed for IPv4 but not IPv6.
    let mut zero = segment.clone();
    zero[6] = 0;
    zero[7] = 0;
    let Some(zero_udp) = parse_udp(&zero) else {
        panic!("udp zero parse failed");
    };
    assert!(zero_udp.validate_ipv4_checksum(&Ipv4Address(src), &Ipv4Address(dst)));
    let src6 = [0_u8; 16];
    let dst6 = [0_u8; 16];
    assert!(!zero_udp.validate_ipv6_checksum(&Ipv6Address(src6), &Ipv6Address(dst6)));
}

#[test]
fn tcp_valid_bad_checksum_and_offsets() {
    let src = [192_u8, 168, 1, 1];
    let dst = [192_u8, 168, 1, 2];
    let segment = build_tcp_segment(Some(&src), Some(&dst), None, None, 8080, 80, b"GET ");

    let Some(tcp) = parse_tcp(&segment) else {
        panic!("tcp parse failed");
    };
    assert_eq!(tcp.src_port, 8080);
    assert_eq!(tcp.dst_port, 80);
    assert_eq!(tcp.seq, 0x0102_0304);
    assert_eq!(tcp.data_offset, 5);
    assert!(tcp.is_syn());
    assert_eq!(tcp.payload_after_header(), b"GET ");
    assert!(tcp.validate_ipv4_checksum(&Ipv4Address(src), &Ipv4Address(dst)));

    // Bad checksum.
    let mut bad = segment.clone();
    bad[17] ^= 0xFF;
    let Some(bad_tcp) = parse_tcp(&bad) else {
        panic!("tcp bad parse failed");
    };
    assert!(!bad_tcp.validate_ipv4_checksum(&Ipv4Address(src), &Ipv4Address(dst)));

    // Truncated.
    assert!(parse_tcp(&segment[..19]).is_none());

    // Bad data offset.
    let mut bad_offset = segment.clone();
    bad_offset[12] = 0x40;
    assert!(parse_tcp(&bad_offset).is_none());

    // Oversized data offset (60 bytes) with small buffer.
    let mut big_offset = segment.clone();
    big_offset[12] = 0xF0;
    assert!(parse_tcp(&big_offset).is_none());
}

#[test]
fn dns_with_compression() {
    let mut message = Vec::new();
    // Header: id 0x1234, flags 0x8180, qd 1, an 1, ns 0, ar 0.
    message.extend_from_slice(&0x1234_u16.to_be_bytes());
    message.extend_from_slice(&0x8180_u16.to_be_bytes());
    message.extend_from_slice(&1_u16.to_be_bytes());
    message.extend_from_slice(&1_u16.to_be_bytes());
    message.extend_from_slice(&0_u16.to_be_bytes());
    message.extend_from_slice(&0_u16.to_be_bytes());
    // QNAME example.com.
    message.extend_from_slice(&[7_u8]);
    message.extend_from_slice(b"example");
    message.extend_from_slice(&[3_u8]);
    message.extend_from_slice(b"com");
    message.extend_from_slice(&[0_u8]);
    message.extend_from_slice(&1_u16.to_be_bytes());
    message.extend_from_slice(&1_u16.to_be_bytes());
    // Answer: pointer to offset 12, type A, class IN, ttl 300, rdlen 4.
    message.extend_from_slice(&[0xC0, 0x0C]);
    message.extend_from_slice(&1_u16.to_be_bytes());
    message.extend_from_slice(&1_u16.to_be_bytes());
    message.extend_from_slice(&300_u32.to_be_bytes());
    message.extend_from_slice(&4_u16.to_be_bytes());
    message.extend_from_slice(&[93, 184, 216, 34]);

    let Some(header) = parse_dns_header(&message) else {
        panic!("dns header parse failed");
    };
    assert_eq!(header.id, 0x1234);
    assert!(header.is_response());
    assert_eq!(header.qd_count, 1);
    assert_eq!(header.an_count, 1);

    let Some(dns) = parse_dns_message(&message) else {
        panic!("dns message parse failed");
    };
    let Some(question) = dns.question() else {
        panic!("dns question missing");
    };
    assert_eq!(question.qtype, 1);
    assert_eq!(question.qclass, 1);
    let mut out = [0_u8; 64];
    let Some(len) = question.decoded_name(&mut out) else {
        panic!("question name decode failed");
    };
    assert_eq!(&out[..len], b"example.com");

    let Some(answer) = dns.answer(0) else {
        panic!("dns answer missing");
    };
    assert!(answer.is_a());
    assert_eq!(answer.a_address(), Some(Ipv4Address([93, 184, 216, 34])));
    let mut out2 = [0_u8; 64];
    let Some(len2) = answer.decoded_name(&mut out2) else {
        panic!("answer name decode failed");
    };
    assert_eq!(&out2[..len2], b"example.com");

    // Free-function views agree.
    let Some(free_question) = parse_question(&message, DNS_HEADER_LEN) else {
        panic!("free question parse failed");
    };
    assert_eq!(free_question.qtype, 1);
    let answer_offset = free_question.next_offset;
    let Some(free_answer) = parse_answer(&message, answer_offset) else {
        panic!("free answer parse failed");
    };
    assert!(free_answer.is_a());
}

#[test]
fn dns_aaaa_and_malformed() {
    // AAAA coverage.
    let src6 = [0x20, 0x01, 0x0D, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let mut aaaa = Vec::new();
    aaaa.extend_from_slice(&0x1234_u16.to_be_bytes());
    aaaa.extend_from_slice(&0x8180_u16.to_be_bytes());
    aaaa.extend_from_slice(&1_u16.to_be_bytes());
    aaaa.extend_from_slice(&1_u16.to_be_bytes());
    aaaa.extend_from_slice(&0_u16.to_be_bytes());
    aaaa.extend_from_slice(&0_u16.to_be_bytes());
    aaaa.extend_from_slice(&[3_u8]);
    aaaa.extend_from_slice(b"www");
    aaaa.extend_from_slice(&[7_u8]);
    aaaa.extend_from_slice(b"example");
    aaaa.extend_from_slice(&[3_u8]);
    aaaa.extend_from_slice(b"com");
    aaaa.extend_from_slice(&[0_u8]);
    aaaa.extend_from_slice(&1_u16.to_be_bytes());
    aaaa.extend_from_slice(&1_u16.to_be_bytes());
    aaaa.extend_from_slice(&[0xC0, 0x0C]);
    aaaa.extend_from_slice(&28_u16.to_be_bytes());
    aaaa.extend_from_slice(&1_u16.to_be_bytes());
    aaaa.extend_from_slice(&60_u32.to_be_bytes());
    aaaa.extend_from_slice(&16_u16.to_be_bytes());
    aaaa.extend_from_slice(&src6);
    let Some(dns6) = parse_dns_message(&aaaa) else {
        panic!("dns aaaa parse failed");
    };
    let Some(ans6) = dns6.answer(0) else {
        panic!("dns aaaa answer missing");
    };
    assert!(ans6.is_aaaa());
    assert_eq!(ans6.aaaa_address(), Some(Ipv6Address(src6)));

    // Truncated pointer (offset beyond message) must be rejected.
    let mut truncated = Vec::new();
    truncated.extend_from_slice(&0x1234_u16.to_be_bytes());
    truncated.extend_from_slice(&0x8180_u16.to_be_bytes());
    truncated.extend_from_slice(&1_u16.to_be_bytes());
    truncated.extend_from_slice(&0_u16.to_be_bytes());
    truncated.extend_from_slice(&0_u16.to_be_bytes());
    truncated.extend_from_slice(&0_u16.to_be_bytes());
    truncated.extend_from_slice(&[0xC0, 0xFF]);
    truncated.extend_from_slice(&[0_u8; 10]);
    let mut scratch = [0_u8; 64];
    assert!(decode_dns_name(&truncated, 12, &mut scratch).is_none());

    // Self-pointing pointer exceeds the jump limit.
    let mut looping = Vec::new();
    looping.extend_from_slice(&0x1234_u16.to_be_bytes());
    looping.extend_from_slice(&0x0100_u16.to_be_bytes());
    looping.extend_from_slice(&1_u16.to_be_bytes());
    looping.extend_from_slice(&0_u16.to_be_bytes());
    looping.extend_from_slice(&0_u16.to_be_bytes());
    looping.extend_from_slice(&0_u16.to_be_bytes());
    looping.extend_from_slice(&[0xC0, 0x0C]);
    looping.extend_from_slice(&[0_u8; 4]);
    assert!(decode_dns_name(&looping, 12, &mut scratch).is_none());
    assert!(parse_question(&looping, 12).is_none());
}

#[test]
fn dhcp_options_and_message_type() {
    let mut packet = [0_u8; 300];
    packet[0] = 1;
    packet[1] = 1;
    packet[2] = 6;
    packet[3] = 0;
    packet[4..8].copy_from_slice(&0x1234_5678_u32.to_be_bytes());
    packet[28] = 0x00;
    packet[29] = 0x11;
    packet[30] = 0x22;
    packet[31] = 0x33;
    packet[32] = 0x44;
    packet[33] = 0x55;
    packet[236] = 0x63;
    packet[237] = 0x53;
    packet[238] = 0x82;
    packet[239] = 0x63;
    // Options at 240: 53 len1 Discover, pad, subnet mask, router, end.
    let mut options = Vec::new();
    options.extend_from_slice(&[53_u8, 1_u8, 1_u8]);
    options.push(0_u8);
    options.extend_from_slice(&[1_u8, 4_u8, 255, 255, 255, 0]);
    options.extend_from_slice(&[3_u8, 4_u8, 192, 168, 1, 1]);
    options.extend_from_slice(&[6_u8, 4_u8, 8, 8, 8, 8]);
    options.extend_from_slice(&[57_u8, 2_u8, 0x05, 0xDC]);
    options.push(255_u8);
    packet[240..240 + options.len()].copy_from_slice(&options);

    let Some(pkt) = parse_dhcp(&packet) else {
        panic!("dhcp parse failed");
    };
    assert_eq!(pkt.op, 1);
    assert_eq!(pkt.xid, 0x1234_5678);
    assert_eq!(pkt.message_type(), Some(1));
    assert_eq!(dhcp_message_type(pkt.options), Some(1));

    let mut iter = pkt.options_iter();
    let mut codes = Vec::new();
    while let Some(option) = iter.next_option() {
        codes.push(option.code);
    }
    assert_eq!(codes, vec![53_u8, 1, 3, 6, 57]);

    // Bad cookie rejected.
    let mut bad = packet;
    bad[236] = 0x00;
    assert!(parse_dhcp(&bad).is_none());

    // Truncated packet rejected.
    assert!(parse_dhcp(&packet[..200]).is_none());

    // Truncated option value stops iteration without panicking.
    let truncated_options = [53_u8, 10_u8, 1_u8, 2_u8];
    let mut truncated_iter = DhcpOptions::new(&truncated_options);
    assert!(truncated_iter.next_option().is_none());

    // End code with no following type returns None.
    assert_eq!(dhcp_message_type(&[255_u8]), None);
}

#[test]
fn http_request_status_and_headers() {
    let request = b"GET /index.html HTTP/1.1\r\nHost: example.com\r\nAccept: */*\r\n\r\nbody";
    let Some(req) = parse_http_request(request) else {
        panic!("http request parse failed");
    };
    assert_eq!(req.method, b"GET");
    assert_eq!(req.target, b"/index.html");
    assert_eq!(req.version, b"HTTP/1.1");
    assert_eq!(req.body(), b"body");

    let mut headers = req.headers_iter();
    let Some(first) = headers.next_header() else {
        panic!("first header missing");
    };
    assert_eq!(first.name, b"Host");
    assert_eq!(first.value, b"example.com");
    let Some(second) = headers.next_header() else {
        panic!("second header missing");
    };
    assert_eq!(second.name, b"Accept");
    assert_eq!(second.value, b"*/*");
    assert!(headers.next_header().is_none());

    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello";
    let Some(resp) = parse_http_response(response) else {
        panic!("http response parse failed");
    };
    assert_eq!(resp.version, b"HTTP/1.1");
    assert_eq!(resp.status_code, 200);
    assert_eq!(resp.reason, b"OK");
    assert_eq!(resp.body(), b"hello");
    let mut resp_headers = resp.headers_iter();
    let Some(content_type) = resp_headers.next_header() else {
        panic!("content-type missing");
    };
    assert_eq!(content_type.name, b"Content-Type");
    assert_eq!(content_type.value, b"text/plain");

    // Request/status line helpers.
    assert_eq!(
        parse_request_line(b"POST /submit HTTP/1.0"),
        Some((&b"POST"[..], &b"/submit"[..], &b"HTTP/1.0"[..]))
    );
    assert_eq!(
        parse_status_line(b"HTTP/1.1 404 Not Found"),
        Some((&b"HTTP/1.1"[..], 404_u16, &b"Not Found"[..]))
    );
    assert!(parse_request_line(b"GET / HTTP/1.1 extra").is_none());
    assert!(parse_status_line(b"HTTP/1.1 ABC Nope").is_none());

    // LF-only endings work too.
    let lf_request = b"HEAD / HTTP/1.1\nHost: a\n\n";
    let Some(lf) = parse_http_request(lf_request) else {
        panic!("lf request parse failed");
    };
    assert_eq!(lf.method, b"HEAD");
}

#[test]
fn malformed_and_adversarial_inputs() {
    // IPv4 IHL claiming 60 bytes with only 20 present.
    let mut ihl_overflow = [
        0x4F_u8, 0, 0x00, 20, 0, 0, 0, 0, 64, 17, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    ];
    ihl_overflow[2] = 0;
    ihl_overflow[3] = 20;
    assert!(parse_ipv4(&ihl_overflow).is_none());

    // IPv6 payload length far beyond the buffer.
    let mut ipv6_overflow = Vec::new();
    ipv6_overflow.extend_from_slice(&[0x60_u8, 0, 0, 0, 0xFF, 0xFF, 6, 64]);
    ipv6_overflow.extend_from_slice(&[0_u8; 32]);
    assert!(parse_ipv6(&ipv6_overflow).is_none());

    // Extension length claiming more than the payload provides.
    let src = [0_u8; 16];
    let dst = [0_u8; 16];
    let mut bad_ext = Vec::new();
    bad_ext.extend_from_slice(&[0x60, 0, 0, 0]);
    bad_ext.extend_from_slice(&8_u16.to_be_bytes());
    bad_ext.extend_from_slice(&[0_u8, 64_u8]);
    bad_ext.extend_from_slice(&src);
    bad_ext.extend_from_slice(&dst);
    bad_ext.extend_from_slice(&[6_u8, 0xFF_u8, 0, 0, 0, 0, 0, 0]);
    assert!(parse_ipv6(&bad_ext).is_none());

    // DNS label claiming beyond the message.
    let mut dns_bad = Vec::new();
    dns_bad.extend_from_slice(&0x1234_u16.to_be_bytes());
    dns_bad.extend_from_slice(&0x0100_u16.to_be_bytes());
    dns_bad.extend_from_slice(&1_u16.to_be_bytes());
    dns_bad.extend_from_slice(&0_u16.to_be_bytes());
    dns_bad.extend_from_slice(&0_u16.to_be_bytes());
    dns_bad.extend_from_slice(&0_u16.to_be_bytes());
    dns_bad.extend_from_slice(&[10_u8]);
    dns_bad.extend_from_slice(b"ab");
    let mut scratch = [0_u8; 64];
    assert!(decode_dns_name(&dns_bad, 12, &mut scratch).is_none());
    assert!(parse_question(&dns_bad, 12).is_none());

    // TCP data offset 15 with a 20-byte buffer.
    let mut tcp_overflow = [0_u8; 20];
    tcp_overflow[12] = 0xF0;
    assert!(parse_tcp(&tcp_overflow).is_none());

    // UDP length wrapping past the buffer.
    let mut udp_overflow = [0_u8; 8];
    udp_overflow[4] = 0xFF;
    udp_overflow[5] = 0xFF;
    assert!(parse_udp(&udp_overflow).is_none());

    // DHCP option length exceeding remaining bytes.
    let truncated_options = [1_u8, 255_u8, 1_u8];
    assert!(parse_dhcp(&[0_u8; 200]).is_none());
    let mut iter = DhcpOptions::new(&truncated_options);
    assert!(iter.next_option().is_none());

    // Helper be reads sanity.
    assert_eq!(read_be_u16(&[0x08, 0x00]), 0x0800);
    assert_eq!(read_be_u32(&[0, 0, 0, 1]), 1);
}
