//! # Simple Pointer Protocol
//!
//! Declarations for `EFI_SIMPLE_POINTER_PROTOCOL` according to the UEFI Specification.
//! Provides relative pointer input (mouse, trackball).

use crate::abi::types::Bool;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};

/// The simple pointer protocol GUID: `{31878c87-0b75-11d5-9a4f-0090273fc14d}`.
pub const SIMPLE_POINTER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3187_8c87,
    0x0b75,
    0x11d5,
    [0x9a, 0x4f, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

/// State of a simple pointer device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiSimplePointerState {
    /// Relative movement along the X axis in counts.
    pub relative_movement_x: i32,
    /// Relative movement along the Y axis in counts.
    pub relative_movement_y: i32,
    /// Relative movement along the Z axis in counts.
    pub relative_movement_z: i32,
    /// True if the left button is pressed.
    pub left_button: Bool,
    /// True if the right button is pressed.
    pub right_button: Bool,
}

/// Mode describing the capabilities of a simple pointer device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiSimplePointerMode {
    /// Resolution on the X axis in counts/mm (0 if X axis unsupported).
    pub resolution_x: u64,
    /// Resolution on the Y axis in counts/mm (0 if Y axis unsupported).
    pub resolution_y: u64,
    /// Resolution on the Z axis in counts/mm (0 if Z axis unsupported).
    pub resolution_z: u64,
    /// True if a left button is present.
    pub left_button: Bool,
    /// True if a right button is present.
    pub right_button: Bool,
}

/// Function pointer for resetting simple pointer hardware.
pub type EfiSimplePointerReset = unsafe extern "efiapi" fn(
    this: *mut EfiSimplePointerProtocol,
    extended_verification: Bool,
) -> EfiStatus;

/// Function pointer for retrieving simple pointer state.
pub type EfiSimplePointerGetState = unsafe extern "efiapi" fn(
    this: *mut EfiSimplePointerProtocol,
    state: *mut EfiSimplePointerState,
) -> EfiStatus;

/// Protocol structure for `EFI_SIMPLE_POINTER_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSimplePointerProtocol {
    /// Resets the pointer device hardware.
    pub reset: EfiSimplePointerReset,
    /// Retrieves the current state of a pointer device.
    pub get_state: EfiSimplePointerGetState,
    /// Event to use with `WaitForEvent()` to wait for pointer input.
    pub wait_for_input: EfiEvent,
    /// Pointer to `EfiSimplePointerMode` data.
    pub mode: *mut EfiSimplePointerMode,
}

/// Safe wrapper for `EfiSimplePointerProtocol`.
pub struct SimplePointer<'a> {
    protocol: &'a EfiSimplePointerProtocol,
}

impl<'a> SimplePointer<'a> {
    /// Creates a wrapper borrowing a pointer protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiSimplePointerProtocol) -> Self {
        Self { protocol }
    }

    /// Resets the pointer hardware.
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

    /// Retrieves the current pointer state.
    ///
    /// # Errors
    ///
    /// Returns `NOT_READY` if state hasn't changed since last call, or device error.
    pub fn get_state(&self) -> Result<EfiSimplePointerState, EfiStatus> {
        let mut state = EfiSimplePointerState::default();
        let status = unsafe {
            (self.protocol.get_state)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut state,
            )
        };
        if status.is_success() {
            Ok(state)
        } else {
            Err(status)
        }
    }

    /// Returns the wait-for-input event.
    #[must_use]
    pub fn wait_for_input_event(&self) -> EfiEvent {
        self.protocol.wait_for_input
    }

    /// Returns a reference to the mode structure if available.
    #[must_use]
    pub fn mode(&self) -> Option<&EfiSimplePointerMode> {
        if self.protocol.mode.is_null() {
            None
        } else {
            // SAFETY: The pointer is non-null and provided by firmware.
            Some(unsafe { &*self.protocol.mode })
        }
    }
}
