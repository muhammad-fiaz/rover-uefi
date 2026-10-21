//! USB configuration tests.

use rover_uefi::protocols::usb::descriptor::{descriptor_iter, parse_config_descriptor};

#[test]
fn configuration_header() {
    let mut buf = [0_u8; 9];
    buf[0] = 9;
    buf[1] = 2;
    buf[2..4].copy_from_slice(&18_u16.to_le_bytes());
    let Some(desc) = parse_config_descriptor(&buf) else {
        panic!("expected config descriptor");
    };
    assert_eq!(desc.total_length, 18);
    assert!(descriptor_iter(&[]).next().is_none());
}
