//! Tests for the native `NDP` message and option encoders.

use rover_uefi::protocols::network::stack::{StackError, icmp, ndp};
use rover_uefi::protocols::network::types::{Ipv6Address, MacAddress};

fn mac(last: u8) -> MacAddress {
    MacAddress([0x02, 0, 0, 0, 0, last])
}

fn address(last: u8) -> Ipv6Address {
    let mut bytes = [0_u8; 16];
    bytes[0] = 0xFE;
    bytes[1] = 0x80;
    bytes[15] = last;
    Ipv6Address(bytes)
}

fn options_with_source() -> ndp::NdpOptions {
    ndp::NdpOptions {
        source_link_layer: Some(mac(7)),
        ..Default::default()
    }
}

#[test]
fn router_solicitation_roundtrip() {
    let mut buffer = [0_u8; 64];
    let Ok(len) = ndp::build_router_solicitation(&mut buffer, &options_with_source()) else {
        panic!("router solicitation did not build");
    };
    assert_eq!(len, ndp::ND_HEADER_LEN + ndp::ND_LINK_LAYER_OPTION_LEN);
    assert_eq!(buffer[0], ndp::ICMPV6_ROUTER_SOLICIT);

    let Some(message) = ndp::RouterSolicitation::parse(&buffer[..len]) else {
        panic!("router solicitation did not parse");
    };
    let mut options = message.options;
    let Some(option) = options.next() else {
        panic!("source option missing");
    };
    assert_eq!(option.source_link_layer(), Some(mac(7)));
    assert!(options.next().is_none());
}

#[test]
fn router_advertisement_roundtrip() {
    let params = ndp::RouterAdvertisementParams {
        hop_limit: 255,
        managed: true,
        other: false,
        router_lifetime: 1800,
        reachable_time: 30_000,
        retrans_timer: 1000,
    };
    let options = ndp::NdpOptions {
        source_link_layer: Some(mac(1)),
        prefix: Some(ndp::PrefixInformation {
            prefix: address(0),
            prefix_length: 64,
            on_link: true,
            autonomous: true,
            valid_lifetime: 2_592_000,
            preferred_lifetime: 604_800,
        }),
        mtu: Some(1500),
        ..Default::default()
    };
    let mut buffer = [0_u8; 128];
    let Ok(len) = ndp::build_router_advertisement(&mut buffer, &params, &options) else {
        panic!("router advertisement did not build");
    };
    assert_eq!(
        len,
        ndp::ND_ROUTER_ADVERT_LEN
            + ndp::ND_LINK_LAYER_OPTION_LEN
            + ndp::ND_PREFIX_OPTION_LEN
            + ndp::ND_MTU_OPTION_LEN
    );

    let Some(message) = ndp::RouterAdvertisement::parse(&buffer[..len]) else {
        panic!("router advertisement did not parse");
    };
    assert_eq!(message.params, params);

    let mut prefix = None;
    let mut mtu = None;
    for option in message.options {
        if let Some(value) = option.prefix_information() {
            prefix = Some(value);
        }
        if let Some(value) = option.mtu() {
            mtu = Some(value);
        }
    }
    let Some(prefix) = prefix else {
        panic!("prefix option missing");
    };
    assert_eq!(prefix.prefix_length, 64);
    assert!(prefix.on_link);
    assert!(prefix.autonomous);
    assert_eq!(prefix.valid_lifetime, 2_592_000);
    assert_eq!(prefix.preferred_lifetime, 604_800);
    assert_eq!(prefix.prefix, address(0));
    assert_eq!(mtu, Some(1500));
}

#[test]
fn neighbor_solicitation_roundtrip_and_checksum() {
    let target = address(2);
    let mut buffer = [0_u8; 64];
    let Ok(len) = ndp::build_neighbor_solicitation(&mut buffer, target, &options_with_source())
    else {
        panic!("neighbor solicitation did not build");
    };
    let source = address(1);
    let destination = address(3);
    assert!(
        ndp::write_checksum(&mut buffer[..len], source, destination).is_ok(),
        "checksum did not write"
    );
    assert!(icmp::verify_checksum_v6(
        source,
        destination,
        &buffer[..len]
    ));

    let Some(message) = ndp::NeighborSolicitation::parse(&buffer[..len]) else {
        panic!("neighbor solicitation did not parse");
    };
    assert_eq!(message.target, target);
    assert_eq!(message.options.count(), 1);
}

#[test]
fn neighbor_advertisement_flags() {
    let flags = ndp::NeighborAdvertisementFlags {
        router: true,
        solicited: true,
        override_entry: false,
    };
    let options = ndp::NdpOptions {
        target_link_layer: Some(mac(9)),
        ..Default::default()
    };
    let mut buffer = [0_u8; 64];
    let Ok(len) = ndp::build_neighbor_advertisement(&mut buffer, &flags, address(9), &options)
    else {
        panic!("neighbor advertisement did not build");
    };
    let Some(message) = ndp::NeighborAdvertisement::parse(&buffer[..len]) else {
        panic!("neighbor advertisement did not parse");
    };
    assert_eq!(message.flags, flags);
    assert_eq!(message.target, address(9));
    let mut options = message.options;
    let Some(option) = options.next() else {
        panic!("target option missing");
    };
    assert_eq!(option.target_link_layer(), Some(mac(9)));
}

#[test]
fn redirect_roundtrip() {
    let mut buffer = [0_u8; 96];
    let Ok(len) = ndp::build_redirect(
        &mut buffer,
        address(4),
        address(5),
        &ndp::NdpOptions::default(),
    ) else {
        panic!("redirect did not build");
    };
    assert_eq!(len, ndp::ND_REDIRECT_LEN);
    let Some(message) = ndp::Redirect::parse(&buffer[..len]) else {
        panic!("redirect did not parse");
    };
    assert_eq!(message.target, address(4));
    assert_eq!(message.destination, address(5));
    assert_eq!(message.options.count(), 0);
}

#[test]
fn short_buffer_is_rejected() {
    let mut small = [0_u8; 8];
    let result =
        ndp::build_neighbor_solicitation(&mut small, address(1), &ndp::NdpOptions::default());
    assert_eq!(result, Err(StackError::BufferTooSmall));
}

#[test]
fn truncated_option_list_is_rejected() {
    let mut buffer = [0_u8; 16];
    assert!(
        ndp::build_router_solicitation(&mut buffer, &options_with_source()).is_ok(),
        "router solicitation did not build"
    );
    assert!(ndp::RouterSolicitation::parse(&buffer[..12]).is_none());
}

#[test]
fn zero_length_option_is_rejected() {
    let mut buffer = [0_u8; 16];
    buffer[0] = ndp::ICMPV6_ROUTER_SOLICIT;
    buffer[8] = 200;
    buffer[9] = 0;
    assert!(ndp::RouterSolicitation::parse(&buffer).is_none());
}

#[test]
fn unknown_option_is_preserved() {
    let mut buffer = [0_u8; 24];
    buffer[0] = ndp::ICMPV6_ROUTER_SOLICIT;
    buffer[8] = 200;
    buffer[9] = 1;
    buffer[10] = 0xAA;
    buffer[16] = 201;
    buffer[17] = 1;
    let Some(message) = ndp::RouterSolicitation::parse(&buffer) else {
        panic!("router solicitation did not parse");
    };
    let mut kinds = [0_u8; 2];
    let mut index = 0;
    for option in message.options {
        kinds[index] = option.kind;
        index += 1;
    }
    assert_eq!(index, 2);
    assert_eq!(kinds, [200, 201]);
}

#[test]
fn wrong_message_type_is_rejected() {
    let mut buffer = [0_u8; 32];
    buffer[0] = ndp::ICMPV6_REDIRECT;
    assert!(ndp::RouterSolicitation::parse(&buffer).is_none());
    assert!(ndp::NeighborSolicitation::parse(&buffer).is_none());
    assert!(ndp::RouterAdvertisement::parse(&buffer).is_none());
}

#[test]
fn checksum_detects_tampering() {
    let mut buffer = [0_u8; 64];
    let Ok(len) = ndp::build_router_solicitation(&mut buffer, &options_with_source()) else {
        panic!("router solicitation did not build");
    };
    let source = address(1);
    let destination = address(0xFF);
    assert!(
        ndp::write_checksum(&mut buffer[..len], source, destination).is_ok(),
        "checksum did not write"
    );
    buffer[len - 1] ^= 0xFF;
    assert!(!icmp::verify_checksum_v6(
        source,
        destination,
        &buffer[..len]
    ));
}

#[test]
fn write_checksum_rejects_short_buffer() {
    let mut small = [0_u8; 4];
    assert_eq!(
        ndp::write_checksum(&mut small, address(1), address(2)),
        Err(StackError::BufferTooSmall)
    );
}
