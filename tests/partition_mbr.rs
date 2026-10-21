//! MBR tests.

use rover_uefi::protocols::storage::partition::mbr::entry::{MbrEntry, parse_mbr_entry};
use rover_uefi::protocols::storage::partition::mbr::sector::{is_protective_mbr, parse_mbr_sector};
use rover_uefi::protocols::storage::partition::types::PartitionTableError;

fn build_sector(entry_type: u8) -> [u8; 512] {
    let mut buf = [0_u8; 512];
    buf[446] = 0x80;
    buf[446 + 4] = entry_type;
    buf[446 + 8..446 + 12].copy_from_slice(&2048_u32.to_le_bytes());
    buf[446 + 12..446 + 16].copy_from_slice(&4096_u32.to_le_bytes());
    buf[510] = 0x55;
    buf[511] = 0xAA;
    buf
}

#[test]
fn valid_mbr() {
    let buf = build_sector(0x83);
    let Ok(sector) = parse_mbr_sector(&buf) else {
        panic!("expected sector");
    };
    assert_eq!(sector.entries[0].partition_type, 0x83);
    assert_eq!(sector.entries[0].starting_lba, 2048);
}

#[test]
fn protective_mbr() {
    let buf = build_sector(0xEE);
    let Ok(sector) = parse_mbr_sector(&buf) else {
        panic!("expected sector");
    };
    assert!(is_protective_mbr(&sector));
}

#[test]
fn invalid_signature() {
    let mut buf = build_sector(0x83);
    buf[510] = 0;
    assert_eq!(
        parse_mbr_sector(&buf),
        Err(PartitionTableError::InvalidSignature)
    );
}

#[test]
fn invalid_boot() {
    let mut buf = build_sector(0x83);
    buf[446] = 0x01;
    assert_eq!(
        parse_mbr_sector(&buf),
        Err(PartitionTableError::InvalidBootIndicator)
    );
}

#[test]
fn empty_entry() {
    let entry = MbrEntry {
        boot_indicator: 0,
        partition_type: 0,
        starting_lba: 0,
        sector_count: 0,
    };
    assert_eq!(entry.sector_count, 0);
    let bytes = [0_u8; 16];
    assert!(parse_mbr_entry(&bytes).is_ok());
}
