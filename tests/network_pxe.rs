//! PXE tests.

use core::mem::{offset_of, size_of};

use rover_uefi::protocols::network::guid::PXE_BASE_CODE_GUID;
use rover_uefi::protocols::network::pxe::mode::is_started;
use rover_uefi::protocols::network::pxe::packets::validate_packet_length;
use rover_uefi::protocols::network::pxe::protocol::EfiPxeBaseCodeProtocol;

#[test]
fn packet_lengths() {
    assert!(validate_packet_length(42));
    assert!(validate_packet_length(64));
    assert!(validate_packet_length(1500));
    assert!(!validate_packet_length(10));
    assert!(!validate_packet_length(0));
    assert!(!validate_packet_length(41));
}

#[test]
fn started_checks() {
    assert!(is_started(1));
    assert!(!is_started(0));
}

#[test]
fn pxe_protocol_layout() {
    assert_eq!(
        size_of::<EfiPxeBaseCodeProtocol>(),
        8 + 13 * core::mem::size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiPxeBaseCodeProtocol, revision), 0);
    assert_eq!(
        offset_of!(EfiPxeBaseCodeProtocol, mode),
        8 + 12 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        PXE_BASE_CODE_GUID.to_bytes(),
        [
            0x03, 0xE6, 0xC4, 0x03, 0x28, 0xAC, 0xD3, 0x11, 0x9A, 0x2D, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
}
