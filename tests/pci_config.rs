//! PCI configuration tests.

use rover_uefi::protocols::pci::header::{is_multifunction, parse_header};

#[test]
fn header_parsing() {
    let mut buf = [0_u8; 64];
    buf[0..2].copy_from_slice(&0x8086_u16.to_le_bytes());
    buf[2..4].copy_from_slice(&0x100E_u16.to_le_bytes());
    buf[0x0B] = 0x02;
    buf[0x0E] = 0x00;
    let Some(header) = parse_header(&buf) else {
        panic!("expected header");
    };
    assert_eq!(header.vendor_id, 0x8086);
    assert_eq!(header.device_id, 0x100E);
    assert_eq!(header.base_class, 0x02);
    assert!(!is_multifunction(&header));
    assert!(parse_header(&[0_u8; 10]).is_none());
}

#[test]
fn header_types() {
    let mut buf = [0_u8; 128];
    buf[0x0E] = 0x01;
    assert!(parse_header(&buf).is_some());
    let mut bad = [0_u8; 64];
    bad[0x0E] = 0x7F;
    assert!(parse_header(&bad).is_none());
}
