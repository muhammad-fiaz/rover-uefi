//! Tests for block I/O helpers.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::storage::block_io::media::EfiBlockIoMedia;
use rover_uefi::protocols::storage::block_io::protocol::EfiBlockIo2Protocol;
use rover_uefi::protocols::storage::block_io::validation::{
    block_count, is_buffer_aligned, validate_lba_range,
};
use rover_uefi::protocols::storage::guid::{BLOCK_IO_GUID, BLOCK_IO2_GUID};

#[test]
fn guid_value() {
    assert_eq!(
        BLOCK_IO_GUID.to_bytes(),
        [
            0x21, 0x5B, 0x4E, 0x96, 0x59, 0x64, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(EfiGuid::from_bytes(BLOCK_IO_GUID.to_bytes()), BLOCK_IO_GUID);

    assert_eq!(
        BLOCK_IO2_GUID.to_bytes(),
        [
            0x72, 0x24, 0x7B, 0xA7, 0x82, 0xE2, 0x9F, 0x4E, 0xA2, 0x45, 0xC2, 0xC0, 0xE2, 0x7B,
            0xBC, 0xC1
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(BLOCK_IO2_GUID.to_bytes()),
        BLOCK_IO2_GUID
    );
}

#[test]
fn media_layout() {
    assert_eq!(size_of::<EfiBlockIoMedia>(), 48);
    assert_eq!(offset_of!(EfiBlockIoMedia, media_id), 0);
    assert_eq!(offset_of!(EfiBlockIoMedia, block_size), 12);
    assert_eq!(offset_of!(EfiBlockIoMedia, last_block), 24);
    assert_eq!(
        offset_of!(EfiBlockIoMedia, optimal_transfer_length_granularity),
        44
    );

    assert_eq!(
        size_of::<EfiBlockIo2Protocol>(),
        5 * core::mem::size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiBlockIo2Protocol, media), 0);
    assert_eq!(offset_of!(EfiBlockIo2Protocol, reset), size_of::<usize>());
}

#[test]
fn validation_helpers() {
    assert!(is_buffer_aligned(0x1000, 512));
    assert!(!is_buffer_aligned(0x1001, 512));
    assert!(is_buffer_aligned(0x1001, 0));
    assert_eq!(block_count(1024, 512), Some(2));
    assert_eq!(block_count(1000, 512), None);
    assert_eq!(block_count(1024, 0), None);
    assert!(validate_lba_range(0, 2, 9));
    assert!(!validate_lba_range(9, 2, 9));
    assert!(!validate_lba_range(u64::MAX, 2, u64::MAX));
}
