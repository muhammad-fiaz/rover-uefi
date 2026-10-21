//! # Simple text input
//!
//! The `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`.
//!
//! The protocol is available on the console
//! input handle in the system table. Input is
//! non-blocking through `ReadKeyStroke` and
//! event-driven through `WaitForKey` with
//! `WaitForEvent`. Not every key produces a
//! printable character; scan codes identify
//! function and navigation keys.

use crate::abi::types::{Bool, Char16};
use crate::base::{EfiEvent, EfiStatus};

/// A keystroke.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiInputKey {
    /// The scan code.
    pub scan_code: u16,
    /// The Unicode character.
    pub unicode_char: Char16,
}

/// Backspace Unicode character.
pub const CHAR_BACKSPACE: Char16 = 0x0008;

/// Tab Unicode character.
pub const CHAR_TAB: Char16 = 0x0009;

/// Line feed Unicode character.
pub const CHAR_LINEFEED: Char16 = 0x000A;

/// Carriage return Unicode character.
pub const CHAR_CARRIAGE_RETURN: Char16 = 0x000D;

/// The null scan code.
pub const SCAN_NULL: u16 = 0x0000;

/// The up arrow scan code.
pub const SCAN_UP: u16 = 0x0001;

/// The down arrow scan code.
pub const SCAN_DOWN: u16 = 0x0002;

/// The right arrow scan code.
pub const SCAN_RIGHT: u16 = 0x0003;

/// The left arrow scan code.
pub const SCAN_LEFT: u16 = 0x0004;

/// The home scan code.
pub const SCAN_HOME: u16 = 0x0005;

/// The end scan code.
pub const SCAN_END: u16 = 0x0006;

/// The insert scan code.
pub const SCAN_INSERT: u16 = 0x0007;

/// The delete scan code.
pub const SCAN_DELETE: u16 = 0x0008;

/// The page up scan code.
pub const SCAN_PAGE_UP: u16 = 0x0009;

/// The page down scan code.
pub const SCAN_PAGE_DOWN: u16 = 0x000A;

/// The F1 scan code.
pub const SCAN_F1: u16 = 0x000B;

/// The F2 scan code.
pub const SCAN_F2: u16 = 0x000C;

/// The F3 scan code.
pub const SCAN_F3: u16 = 0x000D;

/// The F4 scan code.
pub const SCAN_F4: u16 = 0x000E;

/// The F5 scan code.
pub const SCAN_F5: u16 = 0x000F;

/// The F6 scan code.
pub const SCAN_F6: u16 = 0x0010;

/// The F7 scan code.
pub const SCAN_F7: u16 = 0x0011;

/// The F8 scan code.
pub const SCAN_F8: u16 = 0x0012;

/// The F9 scan code.
pub const SCAN_F9: u16 = 0x0013;

/// The F10 scan code.
pub const SCAN_F10: u16 = 0x0014;

/// The escape scan code.
pub const SCAN_ESC: u16 = 0x0017;

/// Resets the input device.
pub type EfiInputReset =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextInputProtocol, Bool) -> EfiStatus;

/// Reads the next keystroke.
pub type EfiInputReadKey =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextInputProtocol, *mut EfiInputKey) -> EfiStatus;

/// The simple text input protocol.
///
/// The struct is `#[repr(C)]` so field order
/// and offsets match the specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    /// Resets the input device.
    pub reset: EfiInputReset,
    /// Reads the next keystroke.
    pub read_key_stroke: EfiInputReadKey,
    /// The key available event.
    pub wait_for_key: EfiEvent,
}

const _: () = {
    assert!(
        core::mem::size_of::<EfiSimpleTextInputProtocol>() == 3 * core::mem::size_of::<usize>()
    );
    assert!(core::mem::offset_of!(EfiSimpleTextInputProtocol, reset) == 0);
    assert!(
        core::mem::offset_of!(EfiSimpleTextInputProtocol, read_key_stroke)
            == core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiSimpleTextInputProtocol, wait_for_key)
            == 2 * core::mem::size_of::<usize>()
    );
    assert!(core::mem::size_of::<EfiInputKey>() == 4);
    assert!(core::mem::offset_of!(EfiInputKey, scan_code) == 0);
    assert!(core::mem::offset_of!(EfiInputKey, unicode_char) == 2);
};

/// A typed simple text input view.
///
/// The view borrows the firmware protocol
/// instance. The console input handle must
/// remain valid and the protocol must not be
/// uninstalled while the view is alive.
pub struct SimpleTextInput<'a> {
    protocol: &'a EfiSimpleTextInputProtocol,
}

impl<'a> SimpleTextInput<'a> {
    /// Creates a view from a protocol reference.
    ///
    /// The caller must ensure the reference
    /// points to a live firmware instance.
    #[must_use]
    pub const fn new(protocol: &'a EfiSimpleTextInputProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSimpleTextInputProtocol {
        self.protocol
    }

    /// Resets the input device.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// device cannot be reset.
    pub fn reset(&self, extended_verification: Bool) -> Result<(), EfiStatus> {
        // SAFETY: The borrowed protocol is valid for the call.
        let status = unsafe {
            (self.protocol.reset)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                extended_verification,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads the next keystroke.
    ///
    /// # Errors
    ///
    /// Returns `NOT_READY` when no key is
    /// available.
    pub fn read_key_stroke(&self) -> Result<EfiInputKey, EfiStatus> {
        let mut key = EfiInputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        // SAFETY: The borrowed protocol is valid and the stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.read_key_stroke)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut key,
            )
        };
        if status.is_success() {
            Ok(key)
        } else {
            Err(status)
        }
    }

    /// Returns the key available event.
    #[must_use]
    pub fn wait_for_key_event(&self) -> EfiEvent {
        self.protocol.wait_for_key
    }
}

impl core::fmt::Debug for SimpleTextInput<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SimpleTextInput")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
