//! IPv6 tests.

use core::mem::size_of;

use rover_uefi::protocols::network::ipv6::{Ipv6Config, is_loopback, is_unspecified};
use rover_uefi::protocols::network::types::Ipv6Address;

#[test]
fn config_sizes() {
    assert_eq!(size_of::<Ipv6Config>(), 17);
    assert_eq!(size_of::<Ipv6Address>(), 16);
}

#[test]
fn unspecified_and_loopback() {
    assert!(is_unspecified(&Ipv6Address([0; 16])));
    assert!(!is_unspecified(&Ipv6Address([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ])));
    let mut loopback = [0_u8; 16];
    loopback[15] = 1;
    assert!(is_loopback(&Ipv6Address(loopback)));
    assert!(!is_loopback(&Ipv6Address([0; 16])));
    let config = Ipv6Config {
        station_address: Ipv6Address([0xFE, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
        prefix_length: 64,
    };
    assert_eq!(config.prefix_length, 64);
    assert_eq!(config.station_address.0[0], 0xFE);
}
