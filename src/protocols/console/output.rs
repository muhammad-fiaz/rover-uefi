//! # Simple text output
//!
//! The `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
//!
//! The wrapper takes null-terminated UTF-16
//! slices and validates termination before the
//! firmware call. No allocation occurs. Mode
//! geometry is firmware dependent and is
//! obtained through query mode.

use crate::abi::types::{Bool, Char16, UIntN};
use crate::base::EfiStatus;
use crate::protocols::console::text::is_null_terminated;

/// Text mode information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiSimpleTextOutputMode {
    /// The number of supported modes.
    pub max_mode: i32,
    /// The current mode.
    pub mode: i32,
    /// The current attribute.
    pub attribute: i32,
    /// The cursor column.
    pub cursor_column: i32,
    /// The cursor row.
    pub cursor_row: i32,
    /// Whether the cursor is visible.
    pub cursor_visible: Bool,
}

/// Black foreground color.
pub const BLACK: u8 = 0x00;

/// Blue foreground color.
pub const BLUE: u8 = 0x01;

/// Green foreground color.
pub const GREEN: u8 = 0x02;

/// Cyan foreground color.
pub const CYAN: u8 = 0x03;

/// Red foreground color.
pub const RED: u8 = 0x04;

/// Magenta foreground color.
pub const MAGENTA: u8 = 0x05;

/// Light gray foreground color.
pub const LIGHTGRAY: u8 = 0x07;

/// White foreground color.
pub const WHITE: u8 = 0x0F;

/// Resets the output device.
pub type EfiTextReset =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, Bool) -> EfiStatus;

/// Writes a string.
pub type EfiTextString =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, *mut Char16) -> EfiStatus;

/// Tests a string.
pub type EfiTextTestString =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, *mut Char16) -> EfiStatus;

/// Queries a mode.
pub type EfiTextQueryMode = unsafe extern "efiapi" fn(
    *mut EfiSimpleTextOutputProtocol,
    UIntN,
    *mut UIntN,
    *mut UIntN,
) -> EfiStatus;

/// Sets a mode.
pub type EfiTextSetMode =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, UIntN) -> EfiStatus;

/// Sets an attribute.
pub type EfiTextSetAttribute =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, UIntN) -> EfiStatus;

/// Clears the screen.
pub type EfiTextClearScreen =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol) -> EfiStatus;

/// Sets the cursor position.
pub type EfiTextSetCursorPosition =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, UIntN, UIntN) -> EfiStatus;

/// Enables the cursor.
pub type EfiTextEnableCursor =
    unsafe extern "efiapi" fn(*mut EfiSimpleTextOutputProtocol, Bool) -> EfiStatus;

/// The simple text output protocol.
///
/// The struct is `#[repr(C)]` so field order
/// and offsets match the specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    /// Resets the output device.
    pub reset: EfiTextReset,
    /// Writes a string.
    pub output_string: EfiTextString,
    /// Tests a string.
    pub test_string: EfiTextTestString,
    /// Queries a mode.
    pub query_mode: EfiTextQueryMode,
    /// Sets a mode.
    pub set_mode: EfiTextSetMode,
    /// Sets an attribute.
    pub set_attribute: EfiTextSetAttribute,
    /// Clears the screen.
    pub clear_screen: EfiTextClearScreen,
    /// Sets the cursor position.
    pub set_cursor_position: EfiTextSetCursorPosition,
    /// Enables the cursor.
    pub enable_cursor: EfiTextEnableCursor,
    /// The mode information pointer.
    pub mode: *mut EfiSimpleTextOutputMode,
}

const _: () = {
    assert!(
        core::mem::size_of::<EfiSimpleTextOutputProtocol>() == 10 * core::mem::size_of::<usize>()
    );
    assert!(core::mem::offset_of!(EfiSimpleTextOutputProtocol, reset) == 0);
    assert!(
        core::mem::offset_of!(EfiSimpleTextOutputProtocol, output_string)
            == core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiSimpleTextOutputProtocol, mode)
            == 9 * core::mem::size_of::<usize>()
    );
    assert!(core::mem::size_of::<EfiSimpleTextOutputMode>() == 24);
    assert!(core::mem::offset_of!(EfiSimpleTextOutputMode, cursor_visible) == 20);
};

/// A typed simple text output view.
pub struct SimpleTextOutput<'a> {
    protocol: &'a EfiSimpleTextOutputProtocol,
}

impl<'a> SimpleTextOutput<'a> {
    /// Creates a view from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiSimpleTextOutputProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSimpleTextOutputProtocol {
        self.protocol
    }

    /// Resets the output device.
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

    /// Writes a null-terminated string.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` without
    /// calling firmware when the string is not
    /// null-terminated.
    pub fn output_string(&self, string: &[Char16]) -> Result<(), EfiStatus> {
        if !is_null_terminated(string) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice is valid and null-terminated for the call.
        let status = unsafe {
            (self.protocol.output_string)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                string.as_ptr().cast_mut(),
            )
        };
        if status.is_success() || status.is_warning() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Tests whether a string can be rendered.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` without
    /// calling firmware when the string is not
    /// null-terminated.
    pub fn test_string(&self, string: &[Char16]) -> Result<(), EfiStatus> {
        if !is_null_terminated(string) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice is valid and null-terminated for the call.
        let status = unsafe {
            (self.protocol.test_string)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                string.as_ptr().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries mode geometry.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// mode is invalid.
    pub fn query_mode(&self, mode_number: UIntN) -> Result<(UIntN, UIntN), EfiStatus> {
        let mut columns: UIntN = 0;
        let mut rows: UIntN = 0;
        // SAFETY: The borrowed protocol is valid and stack locals are valid for out parameters.
        let status = unsafe {
            (self.protocol.query_mode)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                mode_number,
                &raw mut columns,
                &raw mut rows,
            )
        };
        if status.is_success() {
            Ok((columns, rows))
        } else {
            Err(status)
        }
    }

    /// Sets the mode.
    #[must_use]
    pub fn set_mode(&self, mode_number: UIntN) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_mode)(core::ptr::from_ref(self.protocol).cast_mut(), mode_number)
        }
    }

    /// Sets the attribute.
    #[must_use]
    pub fn set_attribute(&self, attribute: UIntN) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_attribute)(core::ptr::from_ref(self.protocol).cast_mut(), attribute)
        }
    }

    /// Clears the screen.
    #[must_use]
    pub fn clear_screen(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.clear_screen)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Sets the cursor position.
    #[must_use]
    pub fn set_cursor_position(&self, column: UIntN, row: UIntN) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_cursor_position)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                column,
                row,
            )
        }
    }

    /// Enables the cursor.
    #[must_use]
    pub fn enable_cursor(&self, visible: Bool) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.enable_cursor)(core::ptr::from_ref(self.protocol).cast_mut(), visible)
        }
    }
}

impl core::fmt::Debug for SimpleTextOutput<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SimpleTextOutput")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
