//! Integration tests for Platform Virtual Keyboard protocol.

use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::console::input::EfiInputKey;
use rover_uefi::protocols::console::virtual_keyboard::{
    EfiPlatformVirtualKeyboardProtocol, PLATFORM_VIRTUAL_KEYBOARD_PROTOCOL_GUID,
    PlatformVirtualKeyboard, VIRTUAL_KEYBOARD_KEY_SIGNATURE, VirtualKbdKey,
};

static mut REGISTER_CALLED: bool = false;
static mut RESET_CALLED: bool = false;
static mut CLEAR_CALLED: bool = false;

unsafe extern "efiapi" fn mock_register() -> EfiStatus {
    unsafe {
        REGISTER_CALLED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_reset() -> EfiStatus {
    unsafe {
        RESET_CALLED = true;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_query(key: *mut VirtualKbdKey) -> Bool {
    if key.is_null() {
        return 0;
    }
    unsafe {
        (*key).key = EfiInputKey {
            scan_code: 0x01,
            unicode_char: 0,
        };
    }
    1
}

unsafe extern "efiapi" fn mock_clear(key: *mut VirtualKbdKey) -> EfiStatus {
    if key.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        CLEAR_CALLED = true;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_virtual_keyboard_guid_and_constants() {
    assert_eq!(
        PLATFORM_VIRTUAL_KEYBOARD_PROTOCOL_GUID,
        EfiGuid::new(
            0x0E36_06D2,
            0x1DC3,
            0x4E6F,
            [0xBE, 0x65, 0x39, 0x49, 0x82, 0xA2, 0x65, 0x47],
        )
    );
    assert_eq!(VIRTUAL_KEYBOARD_KEY_SIGNATURE, 0x6462_6B76);
}

#[test]
fn test_virtual_keyboard_client_wrapper() {
    let mock_proto = EfiPlatformVirtualKeyboardProtocol {
        register: mock_register,
        reset: mock_reset,
        query: mock_query,
        clear: mock_clear,
    };

    let vkbd = unsafe { PlatformVirtualKeyboard::from_raw(&raw const mock_proto) };

    assert!(vkbd.register().is_ok());
    assert!(unsafe { REGISTER_CALLED });

    assert!(vkbd.reset().is_ok());
    assert!(unsafe { RESET_CALLED });

    let mut key = VirtualKbdKey::default();
    assert_eq!(key.signature, VIRTUAL_KEYBOARD_KEY_SIGNATURE);

    assert!(vkbd.query(&mut key));
    assert_eq!(key.key.scan_code, 0x01);

    assert!(vkbd.clear(&mut key).is_ok());
    assert!(unsafe { CLEAR_CALLED });
}
