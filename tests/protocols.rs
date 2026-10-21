//! Tests for protocol infrastructure.
//!
//! The tests cover GUID values, layouts, UTF-16
//! validation, and device path parsing without
//! firmware.

use core::mem::{align_of, offset_of, size_of};

use rover_uefi::abi::types::Char16;
use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::console::input::{EfiInputKey, EfiSimpleTextInputProtocol, SCAN_F1};
use rover_uefi::protocols::console::output::{
    EfiSimpleTextOutputMode, EfiSimpleTextOutputProtocol,
};
use rover_uefi::protocols::console::text::{is_null_terminated, utf16_bytes};
use rover_uefi::protocols::core::{
    DEVICE_PATH_GUID, SIMPLE_TEXT_INPUT_GUID, SIMPLE_TEXT_OUTPUT_GUID,
};
use rover_uefi::protocols::device_path::{
    DEVICE_PATH_END, DEVICE_PATH_HARDWARE, EfiDevicePath, checked_node_end, device_path_instances,
    device_path_iter, node_count, validate_device_path,
};

#[test]
fn guid_values() {
    assert_eq!(
        SIMPLE_TEXT_INPUT_GUID.to_bytes(),
        [
            0xC1, 0x77, 0x74, 0x38, 0xC7, 0x69, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        SIMPLE_TEXT_OUTPUT_GUID.to_bytes(),
        [
            0xC2, 0x77, 0x74, 0x38, 0xC7, 0x69, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        DEVICE_PATH_GUID.to_bytes(),
        [
            0x91, 0x6E, 0x57, 0x09, 0x3F, 0x6D, 0xD2, 0x11, 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69,
            0x72, 0x3B
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(SIMPLE_TEXT_INPUT_GUID.to_bytes()),
        SIMPLE_TEXT_INPUT_GUID
    );
}

#[test]
fn console_layouts() {
    assert_eq!(
        size_of::<EfiSimpleTextInputProtocol>(),
        3 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiSimpleTextOutputProtocol>(),
        10 * core::mem::size_of::<usize>()
    );
    assert_eq!(size_of::<EfiInputKey>(), 4);
    assert_eq!(align_of::<EfiInputKey>(), 2);
    assert_eq!(offset_of!(EfiInputKey, scan_code), 0);
    assert_eq!(offset_of!(EfiInputKey, unicode_char), 2);
    assert_eq!(size_of::<EfiSimpleTextOutputMode>(), 24);
    assert_eq!(offset_of!(EfiSimpleTextOutputMode, cursor_visible), 20);
    assert_eq!(SCAN_F1, 0x000B);
}

#[test]
fn utf16_helpers() {
    let text: [Char16; 3] = [0x41, 0x42, 0x00];
    assert!(is_null_terminated(&text));
    assert_eq!(utf16_bytes(&text), Some(6));
    let bad: [Char16; 2] = [0x41, 0x42];
    assert!(!is_null_terminated(&bad));
}

#[test]
fn device_path_header() {
    assert_eq!(size_of::<EfiDevicePath>(), 4);
    assert_eq!(align_of::<EfiDevicePath>(), 1);
    assert_eq!(offset_of!(EfiDevicePath, device_type), 0);
    assert_eq!(offset_of!(EfiDevicePath, subtype), 1);
    assert_eq!(offset_of!(EfiDevicePath, length), 2);
    let header = EfiDevicePath::new(DEVICE_PATH_HARDWARE, 1, 6);
    assert_eq!(header.node_length(), 6);
    assert_eq!(header.device_type, DEVICE_PATH_HARDWARE);
    assert_eq!(checked_node_end(0, 6), Some(6));
    assert_eq!(checked_node_end(0, 3), None);
}

#[test]
fn device_path_valid() {
    let path: [u8; 20] = [
        1, 1, 6, 0, 0xAA, 0xBB, // PCI payload 2 bytes
        1, 4, 20, 0, 0xCC, 0xDD, 0xEE, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    ];
    // Use a well-formed two-node path ending with end-entire.
    let valid: [u8; 10] = [1, 1, 6, 0, 0xAA, 0xBB, 0x7F, 0xFF, 4, 0];
    assert!(validate_device_path(&valid));
    assert_eq!(node_count(&valid), Some(2));
    assert!(!validate_device_path(&path));
    assert!(!validate_device_path(&[]));
}

#[test]
fn device_path_truncated() {
    let truncated: [u8; 5] = [1, 1, 6, 0, 0xAA];
    assert!(!validate_device_path(&truncated));
    let mut iter = device_path_iter(&truncated);
    let Some(first) = iter.next() else {
        panic!("expected an error item");
    };
    assert!(first.is_err());
}

#[test]
fn device_path_instances_test() {
    let multi: [u8; 14] = [1, 1, 6, 0, 0xAA, 0xBB, 0x7F, 0x01, 4, 0, 0x7F, 0xFF, 4, 0];
    let mut iter = device_path_instances(&multi);
    let Some(first) = iter.next() else {
        panic!("expected a first instance");
    };
    assert_eq!(first.len(), 10);
    let Some(second) = iter.next() else {
        panic!("expected a second instance");
    };
    assert_eq!(second.len(), 4);
    assert!(iter.next().is_none());
    assert_eq!(DEVICE_PATH_END, 0x7F);
}
