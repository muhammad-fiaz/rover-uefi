//! USB descriptor tests.

use rover_uefi::protocols::usb::descriptor::{
    descriptor_iter, parse_config_descriptor, parse_device_descriptor,
};

#[test]
fn device_descriptor() {
    let mut buf = [0_u8; 18];
    buf[0] = 18;
    buf[1] = 1;
    buf[2..4].copy_from_slice(&0x0200_u16.to_le_bytes());
    buf[8..10].copy_from_slice(&0x8086_u16.to_le_bytes());
    buf[10..12].copy_from_slice(&0x100E_u16.to_le_bytes());
    let Some(desc) = parse_device_descriptor(&buf) else {
        panic!("expected device descriptor");
    };
    assert_eq!(desc.id_vendor, 0x8086);
    assert_eq!(desc.id_product, 0x100E);
    assert!(parse_device_descriptor(&[0_u8; 4]).is_none());
}

#[test]
fn config_descriptor() {
    let mut buf = [0_u8; 9];
    buf[0] = 9;
    buf[1] = 2;
    buf[2..4].copy_from_slice(&25_u16.to_le_bytes());
    let Some(desc) = parse_config_descriptor(&buf) else {
        panic!("expected config descriptor");
    };
    assert_eq!(desc.total_length, 25);
}

#[test]
fn descriptor_chain() {
    let bytes: [u8; 18] = [9, 2, 25, 0, 1, 1, 0, 0x80, 50, 9, 4, 0, 0, 1, 3, 0, 0, 0];
    let mut iter = descriptor_iter(&bytes);
    let first = iter.next();
    assert_eq!(first.map(|desc| desc.descriptor_type()), Some(2));
    let second = iter.next();
    assert_eq!(second.map(|desc| desc.descriptor_type()), Some(4));
    assert!(iter.next().is_none());
}
