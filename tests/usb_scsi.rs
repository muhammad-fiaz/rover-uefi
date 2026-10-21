//! Tests for the SCSI command blocks used by USB mass storage.
use rover_uefi::protocols::usb::mass_storage::scsi::{
    OP_INQUIRY, OP_MODE_SENSE_6, OP_READ_10, OP_READ_CAPACITY_10, OP_REQUEST_SENSE,
    OP_SYNCHRONIZE_CACHE_10, OP_TEST_UNIT_READY, OP_WRITE_10, ReadCapacity, ScsiCommand,
};

#[test]
fn opcodes_and_lengths() {
    let cases: [(ScsiCommand, u8, usize); 8] = [
        (ScsiCommand::TestUnitReady, OP_TEST_UNIT_READY, 6),
        (
            ScsiCommand::RequestSense {
                allocation_length: 18,
            },
            OP_REQUEST_SENSE,
            6,
        ),
        (
            ScsiCommand::Inquiry {
                vital_product_data: false,
                page_code: 0,
                allocation_length: 36,
            },
            OP_INQUIRY,
            6,
        ),
        (ScsiCommand::ReadCapacity10, OP_READ_CAPACITY_10, 10),
        (
            ScsiCommand::Read10 {
                logical_block_address: 1,
                transfer_length: 1,
            },
            OP_READ_10,
            10,
        ),
        (
            ScsiCommand::Write10 {
                logical_block_address: 1,
                transfer_length: 1,
            },
            OP_WRITE_10,
            10,
        ),
        (
            ScsiCommand::ModeSense6 {
                page_code: 0x3F,
                allocation_length: 4,
            },
            OP_MODE_SENSE_6,
            6,
        ),
        (
            ScsiCommand::SynchronizeCache10 {
                logical_block_address: 0,
                number_of_blocks: 0,
            },
            OP_SYNCHRONIZE_CACHE_10,
            10,
        ),
    ];
    for (command, opcode, length) in cases {
        assert_eq!(command.opcode(), opcode);
        assert_eq!(command.length(), length);
    }
}

#[test]
fn test_unit_ready_encoding() {
    let mut block = [0xFF_u8; 6];
    assert!(ScsiCommand::TestUnitReady.encode(&mut block));
    assert_eq!(block, [0, 0, 0, 0, 0, 0]);
}

#[test]
fn read_ten_encoding() {
    let command = ScsiCommand::Read10 {
        logical_block_address: 0x1234_5678,
        transfer_length: 0x0102,
    };
    let mut block = [0_u8; 10];
    assert!(command.encode(&mut block));
    assert_eq!(block[0], OP_READ_10);
    assert_eq!(&block[2..6], &[0x78, 0x56, 0x34, 0x12]);
    assert_eq!(&block[7..9], &[0x02, 0x01]);
    assert_eq!(block[9], 0);
}

#[test]
fn write_ten_encoding() {
    let command = ScsiCommand::Write10 {
        logical_block_address: 0x0000_00FF,
        transfer_length: 0x0020,
    };
    let mut block = [0_u8; 10];
    assert!(command.encode(&mut block));
    assert_eq!(block[0], OP_WRITE_10);
    assert_eq!(&block[2..6], &[0xFF, 0x00, 0x00, 0x00]);
    assert_eq!(&block[7..9], &[0x20, 0x00]);
}

#[test]
fn synchronize_cache_uses_24_bit_block_count() {
    let command = ScsiCommand::SynchronizeCache10 {
        logical_block_address: 0x0000_1000,
        number_of_blocks: 0x0001_0203,
    };
    let mut block = [0_u8; 10];
    assert!(command.encode(&mut block));
    assert_eq!(block[0], OP_SYNCHRONIZE_CACHE_10);
    assert_eq!(&block[2..6], &[0x00, 0x10, 0x00, 0x00]);
    assert_eq!(&block[6..9], &[0x03, 0x02, 0x01]);
}

#[test]
fn inquiry_encoding_sets_vpd_and_page_code() {
    let command = ScsiCommand::Inquiry {
        vital_product_data: true,
        page_code: 0x01,
        allocation_length: 8,
    };
    let mut block = [0_u8; 6];
    assert!(command.encode(&mut block));
    assert_eq!(block[0], OP_INQUIRY);
    assert_eq!(block[1], 0x03);
    assert_eq!(block[3], 8);
}

#[test]
fn request_sense_allocation_length_is_little_endian() {
    let command = ScsiCommand::RequestSense {
        allocation_length: 0x1234,
    };
    let mut block = [0_u8; 6];
    assert!(command.encode(&mut block));
    assert_eq!(&block[3..5], &[0x34, 0x12]);
}

#[test]
fn encode_rejects_short_buffers() {
    let mut short = [0_u8; 5];
    assert!(!ScsiCommand::TestUnitReady.encode(&mut short));
    let mut narrow = [0_u8; 9];
    assert!(!ScsiCommand::ReadCapacity10.encode(&mut narrow));
    // A longer buffer is accepted and only the command length is written.
    let mut long = [0xAA_u8; 16];
    assert!(ScsiCommand::TestUnitReady.encode(&mut long));
    assert_eq!(&long[..6], &[0, 0, 0, 0, 0, 0]);
    assert_eq!(long[6], 0xAA);
}

#[test]
fn read_capacity_parsing() {
    let bytes = [
        0x00_u8, 0x00, 0x10, 0x00, // last logical block 0x00100000
        0x00, 0x02, 0x00, 0x00, // block length 512
    ];
    let Some(capacity) = ReadCapacity::parse(&bytes) else {
        panic!("the eight byte response must parse");
    };
    assert_eq!(capacity.last_logical_block, 0x0010_0000);
    assert_eq!(capacity.block_length, 512);
    assert_eq!(capacity.last_logical_block + 1, 0x0010_0001);
}

#[test]
fn read_capacity_rejects_short_responses() {
    assert!(ReadCapacity::parse(&[]).is_none());
    assert!(ReadCapacity::parse(&[0_u8; 7]).is_none());
}
