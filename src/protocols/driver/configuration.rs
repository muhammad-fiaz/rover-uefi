//! # Driver Configuration Protocols
//!
//! Defined by the UEFI Specification Section 11.5: Driver Configuration Protocols.
//!
//! Provides interfaces for managing and setting options and defaults on
//! controllers managed by UEFI drivers.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// The `EFI_DRIVER_CONFIGURATION2_PROTOCOL_GUID` (RFC 4646 language tags).
pub const EFI_DRIVER_CONFIGURATION2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBFD7_DC1D,
    0x24F1,
    0x40D9,
    [0x82, 0xE7, 0x2E, 0x09, 0xBB, 0x6B, 0x4E, 0xBE],
);

/// The `EFI_DRIVER_CONFIGURATION_PROTOCOL_GUID` (ISO 639-2 language codes).
pub const EFI_DRIVER_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x107A_772B,
    0xD5E1,
    0x11D4,
    [0x9A, 0x46, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Safe default configuration options.
pub const EFI_DRIVER_CONFIGURATION_SAFE_DEFAULTS: u32 = 0x0000_0000;
/// Manufacturing default configuration options.
pub const EFI_DRIVER_CONFIGURATION_MANUFACTURING_DEFAULTS: u32 = 0x0000_0001;
/// Custom default configuration options.
pub const EFI_DRIVER_CONFIGURATION_CUSTOM_DEFAULTS: u32 = 0x0000_0002;
/// Performance default configuration options.
pub const EFI_DRIVER_CONFIGURATION_PERFORMANCE_DEFAULTS: u32 = 0x0000_0003;

/// Action required after changing configuration options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiDriverConfigurationAction {
    /// Controller is usable; no action required.
    None = 0,
    /// Controller is not in a usable state and must be stopped.
    StopController = 1,
    /// Controller must be stopped and restarted before use.
    RestartController = 2,
    /// Platform must be restarted before the controller can be used again.
    RestartPlatform = 3,
    /// Maximum action value.
    Maximum = 4,
}

/// Function pointer for `EFI_DRIVER_CONFIGURATION2_PROTOCOL.SetOptions`.
pub type EfiDriverConfiguration2SetOptions = unsafe extern "efiapi" fn(
    this: *const EfiDriverConfiguration2Protocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    language: *const u8,
    action_required: *mut EfiDriverConfigurationAction,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_CONFIGURATION2_PROTOCOL.OptionsValid`.
pub type EfiDriverConfiguration2OptionsValid = unsafe extern "efiapi" fn(
    this: *const EfiDriverConfiguration2Protocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_CONFIGURATION2_PROTOCOL.ForceDefaults`.
pub type EfiDriverConfiguration2ForceDefaults = unsafe extern "efiapi" fn(
    this: *const EfiDriverConfiguration2Protocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    default_type: u32,
    action_required: *mut EfiDriverConfigurationAction,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DRIVER_CONFIGURATION2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverConfiguration2Protocol {
    /// Sets configuration options.
    pub set_options: EfiDriverConfiguration2SetOptions,
    /// Validates current configuration options.
    pub options_valid: EfiDriverConfiguration2OptionsValid,
    /// Forces default configuration options.
    pub force_defaults: EfiDriverConfiguration2ForceDefaults,
    /// Pointer to RFC 4646 supported language list.
    pub supported_languages: *const u8,
}

/// Function pointer for `EFI_DRIVER_CONFIGURATION_PROTOCOL.SetOptions`.
pub type EfiDriverConfigurationSetOptions = unsafe extern "efiapi" fn(
    this: *const EfiDriverConfigurationProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    language: *const u8,
    action_required: *mut EfiDriverConfigurationAction,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_CONFIGURATION_PROTOCOL.OptionsValid`.
pub type EfiDriverConfigurationOptionsValid = unsafe extern "efiapi" fn(
    this: *const EfiDriverConfigurationProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_DRIVER_CONFIGURATION_PROTOCOL.ForceDefaults`.
pub type EfiDriverConfigurationForceDefaults = unsafe extern "efiapi" fn(
    this: *const EfiDriverConfigurationProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    default_type: u32,
    action_required: *mut EfiDriverConfigurationAction,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DRIVER_CONFIGURATION_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverConfigurationProtocol {
    /// Sets configuration options.
    pub set_options: EfiDriverConfigurationSetOptions,
    /// Validates current configuration options.
    pub options_valid: EfiDriverConfigurationOptionsValid,
    /// Forces default configuration options.
    pub force_defaults: EfiDriverConfigurationForceDefaults,
    /// Pointer to ISO 639-2 supported language list.
    pub supported_languages: *const u8,
}

/// Helper to read a null-terminated ASCII string from a raw pointer safely.
unsafe fn read_ascii_str<'a>(ptr: *const u8) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    let mut len = 0;
    while unsafe { *ptr.add(len) } != 0 {
        len += 1;
        if len > 4096 {
            return None;
        }
    }
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    core::str::from_utf8(slice).ok()
}

/// Safe client wrapper around `EfiDriverConfiguration2Protocol`.
pub struct DriverConfiguration2<'a> {
    proto: &'a EfiDriverConfiguration2Protocol,
}

impl<'a> DriverConfiguration2<'a> {
    /// Creates a new `DriverConfiguration2` client wrapper.
    #[must_use]
    pub const fn new(proto: &'a EfiDriverConfiguration2Protocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiDriverConfiguration2Protocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiDriverConfiguration2Protocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiDriverConfiguration2Protocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *const EfiDriverConfiguration2Protocol {
        core::ptr::from_ref(self.proto)
    }

    /// Returns the supported RFC 4646 language codes if available.
    #[must_use]
    pub fn supported_languages(&self) -> Option<&'a str> {
        unsafe { read_ascii_str(self.proto.supported_languages) }
    }

    /// Sets options on the controller or child controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if the driver rejects the options or operation fails.
    pub fn set_options(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        language: &str,
    ) -> Result<EfiDriverConfigurationAction, EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut action = EfiDriverConfigurationAction::None;
        let mut lang_buf = [0u8; 64];
        let bytes = language.as_bytes();
        if bytes.len() >= lang_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        lang_buf[..bytes.len()].copy_from_slice(bytes);

        let status = unsafe {
            (self.proto.set_options)(
                self.proto,
                controller,
                child_handle,
                lang_buf.as_ptr(),
                core::ptr::from_mut(&mut action),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(action)
        } else {
            Err(status)
        }
    }

    /// Tests if the current configuration options are valid.
    ///
    /// # Errors
    ///
    /// Returns an error status if configuration is invalid or query fails.
    pub fn options_valid(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
    ) -> Result<(), EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let status = unsafe { (self.proto.options_valid)(self.proto, controller, child_handle) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Forces default configuration options on the controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if default options cannot be applied.
    pub fn force_defaults(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        default_type: u32,
    ) -> Result<EfiDriverConfigurationAction, EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut action = EfiDriverConfigurationAction::None;

        let status = unsafe {
            (self.proto.force_defaults)(
                self.proto,
                controller,
                child_handle,
                default_type,
                core::ptr::from_mut(&mut action),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(action)
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiDriverConfigurationProtocol`.
pub struct DriverConfiguration<'a> {
    proto: &'a EfiDriverConfigurationProtocol,
}

impl<'a> DriverConfiguration<'a> {
    /// Creates a new `DriverConfiguration` client wrapper.
    #[must_use]
    pub const fn new(proto: &'a EfiDriverConfigurationProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiDriverConfigurationProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiDriverConfigurationProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiDriverConfigurationProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *const EfiDriverConfigurationProtocol {
        core::ptr::from_ref(self.proto)
    }

    /// Returns the supported ISO 639-2 language codes if available.
    #[must_use]
    pub fn supported_languages(&self) -> Option<&'a str> {
        unsafe { read_ascii_str(self.proto.supported_languages) }
    }

    /// Sets options on the controller or child controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if the driver rejects the options or operation fails.
    pub fn set_options(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        language: &str,
    ) -> Result<EfiDriverConfigurationAction, EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut action = EfiDriverConfigurationAction::None;
        let mut lang_buf = [0u8; 64];
        let bytes = language.as_bytes();
        if bytes.len() >= lang_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        lang_buf[..bytes.len()].copy_from_slice(bytes);

        let status = unsafe {
            (self.proto.set_options)(
                self.proto,
                controller,
                child_handle,
                lang_buf.as_ptr(),
                core::ptr::from_mut(&mut action),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(action)
        } else {
            Err(status)
        }
    }

    /// Tests if the current configuration options are valid.
    ///
    /// # Errors
    ///
    /// Returns an error status if configuration is invalid or query fails.
    pub fn options_valid(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
    ) -> Result<(), EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let status = unsafe { (self.proto.options_valid)(self.proto, controller, child_handle) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Forces default configuration options on the controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if default options cannot be applied.
    pub fn force_defaults(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        default_type: u32,
    ) -> Result<EfiDriverConfigurationAction, EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut action = EfiDriverConfigurationAction::None;

        let status = unsafe {
            (self.proto.force_defaults)(
                self.proto,
                controller,
                child_handle,
                default_type,
                core::ptr::from_mut(&mut action),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(action)
        } else {
            Err(status)
        }
    }
}
