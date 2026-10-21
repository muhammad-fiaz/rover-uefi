//! Diagnostic protocol tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::diagnostic::guid::{
    STATUS_CODE_RUNTIME_GUID, TIMER_ARCH_GUID, WATCHDOG_TIMER_ARCH_GUID,
};
use rover_uefi::protocols::diagnostic::status_code::EfiStatusCodeProtocol;
use rover_uefi::protocols::diagnostic::timer::{
    EfiTimerArchProtocol, EfiWatchdogTimerArchProtocol,
};
use rover_uefi::protocols::diagnostic::types::{
    CLASS_SOFTWARE, ERROR_MAJOR, EfiStatusCodeData, PROGRESS_CODE, status_code_subclass,
    validate_status_code_data, validate_status_code_value,
};

#[test]
fn guid_bytes() {
    assert_eq!(
        TIMER_ARCH_GUID.to_bytes(),
        [
            0xB3, 0xCC, 0xBA, 0x26, 0x42, 0x6F, 0xD4, 0x11, 0xBC, 0xE7, 0x00, 0x80, 0xC7, 0x3C,
            0x88, 0x81
        ]
    );
    assert_eq!(
        WATCHDOG_TIMER_ARCH_GUID.to_bytes(),
        [
            0xF5, 0x3F, 0x5E, 0x66, 0xCC, 0x46, 0xD4, 0x11, 0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
    assert_eq!(
        STATUS_CODE_RUNTIME_GUID.to_bytes(),
        [
            0x28, 0xB8, 0xB2, 0xD2, 0x26, 0x08, 0xA7, 0x48, 0xB3, 0xDF, 0x98, 0x3C, 0x00, 0x60,
            0x24, 0xF0
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(TIMER_ARCH_GUID.to_bytes()),
        TIMER_ARCH_GUID
    );
}

#[test]
fn protocol_layouts() {
    assert_eq!(size_of::<EfiStatusCodeProtocol>(), size_of::<usize>());
    assert_eq!(size_of::<EfiTimerArchProtocol>(), 4 * size_of::<usize>());
    assert_eq!(
        size_of::<EfiWatchdogTimerArchProtocol>(),
        3 * size_of::<usize>()
    );
    assert_eq!(offset_of!(EfiTimerArchProtocol, register_handler), 0);
    assert_eq!(
        offset_of!(EfiTimerArchProtocol, generate_soft_interrupt),
        3 * size_of::<usize>()
    );
    assert_eq!(size_of::<EfiStatusCodeData>(), 20);
}

#[test]
fn status_code_helpers() {
    assert_eq!(PROGRESS_CODE, 1);
    assert_eq!(ERROR_MAJOR, 0x8000_0000);
    assert_eq!(CLASS_SOFTWARE, 0x0300_0000);
    assert_eq!(status_code_subclass(0x0301_0100), 1);
    assert_eq!(status_code_subclass(0x0300_0001), 0);
    assert!(validate_status_code_value(0x0300_0001));
    assert!(!validate_status_code_value(0));
    assert!(validate_status_code_data(20, 0, 20));
    assert!(validate_status_code_data(20, 4, 24));
    assert!(!validate_status_code_data(10, 4, 24));
    assert!(!validate_status_code_data(20, 100, 24));
}
