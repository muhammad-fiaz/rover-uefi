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
pub mod ps2_policy;
pub mod shell;
pub mod text;
pub mod unicode_collation;
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
pub use text::{is_null_terminated, is_printable_terminated, utf16_bytes};
pub use unicode_collation::{
    EFI_UNICODE_COLLATION_PROTOCOL_GUID, EFI_UNICODE_COLLATION2_PROTOCOL_GUID,
    EfiUnicodeCollation2Protocol, EfiUnicodeCollationFatToStr, EfiUnicodeCollationMetaiMatch,
    EfiUnicodeCollationProtocol, EfiUnicodeCollationStrLwr, EfiUnicodeCollationStrToFat,
    EfiUnicodeCollationStrUpr, EfiUnicodeCollationStriColl,
};
pub use virtual_keyboard::{
    EfiPlatformVirtualKeyboardProtocol, PLATFORM_VIRTUAL_KEYBOARD_PROTOCOL_GUID,
    PlatformVirtualKbdClear, PlatformVirtualKbdQuery, PlatformVirtualKbdRegister,
    PlatformVirtualKbdReset, PlatformVirtualKeyboard, VIRTUAL_KEYBOARD_KEY_SIGNATURE,
    VirtualKbdKey,
};
