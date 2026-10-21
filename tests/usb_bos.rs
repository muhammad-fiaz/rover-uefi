//! USB BOS tests.

use rover_uefi::protocols::usb::descriptor::descriptor_iter;

#[test]
fn bos_iteration() {
    let bytes: [u8; 10] = [5, 0x0F, 10, 0, 1, 5, 0x10, 2, 0, 0];
    let mut iter = descriptor_iter(&bytes);
    let first = iter.next();
    assert_eq!(first.map(|desc| desc.descriptor_type()), Some(0x0F));
    let second = iter.next();
    assert_eq!(second.map(|desc| desc.descriptor_type()), Some(0x10));
}
