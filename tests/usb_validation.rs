//! USB validation tests.

use rover_uefi::protocols::usb::usb_io::control::validate_control_length;
use rover_uefi::protocols::usb::usb_io::transfers::validate_endpoint;
use rover_uefi::protocols::usb::validation::validate_descriptor_length;

#[test]
fn length_checks() {
    assert!(validate_descriptor_length(18, 18));
    assert!(validate_descriptor_length(19, 18));
    assert!(!validate_descriptor_length(0, 2));
    assert!(!validate_descriptor_length(17, 18));
    assert!(!validate_descriptor_length(0, 0));
    assert!(validate_descriptor_length(1, 1));
}

#[test]
fn endpoint_validation() {
    assert!(validate_endpoint(0x01));
    assert!(validate_endpoint(0x81));
    assert!(validate_endpoint(0x8F));
    assert!(!validate_endpoint(0x00));
    assert!(!validate_endpoint(0x80));
    assert!(!validate_endpoint(0x10));
}

#[test]
fn control_length_checks() {
    assert!(validate_control_length(1));
    assert!(validate_control_length(64));
    assert!(!validate_control_length(0));
}
