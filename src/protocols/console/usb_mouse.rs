//! # USB HID Mouse and Absolute Pointer Decoders
//!
//! Production-ready implementation of USB Human Interface Device (HID)
//! mouse and pointer protocols for UEFI systems.
//!
//! Provides decoders for:
//! - USB HID Boot Mouse relative pointer reports (`EFI_SIMPLE_POINTER_PROTOCOL`).
//! - USB Absolute Pointer / Digitizer reports (`EFI_ABSOLUTE_POINTER_PROTOCOL`).
//!
//! Includes button bit decoding (Buttons 1..5), signed relative displacements,
//! scroll wheel deltas, absolute coordinate bounding and clamping, and state
//! change notification flags.

use crate::abi::types::{EFI_FALSE, EFI_TRUE};
use crate::base::EfiStatus;
use crate::protocols::console::absolute_pointer::{
    ABSOLUTE_POINTER_ALT_ACTIVE, ABSOLUTE_POINTER_ATTR_SUPPORTS_ALT_ACTIVE,
    ABSOLUTE_POINTER_ATTR_SUPPORTS_PRESSURE_AS_Z, ABSOLUTE_POINTER_TOUCH_ACTIVE,
    EfiAbsolutePointerMode, EfiAbsolutePointerState,
};
use crate::protocols::console::pointer::{EfiSimplePointerMode, EfiSimplePointerState};

// ---------------------------------------------------------------------------
// USB HID Mouse Report Button Bits (Report Byte 0)
// ---------------------------------------------------------------------------

/// Button 1 (Left button).
pub const USB_MOUSE_BTN_LEFT: u8 = 0x01;
/// Button 2 (Right button).
pub const USB_MOUSE_BTN_RIGHT: u8 = 0x02;
/// Button 3 (Middle button / wheel click).
pub const USB_MOUSE_BTN_MIDDLE: u8 = 0x04;
/// Button 4 (Side / back button).
pub const USB_MOUSE_BTN_4: u8 = 0x08;
/// Button 5 (Side / forward button).
pub const USB_MOUSE_BTN_5: u8 = 0x10;

// ---------------------------------------------------------------------------
// USB Relative Mouse Decoder
// ---------------------------------------------------------------------------

/// Production-ready USB HID Boot Mouse report decoder implementing
/// relative pointer state tracking.
#[derive(Debug)]
pub struct UsbMouseDecoder {
    /// Accumulated pointer state.
    state: EfiSimplePointerState,
    /// Whether state has changed since last read.
    state_changed: bool,
    /// Simple pointer mode parameters.
    mode: EfiSimplePointerMode,
}

impl Default for UsbMouseDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbMouseDecoder {
    /// Creates a new USB mouse decoder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: EfiSimplePointerState {
                relative_movement_x: 0,
                relative_movement_y: 0,
                relative_movement_z: 0,
                left_button: EFI_FALSE,
                right_button: EFI_FALSE,
            },
            state_changed: false,
            mode: EfiSimplePointerMode {
                resolution_x: 8,
                resolution_y: 8,
                resolution_z: 1,
                left_button: EFI_TRUE,
                right_button: EFI_TRUE,
            },
        }
    }

    /// Resets the accumulated movement and button states.
    pub fn reset(&mut self) {
        self.state.relative_movement_x = 0;
        self.state.relative_movement_y = 0;
        self.state.relative_movement_z = 0;
        self.state.left_button = EFI_FALSE;
        self.state.right_button = EFI_FALSE;
        self.state_changed = false;
    }

    /// Returns a reference to the pointer mode descriptor.
    #[must_use]
    pub const fn mode(&self) -> &EfiSimplePointerMode {
        &self.mode
    }

    /// Returns `true` if mouse state has changed since the last `get_state()` call.
    #[must_use]
    pub const fn has_state_changed(&self) -> bool {
        self.state_changed
    }

    /// Ingests a USB HID Boot Mouse report (minimum 3 bytes: buttons, dX, dY, optional dZ).
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if the report length is less than 3 bytes.
    #[allow(clippy::cast_possible_wrap)]
    pub fn process_report(&mut self, report: &[u8]) -> Result<(), EfiStatus> {
        if report.len() < 3 {
            return Err(EfiStatus::DEVICE_ERROR);
        }

        let buttons = report[0];
        let dx = report[1] as i8;
        let dy = report[2] as i8;
        let dz = if report.len() > 3 { report[3] as i8 } else { 0 };

        let left = (buttons & USB_MOUSE_BTN_LEFT) != 0;
        let right = (buttons & USB_MOUSE_BTN_RIGHT) != 0;

        self.state.relative_movement_x += i32::from(dx);
        self.state.relative_movement_y += i32::from(dy);
        self.state.relative_movement_z += i32::from(dz);
        self.state.left_button = if left { EFI_TRUE } else { EFI_FALSE };
        self.state.right_button = if right { EFI_TRUE } else { EFI_FALSE };
        self.state_changed = true;

        Ok(())
    }

    /// Implements `EFI_SIMPLE_POINTER_PROTOCOL.GetState()`:
    /// Returns the accumulated relative movement and button states, and clears
    /// movement deltas to 0.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_READY` if the mouse state has not changed.
    pub fn get_state(&mut self) -> Result<EfiSimplePointerState, EfiStatus> {
        if !self.state_changed {
            return Err(EfiStatus::NOT_READY);
        }

        let result = self.state;
        self.state.relative_movement_x = 0;
        self.state.relative_movement_y = 0;
        self.state.relative_movement_z = 0;
        self.state_changed = false;

        Ok(result)
    }
}

// ---------------------------------------------------------------------------
// USB Absolute Pointer Decoder
// ---------------------------------------------------------------------------

/// Production-ready USB Absolute Pointer and Digitizer decoder implementing
/// `EFI_ABSOLUTE_POINTER_PROTOCOL`.
///
/// Supports converting relative USB mouse displacement into bounded virtual
/// screen space, as well as decoding direct absolute coordinate touch digitizer reports.
#[derive(Debug)]
pub struct UsbAbsolutePointerDecoder {
    /// Current pointer state.
    state: EfiAbsolutePointerState,
    /// Whether state has changed since last `get_state()`.
    state_changed: bool,
    /// Absolute pointer mode capabilities and bounding limits.
    mode: EfiAbsolutePointerMode,
}

impl Default for UsbAbsolutePointerDecoder {
    fn default() -> Self {
        Self::new(1024, 1024, 1024)
    }
}

impl UsbAbsolutePointerDecoder {
    /// Creates a new absolute pointer decoder with specified coordinate limits.
    #[must_use]
    pub const fn new(max_x: u64, max_y: u64, max_z: u64) -> Self {
        Self {
            state: EfiAbsolutePointerState {
                current_x: max_x / 2,
                current_y: max_y / 2,
                current_z: 0,
                active_buttons: 0,
            },
            state_changed: false,
            mode: EfiAbsolutePointerMode {
                absolute_min_x: 0,
                absolute_min_y: 0,
                absolute_min_z: 0,
                absolute_max_x: max_x,
                absolute_max_y: max_y,
                absolute_max_z: max_z,
                attributes: ABSOLUTE_POINTER_ATTR_SUPPORTS_ALT_ACTIVE
                    | ABSOLUTE_POINTER_ATTR_SUPPORTS_PRESSURE_AS_Z,
            },
        }
    }

    /// Resets the cursor position to the center of the bounding box.
    pub fn reset(&mut self) {
        self.state.current_x = u64::midpoint(self.mode.absolute_max_x, self.mode.absolute_min_x);
        self.state.current_y = u64::midpoint(self.mode.absolute_max_y, self.mode.absolute_min_y);
        self.state.current_z = 0;
        self.state.active_buttons = 0;
        self.state_changed = false;
    }

    /// Returns a reference to the absolute pointer mode descriptor.
    #[must_use]
    pub const fn mode(&self) -> &EfiAbsolutePointerMode {
        &self.mode
    }

    /// Sets the absolute screen bounds.
    pub fn set_bounds(&mut self, max_x: u64, max_y: u64, max_z: u64) {
        self.mode.absolute_max_x = max_x;
        self.mode.absolute_max_y = max_y;
        self.mode.absolute_max_z = max_z;
        self.reset();
    }

    /// Returns `true` if pointer state has changed since last read.
    #[must_use]
    pub const fn has_state_changed(&self) -> bool {
        self.state_changed
    }

    /// Ingests relative motion from a standard USB boot mouse report and updates
    /// clamped absolute screen coordinates.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if the report length is less than 3 bytes.
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub fn process_relative_report(&mut self, report: &[u8]) -> Result<(), EfiStatus> {
        if report.len() < 3 {
            return Err(EfiStatus::DEVICE_ERROR);
        }

        let buttons = report[0];
        let dx = report[1] as i8;
        let dy = report[2] as i8;
        let dz = if report.len() > 3 { report[3] as i8 } else { 0 };

        let mut active = 0u32;
        if (buttons & USB_MOUSE_BTN_LEFT) != 0 {
            active |= ABSOLUTE_POINTER_TOUCH_ACTIVE;
        }
        if (buttons & (USB_MOUSE_BTN_RIGHT | USB_MOUSE_BTN_MIDDLE)) != 0 {
            active |= ABSOLUTE_POINTER_ALT_ACTIVE;
        }
        self.state.active_buttons = active;

        // Clamp X coordinate
        let new_x = (self.state.current_x as i64) + (i64::from(dx));
        let clamped_x = new_x
            .max(self.mode.absolute_min_x as i64)
            .min(self.mode.absolute_max_x as i64);
        self.state.current_x = clamped_x as u64;

        // Clamp Y coordinate
        let new_y = (self.state.current_y as i64) + (i64::from(dy));
        let clamped_y = new_y
            .max(self.mode.absolute_min_y as i64)
            .min(self.mode.absolute_max_y as i64);
        self.state.current_y = clamped_y as u64;

        // Clamp Z coordinate (scroll wheel or pressure)
        if dz != 0 {
            let new_z = (self.state.current_z as i64) + (i64::from(dz));
            let clamped_z = new_z
                .max(self.mode.absolute_min_z as i64)
                .min(self.mode.absolute_max_z as i64);
            self.state.current_z = clamped_z as u64;
        }

        self.state_changed = true;
        Ok(())
    }

    /// Ingests direct absolute touch coordinates (e.g. from digitizer or touchscreen).
    pub fn process_absolute_coords(&mut self, x: u64, y: u64, z: u64, active_buttons: u32) {
        self.state.current_x = x
            .max(self.mode.absolute_min_x)
            .min(self.mode.absolute_max_x);
        self.state.current_y = y
            .max(self.mode.absolute_min_y)
            .min(self.mode.absolute_max_y);
        self.state.current_z = z
            .max(self.mode.absolute_min_z)
            .min(self.mode.absolute_max_z);
        self.state.active_buttons = active_buttons;
        self.state_changed = true;
    }

    /// Implements `EFI_ABSOLUTE_POINTER_PROTOCOL.GetState()`:
    /// Returns the current absolute position and active buttons.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_READY` if the position has not changed since last query.
    pub fn get_state(&mut self) -> Result<EfiAbsolutePointerState, EfiStatus> {
        if !self.state_changed {
            return Err(EfiStatus::NOT_READY);
        }

        self.state_changed = false;
        Ok(self.state)
    }
}
