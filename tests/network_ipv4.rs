//! IPv4 tests.

use core::mem::size_of;

use rover_uefi::protocols::network::ip::is_unspecified;
use rover_uefi::protocols::network::ip_protocol::{EfiIp4Protocol, EfiIp6Protocol};
use rover_uefi::protocols::network::ipv4::{Ipv4Config, is_broadcast};
use rover_uefi::protocols::network::types::Ipv4Address;

#[test]
fn unspecified_checks() {
    assert!(is_unspecified(&Ipv4Address([0, 0, 0, 0])));
    assert!(!is_unspecified(&Ipv4Address([1, 0, 0, 0])));
    assert!(!is_unspecified(&Ipv4Address([0, 0, 0, 1])));
    assert!(is_broadcast(&Ipv4Address([0xFF, 0xFF, 0xFF, 0xFF])));
    assert!(!is_broadcast(&Ipv4Address([0, 0, 0, 0])));
}

#[test]
fn config_roundtrip() {
    let config = Ipv4Config {
        station_address: Ipv4Address([127, 0, 0, 1]),
        subnet_mask: Ipv4Address([255, 255, 255, 0]),
    };
    assert_eq!(config.station_address.0, [127, 0, 0, 1]);
    assert_eq!(config.subnet_mask.0[0], 255);
    assert_eq!(size_of::<Ipv4Config>(), 8);
}

#[test]
fn ip_protocol_sizes() {
    assert_eq!(
        size_of::<EfiIp4Protocol>(),
        8 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiIp6Protocol>(),
        9 * core::mem::size_of::<usize>()
    );
}
