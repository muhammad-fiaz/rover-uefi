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
