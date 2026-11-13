//! # Platform Virtual Keyboard Protocol
//!
//! Provides a mechanism for mobile, embedded, and tablet platforms to register,
//! query, and clear hardware/virtual key events (such as Power button, Volume Up,
//! Volume Down, Home, and Recovery buttons) into the firmware input subsystem.

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::console::input::EfiInputKey;

/// Platform Virtual Keyboard Protocol GUID: `{0e3606d2-1dc3-4e6f-be65-394982a26547}`.
pub const PLATFORM_VIRTUAL_KEYBOARD_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0E36_06D2,
    0x1DC3,
    0x4E6F,
    [0xBE, 0x65, 0x39, 0x49, 0x82, 0xA2, 0x65, 0x47],
);

/// Signature for a virtual keyboard key structure (`'vkbd'`).
pub const VIRTUAL_KEYBOARD_KEY_SIGNATURE: usize = 0x6462_6B76;

/// Structure describing a virtual keyboard keystroke with validation signature.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualKbdKey {
    /// Structure signature (`VIRTUAL_KEYBOARD_KEY_SIGNATURE`).
    pub signature: usize,
    /// Encapsulated UEFI input key representation.
    pub key: EfiInputKey,
}

impl Default for VirtualKbdKey {
    fn default() -> Self {
        Self {
            signature: VIRTUAL_KEYBOARD_KEY_SIGNATURE,
            key: EfiInputKey::default(),
        }
    }
}

/// Function pointer to register platform virtual keyboard resources.
pub type PlatformVirtualKbdRegister = unsafe extern "efiapi" fn() -> EfiStatus;

/// Function pointer to reset virtual keyboard state.
pub type PlatformVirtualKbdReset = unsafe extern "efiapi" fn() -> EfiStatus;

/// Function pointer to query if a virtual key is currently pressed.
pub type PlatformVirtualKbdQuery =
    unsafe extern "efiapi" fn(virtual_key: *mut VirtualKbdKey) -> Bool;

/// Function pointer to clear virtual key latched state.
pub type PlatformVirtualKbdClear =
    unsafe extern "efiapi" fn(virtual_key: *mut VirtualKbdKey) -> EfiStatus;

/// Low-level Platform Virtual Keyboard Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiPlatformVirtualKeyboardProtocol {
    /// Registers virtual keyboard hardware and interrupts.
    pub register: PlatformVirtualKbdRegister,
    /// Resets virtual keyboard controller state.
    pub reset: PlatformVirtualKbdReset,
    /// Queries active keystroke status.
    pub query: PlatformVirtualKbdQuery,
    /// Clears latched key state.
    pub clear: PlatformVirtualKbdClear,
}

/// Safe client wrapper around `EfiPlatformVirtualKeyboardProtocol`.
#[derive(Debug, Clone, Copy)]
pub struct PlatformVirtualKeyboard {
    proto: *const EfiPlatformVirtualKeyboardProtocol,
}

impl PlatformVirtualKeyboard {
    /// Wraps a raw pointer to `EfiPlatformVirtualKeyboardProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiPlatformVirtualKeyboardProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiPlatformVirtualKeyboardProtocol) -> Self {
        Self { proto }
    }

    /// Registers the virtual keyboard with platform hardware.
    ///
    /// # Errors
    ///
    /// Returns an error status if hardware initialization fails.
    pub fn register(&self) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).register)() };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Resets the virtual keyboard state.
    ///
    /// # Errors
    ///
    /// Returns an error status if controller reset fails.
    pub fn reset(&self) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).reset)() };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries whether a specific virtual key is active.
    ///
    /// Updates `key` with retrieved keystroke metadata and returns `true` if active.
    #[must_use]
    pub fn query(&self, key: &mut VirtualKbdKey) -> bool {
        let res = unsafe { ((*self.proto).query)(core::ptr::from_mut(key)) };
        res != 0
    }

    /// Clears the latched state for a virtual key.
    ///
    /// # Errors
    ///
    /// Returns an error status if clearing latched state fails.
    pub fn clear(&self, key: &mut VirtualKbdKey) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).clear)(core::ptr::from_mut(key)) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
