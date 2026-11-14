//! # PS/2 Keyboard Controller and Scancode Decoder
//!
//! Production-ready implementation of the Intel 8042 PS/2 keyboard controller
//! and scan code decoder for UEFI environments.
//!
//! Provides full decoding of Scan Code Set 1 (including translated Set 2),
//! multi-byte extended scancodes (`0xE0`, `0xE1`), modifier key tracking
//! (Shift, Ctrl, Alt, Logo, Menu, `SysReq`), toggle state tracking (Caps Lock,
//! Num Lock, Scroll Lock), LED command generation, keystroke ring buffering,
//! and Ctrl+Alt+Del hotkey detection.

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
    EfiKeyData, EfiKeyState, SCAN_F11, SCAN_F12, SCAN_PAUSE,
};

// ---------------------------------------------------------------------------
// 8042 Controller Ports and Status Register Bits
// ---------------------------------------------------------------------------

/// Intel 8042 Keyboard Controller Data Register I/O Port.
pub const PS2_DATA_PORT: u16 = 0x60;

/// Intel 8042 Keyboard Controller Status Register I/O Port (Read).
pub const PS2_STATUS_PORT: u16 = 0x64;

/// Intel 8042 Keyboard Controller Command Register I/O Port (Write).
pub const PS2_COMMAND_PORT: u16 = 0x64;

/// Status bit 0: Output buffer full (data from device available to CPU).
pub const KBC_STATUS_OUTPUT_BUFFER_FULL: u8 = 0x01;

/// Status bit 1: Input buffer full (CPU wrote data, controller hasn't read yet).
pub const KBC_STATUS_INPUT_BUFFER_FULL: u8 = 0x02;

/// Status bit 2: System flag (cleared on power-on reset, set after self-test passed).
pub const KBC_STATUS_SYSTEM_FLAG: u8 = 0x04;

/// Status bit 3: Command/Data (0 = data written to 0x60, 1 = command written to 0x64).
pub const KBC_STATUS_COMMAND_DATA: u8 = 0x08;

/// Status bit 4: Keyboard inhibit / enable flag (0 = keyboard locked, 1 = free/enabled).
pub const KBC_STATUS_KEYBOARD_ENABLED: u8 = 0x10;

/// Status bit 5: Auxiliary device output buffer full (1 = mouse data in output buffer).
pub const KBC_STATUS_AUX_OUTPUT_BUFFER_FULL: u8 = 0x20;

/// Status bit 6: General timeout error.
pub const KBC_STATUS_TIMEOUT: u8 = 0x40;

/// Status bit 7: Parity error on incoming communication.
pub const KBC_STATUS_PARITY_ERROR: u8 = 0x80;

// ---------------------------------------------------------------------------
// 8042 Controller Commands
// ---------------------------------------------------------------------------

/// Read 8042 controller configuration byte.
pub const KBC_CMD_READ_CONFIG_BYTE: u8 = 0x20;

/// Write 8042 controller configuration byte.
pub const KBC_CMD_WRITE_CONFIG_BYTE: u8 = 0x60;

/// Disable PS/2 auxiliary (mouse) port interface.
pub const KBC_CMD_DISABLE_MOUSE: u8 = 0xA7;

/// Enable PS/2 auxiliary (mouse) port interface.
pub const KBC_CMD_ENABLE_MOUSE: u8 = 0xA8;

/// Test PS/2 auxiliary (mouse) port interface.
pub const KBC_CMD_TEST_MOUSE: u8 = 0xA9;

/// Controller self-test command (returns 0x55 on success).
pub const KBC_CMD_SELF_TEST: u8 = 0xAA;

/// Test PS/2 keyboard port interface.
pub const KBC_CMD_TEST_KEYBOARD: u8 = 0xAB;

/// Disable PS/2 keyboard port interface.
pub const KBC_CMD_DISABLE_KEYBOARD: u8 = 0xAD;

/// Enable PS/2 keyboard port interface.
pub const KBC_CMD_ENABLE_KEYBOARD: u8 = 0xAE;

/// Read controller input port.
pub const KBC_CMD_READ_INPUT_PORT: u8 = 0xC0;

/// Read controller output port.
pub const KBC_CMD_READ_OUTPUT_PORT: u8 = 0xD0;

/// Write controller output port.
pub const KBC_CMD_WRITE_OUTPUT_PORT: u8 = 0xD1;

/// Write next byte to PS/2 auxiliary (mouse) device.
pub const KBC_CMD_WRITE_MOUSE: u8 = 0xD4;

/// Pulse CPU reset line (system reset).
pub const KBC_CMD_PULSE_RESET: u8 = 0xFE;

// ---------------------------------------------------------------------------
// PS/2 Keyboard Device Commands & Responses
// ---------------------------------------------------------------------------

/// Keyboard device command: Set LED indicator lights.
pub const KBD_CMD_SET_LEDS: u8 = 0xED;

/// Keyboard device command: Echo diagnostic byte (replies with 0xEE).
pub const KBD_CMD_ECHO: u8 = 0xEE;

/// Keyboard device command: Select/read scan code set.
pub const KBD_CMD_SELECT_SCAN_CODE_SET: u8 = 0xF0;

/// Keyboard device command: Read keyboard ID bytes.
pub const KBD_CMD_READ_ID: u8 = 0xF2;

/// Keyboard device command: Set typematic delay and repeat rate.
pub const KBD_CMD_SET_TYPEMATIC: u8 = 0xF3;

/// Keyboard device command: Enable key scanning and data reporting.
pub const KBD_CMD_ENABLE_SCANNING: u8 = 0xF4;

/// Keyboard device command: Disable key scanning and data reporting.
pub const KBD_CMD_DISABLE_SCANNING: u8 = 0xF5;

/// Keyboard device command: Reset all parameters to defaults.
pub const KBD_CMD_SET_DEFAULT: u8 = 0xF6;

/// Keyboard device command: Resend last transmission.
pub const KBD_CMD_RESEND: u8 = 0xFE;

/// Keyboard device command: Reset keyboard hardware and perform BAT self-test.
pub const KBD_CMD_RESET: u8 = 0xFF;

/// Keyboard response: Basic Assurance Test (BAT) successful completion.
pub const KBD_RESP_BAT_SUCCESS: u8 = 0xAA;

/// Keyboard response: Diagnostic echo.
pub const KBD_RESP_ECHO: u8 = 0xEE;

/// Keyboard response: Command acknowledge (ACK).
pub const KBD_RESP_ACK: u8 = 0xFA;

/// Keyboard response: Basic Assurance Test (BAT) failure.
pub const KBD_RESP_BAT_ERROR: u8 = 0xFC;

/// Keyboard response: Resend request.
pub const KBD_RESP_RESEND: u8 = 0xFE;

// ---------------------------------------------------------------------------
// Scancode Constants (Scan Code Set 1)
// ---------------------------------------------------------------------------

/// Extended scancode prefix 0 (single byte extended key follows).
pub const SCANCODE_EXTENDED0: u8 = 0xE0;

/// Extended scancode prefix 1 (multi-byte extended sequence follows, e.g. Pause).
pub const SCANCODE_EXTENDED1: u8 = 0xE1;

/// Left Ctrl key make scancode (Set 1).
pub const SCANCODE_CTRL_MAKE: u8 = 0x1D;
/// Left Ctrl key break scancode (Set 1).
pub const SCANCODE_CTRL_BREAK: u8 = 0x9D;

/// Left Alt key make scancode (Set 1).
pub const SCANCODE_ALT_MAKE: u8 = 0x38;
/// Left Alt key break scancode (Set 1).
pub const SCANCODE_ALT_BREAK: u8 = 0xB8;

/// Left Shift key make scancode (Set 1).
pub const SCANCODE_LEFT_SHIFT_MAKE: u8 = 0x2A;
/// Left Shift key break scancode (Set 1).
pub const SCANCODE_LEFT_SHIFT_BREAK: u8 = 0xAA;

/// Right Shift key make scancode (Set 1).
pub const SCANCODE_RIGHT_SHIFT_MAKE: u8 = 0x36;
/// Right Shift key break scancode (Set 1).
pub const SCANCODE_RIGHT_SHIFT_BREAK: u8 = 0xB6;

/// Caps Lock key make scancode (Set 1).
pub const SCANCODE_CAPS_LOCK_MAKE: u8 = 0x3A;
/// Caps Lock key break scancode (Set 1).
pub const SCANCODE_CAPS_LOCK_BREAK: u8 = 0xBA;

/// Num Lock key make scancode (Set 1).
pub const SCANCODE_NUM_LOCK_MAKE: u8 = 0x45;
/// Num Lock key break scancode (Set 1).
pub const SCANCODE_NUM_LOCK_BREAK: u8 = 0xC5;

/// Scroll Lock key make scancode (Set 1).
pub const SCANCODE_SCROLL_LOCK_MAKE: u8 = 0x46;
/// Scroll Lock key break scancode (Set 1).
pub const SCANCODE_SCROLL_LOCK_BREAK: u8 = 0xC6;

/// Delete key make scancode (Set 1).
pub const SCANCODE_DELETE_MAKE: u8 = 0x53;
/// Delete key break scancode (Set 1).
pub const SCANCODE_DELETE_BREAK: u8 = 0xD3;

/// Left GUI/Logo key make scancode (Set 1).
pub const SCANCODE_LEFT_LOGO_MAKE: u8 = 0x5B;
/// Left GUI/Logo key break scancode (Set 1).
pub const SCANCODE_LEFT_LOGO_BREAK: u8 = 0xDB;

/// Right GUI/Logo key make scancode (Set 1).
pub const SCANCODE_RIGHT_LOGO_MAKE: u8 = 0x5C;
/// Right GUI/Logo key break scancode (Set 1).
pub const SCANCODE_RIGHT_LOGO_BREAK: u8 = 0xDC;

/// Application/Menu key make scancode (Set 1).
pub const SCANCODE_MENU_MAKE: u8 = 0x5D;
/// Application/Menu key break scancode (Set 1).
pub const SCANCODE_MENU_BREAK: u8 = 0xDD;

/// System Request (Print Screen) make scancode (Set 1).
pub const SCANCODE_SYS_REQ_MAKE: u8 = 0x37;
/// System Request (Print Screen) break scancode (Set 1).
pub const SCANCODE_SYS_REQ_BREAK: u8 = 0xB7;

/// System Request make scancode with Alt held.
pub const SCANCODE_SYS_REQ_MAKE_WITH_ALT: u8 = 0x54;
/// System Request break scancode with Alt held.
pub const SCANCODE_SYS_REQ_BREAK_WITH_ALT: u8 = 0xD4;

/// Maximum normal make scancode in Scan Code Set 1.
pub const SCANCODE_MAX_MAKE: u8 = 0x60;

// ---------------------------------------------------------------------------
// LED Indicator Flags
// ---------------------------------------------------------------------------

/// Scroll Lock LED flag (bit 0 in PS/2 0xED command).
pub const PS2_LED_SCROLL_LOCK: u8 = 0x01;

/// Num Lock LED flag (bit 1 in PS/2 0xED command).
pub const PS2_LED_NUM_LOCK: u8 = 0x02;

/// Caps Lock LED flag (bit 2 in PS/2 0xED command).
pub const PS2_LED_CAPS_LOCK: u8 = 0x04;

// ---------------------------------------------------------------------------
// Scancode Translation Entry and Table
// ---------------------------------------------------------------------------

/// A static mapping from Scan Code Set 1 to UEFI scan code and Unicode characters.
#[derive(Clone, Copy, Debug)]
pub struct ScancodeMapping {
    /// Make scancode byte (Scan Code Set 1).
    pub scancode: u8,
    /// UEFI scan code (e.g. `SCAN_ESC`, `SCAN_F1`), or `SCAN_NULL` if character-based.
    pub efi_scan_code: u16,
    /// Unshifted Unicode character.
    pub unicode_char: Char16,
    /// Shifted Unicode character.
    pub shift_unicode_char: Char16,
}

/// Static conversion table from PS/2 Scan Code Set 1 to UEFI keystrokes.
pub static SCANCODE_SET1_TABLE: &[ScancodeMapping] = &[
    ScancodeMapping {
        scancode: 0x01,
        efi_scan_code: SCAN_ESC,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x02,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0031,
        shift_unicode_char: 0x0021,
    }, // '1', '!'
    ScancodeMapping {
        scancode: 0x03,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0032,
        shift_unicode_char: 0x0040,
    }, // '2', '@'
    ScancodeMapping {
        scancode: 0x04,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0033,
        shift_unicode_char: 0x0023,
    }, // '3', '#'
    ScancodeMapping {
        scancode: 0x05,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0034,
        shift_unicode_char: 0x0024,
    }, // '4', '$'
    ScancodeMapping {
        scancode: 0x06,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0035,
        shift_unicode_char: 0x0025,
    }, // '5', '%'
    ScancodeMapping {
        scancode: 0x07,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0036,
        shift_unicode_char: 0x005E,
    }, // '6', '^'
    ScancodeMapping {
        scancode: 0x08,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0037,
        shift_unicode_char: 0x0026,
    }, // '7', '&'
    ScancodeMapping {
        scancode: 0x09,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0038,
        shift_unicode_char: 0x002A,
    }, // '8', '*'
    ScancodeMapping {
        scancode: 0x0A,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0039,
        shift_unicode_char: 0x0028,
    }, // '9', '('
    ScancodeMapping {
        scancode: 0x0B,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0030,
        shift_unicode_char: 0x0029,
    }, // '0', ')'
    ScancodeMapping {
        scancode: 0x0C,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002D,
        shift_unicode_char: 0x005F,
    }, // '-', '_'
    ScancodeMapping {
        scancode: 0x0D,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x003D,
        shift_unicode_char: 0x002B,
    }, // '=', '+'
    ScancodeMapping {
        scancode: 0x0E,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_BACKSPACE,
        shift_unicode_char: CHAR_BACKSPACE,
    },
    ScancodeMapping {
        scancode: 0x0F,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_TAB,
        shift_unicode_char: CHAR_TAB,
    },
    ScancodeMapping {
        scancode: 0x10,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0071,
        shift_unicode_char: 0x0051,
    }, // 'q', 'Q'
    ScancodeMapping {
        scancode: 0x11,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0077,
        shift_unicode_char: 0x0057,
    }, // 'w', 'W'
    ScancodeMapping {
        scancode: 0x12,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0065,
        shift_unicode_char: 0x0045,
    }, // 'e', 'E'
    ScancodeMapping {
        scancode: 0x13,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0072,
        shift_unicode_char: 0x0052,
    }, // 'r', 'R'
    ScancodeMapping {
        scancode: 0x14,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0074,
        shift_unicode_char: 0x0054,
    }, // 't', 'T'
    ScancodeMapping {
        scancode: 0x15,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0079,
        shift_unicode_char: 0x0059,
    }, // 'y', 'Y'
    ScancodeMapping {
        scancode: 0x16,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0075,
        shift_unicode_char: 0x0055,
    }, // 'u', 'U'
    ScancodeMapping {
        scancode: 0x17,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0069,
        shift_unicode_char: 0x0049,
    }, // 'i', 'I'
    ScancodeMapping {
        scancode: 0x18,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x006F,
        shift_unicode_char: 0x004F,
    }, // 'o', 'O'
    ScancodeMapping {
        scancode: 0x19,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0070,
        shift_unicode_char: 0x0050,
    }, // 'p', 'P'
    ScancodeMapping {
        scancode: 0x1A,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x005B,
        shift_unicode_char: 0x007B,
    }, // '[', '{'
    ScancodeMapping {
        scancode: 0x1B,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x005D,
        shift_unicode_char: 0x007D,
    }, // ']', '}'
    ScancodeMapping {
        scancode: 0x1C,
        efi_scan_code: SCAN_NULL,
        unicode_char: CHAR_CARRIAGE_RETURN,
        shift_unicode_char: CHAR_CARRIAGE_RETURN,
    },
    ScancodeMapping {
        scancode: 0x1E,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0061,
        shift_unicode_char: 0x0041,
    }, // 'a', 'A'
    ScancodeMapping {
        scancode: 0x1F,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0073,
        shift_unicode_char: 0x0053,
    }, // 's', 'S'
    ScancodeMapping {
        scancode: 0x20,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0064,
        shift_unicode_char: 0x0044,
    }, // 'd', 'D'
    ScancodeMapping {
        scancode: 0x21,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0066,
        shift_unicode_char: 0x0046,
    }, // 'f', 'F'
    ScancodeMapping {
        scancode: 0x22,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0067,
        shift_unicode_char: 0x0047,
    }, // 'g', 'G'
    ScancodeMapping {
        scancode: 0x23,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0068,
        shift_unicode_char: 0x0048,
    }, // 'h', 'H'
    ScancodeMapping {
        scancode: 0x24,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x006A,
        shift_unicode_char: 0x004A,
    }, // 'j', 'J'
    ScancodeMapping {
        scancode: 0x25,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x006B,
        shift_unicode_char: 0x004B,
    }, // 'k', 'K'
    ScancodeMapping {
        scancode: 0x26,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x006C,
        shift_unicode_char: 0x004C,
    }, // 'l', 'L'
    ScancodeMapping {
        scancode: 0x27,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x003B,
        shift_unicode_char: 0x003A,
    }, // ';', ':'
    ScancodeMapping {
        scancode: 0x28,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0027,
        shift_unicode_char: 0x0022,
    }, // '\'', '"'
    ScancodeMapping {
        scancode: 0x29,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0060,
        shift_unicode_char: 0x007E,
    }, // '`', '~'
    ScancodeMapping {
        scancode: 0x2B,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x005C,
        shift_unicode_char: 0x007C,
    }, // '\\', '|'
    ScancodeMapping {
        scancode: 0x2C,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x007A,
        shift_unicode_char: 0x005A,
    }, // 'z', 'Z'
    ScancodeMapping {
        scancode: 0x2D,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0078,
        shift_unicode_char: 0x0058,
    }, // 'x', 'X'
    ScancodeMapping {
        scancode: 0x2E,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0063,
        shift_unicode_char: 0x0043,
    }, // 'c', 'C'
    ScancodeMapping {
        scancode: 0x2F,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0076,
        shift_unicode_char: 0x0056,
    }, // 'v', 'V'
    ScancodeMapping {
        scancode: 0x30,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0062,
        shift_unicode_char: 0x0042,
    }, // 'b', 'B'
    ScancodeMapping {
        scancode: 0x31,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x006E,
        shift_unicode_char: 0x004E,
    }, // 'n', 'N'
    ScancodeMapping {
        scancode: 0x32,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x006D,
        shift_unicode_char: 0x004D,
    }, // 'm', 'M'
    ScancodeMapping {
        scancode: 0x33,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002C,
        shift_unicode_char: 0x003C,
    }, // ',', '<'
    ScancodeMapping {
        scancode: 0x34,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002E,
        shift_unicode_char: 0x003E,
    }, // '.', '>'
    ScancodeMapping {
        scancode: 0x35,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002F,
        shift_unicode_char: 0x003F,
    }, // '/', '?'
    ScancodeMapping {
        scancode: 0x37,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002A,
        shift_unicode_char: 0x002A,
    }, // Keypad '*'
    ScancodeMapping {
        scancode: 0x39,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0020,
        shift_unicode_char: 0x0020,
    }, // Space
    ScancodeMapping {
        scancode: 0x3B,
        efi_scan_code: SCAN_F1,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x3C,
        efi_scan_code: SCAN_F2,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x3D,
        efi_scan_code: SCAN_F3,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x3E,
        efi_scan_code: SCAN_F4,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x3F,
        efi_scan_code: SCAN_F5,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x40,
        efi_scan_code: SCAN_F6,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x41,
        efi_scan_code: SCAN_F7,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x42,
        efi_scan_code: SCAN_F8,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x43,
        efi_scan_code: SCAN_F9,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x44,
        efi_scan_code: SCAN_F10,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x47,
        efi_scan_code: SCAN_HOME,
        unicode_char: 0x0037,
        shift_unicode_char: 0x0037,
    }, // Keypad 7 / Home
    ScancodeMapping {
        scancode: 0x48,
        efi_scan_code: SCAN_UP,
        unicode_char: 0x0038,
        shift_unicode_char: 0x0038,
    }, // Keypad 8 / Up
    ScancodeMapping {
        scancode: 0x49,
        efi_scan_code: SCAN_PAGE_UP,
        unicode_char: 0x0039,
        shift_unicode_char: 0x0039,
    }, // Keypad 9 / PgUp
    ScancodeMapping {
        scancode: 0x4A,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002D,
        shift_unicode_char: 0x002D,
    }, // Keypad '-'
    ScancodeMapping {
        scancode: 0x4B,
        efi_scan_code: SCAN_LEFT,
        unicode_char: 0x0034,
        shift_unicode_char: 0x0034,
    }, // Keypad 4 / Left
    ScancodeMapping {
        scancode: 0x4C,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x0035,
        shift_unicode_char: 0x0035,
    }, // Keypad 5
    ScancodeMapping {
        scancode: 0x4D,
        efi_scan_code: SCAN_RIGHT,
        unicode_char: 0x0036,
        shift_unicode_char: 0x0036,
    }, // Keypad 6 / Right
    ScancodeMapping {
        scancode: 0x4E,
        efi_scan_code: SCAN_NULL,
        unicode_char: 0x002B,
        shift_unicode_char: 0x002B,
    }, // Keypad '+'
    ScancodeMapping {
        scancode: 0x4F,
        efi_scan_code: SCAN_END,
        unicode_char: 0x0031,
        shift_unicode_char: 0x0031,
    }, // Keypad 1 / End
    ScancodeMapping {
        scancode: 0x50,
        efi_scan_code: SCAN_DOWN,
        unicode_char: 0x0032,
        shift_unicode_char: 0x0032,
    }, // Keypad 2 / Down
    ScancodeMapping {
        scancode: 0x51,
        efi_scan_code: SCAN_PAGE_DOWN,
        unicode_char: 0x0033,
        shift_unicode_char: 0x0033,
    }, // Keypad 3 / PgDn
    ScancodeMapping {
        scancode: 0x52,
        efi_scan_code: SCAN_INSERT,
        unicode_char: 0x0030,
        shift_unicode_char: 0x0030,
    }, // Keypad 0 / Ins
    ScancodeMapping {
        scancode: 0x53,
        efi_scan_code: SCAN_DELETE,
        unicode_char: 0x002E,
        shift_unicode_char: 0x002E,
    }, // Keypad . / Del
    ScancodeMapping {
        scancode: 0x57,
        efi_scan_code: SCAN_F11,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
    ScancodeMapping {
        scancode: 0x58,
        efi_scan_code: SCAN_F12,
        unicode_char: 0x0000,
        shift_unicode_char: 0x0000,
    },
];

// ---------------------------------------------------------------------------
// Ring Buffers
// ---------------------------------------------------------------------------

/// Maximum number of raw scancodes buffered in the input queue.
pub const KEYBOARD_SCAN_CODE_MAX_COUNT: usize = 32;

/// Maximum number of decoded keystrokes buffered in the EFI key queue.
pub const KEYBOARD_EFI_KEY_MAX_COUNT: usize = 64;

/// Ring buffer for raw PS/2 scancodes received from the 8042 controller.
#[derive(Debug)]
pub struct ScancodeQueue {
    buffer: [u8; KEYBOARD_SCAN_CODE_MAX_COUNT],
    head: usize,
    tail: usize,
    count: usize,
}

impl Default for ScancodeQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl ScancodeQueue {
    /// Creates a new, empty scancode queue.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; KEYBOARD_SCAN_CODE_MAX_COUNT],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Pushes a scancode byte into the queue. Drops the oldest entry if full.
    pub fn push(&mut self, scancode: u8) {
        if self.count == KEYBOARD_SCAN_CODE_MAX_COUNT {
            // Drop oldest
            self.head = (self.head + 1) % KEYBOARD_SCAN_CODE_MAX_COUNT;
            self.count -= 1;
        }
        self.buffer[self.tail] = scancode;
        self.tail = (self.tail + 1) % KEYBOARD_SCAN_CODE_MAX_COUNT;
        self.count += 1;
    }

    /// Pops the next scancode byte from the queue.
    pub fn pop(&mut self) -> Option<u8> {
        if self.count == 0 {
            None
        } else {
            let scancode = self.buffer[self.head];
            self.head = (self.head + 1) % KEYBOARD_SCAN_CODE_MAX_COUNT;
            self.count -= 1;
            Some(scancode)
        }
    }

    /// Returns the number of scancodes in the queue.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Returns `true` if the queue is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clears all entries from the queue.
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }
}

/// Ring buffer for decoded `EfiKeyData` keystrokes.
#[derive(Debug)]
pub struct EfiKeyQueue {
    buffer: [EfiKeyData; KEYBOARD_EFI_KEY_MAX_COUNT],
    head: usize,
    tail: usize,
    count: usize,
}

impl Default for EfiKeyQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EfiKeyQueue {
    /// Creates a new, empty EFI keystroke queue.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: [EfiKeyData {
                key: EfiInputKey {
                    scan_code: SCAN_NULL,
                    unicode_char: 0,
                },
                key_state: EfiKeyState {
                    key_shift_state: 0,
                    key_toggle_state: 0,
                },
            }; KEYBOARD_EFI_KEY_MAX_COUNT],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Pushes a keystroke into the queue. Drops the oldest entry if full.
    pub fn push(&mut self, key_data: EfiKeyData) {
        if self.count == KEYBOARD_EFI_KEY_MAX_COUNT {
            self.head = (self.head + 1) % KEYBOARD_EFI_KEY_MAX_COUNT;
            self.count -= 1;
        }
        self.buffer[self.tail] = key_data;
        self.tail = (self.tail + 1) % KEYBOARD_EFI_KEY_MAX_COUNT;
        self.count += 1;
    }

    /// Pops the next keystroke from the queue.
    pub fn pop(&mut self) -> Option<EfiKeyData> {
        if self.count == 0 {
            None
        } else {
            let key = self.buffer[self.head];
            self.head = (self.head + 1) % KEYBOARD_EFI_KEY_MAX_COUNT;
            self.count -= 1;
            Some(key)
        }
    }

    /// Returns the number of keystrokes in the queue.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Returns `true` if the queue is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clears all entries from the queue.
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }
}

// ---------------------------------------------------------------------------
// PS/2 Keyboard State Machine & Decoder
// ---------------------------------------------------------------------------

/// State machine tracking modifier and toggle states, decoding PS/2 Scan Code Set 1
/// byte streams into standard UEFI keystroke records.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct Ps2KeyboardDecoder {
    /// Extended 0 flag (`0xE0` prefix encountered).
    pub extend0: bool,
    /// Extended 1 flag (`0xE1` prefix encountered).
    pub extend1: bool,
    /// Count of bytes seen following `0xE1` prefix (for Pause/Break decoding).
    pub extend1_count: u8,

    // Modifiers
    /// Left Ctrl pressed.
    pub left_ctrl: bool,
    /// Right Ctrl pressed.
    pub right_ctrl: bool,
    /// Left Alt pressed.
    pub left_alt: bool,
    /// Right Alt pressed.
    pub right_alt: bool,
    /// Left Shift pressed.
    pub left_shift: bool,
    /// Right Shift pressed.
    pub right_shift: bool,
    /// Left Logo / Windows key pressed.
    pub left_logo: bool,
    /// Right Logo / Windows key pressed.
    pub right_logo: bool,
    /// Application / Menu key pressed.
    pub menu: bool,
    /// System Request / Print Screen key pressed.
    pub sys_req: bool,

    // Toggles
    /// Caps Lock active.
    pub caps_lock: bool,
    /// Num Lock active.
    pub num_lock: bool,
    /// Scroll Lock active.
    pub scroll_lock: bool,

    /// Whether partial keys (modifier updates without a character) should generate keystrokes.
    pub is_support_partial_key: bool,
    /// Flag set when Ctrl+Alt+Del hotkey combination is detected.
    pub ctrl_alt_del_triggered: bool,
    /// Raw scancode incoming buffer.
    pub scancode_queue: ScancodeQueue,
    /// Decoded keystrokes queue.
    pub key_queue: EfiKeyQueue,
}

impl Default for Ps2KeyboardDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Ps2KeyboardDecoder {
    /// Creates a new PS/2 keyboard decoder initialized with default settings
    /// (Num Lock enabled by default for typical PC firmwares).
    #[must_use]
    pub const fn new() -> Self {
        Self {
            extend0: false,
            extend1: false,
            extend1_count: 0,
            left_ctrl: false,
            right_ctrl: false,
            left_alt: false,
            right_alt: false,
            left_shift: false,
            right_shift: false,
            left_logo: false,
            right_logo: false,
            menu: false,
            sys_req: false,
            caps_lock: false,
            num_lock: true,
            scroll_lock: false,
            is_support_partial_key: false,
            ctrl_alt_del_triggered: false,
            scancode_queue: ScancodeQueue::new(),
            key_queue: EfiKeyQueue::new(),
        }
    }

    /// Resets the internal state machine and clears all queues.
    pub fn reset(&mut self) {
        self.extend0 = false;
        self.extend1 = false;
        self.extend1_count = 0;
        self.left_ctrl = false;
        self.right_ctrl = false;
        self.left_alt = false;
        self.right_alt = false;
        self.left_shift = false;
        self.right_shift = false;
        self.left_logo = false;
        self.right_logo = false;
        self.menu = false;
        self.sys_req = false;
        self.caps_lock = false;
        self.num_lock = true;
        self.scroll_lock = false;
        self.ctrl_alt_del_triggered = false;
        self.scancode_queue.clear();
        self.key_queue.clear();
    }

    /// Computes the current [`EfiKeyState`] representation.
    #[must_use]
    pub fn key_state(&self) -> EfiKeyState {
        let mut shift = EFI_SHIFT_STATE_VALID;
        if self.right_shift {
            shift |= EFI_RIGHT_SHIFT_PRESSED;
        }
        if self.left_shift {
            shift |= EFI_LEFT_SHIFT_PRESSED;
        }
        if self.right_ctrl {
            shift |= EFI_RIGHT_CONTROL_PRESSED;
        }
        if self.left_ctrl {
            shift |= EFI_LEFT_CONTROL_PRESSED;
        }
        if self.right_alt {
            shift |= EFI_RIGHT_ALT_PRESSED;
        }
        if self.left_alt {
            shift |= EFI_LEFT_ALT_PRESSED;
        }
        if self.right_logo {
            shift |= EFI_RIGHT_LOGO_PRESSED;
        }
        if self.left_logo {
            shift |= EFI_LEFT_LOGO_PRESSED;
        }
        if self.menu {
            shift |= EFI_MENU_KEY_PRESSED;
        }
        if self.sys_req {
            shift |= EFI_SYS_REQ_PRESSED;
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

    /// Returns the LED status command byte corresponding to current toggle states
    /// for the PS/2 `0xED` command.
    #[must_use]
    pub fn led_flags(&self) -> u8 {
        let mut leds = 0u8;
        if self.scroll_lock {
            leds |= PS2_LED_SCROLL_LOCK;
        }
        if self.num_lock {
            leds |= PS2_LED_NUM_LOCK;
        }
        if self.caps_lock {
            leds |= PS2_LED_CAPS_LOCK;
        }
        leds
    }

    /// Updates the toggle state based on an external toggle byte (e.g. from `EfiSetState`).
    pub fn set_toggle_state(&mut self, toggle_state: u8) {
        if (toggle_state & EFI_TOGGLE_STATE_VALID) != 0 {
            self.scroll_lock = (toggle_state & EFI_SCROLL_LOCK_ACTIVE) != 0;
            self.num_lock = (toggle_state & EFI_NUM_LOCK_ACTIVE) != 0;
            self.caps_lock = (toggle_state & EFI_CAPS_LOCK_ACTIVE) != 0;
        }
    }

    /// Ingests a raw byte from the 8042 data port, updates internal state,
    /// and returns a decoded [`EfiKeyData`] if a valid keystroke was generated.
    #[allow(clippy::too_many_lines)]
    pub fn process_byte(&mut self, byte: u8) -> Option<EfiKeyData> {
        // Handle E1 multi-byte Pause/Break sequence
        if self.extend1 {
            self.extend1_count += 1;
            if self.extend1_count == 1 {
                // Ignore intermediate 0x1D / 0x9D in Pause sequence
                return None;
            }
            if self.extend1_count == 2 {
                self.extend1 = false;
                self.extend1_count = 0;
                if byte == SCANCODE_NUM_LOCK_MAKE {
                    let key_data = EfiKeyData {
                        key: EfiInputKey {
                            scan_code: SCAN_PAUSE,
                            unicode_char: 0,
                        },
                        key_state: self.key_state(),
                    };
                    self.key_queue.push(key_data);
                    return Some(key_data);
                }
                return None;
            }
        }

        if byte == SCANCODE_EXTENDED1 {
            self.extend1 = true;
            self.extend1_count = 0;
            return None;
        }

        if byte == SCANCODE_EXTENDED0 {
            self.extend0 = true;
            return None;
        }

        let is_extend0 = self.extend0;
        self.extend0 = false;

        // Process special modifiers and toggles
        match byte {
            SCANCODE_CTRL_MAKE => {
                if is_extend0 {
                    self.right_ctrl = true;
                } else {
                    self.left_ctrl = true;
                }
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_CTRL_BREAK => {
                if is_extend0 {
                    self.right_ctrl = false;
                } else {
                    self.left_ctrl = false;
                }
                return None;
            }
            SCANCODE_ALT_MAKE => {
                if is_extend0 {
                    self.right_alt = true;
                } else {
                    self.left_alt = true;
                }
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_ALT_BREAK => {
                if is_extend0 {
                    self.right_alt = false;
                } else {
                    self.left_alt = false;
                }
                return None;
            }
            SCANCODE_LEFT_SHIFT_MAKE => {
                if is_extend0 {
                    return None;
                }
                self.left_shift = true;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_LEFT_SHIFT_BREAK => {
                if !is_extend0 {
                    self.left_shift = false;
                }
                return None;
            }
            SCANCODE_RIGHT_SHIFT_MAKE => {
                self.right_shift = true;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_RIGHT_SHIFT_BREAK => {
                self.right_shift = false;
                return None;
            }
            SCANCODE_LEFT_LOGO_MAKE => {
                self.left_logo = true;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_LEFT_LOGO_BREAK => {
                self.left_logo = false;
                return None;
            }
            SCANCODE_RIGHT_LOGO_MAKE => {
                self.right_logo = true;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_RIGHT_LOGO_BREAK => {
                self.right_logo = false;
                return None;
            }
            SCANCODE_MENU_MAKE => {
                self.menu = true;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_MENU_BREAK => {
                self.menu = false;
                return None;
            }
            SCANCODE_SYS_REQ_MAKE => {
                if is_extend0 {
                    self.sys_req = true;
                    if self.is_support_partial_key {
                        return Some(self.make_partial_key());
                    }
                }
                return None;
            }
            SCANCODE_SYS_REQ_BREAK => {
                if is_extend0 {
                    self.sys_req = false;
                }
                return None;
            }
            SCANCODE_SYS_REQ_MAKE_WITH_ALT => {
                self.sys_req = true;
                return None;
            }
            SCANCODE_SYS_REQ_BREAK_WITH_ALT => {
                self.sys_req = false;
                return None;
            }
            SCANCODE_CAPS_LOCK_MAKE => {
                self.caps_lock = !self.caps_lock;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_NUM_LOCK_MAKE => {
                self.num_lock = !self.num_lock;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            SCANCODE_SCROLL_LOCK_MAKE => {
                if is_extend0 {
                    // E0 + 46 = Pause key
                    let key_data = EfiKeyData {
                        key: EfiInputKey {
                            scan_code: SCAN_PAUSE,
                            unicode_char: 0,
                        },
                        key_state: self.key_state(),
                    };
                    self.key_queue.push(key_data);
                    return Some(key_data);
                }
                self.scroll_lock = !self.scroll_lock;
                if self.is_support_partial_key {
                    return Some(self.make_partial_key());
                }
                return None;
            }
            _ => {}
        }

        // Break codes (bit 7 set) that are not modifier releases are ignored
        if (byte & 0x80) != 0 {
            return None;
        }

        // Check for Ctrl+Alt+Del hotkey
        if (self.left_ctrl || self.right_ctrl)
            && (self.left_alt || self.right_alt)
            && (byte == SCANCODE_DELETE_MAKE)
        {
            self.ctrl_alt_del_triggered = true;
        }

        // Check extended special cases
        if is_extend0 {
            // Keypad '/' has E0 prefix
            if byte == 0x35 {
                let key_data = EfiKeyData {
                    key: EfiInputKey {
                        scan_code: SCAN_NULL,
                        unicode_char: 0x002F, // '/'
                    },
                    key_state: self.key_state(),
                };
                self.key_queue.push(key_data);
                return Some(key_data);
            }
            // Keypad Enter has E0 prefix
            if byte == 0x1C {
                let key_data = EfiKeyData {
                    key: EfiInputKey {
                        scan_code: SCAN_NULL,
                        unicode_char: CHAR_CARRIAGE_RETURN,
                    },
                    key_state: self.key_state(),
                };
                self.key_queue.push(key_data);
                return Some(key_data);
            }
        }

        // Search in scancode translation table
        let mut matched = None;
        for entry in SCANCODE_SET1_TABLE {
            if entry.scancode == byte {
                matched = Some(*entry);
                break;
            }
        }

        if let Some(entry) = matched {
            let mut scan_code = entry.efi_scan_code;
            let mut unicode_char = entry.unicode_char;

            // Handle Shift modification for characters
            let is_shifted = self.left_shift || self.right_shift;
            if is_shifted && (entry.unicode_char != entry.shift_unicode_char) {
                unicode_char = entry.shift_unicode_char;
            }

            // Handle CapsLock modification for alphabetic characters
            if self.caps_lock {
                if (0x0061..=0x007A).contains(&unicode_char) {
                    // Lowercase a-z to uppercase A-Z
                    unicode_char = unicode_char - 0x0061 + 0x0041;
                } else if (0x0041..=0x005A).contains(&unicode_char) {
                    // Uppercase A-Z to lowercase a-z
                    unicode_char = unicode_char - 0x0041 + 0x0061;
                }
            }

            // Distinguish numeric keypad keys (0x47..=0x53):
            // In NumLock without Shift and not extended, they produce digits.
            // Otherwise, they produce navigation scan codes.
            if (0x47..=0x53).contains(&byte) {
                if self.num_lock && !is_shifted && !is_extend0 {
                    scan_code = SCAN_NULL;
                } else if byte != 0x4A && byte != 0x4E {
                    // Not '+' or '-'
                    unicode_char = 0;
                }
            }

            if scan_code != SCAN_NULL || unicode_char != 0 {
                let mut state = self.key_state();
                // If character was shifted, remove shift flags per UEFI convention
                if is_shifted && unicode_char != entry.unicode_char {
                    state.key_shift_state &= !(EFI_LEFT_SHIFT_PRESSED | EFI_RIGHT_SHIFT_PRESSED);
                }
                let key_data = EfiKeyData {
                    key: EfiInputKey {
                        scan_code,
                        unicode_char,
                    },
                    key_state: state,
                };
                self.key_queue.push(key_data);
                return Some(key_data);
            }
        }

        None
    }

    fn make_partial_key(&mut self) -> EfiKeyData {
        let key_data = EfiKeyData {
            key: EfiInputKey {
                scan_code: SCAN_NULL,
                unicode_char: 0,
            },
            key_state: self.key_state(),
        };
        self.key_queue.push(key_data);
        key_data
    }

    /// Pops the next decoded keystroke from the internal queue.
    pub fn read_key_stroke(&mut self) -> Option<EfiKeyData> {
        self.key_queue.pop()
    }
}
