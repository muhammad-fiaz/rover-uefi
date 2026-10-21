//! USB enumeration tests.

use rover_uefi::protocols::usb::device::{
    CLASS_HID, CLASS_HUB, CLASS_MASS_STORAGE, is_hid, is_hub, is_mass_storage,
};

#[test]
fn class_helpers() {
    assert!(is_hub(0x09));
    assert!(is_mass_storage(0x08));
    assert!(is_hid(0x03));
    assert!(!is_hub(0x08));
    assert!(!is_mass_storage(0x09));
    assert!(!is_hid(0x08));
    assert_eq!(CLASS_HUB, 0x09);
    assert_eq!(CLASS_MASS_STORAGE, 0x08);
    assert_eq!(CLASS_HID, 0x03);
}

#[test]
fn class_boundary_values() {
    assert!(!is_hub(0));
    assert!(!is_hub(0xFF));
    assert!(!is_mass_storage(0));
    assert!(!is_hid(0));
}
