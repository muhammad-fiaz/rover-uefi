//! FADT tests.

use rover_uefi::protocols::acpi::fadt::parse_fadt;

#[test]
fn fadt_parsing() {
    let mut buf = [0_u8; 116];
    buf[36..40].copy_from_slice(&0x1111_0000_u32.to_le_bytes());
    buf[40..44].copy_from_slice(&0x2222_0000_u32.to_le_bytes());
    buf[45] = 1;
    let Some(info) = parse_fadt(&buf) else {
        panic!("expected fadt");
    };
    assert_eq!(info.firmware_control, 0x1111_0000);
    assert_eq!(info.dsdt_address, 0x2222_0000);
    assert!(parse_fadt(&[0_u8; 10]).is_none());
}
