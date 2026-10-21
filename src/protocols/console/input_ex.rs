//! # Simple Text Input Ex Protocol
//!
//! Declarations for `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` according to the UEFI Specification.
//!
//! Provides extended input features including keystroke notification, shift/modifier states,
//! and toggle states (Caps Lock, Num Lock, Scroll Lock).

use crate::abi::types::Bool;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::console::input::EfiInputKey;
use core::ffi::c_void;

/// The simple text input ex protocol GUID: `{dd9e7534-7762-4698-8c14-f58517a625aa}`.
pub const SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xdd9e_7534,
    0x7762,
    0x4698,
    [0x8c, 0x14, 0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa],
);

// Shift states
/// Shift state valid bit (high bit set).
pub const EFI_SHIFT_STATE_VALID: u32 = 0x8000_0000;
/// Right Shift pressed.
pub const EFI_RIGHT_SHIFT_PRESSED: u32 = 0x0000_0001;
/// Left Shift pressed.
pub const EFI_LEFT_SHIFT_PRESSED: u32 = 0x0000_0002;
/// Right Control pressed.
pub const EFI_RIGHT_CONTROL_PRESSED: u32 = 0x0000_0004;
/// Left Control pressed.
pub const EFI_LEFT_CONTROL_PRESSED: u32 = 0x0000_0008;
/// Right Alt pressed.
pub const EFI_RIGHT_ALT_PRESSED: u32 = 0x0000_0010;
/// Left Alt pressed.
pub const EFI_LEFT_ALT_PRESSED: u32 = 0x0000_0020;
/// Right Logo pressed.
pub const EFI_RIGHT_LOGO_PRESSED: u32 = 0x0000_0040;
/// Left Logo pressed.
pub const EFI_LEFT_LOGO_PRESSED: u32 = 0x0000_0080;
/// Menu key pressed.
pub const EFI_MENU_KEY_PRESSED: u32 = 0x0000_0100;
/// `SysReq` key pressed.
pub const EFI_SYS_REQ_PRESSED: u32 = 0x0000_0200;

// Toggle states
/// Toggle state valid bit (high bit set).
pub const EFI_TOGGLE_STATE_VALID: u8 = 0x80;
/// Key state exposed bit.
pub const EFI_KEY_STATE_EXPOSED: u8 = 0x40;
/// Scroll Lock active.
pub const EFI_SCROLL_LOCK_ACTIVE: u8 = 0x01;
/// Num Lock active.
pub const EFI_NUM_LOCK_ACTIVE: u8 = 0x02;
/// Caps Lock active.
pub const EFI_CAPS_LOCK_ACTIVE: u8 = 0x04;

// Additional scan codes defined in SimpleTextInEx
/// F11 function key.
pub const SCAN_F11: u16 = 0x0015;
/// F12 function key.
pub const SCAN_F12: u16 = 0x0016;
/// Pause key.
pub const SCAN_PAUSE: u16 = 0x0048;
/// F13 function key.
pub const SCAN_F13: u16 = 0x0068;
/// F14 function key.
pub const SCAN_F14: u16 = 0x0069;
/// F15 function key.
pub const SCAN_F15: u16 = 0x006A;
/// F16 function key.
pub const SCAN_F16: u16 = 0x006B;
/// F17 function key.
pub const SCAN_F17: u16 = 0x006C;
/// F18 function key.
pub const SCAN_F18: u16 = 0x006D;
/// F19 function key.
pub const SCAN_F19: u16 = 0x006E;
/// F20 function key.
pub const SCAN_F20: u16 = 0x006F;
/// F21 function key.
pub const SCAN_F21: u16 = 0x0070;
/// F22 function key.
pub const SCAN_F22: u16 = 0x0071;
/// F23 function key.
pub const SCAN_F23: u16 = 0x0072;
/// F24 function key.
pub const SCAN_F24: u16 = 0x0073;
/// Mute key.
pub const SCAN_MUTE: u16 = 0x007F;
/// Volume up key.
pub const SCAN_VOLUME_UP: u16 = 0x0080;
/// Volume down key.
pub const SCAN_VOLUME_DOWN: u16 = 0x0081;
/// Brightness up key.
pub const SCAN_BRIGHTNESS_UP: u16 = 0x0100;
/// Brightness down key.
pub const SCAN_BRIGHTNESS_DOWN: u16 = 0x0101;
/// Suspend key.
pub const SCAN_SUSPEND: u16 = 0x0102;
/// Hibernate key.
pub const SCAN_HIBERNATE: u16 = 0x0103;
/// Toggle display key.
pub const SCAN_TOGGLE_DISPLAY: u16 = 0x0104;
/// Recovery key.
pub const SCAN_RECOVERY: u16 = 0x0105;
/// Eject key.
pub const SCAN_EJECT: u16 = 0x0106;

/// State of keyboard modifiers and toggles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiKeyState {
    /// Currently pressed shift modifiers. Valid only if `EFI_SHIFT_STATE_VALID` is set.
    pub key_shift_state: u32,
    /// Current toggle state. Valid only if `EFI_TOGGLE_STATE_VALID` is set.
    pub key_toggle_state: u8,
}

/// Keystroke data including character, scan code, and modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiKeyData {
    /// Keystroke (scan code and unicode character).
    pub key: EfiInputKey,
    /// Modifier and toggle state.
    pub key_state: EfiKeyState,
}

/// Callback function invoked when registered key combination is typed.
pub type EfiKeyNotifyFunction = unsafe extern "efiapi" fn(key_data: *mut EfiKeyData) -> EfiStatus;

/// Function pointer for resetting input device hardware.
pub type EfiInputResetEx = unsafe extern "efiapi" fn(
    this: *mut EfiSimpleTextInputExProtocol,
    extended_verification: Bool,
) -> EfiStatus;

/// Function pointer for reading next keystroke and state.
pub type EfiInputReadKeyEx = unsafe extern "efiapi" fn(
    this: *mut EfiSimpleTextInputExProtocol,
    key_data: *mut EfiKeyData,
) -> EfiStatus;

/// Function pointer for setting device toggle state.
pub type EfiSetState = unsafe extern "efiapi" fn(
    this: *mut EfiSimpleTextInputExProtocol,
    key_toggle_state: *mut u8,
) -> EfiStatus;

/// Function pointer for registering a keystroke notification.
pub type EfiRegisterKeystrokeNotify = unsafe extern "efiapi" fn(
    this: *mut EfiSimpleTextInputExProtocol,
    key_data: *const EfiKeyData,
    key_notification_function: EfiKeyNotifyFunction,
    notify_handle: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for unregistering a keystroke notification.
pub type EfiUnregisterKeystrokeNotify = unsafe extern "efiapi" fn(
    this: *mut EfiSimpleTextInputExProtocol,
    notification_handle: *mut c_void,
) -> EfiStatus;

/// Protocol structure for `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSimpleTextInputExProtocol {
    /// Resets the input device hardware.
    pub reset: EfiInputResetEx,
    /// Reads the next keystroke and modifier state.
    pub read_key_stroke_ex: EfiInputReadKeyEx,
    /// Event to use with `WaitForEvent()` to wait for a key to be available.
    pub wait_for_key_ex: EfiEvent,
    /// Sets keyboard toggle state.
    pub set_state: EfiSetState,
    /// Registers a keystroke notification callback.
    pub register_key_notify: EfiRegisterKeystrokeNotify,
    /// Unregisters a previously registered callback.
    pub unregister_key_notify: EfiUnregisterKeystrokeNotify,
}

/// Safe wrapper for `EfiSimpleTextInputExProtocol`.
pub struct SimpleTextInputEx<'a> {
    protocol: &'a EfiSimpleTextInputExProtocol,
}

impl<'a> SimpleTextInputEx<'a> {
    /// Creates a wrapper borrowing an input ex protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiSimpleTextInputExProtocol) -> Self {
        Self { protocol }
    }

    /// Resets the input device hardware.
    ///
    /// # Errors
    ///
    /// Returns an error if hardware reset fails.
    pub fn reset(&self, extended_verification: bool) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.reset)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                Bool::from(extended_verification),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads the next keystroke and state.
    ///
    /// # Errors
    ///
    /// Returns `NOT_READY` if no key is available.
    pub fn read_key_stroke_ex(&self) -> Result<EfiKeyData, EfiStatus> {
        let mut key_data = EfiKeyData::default();
        let status = unsafe {
            (self.protocol.read_key_stroke_ex)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut key_data,
            )
        };
        if status.is_success() {
            Ok(key_data)
        } else {
            Err(status)
        }
    }

    /// Sets keyboard toggle state (Scroll Lock, Num Lock, Caps Lock).
    ///
    /// # Errors
    ///
    /// Returns an error if state cannot be set.
    pub fn set_state(&self, mut toggle_state: u8) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.set_state)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut toggle_state,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the wait-for-key event.
    #[must_use]
    pub fn wait_for_key_event(&self) -> EfiEvent {
        self.protocol.wait_for_key_ex
    }
}
