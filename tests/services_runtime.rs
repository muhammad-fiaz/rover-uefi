//! Tests for runtime services helpers.
//!
//! The tests cover constants, validation, and
//! layout checks that do not require firmware.
//! Mock forwarding is exercised through pure
//! helpers where firmware is not available.

use core::mem::{align_of, offset_of, size_of};

use rover_uefi::abi::types::{Char16, UIntN};
use rover_uefi::base::{EFI_TIME_ADJUST_DAYLIGHT, EfiStatus, ResetType};
use rover_uefi::services::runtime::capsules::{checked_capsule_array_size, validate_capsule_count};
use rover_uefi::services::runtime::variables::{
    VARIABLE_APPEND_WRITE, VARIABLE_NON_VOLATILE, VARIABLE_RUNTIME_ACCESS, validate_variable_name,
    variable_name_bytes,
};
use rover_uefi::services::runtime::virtual_memory::{
    OPTIONAL_PTR, checked_map_size, descriptor_count, validate_descriptor_size,
};
use rover_uefi::tables::runtime::{
    CAPSULE_FLAGS_PERSIST_ACROSS_RESET, EfiCapsuleHeader, EfiConvertPointer, EfiGetTime,
    EfiGetVariable, EfiQueryVariableInfo, EfiResetSystem, EfiSetTime, EfiSetVirtualAddressMap,
    EfiUpdateCapsule,
};

#[test]
fn time_constants() {
    assert_eq!(EFI_TIME_ADJUST_DAYLIGHT, 0x01);
}

#[test]
fn variable_attributes() {
    assert_eq!(VARIABLE_NON_VOLATILE, 0x0000_0001);
    assert_eq!(VARIABLE_RUNTIME_ACCESS, 0x0000_0004);
    assert_eq!(VARIABLE_APPEND_WRITE, 0x0000_0040);
}

#[test]
fn utf16_name_helpers() {
    let name: [Char16; 3] = [0x41, 0x00, 0x00];
    assert!(validate_variable_name(&name));
    let empty: [Char16; 0] = [];
    assert!(!validate_variable_name(&empty));
    let no_term: [Char16; 2] = [0x41, 0x42];
    assert!(!validate_variable_name(&no_term));
    assert_eq!(variable_name_bytes(&name), Some(6));
    assert_eq!(variable_name_bytes(&empty), Some(0));
}

#[test]
fn descriptor_helpers() {
    assert!(validate_descriptor_size(40));
    assert!(validate_descriptor_size(48));
    assert!(!validate_descriptor_size(0));
    assert!(!validate_descriptor_size(32));
    assert_eq!(descriptor_count(80, 40), Some(2));
    assert_eq!(descriptor_count(81, 40), None);
    assert_eq!(descriptor_count(80, 0), None);
    assert_eq!(checked_map_size(2, 40), Some(80));
    assert_eq!(OPTIONAL_PTR, 1);
}

#[test]
fn capsule_helpers() {
    assert!(validate_capsule_count(1));
    assert!(!validate_capsule_count(0));
    assert_eq!(
        checked_capsule_array_size(2),
        Some(2 * core::mem::size_of::<*mut EfiCapsuleHeader>())
    );
    assert_eq!(CAPSULE_FLAGS_PERSIST_ACROSS_RESET, 0x0001_0000);
}

#[test]
fn capsule_layout() {
    assert_eq!(size_of::<EfiCapsuleHeader>(), 28);
    assert_eq!(align_of::<EfiCapsuleHeader>(), 4);
    assert_eq!(offset_of!(EfiCapsuleHeader, capsule_guid), 0);
    assert_eq!(offset_of!(EfiCapsuleHeader, header_size), 16);
    assert_eq!(offset_of!(EfiCapsuleHeader, flags), 20);
    assert_eq!(offset_of!(EfiCapsuleHeader, capsule_image_size), 24);
}

#[test]
fn reset_values() {
    assert_eq!(ResetType::COLD.to_raw(), 0);
    assert_eq!(ResetType::PLATFORM_SPECIFIC.to_raw(), 3);
}

#[test]
fn function_pointer_sizes() {
    assert_eq!(size_of::<EfiGetTime>(), size_of::<usize>());
    assert_eq!(size_of::<EfiSetTime>(), size_of::<usize>());
    assert_eq!(size_of::<EfiGetVariable>(), size_of::<usize>());
    assert_eq!(size_of::<EfiSetVirtualAddressMap>(), size_of::<usize>());
    assert_eq!(size_of::<EfiConvertPointer>(), size_of::<usize>());
    assert_eq!(size_of::<EfiResetSystem>(), size_of::<usize>());
    assert_eq!(size_of::<EfiUpdateCapsule>(), size_of::<usize>());
    assert_eq!(size_of::<EfiQueryVariableInfo>(), size_of::<usize>());
    assert_eq!(size_of::<UIntN>(), size_of::<usize>());
}

#[test]
fn status_isolation() {
    assert!(EfiStatus::SUCCESS.is_success());
    assert!(EfiStatus::BUFFER_TOO_SMALL.is_error());
}
