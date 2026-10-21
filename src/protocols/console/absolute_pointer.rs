//! # Absolute Pointer Protocol
//!
//! Declarations for `EFI_ABSOLUTE_POINTER_PROTOCOL` according to the UEFI Specification.
//! Provides absolute coordinate pointer input (touchscreen, tablet).

use crate::abi::types::Bool;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};

/// The absolute pointer protocol GUID: `{8d59d32b-c655-4ae9-9b15-f25904992a43}`.
pub const ABSOLUTE_POINTER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8d59_d32b,
    0xc655,
    0x4ae9,
    [0x9b, 0x15, 0xf2, 0x59, 0x04, 0x99, 0x2a, 0x43],
);

/// Attribute bit: device supports alternate button input.
pub const ABSOLUTE_POINTER_ATTR_SUPPORTS_ALT_ACTIVE: u32 = 0x0000_0001;
/// Attribute bit: device returns pressure data in `current_z`.
pub const ABSOLUTE_POINTER_ATTR_SUPPORTS_PRESSURE_AS_Z: u32 = 0x0000_0002;

/// State bit: touch sensor is active.
pub const ABSOLUTE_POINTER_TOUCH_ACTIVE: u32 = 0x0000_0001;
/// State bit: alternate sensor (e.g. pen side button) is active.
pub const ABSOLUTE_POINTER_ALT_ACTIVE: u32 = 0x0000_0002;

/// Mode describing absolute pointer coordinate ranges and capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiAbsolutePointerMode {
    /// Absolute minimum X coordinate.
    pub absolute_min_x: u64,
    /// Absolute minimum Y coordinate.
    pub absolute_min_y: u64,
    /// Absolute minimum Z coordinate.
    pub absolute_min_z: u64,
    /// Absolute maximum X coordinate (if min and max are both 0, X axis unsupported).
    pub absolute_max_x: u64,
    /// Absolute maximum Y coordinate (if min and max are both 0, Y axis unsupported).
    pub absolute_max_y: u64,
    /// Absolute maximum Z coordinate (if min and max are both 0, Z axis unsupported).
    pub absolute_max_z: u64,
    /// Capabilities bitmask (`ABSOLUTE_POINTER_ATTR_*`).
    pub attributes: u32,
}

/// State of an absolute pointer activation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiAbsolutePointerState {
    /// Unsigned X position of the activation.
    pub current_x: u64,
    /// Unsigned Y position of the activation.
    pub current_y: u64,
    /// Unsigned Z position or pressure measurement.
    pub current_z: u64,
    /// Active buttons bitmask (`ABSOLUTE_POINTER_*_ACTIVE`).
    pub active_buttons: u32,
}

/// Function pointer for resetting absolute pointer hardware.
pub type EfiAbsolutePointerReset = unsafe extern "efiapi" fn(
    this: *mut EfiAbsolutePointerProtocol,
    extended_verification: Bool,
) -> EfiStatus;

/// Function pointer for retrieving absolute pointer state.
pub type EfiAbsolutePointerGetState = unsafe extern "efiapi" fn(
    this: *mut EfiAbsolutePointerProtocol,
    state: *mut EfiAbsolutePointerState,
) -> EfiStatus;

/// Protocol structure for `EFI_ABSOLUTE_POINTER_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiAbsolutePointerProtocol {
    /// Resets the pointer device hardware.
    pub reset: EfiAbsolutePointerReset,
    /// Retrieves the current state of a pointer device.
    pub get_state: EfiAbsolutePointerGetState,
    /// Event to use with `WaitForEvent()` to wait for pointer input.
    pub wait_for_input: EfiEvent,
    /// Pointer to `EfiAbsolutePointerMode` data.
    pub mode: *mut EfiAbsolutePointerMode,
}

/// Safe wrapper for `EfiAbsolutePointerProtocol`.
pub struct AbsolutePointer<'a> {
    protocol: &'a EfiAbsolutePointerProtocol,
}

impl<'a> AbsolutePointer<'a> {
    /// Creates a wrapper borrowing an absolute pointer protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiAbsolutePointerProtocol) -> Self {
        Self { protocol }
    }

    /// Resets the absolute pointer hardware.
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
    pub fn get_state(&self) -> Result<EfiAbsolutePointerState, EfiStatus> {
        let mut state = EfiAbsolutePointerState::default();
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
    pub fn mode(&self) -> Option<&EfiAbsolutePointerMode> {
        if self.protocol.mode.is_null() {
            None
        } else {
            // SAFETY: The pointer is non-null and provided by firmware.
            Some(unsafe { &*self.protocol.mode })
        }
    }
}
