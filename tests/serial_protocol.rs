//! Serial I/O tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::serial::guid::{PC_ANSI_GUID, SERIAL_IO_GUID, VT_100_GUID};
use rover_uefi::protocols::serial::protocol::{EfiSerialIoMode, EfiSerialIoProtocol};
use rover_uefi::protocols::serial::types::{ParityType, StopBitsType, validate_data_bits};

#[test]
fn guid_bytes() {
    assert_eq!(
        SERIAL_IO_GUID.to_bytes(),
        [
            0x6F, 0xCF, 0x25, 0xBB, 0xD4, 0xF1, 0xD2, 0x11, 0x9A, 0x0C, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0xFD
        ]
    );
    assert_eq!(
        PC_ANSI_GUID.to_bytes(),
        [
            0x53, 0x47, 0xC1, 0xE0, 0xBE, 0xF9, 0xD2, 0x11, 0x9A, 0x0C, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(SERIAL_IO_GUID.to_bytes()),
        SERIAL_IO_GUID
    );
    assert_ne!(SERIAL_IO_GUID, VT_100_GUID);
}

#[test]
fn protocol_layout() {
    assert_eq!(size_of::<EfiSerialIoProtocol>(), 9 * size_of::<usize>());
    assert_eq!(offset_of!(EfiSerialIoProtocol, revision), 0);
    assert_eq!(offset_of!(EfiSerialIoProtocol, reset), size_of::<usize>());
    assert_eq!(
        offset_of!(EfiSerialIoProtocol, set_attributes),
        2 * size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiSerialIoProtocol, mode),
        7 * size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiSerialIoProtocol, device_type_guid),
        size_of::<usize>()
            + size_of::<usize>()
            + size_of::<usize>()
            + size_of::<usize>()
            + size_of::<usize>()
            + size_of::<usize>()
            + size_of::<usize>()
            + size_of::<usize>()
    );
}

#[test]
fn mode_layout() {
    assert_eq!(size_of::<EfiSerialIoMode>(), 32);
    assert_eq!(offset_of!(EfiSerialIoMode, control_mask), 0);
    assert_eq!(offset_of!(EfiSerialIoMode, timeout), 4);
    assert_eq!(offset_of!(EfiSerialIoMode, baud_rate), 8);
    assert_eq!(offset_of!(EfiSerialIoMode, receive_fifo_depth), 16);
}

#[test]
fn type_validation() {
    assert_eq!(ParityType::NO_PARITY.to_raw(), 1);
    assert_eq!(ParityType::from_raw(5), ParityType::SPACE);
    assert_eq!(StopBitsType::ONE_FIVE.to_raw(), 2);
    assert!(validate_data_bits(0));
    assert!(validate_data_bits(8));
    assert!(!validate_data_bits(4));
    assert!(!validate_data_bits(9));
}
