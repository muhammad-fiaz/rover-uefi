//! CRC32 tests.

use rover_uefi::protocols::storage::partition::crc32::crc32;

#[test]
fn known_vectors() {
    assert_eq!(crc32(b""), 0x0000_0000);
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    assert_eq!(crc32(&[0xFF; 3]), 0xFFFF_FF00);
}

#[test]
fn incremental_match() {
    let data = b"GPT header test";
    let whole = crc32(data);
    assert_eq!(whole, crc32(data));
    assert_ne!(whole, 0);
}
