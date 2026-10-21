//! Network type tests.

use core::mem::size_of;

use rover_uefi::protocols::network::mac::is_multicast;
use rover_uefi::protocols::network::types::{Ipv4Address, Ipv6Address, MacAddress};

#[test]
fn address_sizes() {
    assert_eq!(size_of::<MacAddress>(), 6);
    assert_eq!(size_of::<Ipv4Address>(), 4);
    assert_eq!(size_of::<Ipv6Address>(), 16);
}

#[test]
fn multicast_detection() {
    assert!(is_multicast(&MacAddress([0x01, 0, 0, 0, 0, 0])));
    assert!(is_multicast(&MacAddress([
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ])));
    assert!(!is_multicast(&MacAddress([
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55
    ])));
    assert!(!is_multicast(&MacAddress([0x02, 0, 0, 0, 0, 0])));
}

#[test]
fn address_equality_and_copy() {
    let a = Ipv4Address([192, 168, 1, 1]);
    let b = a;
    assert_eq!(a, b);
    assert_ne!(a, Ipv4Address([192, 168, 1, 2]));
}
