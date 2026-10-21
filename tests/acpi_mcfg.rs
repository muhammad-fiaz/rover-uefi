//! MCFG tests.

use rover_uefi::protocols::acpi::mcfg::mcfg_iter;

#[test]
fn mcfg_entries() {
    let mut buf = [0_u8; 52];
    buf[36..44].copy_from_slice(&0xE000_0000_u64.to_le_bytes());
    buf[44..46].copy_from_slice(&0_u16.to_le_bytes());
    buf[46] = 0;
    buf[47] = 0xFF;
    let Some(mut iter) = mcfg_iter(&buf) else {
        panic!("expected iterator");
    };
    let Some(entry) = iter.next() else {
        panic!("expected entry");
    };
    assert_eq!(entry.base_address, 0xE000_0000);
    assert_eq!(entry.end_bus, 0xFF);
}
