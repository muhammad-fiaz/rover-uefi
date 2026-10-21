//! DNS tests.

use core::mem::size_of;

use rover_uefi::protocols::network::dns::protocol::{EfiDns4Protocol, EfiDns6Protocol};
use rover_uefi::protocols::network::guid::{DNS4_GUID, DNS6_GUID};

#[test]
fn dns_guids_distinct_with_bytes() {
    assert_ne!(DNS4_GUID, DNS6_GUID);
    assert_eq!(
        DNS4_GUID.to_bytes(),
        [
            0xCC, 0x28, 0x3D, 0xAE, 0x5B, 0xE0, 0xA1, 0x4F, 0xA0, 0x11, 0x7E, 0xB5, 0x5A, 0x3F,
            0x14, 0x01
        ]
    );
    assert_eq!(
        DNS6_GUID.to_bytes(),
        [
            0x1F, 0xBC, 0x37, 0xCA, 0x27, 0xA3, 0xE9, 0x4A, 0x82, 0x8A, 0x8C, 0x40, 0xD8, 0x50,
            0x6A, 0x17
        ]
    );
}

#[test]
fn dns_protocol_sizes() {
    assert_eq!(
        size_of::<EfiDns4Protocol>(),
        8 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiDns6Protocol>(),
        8 * core::mem::size_of::<usize>()
    );
}
