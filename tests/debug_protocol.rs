//! Debug protocol tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::debug::guid::{DEBUG_PORT_GUID, DEBUG_SUPPORT_GUID};
use rover_uefi::protocols::debug::protocol::{EfiDebugPortProtocol, EfiDebugSupportProtocol};
use rover_uefi::protocols::debug::types::{
    EXCEPT_BREAKPOINT, EXCEPT_PAGE_FAULT, InstructionSetArchitecture, is_defined_x64_exception,
    validate_processor_index,
};

#[test]
fn guid_bytes() {
    assert_eq!(
        DEBUG_SUPPORT_GUID.to_bytes(),
        [
            0x0C, 0x59, 0x55, 0x27, 0x3C, 0x6F, 0xFA, 0x42, 0x9E, 0xA4, 0xA3, 0xBA, 0x54, 0x3C,
            0xDA, 0x25
        ]
    );
    assert_eq!(
        DEBUG_PORT_GUID.to_bytes(),
        [
            0xD2, 0xE8, 0xA4, 0xEB, 0x58, 0x38, 0xEC, 0x41, 0xA2, 0x81, 0x26, 0x47, 0xBA, 0x96,
            0x60, 0xD0
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(DEBUG_SUPPORT_GUID.to_bytes()),
        DEBUG_SUPPORT_GUID
    );
    assert_ne!(DEBUG_SUPPORT_GUID, DEBUG_PORT_GUID);
}

#[test]
fn debug_support_layout() {
    assert_eq!(size_of::<EfiDebugSupportProtocol>(), 5 * size_of::<usize>());
    assert_eq!(offset_of!(EfiDebugSupportProtocol, isa), 0);
    assert_eq!(
        offset_of!(EfiDebugSupportProtocol, get_maximum_processor_index),
        size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiDebugSupportProtocol, invalidate_instruction_cache),
        4 * size_of::<usize>()
    );
}

#[test]
fn debug_port_layout() {
    assert_eq!(size_of::<EfiDebugPortProtocol>(), 4 * size_of::<usize>());
    assert_eq!(offset_of!(EfiDebugPortProtocol, reset), 0);
    assert_eq!(
        offset_of!(EfiDebugPortProtocol, poll),
        3 * size_of::<usize>()
    );
}

#[test]
fn types_and_validation() {
    assert_eq!(InstructionSetArchitecture::X64.to_raw(), 0x8664);
    assert_eq!(InstructionSetArchitecture::AARCH64.to_raw(), 0xAA64);
    assert_eq!(EXCEPT_BREAKPOINT, 3);
    assert_eq!(EXCEPT_PAGE_FAULT, 14);
    assert!(is_defined_x64_exception(14));
    assert!(!is_defined_x64_exception(7));
    assert!(!is_defined_x64_exception(9));
    assert!(validate_processor_index(0, 0));
    assert!(validate_processor_index(3, 7));
    assert!(!validate_processor_index(8, 7));
}
