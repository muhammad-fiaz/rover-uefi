//! # Driver Diagnostics Protocols
//!
//! Defined by the UEFI Specification Section 11.6: Driver Diagnostics Protocols.
//!
//! Provides interfaces for running diagnostic self-tests on controllers and devices.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// The `EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID` (RFC 4646 language tags).
pub const EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4D33_0321,
    0x025F,
    0x4AAC,
    [0x90, 0xD8, 0x5E, 0xD9, 0x00, 0x17, 0x3B, 0x63],
);

/// The `EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID` (ISO 639-2 language codes).
pub const EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0784_924F,
    0xE296,
    0x11D4,
    [0x9A, 0x49, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Diagnostic test depth or level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiDriverDiagnosticType {
    /// Standard execution self-test.
    Standard = 0,
    /// Extended in-depth hardware self-test.
    Extended = 1,
    /// Manufacturing or factory test mode.
    Manufacturing = 2,
    /// Maximum diagnostic type value.
    Maximum = 3,
}

/// Function pointer for `EFI_DRIVER_DIAGNOSTICS2.RunDiagnostics`.
pub type EfiDriverDiagnostics2RunDiagnostics = unsafe extern "efiapi" fn(
    this: *const EfiDriverDiagnostics2Protocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    diagnostic_type: EfiDriverDiagnosticType,
    language: *const u8,
    error_type: *mut *mut EfiGuid,
    buffer_size: *mut UIntN,
    buffer: *mut *mut u16,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DRIVER_DIAGNOSTICS2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverDiagnostics2Protocol {
    /// Executes diagnostic self-tests.
    pub run_diagnostics: EfiDriverDiagnostics2RunDiagnostics,
    /// Pointer to RFC 4646 supported language list.
    pub supported_languages: *const u8,
}

/// Function pointer for `EFI_DRIVER_DIAGNOSTICS.RunDiagnostics`.
pub type EfiDriverDiagnosticsRunDiagnostics = unsafe extern "efiapi" fn(
    this: *const EfiDriverDiagnosticsProtocol,
    controller_handle: EfiHandle,
    child_handle: EfiHandle,
    diagnostic_type: EfiDriverDiagnosticType,
    language: *const u8,
    error_type: *mut *mut EfiGuid,
    buffer_size: *mut UIntN,
    buffer: *mut *mut u16,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DRIVER_DIAGNOSTICS_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverDiagnosticsProtocol {
    /// Executes diagnostic self-tests.
    pub run_diagnostics: EfiDriverDiagnosticsRunDiagnostics,
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

/// Safe client wrapper around `EfiDriverDiagnostics2Protocol`.
pub struct DriverDiagnostics2<'a> {
    proto: &'a EfiDriverDiagnostics2Protocol,
}

impl<'a> DriverDiagnostics2<'a> {
    /// Creates a new `DriverDiagnostics2` client wrapper.
    #[must_use]
    pub const fn new(proto: &'a EfiDriverDiagnostics2Protocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiDriverDiagnostics2Protocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiDriverDiagnostics2Protocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiDriverDiagnostics2Protocol) -> Option<Self> {
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
    pub fn raw(&self) -> *const EfiDriverDiagnostics2Protocol {
        core::ptr::from_ref(self.proto)
    }

    /// Returns the supported RFC 4646 language codes if available.
    #[must_use]
    pub fn supported_languages(&self) -> Option<&'a str> {
        unsafe { read_ascii_str(self.proto.supported_languages) }
    }

    /// Runs diagnostics on the controller or child controller.
    ///
    /// Returns `(error_type_guid, buffer_pointer, buffer_size_in_bytes)` on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if the diagnostic run fails or hardware error occurs.
    pub fn run_diagnostics(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        diagnostic_type: EfiDriverDiagnosticType,
        language: &str,
    ) -> Result<(Option<EfiGuid>, *mut u16, usize), EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut lang_buf = [0u8; 64];
        let bytes = language.as_bytes();
        if bytes.len() >= lang_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        lang_buf[..bytes.len()].copy_from_slice(bytes);

        let mut error_type_ptr: *mut EfiGuid = core::ptr::null_mut();
        let mut buffer_size: UIntN = 0;
        let mut buffer_ptr: *mut u16 = core::ptr::null_mut();

        let status = unsafe {
            (self.proto.run_diagnostics)(
                self.proto,
                controller,
                child_handle,
                diagnostic_type,
                lang_buf.as_ptr(),
                core::ptr::from_mut(&mut error_type_ptr),
                core::ptr::from_mut(&mut buffer_size),
                core::ptr::from_mut(&mut buffer_ptr),
            )
        };

        if status == EfiStatus::SUCCESS {
            let error_guid = if error_type_ptr.is_null() {
                None
            } else {
                Some(unsafe { *error_type_ptr })
            };
            Ok((error_guid, buffer_ptr, buffer_size as usize))
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiDriverDiagnosticsProtocol`.
pub struct DriverDiagnostics<'a> {
    proto: &'a EfiDriverDiagnosticsProtocol,
}

impl<'a> DriverDiagnostics<'a> {
    /// Creates a new `DriverDiagnostics` client wrapper.
    #[must_use]
    pub const fn new(proto: &'a EfiDriverDiagnosticsProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiDriverDiagnosticsProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiDriverDiagnosticsProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiDriverDiagnosticsProtocol) -> Option<Self> {
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
    pub fn raw(&self) -> *const EfiDriverDiagnosticsProtocol {
        core::ptr::from_ref(self.proto)
    }

    /// Returns the supported ISO 639-2 language codes if available.
    #[must_use]
    pub fn supported_languages(&self) -> Option<&'a str> {
        unsafe { read_ascii_str(self.proto.supported_languages) }
    }

    /// Runs diagnostics on the controller or child controller.
    ///
    /// Returns `(error_type_guid, buffer_pointer, buffer_size_in_bytes)` on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if the diagnostic run fails or hardware error occurs.
    pub fn run_diagnostics(
        &self,
        controller: EfiHandle,
        child: Option<EfiHandle>,
        diagnostic_type: EfiDriverDiagnosticType,
        language: &str,
    ) -> Result<(Option<EfiGuid>, *mut u16, usize), EfiStatus> {
        let child_handle = child.unwrap_or_else(|| EfiHandle::from_ptr(core::ptr::null_mut()));
        let mut lang_buf = [0u8; 64];
        let bytes = language.as_bytes();
        if bytes.len() >= lang_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        lang_buf[..bytes.len()].copy_from_slice(bytes);

        let mut error_type_ptr: *mut EfiGuid = core::ptr::null_mut();
        let mut buffer_size: UIntN = 0;
        let mut buffer_ptr: *mut u16 = core::ptr::null_mut();

        let status = unsafe {
            (self.proto.run_diagnostics)(
                self.proto,
                controller,
                child_handle,
                diagnostic_type,
                lang_buf.as_ptr(),
                core::ptr::from_mut(&mut error_type_ptr),
                core::ptr::from_mut(&mut buffer_size),
                core::ptr::from_mut(&mut buffer_ptr),
            )
        };

        if status == EfiStatus::SUCCESS {
            let error_guid = if error_type_ptr.is_null() {
                None
            } else {
                Some(unsafe { *error_type_ptr })
            };
            Ok((error_guid, buffer_ptr, buffer_size as usize))
        } else {
            Err(status)
        }
    }
}
