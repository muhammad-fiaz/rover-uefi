//! Tests for the CRC-32 variants and the byte-sum checksum helpers.
use rover_uefi::base::crc::{
    byte_sum_checksum, byte_sum_checksum_ok, crc32, crc32_table, crc32_update,
};

#[test]
fn crc32_known_answer() {
    // The standard CRC-32/ISO-HDLC check value.
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    assert_eq!(crc32(b""), 0);
    assert_eq!(crc32(b"a"), 0xE8B7_BE43);
}

#[test]
fn table_and_bitwise_crc32_agree() {
    let cases: [&[u8]; 5] = [b"", b"a", b"123456789", b"firmware", &[0xFF; 64]];
    for data in cases {
        assert_eq!(crc32(data), crc32_table(data));
    }
}

#[test]
fn streaming_crc32_matches_one_shot() {
    let data = b"the quick brown fox jumps over the lazy dog";
    let split = 17;
    // The stream starts at 0xFFFF_FFFF and applies the final XOR once.
    let state = crc32_update(0xFFFF_FFFF, &data[..split]);
    let state = crc32_update(state, &data[split..]);
    assert_eq!(state ^ 0xFFFF_FFFF, crc32(data));
}

#[test]
fn byte_sum_checksum_pairs() {
    let data = [0x01_u8, 0x02, 0x03, 0x04];
    // The helper returns the value that makes the total sum zero.
    let sum = byte_sum_checksum(&data);
    assert_eq!(sum, 0xF6);
    assert!(!byte_sum_checksum_ok(&data));

    let mut checked = [0_u8; 5];
    checked[..4].copy_from_slice(&data);
    checked[4] = sum;
    assert!(byte_sum_checksum_ok(&checked));
}

#[test]
fn byte_sum_wraps_without_panicking() {
    assert_eq!(byte_sum_checksum(&[0xFF; 3]), 0x03);
    assert_eq!(byte_sum_checksum(&[]), 0);
    assert!(byte_sum_checksum_ok(&[]));
}
