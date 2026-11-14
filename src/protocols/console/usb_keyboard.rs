//! # USB HID Keyboard Driver and Report Decoder
//!
//! Production-ready implementation of the USB Human Interface Device (HID)
//! Boot Keyboard protocol and report decoder for UEFI systems.
//!
//! Conforms to the USB HID 1.11 Boot Keyboard specification and the UEFI
//! `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` and `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`.
//!
//! Provides report decoding, 6-key rollover handling, modifier mapping,
//! Caps/Num/Scroll Lock toggle tracking, LED output report formatting,
//! and standard UEFI keystroke buffering.

use crate::abi::types::Char16;
use crate::protocols::console::input::{
    CHAR_BACKSPACE, CHAR_CARRIAGE_RETURN, CHAR_TAB, EfiInputKey, SCAN_DELETE, SCAN_DOWN, SCAN_END,
    SCAN_ESC, SCAN_F1, SCAN_F2, SCAN_F3, SCAN_F4, SCAN_F5, SCAN_F6, SCAN_F7, SCAN_F8, SCAN_F9,
    SCAN_F10, SCAN_HOME, SCAN_INSERT, SCAN_LEFT, SCAN_NULL, SCAN_PAGE_DOWN, SCAN_PAGE_UP,
    SCAN_RIGHT, SCAN_UP,
};
use crate::protocols::console::input_ex::{
    EFI_CAPS_LOCK_ACTIVE, EFI_LEFT_ALT_PRESSED, EFI_LEFT_CONTROL_PRESSED, EFI_LEFT_LOGO_PRESSED,
    EFI_LEFT_SHIFT_PRESSED, EFI_MENU_KEY_PRESSED, EFI_NUM_LOCK_ACTIVE, EFI_RIGHT_ALT_PRESSED,
    EFI_RIGHT_CONTROL_PRESSED, EFI_RIGHT_LOGO_PRESSED, EFI_RIGHT_SHIFT_PRESSED,
    EFI_SCROLL_LOCK_ACTIVE, EFI_SHIFT_STATE_VALID, EFI_SYS_REQ_PRESSED, EFI_TOGGLE_STATE_VALID,
    EfiKeyData, EfiKeyState, SCAN_F11, SCAN_F12, SCAN_F13, SCAN_F14, SCAN_F15, SCAN_F16, SCAN_F17,
    SCAN_F18, SCAN_F19, SCAN_F20, SCAN_F21, SCAN_F22, SCAN_F23, SCAN_F24, SCAN_MUTE, SCAN_PAUSE,
    SCAN_VOLUME_DOWN, SCAN_VOLUME_UP,
};
use crate::protocols::console::ps2_keyboard::EfiKeyQueue;

// ---------------------------------------------------------------------------
// USB HID Boot Keyboard Modifier Bits (Report Byte 0)
// ---------------------------------------------------------------------------

/// Left Control key pressed.
pub const USB_KBD_MOD_LEFT_CTRL: u8 = 0x01;
/// Left Shift key pressed.
pub const USB_KBD_MOD_LEFT_SHIFT: u8 = 0x02;
/// Left Alt key pressed.
pub const USB_KBD_MOD_LEFT_ALT: u8 = 0x04;
/// Left GUI / Windows / Command key pressed.
pub const USB_KBD_MOD_LEFT_GUI: u8 = 0x08;
/// Right Control key pressed.
pub const USB_KBD_MOD_RIGHT_CTRL: u8 = 0x10;
/// Right Shift key pressed.
pub const USB_KBD_MOD_RIGHT_SHIFT: u8 = 0x20;
/// Right Alt key pressed.
pub const USB_KBD_MOD_RIGHT_ALT: u8 = 0x40;
/// Right GUI / Windows / Command key pressed.
pub const USB_KBD_MOD_RIGHT_GUI: u8 = 0x80;

// ---------------------------------------------------------------------------
// USB HID LED Output Report Bits
// ---------------------------------------------------------------------------

/// Num Lock indicator LED.
pub const USB_KBD_LED_NUM_LOCK: u8 = 0x01;
/// Caps Lock indicator LED.
pub const USB_KBD_LED_CAPS_LOCK: u8 = 0x02;
/// Scroll Lock indicator LED.
pub const USB_KBD_LED_SCROLL_LOCK: u8 = 0x04;
/// Compose indicator LED.
pub const USB_KBD_LED_COMPOSE: u8 = 0x08;
/// Kana indicator LED.
pub const USB_KBD_LED_KANA: u8 = 0x10;

// ---------------------------------------------------------------------------
// USB HID Usage IDs (Usage Page 0x07: Keyboard / Keypad)
// ---------------------------------------------------------------------------

/// No key pressed.
pub const USB_USAGE_RESERVED: u8 = 0x00;
/// Error `RollOver` (more than 6 keys pressed simultaneously).
pub const USB_USAGE_ERROR_ROLLOVER: u8 = 0x01;
/// `POSTFail` error condition.
pub const USB_USAGE_POST_FAIL: u8 = 0x02;
/// Error Undefined.
pub const USB_USAGE_ERROR_UNDEFINED: u8 = 0x03;

/// Letter 'A'.
pub const USB_USAGE_KEY_A: u8 = 0x04;
/// Letter 'Z'.
pub const USB_USAGE_KEY_Z: u8 = 0x1D;

/// Digit '1'.
pub const USB_USAGE_KEY_1: u8 = 0x1E;
/// Digit '0'.
pub const USB_USAGE_KEY_0: u8 = 0x27;

/// Return / Enter key.
pub const USB_USAGE_KEY_ENTER: u8 = 0x28;
/// Escape key.
pub const USB_USAGE_KEY_ESCAPE: u8 = 0x29;
/// Backspace / Delete key.
pub const USB_USAGE_KEY_BACKSPACE: u8 = 0x2A;
/// Tab key.
pub const USB_USAGE_KEY_TAB: u8 = 0x2B;
/// Spacebar.
pub const USB_USAGE_KEY_SPACE: u8 = 0x2C;
/// '-' and '_'.
pub const USB_USAGE_KEY_MINUS: u8 = 0x2D;
/// '=' and '+'.
pub const USB_USAGE_KEY_EQUAL: u8 = 0x2E;
/// '[' and '{'.
pub const USB_USAGE_KEY_LBRACKET: u8 = 0x2F;
/// ']' and '}'.
pub const USB_USAGE_KEY_RBRACKET: u8 = 0x30;
/// '\' and '|'.
pub const USB_USAGE_KEY_BACKSLASH: u8 = 0x31;
/// Non-US '#' and '~'.
pub const USB_USAGE_KEY_NON_US_HASH: u8 = 0x32;
/// ';' and ':'.
pub const USB_USAGE_KEY_SEMICOLON: u8 = 0x33;
/// '\'' and '"'.
pub const USB_USAGE_KEY_QUOTE: u8 = 0x34;
/// Grave accent and tilde.
pub const USB_USAGE_KEY_GRAVE: u8 = 0x35;
/// ',' and '<'.
pub const USB_USAGE_KEY_COMMA: u8 = 0x36;
/// '.' and '>'.
pub const USB_USAGE_KEY_PERIOD: u8 = 0x37;
/// '/' and '?'.
pub const USB_USAGE_KEY_SLASH: u8 = 0x38;

/// Caps Lock.
pub const USB_USAGE_KEY_CAPS_LOCK: u8 = 0x39;

/// Function key F1.
pub const USB_USAGE_KEY_F1: u8 = 0x3A;
/// Function key F12.
pub const USB_USAGE_KEY_F12: u8 = 0x45;

/// Print Screen / `SysRq`.
pub const USB_USAGE_KEY_PRINT_SCREEN: u8 = 0x46;
/// Scroll Lock.
pub const USB_USAGE_KEY_SCROLL_LOCK: u8 = 0x47;
/// Pause / Break.
pub const USB_USAGE_KEY_PAUSE: u8 = 0x48;

/// Insert.
pub const USB_USAGE_KEY_INSERT: u8 = 0x49;
/// Home.
pub const USB_USAGE_KEY_HOME: u8 = 0x4A;
/// Page Up.
pub const USB_USAGE_KEY_PAGE_UP: u8 = 0x4B;
/// Delete Forward.
pub const USB_USAGE_KEY_DELETE: u8 = 0x4C;
/// End.
pub const USB_USAGE_KEY_END: u8 = 0x4D;
/// Page Down.
pub const USB_USAGE_KEY_PAGE_DOWN: u8 = 0x4E;
/// Right Arrow.
pub const USB_USAGE_KEY_RIGHT_ARROW: u8 = 0x4F;
/// Left Arrow.
pub const USB_USAGE_KEY_LEFT_ARROW: u8 = 0x50;
/// Down Arrow.
pub const USB_USAGE_KEY_DOWN_ARROW: u8 = 0x51;
/// Up Arrow.
pub const USB_USAGE_KEY_UP_ARROW: u8 = 0x52;

/// Keypad Num Lock / Clear.
pub const USB_USAGE_KEYPAD_NUM_LOCK: u8 = 0x53;
/// Keypad '/'.
pub const USB_USAGE_KEYPAD_SLASH: u8 = 0x54;
/// Keypad '*'.
pub const USB_USAGE_KEYPAD_ASTERISK: u8 = 0x55;
/// Keypad '-'.
pub const USB_USAGE_KEYPAD_MINUS: u8 = 0x56;
/// Keypad '+'.
pub const USB_USAGE_KEYPAD_PLUS: u8 = 0x57;
/// Keypad Enter.
pub const USB_USAGE_KEYPAD_ENTER: u8 = 0x58;
/// Keypad digit '1'.
pub const USB_USAGE_KEYPAD_1: u8 = 0x59;
/// Keypad digit '0'.
pub const USB_USAGE_KEYPAD_0: u8 = 0x62;
/// Keypad '.' and Delete.
pub const USB_USAGE_KEYPAD_PERIOD: u8 = 0x63;
/// Application / Menu key.
pub const USB_USAGE_KEY_APPLICATION: u8 = 0x65;

/// Function key F13.
pub const USB_USAGE_KEY_F13: u8 = 0x68;
/// Function key F24.
pub const USB_USAGE_KEY_F24: u8 = 0x73;

/// Mute key.
pub const USB_USAGE_KEY_MUTE: u8 = 0x7F;
/// Volume up key.
pub const USB_USAGE_KEY_VOLUME_UP: u8 = 0x80;
/// Volume down key.
pub const USB_USAGE_KEY_VOLUME_DOWN: u8 = 0x81;

// ---------------------------------------------------------------------------
// USB HID Usage Mapping
// ---------------------------------------------------------------------------

/// Mapping entry from USB HID Usage ID to UEFI keystroke representation.
#[derive(Clone, Copy, Debug)]
pub struct UsbKeyMapping {
    /// USB HID Usage ID (Page 0x07).
    pub usage: u8,
    /// UEFI Scan Code (e.g. `SCAN_ESC`, `SCAN_F1`), or `SCAN_NULL`.
    pub efi_scan_code: u16,
    /// Unshifted Unicode character.
    pub unicode_char: Char16,
    /// Shifted Unicode character.
    pub shift_unicode_char: Char16,
}

/// Conversion lookup table for standard USB HID key usages.
pub static USB_KEY_MAP_TABLE: &[UsbKeyMapping] = &[
    UsbKeyMapping {
        usage: USB_USAGE_KEY_ENTER,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_CARRIAGE_RETURN,
        shift_unicode_char: CHAR_CARRIAGE_RETURN,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_ESCAPE,
        efi_scan_code: SCAN_ESC,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_BACKSPACE,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_BACKSPACE,
        shift_unicode_char: CHAR_BACKSPACE,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_TAB,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_TAB,
        shift_unicode_char: CHAR_TAB,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_SPACE,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0020,
        shift_unicode_char: 0x0020,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_MINUS,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002D,
        shift_unicode_char: 0x005F,
    }, // '-', '_'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_EQUAL,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x003D,
        shift_unicode_char: 0x002B,
    }, // '=', '+'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_LBRACKET,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x005B,
        shift_unicode_char: 0x007B,
    }, // '[', '{'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_RBRACKET,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x005D,
        shift_unicode_char: 0x007D,
    }, // ']', '}'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_BACKSLASH,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x005C,
        shift_unicode_char: 0x007C,
    }, // '\\', '|'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_SEMICOLON,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x003B,
        shift_unicode_char: 0x003A,
    }, // ';', ':'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_QUOTE,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0027,
        shift_unicode_char: 0x0022,
    }, // '\'', '"'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_GRAVE,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0060,
        shift_unicode_char: 0x007E,
    }, // '`', '~'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_COMMA,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002C,
        shift_unicode_char: 0x003C,
    }, // ',', '<'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_PERIOD,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002E,
        shift_unicode_char: 0x003E,
    }, // '.', '>'
    UsbKeyMapping {
        usage: USB_USAGE_KEY_SLASH,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002F,
        shift_unicode_char: 0x003F,
    }, // '/', '?'
    // Function keys
    UsbKeyMapping {
        usage: 0x3A,
        efi_scan_code: SCAN_F1,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x3B,
        efi_scan_code: SCAN_F2,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x3C,
        efi_scan_code: SCAN_F3,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x3D,
        efi_scan_code: SCAN_F4,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x3E,
        efi_scan_code: SCAN_F5,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x3F,
        efi_scan_code: SCAN_F6,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x40,
        efi_scan_code: SCAN_F7,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x41,
        efi_scan_code: SCAN_F8,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x42,
        efi_scan_code: SCAN_F9,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x43,
        efi_scan_code: SCAN_F10,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x44,
        efi_scan_code: SCAN_F11,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x45,
        efi_scan_code: SCAN_F12,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    // Controls & Navigation
    UsbKeyMapping {
        usage: USB_USAGE_KEY_PAUSE,
        efi_scan_code: SCAN_PAUSE,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_INSERT,
        efi_scan_code: SCAN_INSERT,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_HOME,
        efi_scan_code: SCAN_HOME,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_PAGE_UP,
        efi_scan_code: SCAN_PAGE_UP,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_DELETE,
        efi_scan_code: SCAN_DELETE,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_END,
        efi_scan_code: SCAN_END,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_PAGE_DOWN,
        efi_scan_code: SCAN_PAGE_DOWN,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_RIGHT_ARROW,
        efi_scan_code: SCAN_RIGHT,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_LEFT_ARROW,
        efi_scan_code: SCAN_LEFT,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_DOWN_ARROW,
        efi_scan_code: SCAN_DOWN,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_UP_ARROW,
        efi_scan_code: SCAN_UP,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    // Keypad symbols
    UsbKeyMapping {
        usage: USB_USAGE_KEYPAD_SLASH,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002F,
        shift_unicode_char: 0x002F,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEYPAD_ASTERISK,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002A,
        shift_unicode_char: 0x002A,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEYPAD_MINUS,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002D,
        shift_unicode_char: 0x002D,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEYPAD_PLUS,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002B,
        shift_unicode_char: 0x002B,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEYPAD_ENTER,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_CARRIAGE_RETURN,
        shift_unicode_char: CHAR_CARRIAGE_RETURN,
    },
    // Additional function keys F13..F24
    UsbKeyMapping {
        usage: 0x68,
        efi_scan_code: SCAN_F13,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x69,
        efi_scan_code: SCAN_F14,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x6A,
        efi_scan_code: SCAN_F15,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x6B,
        efi_scan_code: SCAN_F16,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x6C,
        efi_scan_code: SCAN_F17,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x6D,
        efi_scan_code: SCAN_F18,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x6E,
        efi_scan_code: SCAN_F19,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x6F,
        efi_scan_code: SCAN_F20,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x70,
        efi_scan_code: SCAN_F21,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x71,
        efi_scan_code: SCAN_F22,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x72,
        efi_scan_code: SCAN_F23,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: 0x73,
        efi_scan_code: SCAN_F24,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    // Multimedia
    UsbKeyMapping {
        usage: USB_USAGE_KEY_MUTE,
        efi_scan_code: SCAN_MUTE,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_VOLUME_UP,
        efi_scan_code: SCAN_VOLUME_UP,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
    UsbKeyMapping {
        usage: USB_USAGE_KEY_VOLUME_DOWN,
        efi_scan_code: SCAN_VOLUME_DOWN,
        unicode_char: 0,
        shift_unicode_char: 0,
    },
];

// ---------------------------------------------------------------------------
// USB Keyboard Decoder
// ---------------------------------------------------------------------------

/// Production-ready USB HID Boot Keyboard report decoder.
#[derive(Debug)]
pub struct UsbKeyboardDecoder {
    /// Previous report bytes for rollover diffing.
    last_report: [u8; 8],
    /// Current active modifiers bitmask.
    modifiers: u8,
    /// Caps Lock active.
    pub caps_lock: bool,
    /// Num Lock active.
    pub num_lock: bool,
    /// Scroll Lock active.
    pub scroll_lock: bool,
    /// Internal queue of decoded keystrokes.
    pub key_queue: EfiKeyQueue,
}

impl Default for UsbKeyboardDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbKeyboardDecoder {
    /// Creates a new USB keyboard decoder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            last_report: [0u8; 8],
            modifiers: 0,
            caps_lock: false,
            num_lock: true,
            scroll_lock: false,
            key_queue: EfiKeyQueue::new(),
        }
    }

    /// Resets the internal decoder state and clears the keystroke queue.
    pub fn reset(&mut self) {
        self.last_report = [0u8; 8];
        self.modifiers = 0;
        self.caps_lock = false;
        self.num_lock = true;
        self.scroll_lock = false;
        self.key_queue.clear();
    }

    /// Formats the USB HID output report byte for keyboard LED indicators.
    #[must_use]
    pub fn led_report(&self) -> u8 {
        let mut leds = 0u8;
        if self.num_lock {
            leds |= USB_KBD_LED_NUM_LOCK;
        }
        if self.caps_lock {
            leds |= USB_KBD_LED_CAPS_LOCK;
        }
        if self.scroll_lock {
            leds |= USB_KBD_LED_SCROLL_LOCK;
        }
        leds
    }

    /// Sets toggle state from an external toggle state byte.
    pub fn set_toggle_state(&mut self, state: u8) {
        if (state & EFI_TOGGLE_STATE_VALID) != 0 {
            self.scroll_lock = (state & EFI_SCROLL_LOCK_ACTIVE) != 0;
            self.num_lock = (state & EFI_NUM_LOCK_ACTIVE) != 0;
            self.caps_lock = (state & EFI_CAPS_LOCK_ACTIVE) != 0;
        }
    }

    /// Computes the current [`EfiKeyState`] representation.
    #[must_use]
    pub fn key_state(&self) -> EfiKeyState {
        let mut shift = EFI_SHIFT_STATE_VALID;
        if (self.modifiers & USB_KBD_MOD_LEFT_SHIFT) != 0 {
            shift |= EFI_LEFT_SHIFT_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_RIGHT_SHIFT) != 0 {
            shift |= EFI_RIGHT_SHIFT_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_LEFT_CTRL) != 0 {
            shift |= EFI_LEFT_CONTROL_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_RIGHT_CTRL) != 0 {
            shift |= EFI_RIGHT_CONTROL_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_LEFT_ALT) != 0 {
            shift |= EFI_LEFT_ALT_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_RIGHT_ALT) != 0 {
            shift |= EFI_RIGHT_ALT_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_LEFT_GUI) != 0 {
            shift |= EFI_LEFT_LOGO_PRESSED;
        }
        if (self.modifiers & USB_KBD_MOD_RIGHT_GUI) != 0 {
            shift |= EFI_RIGHT_LOGO_PRESSED;
        }

        let mut toggle = EFI_TOGGLE_STATE_VALID;
        if self.scroll_lock {
            toggle |= EFI_SCROLL_LOCK_ACTIVE;
        }
        if self.num_lock {
            toggle |= EFI_NUM_LOCK_ACTIVE;
        }
        if self.caps_lock {
            toggle |= EFI_CAPS_LOCK_ACTIVE;
        }

        EfiKeyState {
            key_shift_state: shift,
            key_toggle_state: toggle,
        }
    }

    /// Ingests an 8-byte USB HID Boot Keyboard input report.
    ///
    /// Diffs the keys against the previous report, updates toggle states,
    /// buffers new keystrokes in the internal queue, and returns the number
    /// of newly decoded keystrokes.
    pub fn process_report(&mut self, report: &[u8]) -> usize {
        if report.len() < 8 {
            return 0;
        }

        let new_modifiers = report[0];
        self.modifiers = new_modifiers;

        // Check for Error RollOver condition
        let mut has_rollover = false;
        for &usage in &report[2..8] {
            if usage == USB_USAGE_ERROR_ROLLOVER {
                has_rollover = true;
                break;
            }
        }
        if has_rollover {
            return 0;
        }

        let mut count = 0;
        let is_shifted = (new_modifiers & (USB_KBD_MOD_LEFT_SHIFT | USB_KBD_MOD_RIGHT_SHIFT)) != 0;

        // Find keys present in current report that were not in last report
        for &usage in &report[2..8] {
            if usage == USB_USAGE_RESERVED {
                continue;
            }

            let was_pressed = self.last_report[2..8].contains(&usage);
            if !was_pressed {
                // New key press
                if usage == USB_USAGE_KEY_CAPS_LOCK {
                    self.caps_lock = !self.caps_lock;
                    continue;
                }
                if usage == USB_USAGE_KEYPAD_NUM_LOCK {
                    self.num_lock = !self.num_lock;
                    continue;
                }
                if usage == USB_USAGE_KEY_SCROLL_LOCK {
                    self.scroll_lock = !self.scroll_lock;
                    continue;
                }

                if let Some(key_data) = self.decode_usage(usage, is_shifted) {
                    self.key_queue.push(key_data);
                    count += 1;
                }
            }
        }

        self.last_report.copy_from_slice(&report[..8]);
        count
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    fn decode_usage(&self, usage: u8, is_shifted: bool) -> Option<EfiKeyData> {
        // Alphanumeric letters 'A'..'Z' (0x04..0x1D)
        if (USB_USAGE_KEY_A..=USB_USAGE_KEY_Z).contains(&usage) {
            let offset = usage - USB_USAGE_KEY_A;
            let mut ch = 0x0061 + u16::from(offset); // 'a'..'z'

            let mut upper = is_shifted;
            if self.caps_lock {
                upper = !upper;
            }
            if upper {
                ch = 0x0041 + u16::from(offset); // 'A'..'Z'
            }

            let mut state = self.key_state();
            if is_shifted {
                state.key_shift_state &= !(EFI_LEFT_SHIFT_PRESSED | EFI_RIGHT_SHIFT_PRESSED);
            }

            return Some(EfiKeyData {
                key: EfiInputKey {
                    scan_code: SCAN_NULL,
                    unicode_char: ch,
                },
                key_state: state,
            });
        }

        // Top-row numbers '1'..'9', '0' (0x1E..0x27)
        if (USB_USAGE_KEY_1..=USB_USAGE_KEY_0).contains(&usage) {
            let (normal, shifted) = match usage {
                0x1E => (0x0031, 0x0021), // '1', '!'
                0x1F => (0x0032, 0x0040), // '2', '@'
                0x20 => (0x0033, 0x0023), // '3', '#'
                0x21 => (0x0034, 0x0024), // '4', '$'
                0x22 => (0x0035, 0x0025), // '5', '%'
                0x23 => (0x0036, 0x005E), // '6', '^'
                0x24 => (0x0037, 0x0026), // '7', '&'
                0x25 => (0x0038, 0x002A), // '8', '*'
                0x26 => (0x0039, 0x0028), // '9', '('
                0x27 => (0x0030, 0x0029), // '0', ')'
                _ => (0, 0),
            };

            let unicode_char = if is_shifted { shifted } else { normal };
            let mut state = self.key_state();
            if is_shifted {
                state.key_shift_state &= !(EFI_LEFT_SHIFT_PRESSED | EFI_RIGHT_SHIFT_PRESSED);
            }

            return Some(EfiKeyData {
                key: EfiInputKey {
                    scan_code: SCAN_NULL,
                    unicode_char,
                },
                key_state: state,
            });
        }

        // Keypad digits '1'..'9', '0', '.' (0x59..0x63)
        if (USB_USAGE_KEYPAD_1..=USB_USAGE_KEYPAD_PERIOD).contains(&usage) {
            if self.num_lock && !is_shifted {
                // Keypad produces digits
                let ch = match usage {
                    0x59 => 0x0031, // '1'
                    0x5A => 0x0032, // '2'
                    0x5B => 0x0033, // '3'
                    0x5C => 0x0034, // '4'
                    0x5D => 0x0035, // '5'
                    0x5E => 0x0036, // '6'
                    0x5F => 0x0037, // '7'
                    0x60 => 0x0038, // '8'
                    0x61 => 0x0039, // '9'
                    0x62 => 0x0030, // '0'
                    0x63 => 0x002E, // '.'
                    _ => 0,
                };
                return Some(EfiKeyData {
                    key: EfiInputKey {
                        scan_code: SCAN_NULL,
                        unicode_char: ch,
                    },
                    key_state: self.key_state(),
                });
            }

            // Otherwise produces navigation keys
            let scan_code = match usage {
                0x59 => SCAN_END,
                0x5A => SCAN_DOWN,
                0x5B => SCAN_PAGE_DOWN,
                0x5C => SCAN_LEFT,
                0x5D => SCAN_NULL, // Center 5
                0x5E => SCAN_RIGHT,
                0x5F => SCAN_HOME,
                0x60 => SCAN_UP,
                0x61 => SCAN_PAGE_UP,
                0x62 => SCAN_INSERT,
                0x63 => SCAN_DELETE,
                _ => SCAN_NULL,
            };
            return Some(EfiKeyData {
                key: EfiInputKey {
                    scan_code,
                    unicode_char: 0,
                },
                key_state: self.key_state(),
            });
        }

        // Application / Menu key (0x65)
        if usage == USB_USAGE_KEY_APPLICATION {
            let mut state = self.key_state();
            state.key_shift_state |= EFI_MENU_KEY_PRESSED;
            return Some(EfiKeyData {
                key: EfiInputKey {
                    scan_code: SCAN_NULL,
                    unicode_char: 0,
                },
                key_state: state,
            });
        }

        // Print Screen / SysRq (0x46)
        if usage == USB_USAGE_KEY_PRINT_SCREEN {
            let mut state = self.key_state();
            state.key_shift_state |= EFI_SYS_REQ_PRESSED;
            return Some(EfiKeyData {
                key: EfiInputKey {
                    scan_code: SCAN_NULL,
                    unicode_char: 0,
                },
                key_state: state,
            });
        }

        // Look up in static table
        for entry in USB_KEY_MAP_TABLE {
            if entry.usage == usage {
                let unicode_char = if is_shifted && entry.shift_unicode_char != 0 {
                    entry.shift_unicode_char
                } else {
                    entry.unicode_char
                };

                let mut state = self.key_state();
                if is_shifted && unicode_char != entry.unicode_char {
                    state.key_shift_state &= !(EFI_LEFT_SHIFT_PRESSED | EFI_RIGHT_SHIFT_PRESSED);
                }

                return Some(EfiKeyData {
                    key: EfiInputKey {
                        scan_code: entry.efi_scan_code,
                        unicode_char,
                    },
                    key_state: state,
                });
            }
        }

        None
    }

    /// Pops the next keystroke from the internal queue.
    pub fn read_key_stroke(&mut self) -> Option<EfiKeyData> {
        self.key_queue.pop()
    }
}
