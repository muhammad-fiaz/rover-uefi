//! USB endpoint tests.

use rover_uefi::protocols::usb::descriptor::parse_endpoint_descriptor;
use rover_uefi::protocols::usb::endpoint::{is_in, transfer_type};

#[test]
fn endpoint_parsing() {
    let mut buf = [0_u8; 7];
    buf[0] = 7;
    buf[1] = 5;
    buf[2] = 0x81;
    buf[3] = 2;
    buf[4..6].copy_from_slice(&64_u16.to_le_bytes());
    let Some(desc) = parse_endpoint_descriptor(&buf) else {
        panic!("expected endpoint descriptor");
    };
    assert!(is_in(desc.endpoint_address));
    assert_eq!(transfer_type(desc.attributes), 2);
}
