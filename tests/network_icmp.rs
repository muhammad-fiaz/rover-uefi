//! Tests for the native `ICMP` and `ICMPv6` encoders and decoders.
use rover_uefi::protocols::network::stack::icmp::{
    ICMP_HEADER_LEN, ICMPV4_DESTINATION_UNREACHABLE, ICMPV4_ECHO_REPLY, ICMPV4_ECHO_REQUEST,
    ICMPV6_ECHO_REQUEST, IcmpMessage, build_echo_reply, build_echo_reply_v6, build_echo_request,
    build_echo_request_v6, checksum, checksum_v6, verify_checksum, verify_checksum_v6,
};
use rover_uefi::protocols::network::types::Ipv6Address;

fn address(bytes: [u8; 16]) -> Ipv6Address {
    Ipv6Address(bytes)
}

#[test]
fn echo_request_roundtrip() {
    let mut buffer = [0_u8; 32];
    let Some(length) = build_echo_request(&mut buffer, 0x1234, 0x0001, b"abcd") else {
        panic!("the request must fit")
    };
    assert_eq!(length, ICMP_HEADER_LEN + 4);
    assert!(verify_checksum(&buffer[..length]));

    let Some(message) = IcmpMessage::parse(&buffer[..length]) else {
        panic!("the message must parse")
    };
    assert_eq!(message.kind, ICMPV4_ECHO_REQUEST);
    assert_eq!(message.code, 0);
    assert_eq!(message.body, b"abcd");
    assert_eq!(message.echo_identity(), (0x1234, 0x0001));
    assert!(message.is_echo_request(4));
    assert!(!message.is_echo_reply(4));
    assert!(!message.is_error(4));
}

#[test]
fn echo_reply_roundtrip() {
    let mut buffer = [0_u8; 16];
    let Some(length) = build_echo_reply(&mut buffer, 7, 9, &[]) else {
        panic!("the reply must fit")
    };
    assert_eq!(length, ICMP_HEADER_LEN);
    assert!(verify_checksum(&buffer[..length]));
    let Some(message) = IcmpMessage::parse(&buffer[..length]) else {
        panic!("the message must parse")
    };
    assert_eq!(message.kind, ICMPV4_ECHO_REPLY);
    assert!(message.is_echo_reply(4));
    assert_eq!(message.echo_identity(), (7, 9));
}

#[test]
fn odd_payload_is_padded_for_the_checksum() {
    let mut buffer = [0_u8; 16];
    let Some(length) = build_echo_request(&mut buffer, 1, 2, b"abc") else {
        panic!("the request must fit")
    };
    assert!(verify_checksum(&buffer[..length]));
    // Flipping one payload bit must invalidate the checksum.
    buffer[ICMP_HEADER_LEN] ^= 0x01;
    assert!(!verify_checksum(&buffer[..length]));
}

#[test]
fn build_rejects_a_short_buffer() {
    let mut buffer = [0_u8; ICMP_HEADER_LEN + 1];
    assert!(build_echo_request(&mut buffer, 1, 2, b"xyz").is_none());
    assert!(build_echo_request(&mut buffer, 1, 2, b"x").is_some());
}

#[test]
fn checksum_of_empty_input_is_all_ones() {
    assert_eq!(checksum(&[]), 0xFFFF);
    assert_eq!(checksum(&[0, 0]), 0xFFFF);
}

#[test]
fn parse_rejects_a_truncated_message() {
    assert!(IcmpMessage::parse(&[]).is_none());
    assert!(IcmpMessage::parse(&[0_u8; 7]).is_none());
    let Some(message) = IcmpMessage::parse(&[0_u8; 8]) else {
        panic!("an eight byte header must parse")
    };
    assert_eq!(message.echo_identity(), (0, 0));
}

#[test]
fn error_messages_are_classified() {
    let mut bytes = [0_u8; ICMP_HEADER_LEN];
    bytes[0] = ICMPV4_DESTINATION_UNREACHABLE;
    let Some(message) = IcmpMessage::parse(&bytes) else {
        panic!("the message must parse")
    };
    assert!(message.is_error(4));
    assert!(!message.is_echo_request(4));
    assert!(!message.is_error(5));
    assert!(!message.is_echo_request(6));
}

#[test]
fn icmpv6_echo_uses_the_pseudo_header() {
    let source = address([0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let destination = address([0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    let mut buffer = [0_u8; 64];
    let Some(length) =
        build_echo_request_v6(&mut buffer, source, destination, 0xAAAA, 0x5555, b"ping")
    else {
        panic!("the request must fit")
    };
    assert_eq!(length, ICMP_HEADER_LEN + 4);
    assert!(verify_checksum_v6(source, destination, &buffer[..length]));

    let Some(message) = IcmpMessage::parse(&buffer[..length]) else {
        panic!("the message must parse")
    };
    assert_eq!(message.kind, ICMPV6_ECHO_REQUEST);
    assert!(message.is_echo_request(6));
    assert_eq!(message.echo_identity(), (0xAAAA, 0x5555));

    // The checksum is bound to the pseudo header addresses.
    let other = address([0xFE, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9]);
    assert!(!verify_checksum_v6(other, destination, &buffer[..length]));

    // A modified payload must fail verification.
    buffer[ICMP_HEADER_LEN] ^= 0x10;
    assert!(!verify_checksum_v6(source, destination, &buffer[..length]));
}

#[test]
fn icmpv6_reply_roundtrip() {
    let source = address([0xFE, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let destination = address([0xFE, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    let mut buffer = [0_u8; 32];
    let Some(length) = build_echo_reply_v6(&mut buffer, source, destination, 3, 4, b"ab") else {
        panic!("the reply must fit")
    };
    assert!(verify_checksum_v6(source, destination, &buffer[..length]));
    assert_eq!(checksum_v6(source, destination, &buffer[..length]), 0);
    let Some(message) = IcmpMessage::parse(&buffer[..length]) else {
        panic!("the message must parse")
    };
    assert!(message.is_echo_reply(6));
    assert!(!message.is_error(6));
}
