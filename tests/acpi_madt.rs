//! MADT tests.

use rover_uefi::protocols::acpi::madt::{madt_iter, parse_madt_header};

#[test]
fn madt_entries() {
    let mut buf = [0_u8; 50];
    buf[36..40].copy_from_slice(&0xFEE0_0000_u32.to_le_bytes());
    buf[44] = 0;
    buf[45] = 6;
    buf[46..50].copy_from_slice(&0x0102_0304_u32.to_le_bytes());
    let Some(header) = parse_madt_header(&buf) else {
        panic!("expected header");
    };
    assert_eq!(header.local_apic_address, 0xFEE0_0000);
    let Some(mut iter) = madt_iter(&buf) else {
        panic!("expected iterator");
    };
    let Some(entry) = iter.next() else {
        panic!("expected entry");
    };
    assert_eq!(entry.entry_type(), 0);
    assert!(iter.next().is_none());
}
