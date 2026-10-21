//! Simple network tests.

use core::mem::{offset_of, size_of};

use rover_uefi::protocols::network::simple_network::mode::{
    EfiSimpleNetworkMode, is_media_present,
};
use rover_uefi::protocols::network::simple_network::protocol::EfiSimpleNetworkProtocol;

#[test]
fn media_checks() {
    assert!(is_media_present(1));
    assert!(is_media_present(255));
    assert!(!is_media_present(0));
}

#[test]
fn mode_layout() {
    assert_eq!(size_of::<EfiSimpleNetworkMode>(), 160);
    assert_eq!(offset_of!(EfiSimpleNetworkMode, state), 0);
    assert_eq!(offset_of!(EfiSimpleNetworkMode, current_address), 136);
    assert_eq!(offset_of!(EfiSimpleNetworkMode, if_type), 154);
    assert_eq!(offset_of!(EfiSimpleNetworkMode, media_present), 158);
}

#[test]
fn protocol_layout() {
    assert_eq!(
        size_of::<EfiSimpleNetworkProtocol>(),
        8 + 15 * core::mem::size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiSimpleNetworkProtocol, revision), 0);
    assert_eq!(
        offset_of!(EfiSimpleNetworkProtocol, mode),
        8 + 14 * core::mem::size_of::<usize>()
    );
}
