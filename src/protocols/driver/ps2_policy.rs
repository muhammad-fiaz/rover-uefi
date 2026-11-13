//! # PS/2 Controller Platform Policy Protocol
//!
//! Abstracts platform-specific initialization and hardware settings for PS/2 keyboard
//! and mouse auxiliary controllers.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// PS/2 Policy Protocol GUID: `{4df19259-dc71-4d46-bef1-357bb578c418}`.
pub const PS2_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4DF1_9259,
    0xDC71,
    0x4D46,
    [0xBE, 0xF1, 0x35, 0x7B, 0xB5, 0x78, 0xC4, 0x18],
);

/// Scroll Lock LED indicator bitmask.
pub const KEYBOARD_LIGHT_SCROLLLOCK: u8 = 0x01;
/// Num Lock LED indicator bitmask.
pub const KEYBOARD_LIGHT_NUMLOCK: u8 = 0x02;
/// Caps Lock LED indicator bitmask.
pub const KEYBOARD_LIGHT_CAPSLOCK: u8 = 0x04;

/// Function pointer type for platform-specific PS/2 hardware initialization.
pub type Ps2InitHardwareFn = unsafe extern "efiapi" fn(handle: EfiHandle) -> EfiStatus;

/// Raw PS/2 Policy Protocol table.
#[repr(C)]
pub struct Ps2PolicyProtocol {
    /// Initial keyboard indicator LED states (combination of `KEYBOARD_LIGHT_*` flags).
    pub keyboard_light: u8,
    /// Platform hardware initialization routine.
    pub ps2_init_hardware: Option<Ps2InitHardwareFn>,
}

/// Safe client wrapper around [`Ps2PolicyProtocol`].
pub struct Ps2Policy<'a> {
    raw: &'a mut Ps2PolicyProtocol,
}

impl Ps2Policy<'_> {
    /// Wraps a raw [`Ps2PolicyProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut Ps2PolicyProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Returns the initial keyboard LED bitmask.
    #[must_use]
    pub fn keyboard_light(&self) -> u8 {
        self.raw.keyboard_light
    }

    /// Sets the keyboard LED bitmask.
    pub fn set_keyboard_light(&mut self, lights: u8) {
        self.raw.keyboard_light = lights;
    }

    /// Returns true if the Caps Lock LED flag is enabled.
    #[must_use]
    pub fn is_capslock_enabled(&self) -> bool {
        (self.raw.keyboard_light & KEYBOARD_LIGHT_CAPSLOCK) != 0
    }

    /// Returns true if the Num Lock LED flag is enabled.
    #[must_use]
    pub fn is_numlock_enabled(&self) -> bool {
        (self.raw.keyboard_light & KEYBOARD_LIGHT_NUMLOCK) != 0
    }

    /// Returns true if the Scroll Lock LED flag is enabled.
    #[must_use]
    pub fn is_scrolllock_enabled(&self) -> bool {
        (self.raw.keyboard_light & KEYBOARD_LIGHT_SCROLLLOCK) != 0
    }

    /// Executes the platform-specific hardware initialization hook for a PS/2 controller.
    ///
    /// # Errors
    ///
    /// Returns the firmware status error code returned by the platform initialization handler.
    pub fn init_hardware(&mut self, handle: EfiHandle) -> Result<(), EfiStatus> {
        if let Some(func) = self.raw.ps2_init_hardware {
            let status = unsafe { func(handle) };
            if status.is_success() {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Ok(())
        }
    }
}
