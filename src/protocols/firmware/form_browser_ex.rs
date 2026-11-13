//! # Form Browser Extension Protocols
//!
//! Provides services for registering custom hot keys, configuring action scopes,
//! tracking form data modifications, and triggering browser actions (discard, default,
//! submit, reset, exit, goto) in the HII user interface environment.

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::console::input::EfiInputKey;

/// Form Browser Extension Protocol GUID: `{1f73b18d-4630-43c1-a1de-6f80855d7da4}`.
pub const FORM_BROWSER_EXTENSION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1F73_B18D,
    0x4630,
    0x43C1,
    [0xA1, 0xDE, 0x6F, 0x80, 0x85, 0x5D, 0x7D, 0xA4],
);

/// Form Browser Extension 2 Protocol GUID: `{a770c357-b693-4e6d-a6cf-d21c728e550b}`.
pub const FORM_BROWSER_EXTENSION2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA770_C357,
    0xB693,
    0x4E6D,
    [0xA6, 0xCF, 0xD2, 0x1C, 0x72, 0x8E, 0x55, 0x0B],
);

/// Form Browser Extension 2 revision 1.0.
pub const BROWSER_EXTENSION2_VERSION_1: u32 = 0x0001_0000;
/// Form Browser Extension 2 revision 1.1.
pub const BROWSER_EXTENSION2_VERSION_1_1: u32 = 0x0001_0001;

/// Browser action: Unregister the designated hot key.
pub const BROWSER_ACTION_UNREGISTER: u32 = 0;
/// Browser action: Discard pending form changes.
pub const BROWSER_ACTION_DISCARD: u32 = 1 << 0;
/// Browser action: Reset questions to their default values.
pub const BROWSER_ACTION_DEFAULT: u32 = 1 << 1;
/// Browser action: Submit pending form changes.
pub const BROWSER_ACTION_SUBMIT: u32 = 1 << 2;
/// Browser action: Request a system reset.
pub const BROWSER_ACTION_RESET: u32 = 1 << 3;
/// Browser action: Exit the browser application.
pub const BROWSER_ACTION_EXIT: u32 = 1 << 4;
/// Browser action: Navigate to a specified target form or question.
pub const BROWSER_ACTION_GOTO: u32 = 1 << 5;

/// Scope level for registered browser actions.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserSettingScope {
    /// Action affects only the currently active form.
    FormLevel = 0,
    /// Action affects the entire form set.
    FormSetLevel = 1,
    /// Action affects the global system settings.
    SystemLevel = 2,
    /// Maximum scope delimiter.
    MaxLevel = 3,
}

/// Function pointer to set the browser hot key scope.
pub type BrowserSetScope = unsafe extern "efiapi" fn(scope: BrowserSettingScope) -> EfiStatus;

/// Function pointer to register or unregister a browser hot key.
pub type BrowserRegisterHotKey = unsafe extern "efiapi" fn(
    key_data: *const EfiInputKey,
    action: u32,
    default_id: u16,
    help_string: *const u16,
) -> EfiStatus;

/// Function pointer to register an exit handler invoked when the browser terminates.
pub type BrowserRegisterExitHandler =
    unsafe extern "efiapi" fn(handler: Option<unsafe extern "efiapi" fn()>) -> EfiStatus;

/// Function pointer to query if browser form data has been modified.
pub type BrowserIsDataModified = unsafe extern "efiapi" fn() -> Bool;

/// Function pointer to execute a designated browser action.
pub type BrowserExecuteAction =
    unsafe extern "efiapi" fn(action: u32, default_id: u16) -> EfiStatus;

/// Function pointer to query if a system reset is required upon exit.
pub type BrowserIsResetRequired = unsafe extern "efiapi" fn() -> Bool;

/// Low-level Form Browser Extension Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiFormBrowserExtensionProtocol {
    /// Sets the scope of hot key actions.
    pub set_scope: BrowserSetScope,
    /// Registers a keystroke binding for a browser action.
    pub register_hot_key: BrowserRegisterHotKey,
    /// Registers a callback handler invoked on browser exit.
    pub register_exit_handler: BrowserRegisterExitHandler,
}

/// Low-level Form Browser Extension 2 Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiFormBrowserExtension2Protocol {
    /// Protocol revision (`BROWSER_EXTENSION2_VERSION_1` or `BROWSER_EXTENSION2_VERSION_1_1`).
    pub version: u32,
    /// Sets the scope of hot key actions.
    pub set_scope: BrowserSetScope,
    /// Registers a keystroke binding for a browser action.
    pub register_hot_key: BrowserRegisterHotKey,
    /// Registers a callback handler invoked on browser exit.
    pub register_exit_handler: BrowserRegisterExitHandler,
    /// Checks whether form data has been modified.
    pub is_browser_data_modified: BrowserIsDataModified,
    /// Programmatically executes a browser action.
    pub execute_action: BrowserExecuteAction,
    /// Checks whether exit requires a system reset.
    pub is_reset_required: BrowserIsResetRequired,
}

/// Safe client wrapper around `EfiFormBrowserExtensionProtocol`.
#[derive(Debug, Clone, Copy)]
pub struct FormBrowserExtension {
    proto: *const EfiFormBrowserExtensionProtocol,
}

impl FormBrowserExtension {
    /// Wraps a raw pointer to `EfiFormBrowserExtensionProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiFormBrowserExtensionProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiFormBrowserExtensionProtocol) -> Self {
        Self { proto }
    }

    /// Sets the action scope for registered hot keys.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if the scope is invalid.
    /// Returns `EfiStatus::UNSUPPORTED` if the scope conflicts with existing hot keys.
    pub fn set_scope(&self, scope: BrowserSettingScope) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).set_scope)(scope) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers a hot key for a browser action.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `key` is invalid or unsupported.
    pub fn register_hot_key(
        &self,
        key: &EfiInputKey,
        action: u32,
        default_id: u16,
        help_string: Option<&[u16]>,
    ) -> Result<(), EfiStatus> {
        let help_ptr = help_string.map_or(core::ptr::null(), <[u16]>::as_ptr);
        let status = unsafe { ((*self.proto).register_hot_key)(key, action, default_id, help_ptr) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters a hot key.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `key` is invalid.
    pub fn unregister_hot_key(&self, key: &EfiInputKey) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).register_hot_key)(key, BROWSER_ACTION_UNREGISTER, 0, core::ptr::null())
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers an exit handler.
    ///
    /// # Errors
    ///
    /// Returns an error status if handler registration fails.
    pub fn register_exit_handler(
        &self,
        handler: Option<unsafe extern "efiapi" fn()>,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).register_exit_handler)(handler) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiFormBrowserExtension2Protocol`.
#[derive(Debug, Clone, Copy)]
pub struct FormBrowserExtension2 {
    proto: *const EfiFormBrowserExtension2Protocol,
}

impl FormBrowserExtension2 {
    /// Wraps a raw pointer to `EfiFormBrowserExtension2Protocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiFormBrowserExtension2Protocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiFormBrowserExtension2Protocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol version.
    #[must_use]
    pub fn version(&self) -> u32 {
        unsafe { (*self.proto).version }
    }

    /// Sets the action scope for registered hot keys.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if the scope is invalid.
    /// Returns `EfiStatus::UNSUPPORTED` if the scope conflicts with existing hot keys.
    pub fn set_scope(&self, scope: BrowserSettingScope) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).set_scope)(scope) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers a hot key for a browser action.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `key` is invalid or unsupported.
    pub fn register_hot_key(
        &self,
        key: &EfiInputKey,
        action: u32,
        default_id: u16,
        help_string: Option<&[u16]>,
    ) -> Result<(), EfiStatus> {
        let help_ptr = help_string.map_or(core::ptr::null(), <[u16]>::as_ptr);
        let status = unsafe { ((*self.proto).register_hot_key)(key, action, default_id, help_ptr) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters a hot key.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `key` is invalid.
    pub fn unregister_hot_key(&self, key: &EfiInputKey) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).register_hot_key)(key, BROWSER_ACTION_UNREGISTER, 0, core::ptr::null())
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers an exit handler.
    ///
    /// # Errors
    ///
    /// Returns an error status if handler registration fails.
    pub fn register_exit_handler(
        &self,
        handler: Option<unsafe extern "efiapi" fn()>,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).register_exit_handler)(handler) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns whether browser data has been modified.
    #[must_use]
    pub fn is_browser_data_modified(&self) -> bool {
        let res = unsafe { ((*self.proto).is_browser_data_modified)() };
        res != 0
    }

    /// Executes a browser action programmatically.
    ///
    /// # Errors
    ///
    /// Returns an error status if the browser action fails.
    pub fn execute_action(&self, action: u32, default_id: u16) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).execute_action)(action, default_id) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns whether a system reset is required upon exit.
    #[must_use]
    pub fn is_reset_required(&self) -> bool {
        let res = unsafe { ((*self.proto).is_reset_required)() };
        res != 0
    }
}
