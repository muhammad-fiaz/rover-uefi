//! Tests for disk I/O layouts.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::storage::disk_io::{EfiDiskIo2Protocol, EfiDiskIoProtocol};
use rover_uefi::protocols::storage::guid::{DISK_IO_GUID, DISK_IO2_GUID};

#[test]
fn guid_value() {
    assert_eq!(
        DISK_IO_GUID.to_bytes(),
        [
            0x71, 0x51, 0x34, 0xCE, 0x0B, 0xBA, 0xD2, 0x11, 0x8E, 0x4F, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(EfiGuid::from_bytes(DISK_IO_GUID.to_bytes()), DISK_IO_GUID);

    assert_eq!(
        DISK_IO2_GUID.to_bytes(),
        [
            0xAE, 0x8E, 0x1C, 0x15, 0x2C, 0x7F, 0x2C, 0x47, 0x9E, 0x54, 0x98, 0x28, 0x19, 0x4F,
            0x6A, 0x88
        ]
    );
    assert_eq!(EfiGuid::from_bytes(DISK_IO2_GUID.to_bytes()), DISK_IO2_GUID);
}

#[test]
fn protocol_layout() {
    assert_eq!(
        size_of::<EfiDiskIoProtocol>(),
        8 + 2 * core::mem::size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiDiskIoProtocol, revision), 0);
    assert_eq!(offset_of!(EfiDiskIoProtocol, read_disk), 8);
    assert_eq!(
        offset_of!(EfiDiskIoProtocol, write_disk),
        8 + size_of::<usize>()
    );

    assert_eq!(
        size_of::<EfiDiskIo2Protocol>(),
        8 + 4 * core::mem::size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiDiskIo2Protocol, revision), 0);
    assert_eq!(offset_of!(EfiDiskIo2Protocol, cancel), 8);
}
