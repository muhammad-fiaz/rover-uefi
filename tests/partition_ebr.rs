//! Tests for extended boot record chain validation.
use rover_uefi::protocols::storage::partition::mbr::{
    MbrEntry, absolute_lba, is_extended, validate_ebr_chain,
};

fn entry(partition_type: u8) -> MbrEntry {
    MbrEntry {
        boot_indicator: 0,
        partition_type,
        starting_lba: 0,
        sector_count: 0,
    }
}

#[test]
fn empty_and_simple_chains_are_valid() {
    assert!(validate_ebr_chain(&[], 4));
    assert!(validate_ebr_chain(&[1], 4));
    assert!(validate_ebr_chain(&[63, 128, 256], 4));
}

#[test]
fn chains_longer_than_the_limit_are_rejected() {
    assert!(validate_ebr_chain(&[1, 2, 3, 4], 4));
    assert!(!validate_ebr_chain(&[1, 2, 3, 4, 5], 4));
    assert!(!validate_ebr_chain(&[1], 0));
}

#[test]
fn loops_are_rejected() {
    assert!(!validate_ebr_chain(&[63, 63], 8));
    assert!(!validate_ebr_chain(&[1, 2, 3, 1], 8));
    assert!(validate_ebr_chain(&[1, 2, 3, 4], 8));
}

#[test]
fn absolute_lba_is_checked() {
    assert_eq!(absolute_lba(0, 0), Some(0));
    assert_eq!(absolute_lba(1000, 5), Some(1005));
    assert_eq!(
        absolute_lba(u64::from(u32::MAX), u32::MAX),
        Some(8_589_934_590)
    );
    assert_eq!(absolute_lba(u64::MAX, 0), Some(u64::MAX));
    assert_eq!(absolute_lba(u64::MAX, 1), None);
}

#[test]
fn extended_container_types() {
    assert!(is_extended(&entry(0x05)));
    assert!(is_extended(&entry(0x0F)));
    assert!(!is_extended(&entry(0x07)));
    assert!(!is_extended(&entry(0x0B)));
    assert!(!is_extended(&entry(0x00)));
    assert!(!is_extended(&entry(0xEF)));
}
