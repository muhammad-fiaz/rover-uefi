// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::cast_sign_loss)]
//! Integration tests for USB HID keyboard, mouse, and absolute pointer decoders.

use rover_uefi::abi::types::{EFI_FALSE, EFI_TRUE};
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::console::absolute_pointer::{
    ABSOLUTE_POINTER_ALT_ACTIVE, ABSOLUTE_POINTER_TOUCH_ACTIVE,
};
use rover_uefi::protocols::console::input::{SCAN_HOME, SCAN_NULL};
use rover_uefi::protocols::console::input_ex::{SCAN_F12, SCAN_F24, SCAN_VOLUME_UP};
use rover_uefi::protocols::console::usb_keyboard::{
    USB_KBD_LED_CAPS_LOCK, USB_KBD_LED_NUM_LOCK, USB_KBD_MOD_LEFT_SHIFT, USB_USAGE_ERROR_ROLLOVER,
    USB_USAGE_KEY_A, USB_USAGE_KEY_CAPS_LOCK, USB_USAGE_KEYPAD_1, USB_USAGE_KEYPAD_NUM_LOCK,
    UsbKeyboardDecoder,
};
use rover_uefi::protocols::console::usb_mouse::{
    USB_MOUSE_BTN_LEFT, USB_MOUSE_BTN_MIDDLE, UsbAbsolutePointerDecoder, UsbMouseDecoder,
};

#[test]
fn test_usb_keyboard_basic_report() {
    let mut decoder = UsbKeyboardDecoder::new();

    // Report: No modifiers, Key 'A' (usage 0x04) pressed
    let report = [0x00, 0x00, USB_USAGE_KEY_A, 0x00, 0x00, 0x00, 0x00, 0x00];
    let count = decoder.process_report(&report);
    assert_eq!(count, 1);

    let key = decoder.read_key_stroke().unwrap();
    assert_eq!(key.key.scan_code, SCAN_NULL);
    assert_eq!(key.key.unicode_char, 0x0061); // 'a'

    // Empty report (key released)
    let release_report = [0u8; 8];
    assert_eq!(decoder.process_report(&release_report), 0);
    assert_eq!(decoder.read_key_stroke(), None);
}

#[test]
fn test_usb_keyboard_modifiers_and_shift() {
    let mut decoder = UsbKeyboardDecoder::new();

    // Report: Left Shift held, Key 'A' pressed -> 'A'
    let report = [
        USB_KBD_MOD_LEFT_SHIFT,
        0x00,
        USB_USAGE_KEY_A,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    decoder.process_report(&report);

    let key = decoder.read_key_stroke().unwrap();
    assert_eq!(key.key.unicode_char, 0x0041); // 'A'

    // Shifted digit '1' (usage 0x1E) -> '!'
    let report_excl = [
        USB_KBD_MOD_LEFT_SHIFT,
        0x00,
        0x1E,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    decoder.process_report(&report_excl);
    let key_excl = decoder.read_key_stroke().unwrap();
    assert_eq!(key_excl.key.unicode_char, 0x0021); // '!'
}

#[test]
fn test_usb_keyboard_toggles_and_leds() {
    let mut decoder = UsbKeyboardDecoder::new();
    assert!(decoder.num_lock); // NumLock is on by default
    assert_eq!(decoder.led_report(), USB_KBD_LED_NUM_LOCK);

    // Press Caps Lock (usage 0x39)
    let report_caps = [
        0x00,
        0x00,
        USB_USAGE_KEY_CAPS_LOCK,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    decoder.process_report(&report_caps);
    assert!(decoder.caps_lock);
    assert_eq!(
        decoder.led_report(),
        USB_KBD_LED_NUM_LOCK | USB_KBD_LED_CAPS_LOCK
    );

    // Letter 'B' (usage 0x05) with Caps Lock on -> uppercase 'B'
    let report_b = [
        0x00,
        0x00,
        USB_USAGE_KEY_A + 1,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    decoder.process_report(&report_b);
    let key_b = decoder.read_key_stroke().unwrap();
    assert_eq!(key_b.key.unicode_char, 0x0042); // 'B'
}

#[test]
fn test_usb_keyboard_keypad_modes() {
    let mut decoder = UsbKeyboardDecoder::new();
    assert!(decoder.num_lock);

    // Keypad '7' (usage 0x5F) with NumLock -> digit '7'
    let report_kp7 = [
        0x00,
        0x00,
        USB_USAGE_KEYPAD_1 + 6,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    decoder.process_report(&report_kp7);
    let k7 = decoder.read_key_stroke().unwrap();
    assert_eq!(k7.key.unicode_char, 0x0037);

    // Release and toggle NumLock off
    decoder.process_report(&[0u8; 8]);
    let report_numlock = [
        0x00,
        0x00,
        USB_USAGE_KEYPAD_NUM_LOCK,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    decoder.process_report(&report_numlock);
    assert!(!decoder.num_lock);

    // Keypad '7' without NumLock -> Home scan code
    decoder.process_report(&[0u8; 8]);
    decoder.process_report(&report_kp7);
    let k_home = decoder.read_key_stroke().unwrap();
    assert_eq!(k_home.key.scan_code, SCAN_HOME);
    assert_eq!(k_home.key.unicode_char, 0);
}

#[test]
fn test_usb_keyboard_function_and_multimedia() {
    let mut decoder = UsbKeyboardDecoder::new();

    // F12 (usage 0x45)
    let report_f12 = [0x00, 0x00, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00];
    decoder.process_report(&report_f12);
    assert_eq!(decoder.read_key_stroke().unwrap().key.scan_code, SCAN_F12);

    // F24 (usage 0x73)
    decoder.process_report(&[0u8; 8]);
    let report_f24 = [0x00, 0x00, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00];
    decoder.process_report(&report_f24);
    assert_eq!(decoder.read_key_stroke().unwrap().key.scan_code, SCAN_F24);

    // Volume Up (usage 0x80)
    decoder.process_report(&[0u8; 8]);
    let report_vol = [0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00];
    decoder.process_report(&report_vol);
    assert_eq!(
        decoder.read_key_stroke().unwrap().key.scan_code,
        SCAN_VOLUME_UP
    );
}

#[test]
fn test_usb_keyboard_rollover_error() {
    let mut decoder = UsbKeyboardDecoder::new();

    // Report contains error rollover
    let report_err = [
        0x00,
        0x00,
        USB_USAGE_ERROR_ROLLOVER,
        0x04,
        0x05,
        0x06,
        0x07,
        0x08,
    ];
    let count = decoder.process_report(&report_err);
    assert_eq!(count, 0);
    assert_eq!(decoder.read_key_stroke(), None);
}

#[test]
fn test_usb_mouse_relative_decoder() {
    let mut decoder = UsbMouseDecoder::new();

    // 4-byte report: Left button + middle button, dX = +12, dY = -8, dZ = +2
    let report = [
        USB_MOUSE_BTN_LEFT | USB_MOUSE_BTN_MIDDLE,
        12u8,
        (-8i8) as u8,
        2u8,
    ];

    decoder.process_report(&report).unwrap();
    assert!(decoder.has_state_changed());

    let state = decoder.get_state().unwrap();
    assert_eq!(state.relative_movement_x, 12);
    assert_eq!(state.relative_movement_y, -8);
    assert_eq!(state.relative_movement_z, 2);
    assert_eq!(state.left_button, EFI_TRUE);
    assert_eq!(state.right_button, EFI_FALSE);

    // Immediate second get_state() returns NOT_READY
    assert_eq!(decoder.get_state(), Err(EfiStatus::NOT_READY));
}

#[test]
fn test_usb_absolute_pointer_decoder() {
    let mut decoder = UsbAbsolutePointerDecoder::new(1920, 1080, 100);
    assert_eq!(decoder.mode().absolute_max_x, 1920);
    assert_eq!(decoder.mode().absolute_max_y, 1080);

    // Centered initially
    let initial = decoder.get_state();
    // Initially not changed until an event happens
    assert_eq!(initial, Err(EfiStatus::NOT_READY));

    // Ingest relative motion: dX = +100, dY = -50, Left button pressed (touch active)
    let report = [USB_MOUSE_BTN_LEFT, 100u8, (-50i8) as u8, 0u8];
    decoder.process_relative_report(&report).unwrap();
    assert!(decoder.has_state_changed());

    let state = decoder.get_state().unwrap();
    assert_eq!(state.current_x, 1920 / 2 + 100);
    assert_eq!(state.current_y, 1080 / 2 - 50);
    assert_eq!(state.active_buttons, ABSOLUTE_POINTER_TOUCH_ACTIVE);

    // Direct absolute coordinate touch
    decoder.process_absolute_coords(
        300,
        400,
        50,
        ABSOLUTE_POINTER_TOUCH_ACTIVE | ABSOLUTE_POINTER_ALT_ACTIVE,
    );
    let touch_state = decoder.get_state().unwrap();
    assert_eq!(touch_state.current_x, 300);
    assert_eq!(touch_state.current_y, 400);
    assert_eq!(touch_state.current_z, 50);
    assert_eq!(
        touch_state.active_buttons,
        ABSOLUTE_POINTER_TOUCH_ACTIVE | ABSOLUTE_POINTER_ALT_ACTIVE
    );
}
