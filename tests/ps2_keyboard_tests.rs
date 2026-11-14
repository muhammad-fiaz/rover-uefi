// SPDX-License-Identifier: Apache-2.0
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
//! Integration tests for PS/2 keyboard controller and scancode decoder.

use rover_uefi::protocols::console::input::{
    CHAR_BACKSPACE, CHAR_CARRIAGE_RETURN, CHAR_TAB, SCAN_DOWN, SCAN_ESC, SCAN_F1, SCAN_F10,
    SCAN_HOME, SCAN_LEFT, SCAN_NULL, SCAN_RIGHT, SCAN_UP,
};
use rover_uefi::protocols::console::input_ex::{
    EFI_CAPS_LOCK_ACTIVE, EFI_SCROLL_LOCK_ACTIVE, EFI_SHIFT_STATE_VALID, EFI_TOGGLE_STATE_VALID,
    SCAN_F11, SCAN_F12, SCAN_PAUSE,
};
use rover_uefi::protocols::console::ps2_keyboard::{
    EfiKeyQueue, KBC_CMD_DISABLE_KEYBOARD, KBC_CMD_DISABLE_MOUSE, KBC_CMD_ENABLE_KEYBOARD,
    KBC_CMD_ENABLE_MOUSE, KBC_CMD_READ_CONFIG_BYTE, KBC_CMD_SELF_TEST, KBC_CMD_WRITE_CONFIG_BYTE,
    KBC_CMD_WRITE_MOUSE, KBD_CMD_ENABLE_SCANNING, KBD_CMD_RESET, KBD_CMD_SET_LEDS, KBD_RESP_ACK,
    KBD_RESP_BAT_SUCCESS, KEYBOARD_SCAN_CODE_MAX_COUNT, PS2_COMMAND_PORT, PS2_DATA_PORT,
    PS2_LED_CAPS_LOCK, PS2_LED_NUM_LOCK, PS2_LED_SCROLL_LOCK, PS2_STATUS_PORT, Ps2KeyboardDecoder,
    SCANCODE_ALT_MAKE, SCANCODE_CAPS_LOCK_MAKE, SCANCODE_CTRL_MAKE, SCANCODE_DELETE_MAKE,
    SCANCODE_EXTENDED0, SCANCODE_EXTENDED1, SCANCODE_LEFT_SHIFT_MAKE, SCANCODE_NUM_LOCK_MAKE,
    SCANCODE_RIGHT_SHIFT_MAKE, ScancodeQueue,
};

#[test]
fn test_ps2_hardware_constants() {
    assert_eq!(PS2_DATA_PORT, 0x60);
    assert_eq!(PS2_STATUS_PORT, 0x64);
    assert_eq!(PS2_COMMAND_PORT, 0x64);

    assert_eq!(KBC_CMD_READ_CONFIG_BYTE, 0x20);
    assert_eq!(KBC_CMD_WRITE_CONFIG_BYTE, 0x60);
    assert_eq!(KBC_CMD_DISABLE_MOUSE, 0xA7);
    assert_eq!(KBC_CMD_ENABLE_MOUSE, 0xA8);
    assert_eq!(KBC_CMD_SELF_TEST, 0xAA);
    assert_eq!(KBC_CMD_DISABLE_KEYBOARD, 0xAD);
    assert_eq!(KBC_CMD_ENABLE_KEYBOARD, 0xAE);
    assert_eq!(KBC_CMD_WRITE_MOUSE, 0xD4);

    assert_eq!(KBD_CMD_SET_LEDS, 0xED);
    assert_eq!(KBD_CMD_ENABLE_SCANNING, 0xF4);
    assert_eq!(KBD_CMD_RESET, 0xFF);
    assert_eq!(KBD_RESP_BAT_SUCCESS, 0xAA);
    assert_eq!(KBD_RESP_ACK, 0xFA);
}

#[test]
fn test_scancode_queue() {
    let mut q = ScancodeQueue::new();
    assert!(q.is_empty());
    assert_eq!(q.len(), 0);

    for i in 0..10 {
        q.push(i as u8);
    }
    assert_eq!(q.len(), 10);
    assert_eq!(q.pop(), Some(0));
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.len(), 8);

    // Fill to capacity and check drop oldest behavior
    for i in 0..KEYBOARD_SCAN_CODE_MAX_COUNT {
        q.push(100 + i as u8);
    }
    assert_eq!(q.len(), KEYBOARD_SCAN_CODE_MAX_COUNT);

    q.clear();
    assert!(q.is_empty());
    assert_eq!(q.pop(), None);
}

#[test]
fn test_efi_key_queue() {
    let mut q = EfiKeyQueue::new();
    assert!(q.is_empty());

    let key = rover_uefi::protocols::console::input_ex::EfiKeyData {
        key: rover_uefi::protocols::console::input::EfiInputKey {
            scan_code: SCAN_NULL,
            unicode_char: 0x0061, // 'a'
        },
        key_state: rover_uefi::protocols::console::input_ex::EfiKeyState {
            key_shift_state: EFI_SHIFT_STATE_VALID,
            key_toggle_state: EFI_TOGGLE_STATE_VALID,
        },
    };

    q.push(key);
    assert_eq!(q.len(), 1);
    let popped = q.pop().unwrap();
    assert_eq!(popped.key.unicode_char, 0x0061);
    assert!(q.is_empty());
}

#[test]
fn test_ps2_decoder_basic_keys() {
    let mut decoder = Ps2KeyboardDecoder::new();

    // Key 'a' (make scancode 0x1E)
    let key = decoder.process_byte(0x1E).unwrap();
    assert_eq!(key.key.scan_code, SCAN_NULL);
    assert_eq!(key.key.unicode_char, 0x0061); // 'a'

    // Break code for 'a' (0x1E | 0x80 = 0x9E) -> should not produce keystroke
    assert!(decoder.process_byte(0x9E).is_none());

    // Digits '1' (0x02) and '0' (0x0B)
    let k1 = decoder.process_byte(0x02).unwrap();
    assert_eq!(k1.key.unicode_char, 0x0031); // '1'

    let k0 = decoder.process_byte(0x0B).unwrap();
    assert_eq!(k0.key.unicode_char, 0x0030); // '0'

    // Enter (0x1C), Escape (0x01), Tab (0x0F), Backspace (0x0E)
    let k_enter = decoder.process_byte(0x1C).unwrap();
    assert_eq!(k_enter.key.unicode_char, CHAR_CARRIAGE_RETURN);

    let k_esc = decoder.process_byte(0x01).unwrap();
    assert_eq!(k_esc.key.scan_code, SCAN_ESC);

    let k_tab = decoder.process_byte(0x0F).unwrap();
    assert_eq!(k_tab.key.unicode_char, CHAR_TAB);

    let k_bs = decoder.process_byte(0x0E).unwrap();
    assert_eq!(k_bs.key.unicode_char, CHAR_BACKSPACE);
}

#[test]
fn test_ps2_decoder_shift_and_capslock() {
    let mut decoder = Ps2KeyboardDecoder::new();

    // Press Left Shift (0x2A)
    assert!(decoder.process_byte(SCANCODE_LEFT_SHIFT_MAKE).is_none());
    assert!(decoder.left_shift);

    // 'a' while shifted -> 'A'
    let k_shift_a = decoder.process_byte(0x1E).unwrap();
    assert_eq!(k_shift_a.key.unicode_char, 0x0041); // 'A'

    // '1' while shifted -> '!' (0x21)
    let k_shift_1 = decoder.process_byte(0x02).unwrap();
    assert_eq!(k_shift_1.key.unicode_char, 0x0021); // '!'

    // Release Left Shift (0xAA)
    assert!(decoder.process_byte(0xAA).is_none());
    assert!(!decoder.left_shift);

    // Toggle Caps Lock (0x3A)
    assert!(decoder.process_byte(SCANCODE_CAPS_LOCK_MAKE).is_none());
    assert!(decoder.caps_lock);
    assert_eq!(decoder.led_flags() & PS2_LED_CAPS_LOCK, PS2_LED_CAPS_LOCK);

    // 'a' under Caps Lock -> 'A'
    let k_caps_a = decoder.process_byte(0x1E).unwrap();
    assert_eq!(k_caps_a.key.unicode_char, 0x0041);

    // '1' under Caps Lock is unchanged -> '1'
    let k_caps_1 = decoder.process_byte(0x02).unwrap();
    assert_eq!(k_caps_1.key.unicode_char, 0x0031);

    // Shift + Caps Lock on 'a' -> lowercase 'a'
    decoder.process_byte(SCANCODE_RIGHT_SHIFT_MAKE);
    let k_shift_caps_a = decoder.process_byte(0x1E).unwrap();
    assert_eq!(k_shift_caps_a.key.unicode_char, 0x0061);
    decoder.process_byte(0xB6); // Release Right Shift
}

#[test]
fn test_ps2_decoder_keypad_and_numlock() {
    let mut decoder = Ps2KeyboardDecoder::new();
    assert!(decoder.num_lock); // NumLock is on by default

    // Keypad 7 (0x47) with NumLock -> digit '7'
    let k7 = decoder.process_byte(0x47).unwrap();
    assert_eq!(k7.key.scan_code, SCAN_NULL);
    assert_eq!(k7.key.unicode_char, 0x0037);

    // Toggle NumLock off (0x45)
    decoder.process_byte(SCANCODE_NUM_LOCK_MAKE);
    assert!(!decoder.num_lock);
    assert_eq!(decoder.led_flags() & PS2_LED_NUM_LOCK, 0);

    // Keypad 7 without NumLock -> Home scan code
    let k_home = decoder.process_byte(0x47).unwrap();
    assert_eq!(k_home.key.scan_code, SCAN_HOME);
    assert_eq!(k_home.key.unicode_char, 0);

    // Keypad 8 -> Up, Keypad 4 -> Left, Keypad 6 -> Right, Keypad 2 -> Down
    assert_eq!(decoder.process_byte(0x48).unwrap().key.scan_code, SCAN_UP);
    assert_eq!(decoder.process_byte(0x4B).unwrap().key.scan_code, SCAN_LEFT);
    assert_eq!(
        decoder.process_byte(0x4D).unwrap().key.scan_code,
        SCAN_RIGHT
    );
    assert_eq!(decoder.process_byte(0x50).unwrap().key.scan_code, SCAN_DOWN);
}

#[test]
fn test_ps2_decoder_function_keys() {
    let mut decoder = Ps2KeyboardDecoder::new();

    // F1..F10 (0x3B..0x44)
    assert_eq!(decoder.process_byte(0x3B).unwrap().key.scan_code, SCAN_F1);
    assert_eq!(decoder.process_byte(0x44).unwrap().key.scan_code, SCAN_F10);

    // F11 (0x57) and F12 (0x58)
    assert_eq!(decoder.process_byte(0x57).unwrap().key.scan_code, SCAN_F11);
    assert_eq!(decoder.process_byte(0x58).unwrap().key.scan_code, SCAN_F12);
}

#[test]
fn test_ps2_decoder_extended_keys() {
    let mut decoder = Ps2KeyboardDecoder::new();

    // E0 prefix + 0x35 -> Keypad '/'
    assert!(decoder.process_byte(SCANCODE_EXTENDED0).is_none());
    let k_slash = decoder.process_byte(0x35).unwrap();
    assert_eq!(k_slash.key.unicode_char, 0x002F);

    // E0 prefix + 0x1C -> Keypad Enter
    assert!(decoder.process_byte(SCANCODE_EXTENDED0).is_none());
    let k_enter = decoder.process_byte(0x1C).unwrap();
    assert_eq!(k_enter.key.unicode_char, CHAR_CARRIAGE_RETURN);

    // E0 prefix + 0x1D -> Right Ctrl
    assert!(decoder.process_byte(SCANCODE_EXTENDED0).is_none());
    assert!(decoder.process_byte(SCANCODE_CTRL_MAKE).is_none());
    assert!(decoder.right_ctrl);

    // E0 prefix + 0x38 -> Right Alt
    assert!(decoder.process_byte(SCANCODE_EXTENDED0).is_none());
    assert!(decoder.process_byte(SCANCODE_ALT_MAKE).is_none());
    assert!(decoder.right_alt);

    // E0 prefix + 0x5B -> Left Logo
    assert!(decoder.process_byte(SCANCODE_EXTENDED0).is_none());
    assert!(decoder.process_byte(0x5B).is_none());
    assert!(decoder.left_logo);

    // E0 prefix + 0x46 -> Pause key
    assert!(decoder.process_byte(SCANCODE_EXTENDED0).is_none());
    let k_pause = decoder.process_byte(0x46).unwrap();
    assert_eq!(k_pause.key.scan_code, SCAN_PAUSE);
}

#[test]
fn test_ps2_decoder_pause_sequence() {
    let mut decoder = Ps2KeyboardDecoder::new();

    // Pause key sequence: E1 1D 45
    assert!(decoder.process_byte(SCANCODE_EXTENDED1).is_none());
    assert!(decoder.process_byte(0x1D).is_none());
    let pause_key = decoder.process_byte(SCANCODE_NUM_LOCK_MAKE).unwrap();
    assert_eq!(pause_key.key.scan_code, SCAN_PAUSE);
}

#[test]
fn test_ps2_decoder_ctrl_alt_del_detection() {
    let mut decoder = Ps2KeyboardDecoder::new();
    assert!(!decoder.ctrl_alt_del_triggered);

    // Press Ctrl
    decoder.process_byte(SCANCODE_CTRL_MAKE);
    // Press Alt
    decoder.process_byte(SCANCODE_ALT_MAKE);
    // Press Del (0x53)
    decoder.process_byte(SCANCODE_DELETE_MAKE);

    assert!(decoder.ctrl_alt_del_triggered);
}

#[test]
fn test_ps2_decoder_led_flags_and_toggle_state() {
    let mut decoder = Ps2KeyboardDecoder::new();
    // Default: NumLock active
    assert_eq!(decoder.led_flags(), PS2_LED_NUM_LOCK);

    // Set toggle state externally (ScrollLock + CapsLock)
    decoder
        .set_toggle_state(EFI_TOGGLE_STATE_VALID | EFI_SCROLL_LOCK_ACTIVE | EFI_CAPS_LOCK_ACTIVE);
    assert!(decoder.scroll_lock);
    assert!(decoder.caps_lock);
    assert!(!decoder.num_lock);
    assert_eq!(decoder.led_flags(), PS2_LED_SCROLL_LOCK | PS2_LED_CAPS_LOCK);
}
