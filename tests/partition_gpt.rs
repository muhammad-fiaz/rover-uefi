//! GPT tests.

use rover_uefi::protocols::storage::partition::crc32::crc32;
use rover_uefi::protocols::storage::partition::gpt::header::{GPT_SIGNATURE, parse_gpt_header};
use rover_uefi::protocols::storage::partition::types::PartitionTableError;

fn build_header() -> [u8; 512] {
    let mut buf = [0_u8; 512];
    buf[0..8].copy_from_slice(&GPT_SIGNATURE);
    buf[12..16].copy_from_slice(&92_u32.to_le_bytes());
    buf[24..32].copy_from_slice(&1_u64.to_le_bytes());
    buf[32..40].copy_from_slice(&100_u64.to_le_bytes());
    buf[40..48].copy_from_slice(&34_u64.to_le_bytes());
    buf[48..56].copy_from_slice(&66_u64.to_le_bytes());
    buf[72..80].copy_from_slice(&2_u64.to_le_bytes());
    buf[80..84].copy_from_slice(&4_u32.to_le_bytes());
    buf[84..88].copy_from_slice(&128_u32.to_le_bytes());
    buf[88..92].copy_from_slice(&0x1234_5678_u32.to_le_bytes());
    let mut zeroed = buf;
    zeroed[16..20].copy_from_slice(&0_u32.to_le_bytes());
    let crc = crc32(&zeroed[..92]);
    buf[16..20].copy_from_slice(&crc.to_le_bytes());
    buf
}

#[test]
fn valid_header() {
    let buf = build_header();
    let Ok(header) = parse_gpt_header(&buf) else {
        panic!("expected header");
    };
    assert_eq!(header.current_lba, 1);
    assert_eq!(header.number_of_entries, 4);
}

#[test]
fn invalid_signature() {
    let mut buf = build_header();
    buf[0] = b'X';
    assert_eq!(
        parse_gpt_header(&buf),
        Err(PartitionTableError::InvalidSignature)
    );
}

#[test]
fn invalid_crc() {
    let mut buf = build_header();
    buf[30] ^= 0xFF;
    assert_eq!(parse_gpt_header(&buf), Err(PartitionTableError::InvalidCrc));
}

#[test]
fn invalid_entry_size() {
    let mut buf = build_header();
    buf[84..88].copy_from_slice(&64_u32.to_le_bytes());
    // Recompute CRC for the changed size field.
    buf[16..20].copy_from_slice(&0_u32.to_le_bytes());
    let crc = crc32(&buf[..92]);
    buf[16..20].copy_from_slice(&crc.to_le_bytes());
    assert_eq!(
        parse_gpt_header(&buf),
        Err(PartitionTableError::InvalidEntrySize)
    );
}

#[test]
fn truncated() {
    assert_eq!(
        parse_gpt_header(&[0_u8; 10]),
        Err(PartitionTableError::Truncated)
    );
}
