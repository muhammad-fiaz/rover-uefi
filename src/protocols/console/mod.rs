//! # Console
//!
//! Simple text input and output protocols.
//!
//! Input is polled with `ReadKeyStroke` or
//! waited on through `WaitForKey`. Output
//! writes null-terminated UTF-16 strings.
//! Wrappers borrow the firmware protocol
//! instance and preserve the firmware status.

pub mod absolute_pointer;
pub mod console_control;
pub mod input;
pub mod input_ex;
pub mod output;
pub mod pointer;
pub mod print2;
pub mod ps2_keyboard;
pub mod ps2_mouse;
pub mod ps2_policy;
pub mod shell;
pub mod shell_commands;
pub mod text;
pub mod unicode_collation;
pub mod usb_keyboard;
pub mod usb_mouse;
pub mod virtual_keyboard;

pub use absolute_pointer::{
    ABSOLUTE_POINTER_ATTR_SUPPORTS_ALT_ACTIVE, ABSOLUTE_POINTER_ATTR_SUPPORTS_PRESSURE_AS_Z,
    ABSOLUTE_POINTER_PROTOCOL_GUID, ABSOLUTE_POINTER_TOUCH_ACTIVE, AbsolutePointer,
    EfiAbsolutePointerMode, EfiAbsolutePointerProtocol, EfiAbsolutePointerReset,
    EfiAbsolutePointerState,
};
pub use console_control::{
    ConsoleControl, ConsoleControlModeInfo, EFI_CONSOLE_CONTROL_PROTOCOL_GUID,
    EfiConsoleControlGetMode, EfiConsoleControlLockStdIn, EfiConsoleControlProtocol,
    EfiConsoleControlScreenMode, EfiConsoleControlSetMode,
};
pub use input::{
    CHAR_BACKSPACE, CHAR_CARRIAGE_RETURN, CHAR_LINEFEED, CHAR_TAB, EfiInputKey, EfiInputReadKey,
    EfiInputReset, EfiSimpleTextInputProtocol, SCAN_DELETE, SCAN_DOWN, SCAN_END, SCAN_ESC, SCAN_F1,
    SCAN_F2, SCAN_F3, SCAN_F4, SCAN_F5, SCAN_F6, SCAN_F7, SCAN_F8, SCAN_F9, SCAN_F10, SCAN_HOME,
    SCAN_INSERT, SCAN_LEFT, SCAN_NULL, SCAN_PAGE_DOWN, SCAN_PAGE_UP, SCAN_RIGHT, SCAN_UP,
    SimpleTextInput,
};
pub use input_ex::{
    EFI_CAPS_LOCK_ACTIVE, EFI_KEY_STATE_EXPOSED, EFI_LEFT_ALT_PRESSED, EFI_LEFT_CONTROL_PRESSED,
    EFI_LEFT_LOGO_PRESSED, EFI_LEFT_SHIFT_PRESSED, EFI_MENU_KEY_PRESSED, EFI_NUM_LOCK_ACTIVE,
    EFI_RIGHT_ALT_PRESSED, EFI_RIGHT_CONTROL_PRESSED, EFI_RIGHT_LOGO_PRESSED,
    EFI_RIGHT_SHIFT_PRESSED, EFI_SCROLL_LOCK_ACTIVE, EFI_SHIFT_STATE_VALID, EFI_SYS_REQ_PRESSED,
    EFI_TOGGLE_STATE_VALID, EfiInputReadKeyEx, EfiInputResetEx, EfiKeyData, EfiKeyState,
    EfiRegisterKeystrokeNotify, EfiSetState, EfiSimpleTextInputExProtocol,
    EfiUnregisterKeystrokeNotify, SCAN_BRIGHTNESS_DOWN, SCAN_BRIGHTNESS_UP, SCAN_EJECT, SCAN_F11,
    SCAN_F12, SCAN_F13, SCAN_F14, SCAN_F15, SCAN_F16, SCAN_F17, SCAN_F18, SCAN_F19, SCAN_F20,
    SCAN_F21, SCAN_F22, SCAN_F23, SCAN_F24, SCAN_HIBERNATE, SCAN_MUTE, SCAN_PAUSE, SCAN_RECOVERY,
    SCAN_SUSPEND, SCAN_TOGGLE_DISPLAY, SCAN_VOLUME_DOWN, SCAN_VOLUME_UP,
    SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID, SimpleTextInputEx,
};
pub use output::{
    BLACK, BLUE, CYAN, EfiSimpleTextOutputMode, EfiSimpleTextOutputProtocol, EfiTextClearScreen,
    EfiTextEnableCursor, EfiTextQueryMode, EfiTextReset, EfiTextSetAttribute,
    EfiTextSetCursorPosition, EfiTextSetMode, EfiTextString, EfiTextTestString, GREEN, LIGHTGRAY,
    MAGENTA, RED, SimpleTextOutput, WHITE,
};
pub use pointer::{
    EfiSimplePointerGetState, EfiSimplePointerMode, EfiSimplePointerProtocol,
    EfiSimplePointerReset, EfiSimplePointerState, SIMPLE_POINTER_PROTOCOL_GUID, SimplePointer,
};
pub use print2::{
    AsciiBsPrint, AsciiBsPrintUnicodeFormat, AsciiSPrint, AsciiSPrintUnicodeFormat,
    AsciiValueToString, AsciiValueToStringS, EFI_PRINT2_PROTOCOL_GUID, EFI_PRINT2S_PROTOCOL_GUID,
    EfiPrint2Protocol, EfiPrint2SProtocol, Print2, Print2S, UnicodeBsPrint,
    UnicodeBsPrintAsciiFormat, UnicodeSPrint, UnicodeSPrintAsciiFormat, UnicodeValueToString,
    UnicodeValueToStringS, format_int64_ascii, format_int64_unicode, print_flags,
};
pub use ps2_keyboard::{
    EfiKeyQueue, KBC_CMD_DISABLE_KEYBOARD, KBC_CMD_DISABLE_MOUSE, KBC_CMD_ENABLE_KEYBOARD,
    KBC_CMD_ENABLE_MOUSE, KBC_CMD_READ_CONFIG_BYTE, KBC_CMD_SELF_TEST, KBC_CMD_WRITE_CONFIG_BYTE,
    KBC_CMD_WRITE_MOUSE, KBD_CMD_ENABLE_SCANNING, KBD_CMD_RESET, KBD_CMD_SET_LEDS, KBD_RESP_ACK,
    KBD_RESP_BAT_SUCCESS, KEYBOARD_EFI_KEY_MAX_COUNT, KEYBOARD_SCAN_CODE_MAX_COUNT,
    PS2_COMMAND_PORT, PS2_DATA_PORT, PS2_LED_CAPS_LOCK, PS2_LED_NUM_LOCK, PS2_LED_SCROLL_LOCK,
    PS2_STATUS_PORT, Ps2KeyboardDecoder, SCANCODE_ALT_BREAK, SCANCODE_ALT_MAKE,
    SCANCODE_CAPS_LOCK_MAKE, SCANCODE_CTRL_BREAK, SCANCODE_CTRL_MAKE, SCANCODE_DELETE_MAKE,
    SCANCODE_EXTENDED0, SCANCODE_EXTENDED1, SCANCODE_LEFT_SHIFT_MAKE, SCANCODE_NUM_LOCK_MAKE,
    SCANCODE_RIGHT_SHIFT_MAKE, SCANCODE_SCROLL_LOCK_MAKE, SCANCODE_SET1_TABLE, ScancodeMapping,
    ScancodeQueue,
};
pub use ps2_mouse::{
    MOUSE_CMD_ENABLE_STREAMING, MOUSE_CMD_GET_DEVICE_ID, MOUSE_CMD_RESET, MOUSE_CMD_SET_RESOLUTION,
    MOUSE_CMD_SET_SAMPLE_RATE, MOUSE_ID_5_BUTTON, MOUSE_ID_INTELLIMOUSE, MOUSE_ID_STANDARD,
    MOUSE_RESP_ACK, MOUSE_RESP_BAT1, MOUSE_RESP_BAT2, MouseResolution, MouseSampleRate,
    PS2_MOUSE_FLAG_BUTTON_4, PS2_MOUSE_FLAG_BUTTON_5, PS2_MOUSE_FLAG_LEFT_BUTTON,
    PS2_MOUSE_FLAG_MIDDLE_BUTTON, PS2_MOUSE_FLAG_RIGHT_BUTTON, PS2_MOUSE_FLAG_SYNC,
    PS2_MOUSE_FLAG_X_OVERFLOW, PS2_MOUSE_FLAG_Y_OVERFLOW, Ps2MouseDecoder, Ps2MouseError,
};
pub use ps2_policy::{
    EFI_KEYBOARD_CAPSLOCK, EFI_KEYBOARD_NUMLOCK, EFI_KEYBOARD_SCROLLLOCK,
    EFI_PS2_POLICY_PROTOCOL_GUID, EfiPs2InitHardware, EfiPs2PolicyProtocol,
};
pub use shell::{
    ARG_FIRST_CHAR_IS_ESC, ARG_FIRST_HALF_QUOTED, ARG_IS_QUOTED, ARG_NO_ATTRIB,
    ARG_PARTIALLY_QUOTED, EFI_SHELL_DYNAMIC_COMMAND_PROTOCOL_GUID,
    EFI_SHELL_PARAMETERS_PROTOCOL_GUID, EFI_SHELL_PROTOCOL_GUID, EfiShellArgInfo,
    EfiShellDynamicCommandProtocol, EfiShellExecute, EfiShellGetEnv, EfiShellInterface,
    EfiShellParametersProtocol, EfiShellProtocol, EfiShellSetEnv, SHELL_ALIAS_VARIABLE_GUID,
    SHELL_ENVIRONMENT_PROTOCOL_GUID, SHELL_ENVIRONMENT2_EXTENSION_GUID,
    SHELL_INTERFACE_PROTOCOL_GUID, SHELL_MAP_GUID, SHELL_VARIABLE_GUID, Shell, ShellCommandGetHelp,
    ShellCommandHandler, ShellFileHandle, ShellInterface, ShellParameters,
};
pub use shell_commands::{
    BUILTIN_COMMANDS, MAX_REGISTERED_COMMANDS, SHELL_PROFILE_BCFG, SHELL_PROFILE_DEBUG1,
    SHELL_PROFILE_DRIVER1, SHELL_PROFILE_LEVEL1, SHELL_PROFILE_LEVEL2, SHELL_PROFILE_LEVEL3,
    SHELL_PROFILE_NETWORK1, SHELL_SUPPORT_LEVEL_0, SHELL_SUPPORT_LEVEL_1, SHELL_SUPPORT_LEVEL_2,
    SHELL_SUPPORT_LEVEL_3, ShellCommandDescriptor, ShellCommandRegistry, ShellStatus,
    commands_for_level, commands_for_profile, is_builtin_command, lookup_command,
};
pub use text::{is_null_terminated, is_printable_terminated, utf16_bytes};
pub use unicode_collation::{
    EFI_UNICODE_COLLATION_PROTOCOL_GUID, EFI_UNICODE_COLLATION2_PROTOCOL_GUID,
    EfiUnicodeCollation2Protocol, EfiUnicodeCollationFatToStr, EfiUnicodeCollationMetaiMatch,
    EfiUnicodeCollationProtocol, EfiUnicodeCollationStrLwr, EfiUnicodeCollationStrToFat,
    EfiUnicodeCollationStrUpr, EfiUnicodeCollationStriColl,
};
pub use usb_keyboard::{
    USB_KBD_LED_CAPS_LOCK, USB_KBD_LED_COMPOSE, USB_KBD_LED_KANA, USB_KBD_LED_NUM_LOCK,
    USB_KBD_LED_SCROLL_LOCK, USB_KBD_MOD_LEFT_ALT, USB_KBD_MOD_LEFT_CTRL, USB_KBD_MOD_LEFT_GUI,
    USB_KBD_MOD_LEFT_SHIFT, USB_KBD_MOD_RIGHT_ALT, USB_KBD_MOD_RIGHT_CTRL, USB_KBD_MOD_RIGHT_GUI,
    USB_KBD_MOD_RIGHT_SHIFT, USB_KEY_MAP_TABLE, USB_USAGE_ERROR_ROLLOVER, USB_USAGE_KEY_A,
    USB_USAGE_KEY_CAPS_LOCK, USB_USAGE_KEY_ENTER, USB_USAGE_KEY_ESCAPE, USB_USAGE_KEY_SPACE,
    USB_USAGE_KEY_Z, USB_USAGE_KEYPAD_NUM_LOCK, UsbKeyMapping, UsbKeyboardDecoder,
};
pub use usb_mouse::{
    USB_MOUSE_BTN_4, USB_MOUSE_BTN_5, USB_MOUSE_BTN_LEFT, USB_MOUSE_BTN_MIDDLE,
    USB_MOUSE_BTN_RIGHT, UsbAbsolutePointerDecoder, UsbMouseDecoder,
};
pub use virtual_keyboard::{
    EfiPlatformVirtualKeyboardProtocol, PLATFORM_VIRTUAL_KEYBOARD_PROTOCOL_GUID,
    PlatformVirtualKbdClear, PlatformVirtualKbdQuery, PlatformVirtualKbdRegister,
    PlatformVirtualKbdReset, PlatformVirtualKeyboard, VIRTUAL_KEYBOARD_KEY_SIGNATURE,
    VirtualKbdKey,
};
