//! # UEFI Shell Protocols
//!
//! Provides the UEFI Shell 2.0+ protocol and parameters protocol interfaces
//! defined in the UEFI Shell Specification.

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// The UEFI Shell protocol `GUID`.
pub const EFI_SHELL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6302_D008,
    0x7F9B,
    0x4F30,
    [0x87, 0xAC, 0x60, 0xC9, 0xFE, 0xF5, 0xDA, 0x4E],
);

/// The UEFI Shell parameters protocol `GUID`.
pub const EFI_SHELL_PARAMETERS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x752F_3136,
    0x4E16,
    0x4FDC,
    [0xA2, 0x2A, 0xE5, 0xF4, 0x68, 0x12, 0xF4, 0xCA],
);

/// Shell file handle alias.
pub type ShellFileHandle = *mut c_void;

/// The UEFI Shell Parameters Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiShellParametersProtocol {
    /// Array of pointers to null-terminated UTF-16 argument strings.
    pub argv: *mut *mut u16,
    /// Number of elements in argv.
    pub argc: usize,
    /// Standard input handle.
    pub std_in: ShellFileHandle,
    /// Standard output handle.
    pub std_out: ShellFileHandle,
    /// Standard error handle.
    pub std_err: ShellFileHandle,
}

/// Executes a child process from a command line.
pub type EfiShellExecute = unsafe extern "efiapi" fn(
    image_handle: *mut EfiHandle,
    command_line: *mut u16,
    environment: *mut *mut u16,
    status: *mut EfiStatus,
) -> EfiStatus;

/// Gets an environment variable.
pub type EfiShellGetEnv = unsafe extern "efiapi" fn(name: *const u16) -> *const u16;

/// Sets an environment variable.
pub type EfiShellSetEnv =
    unsafe extern "efiapi" fn(name: *const u16, value: *const u16, volatile: bool) -> EfiStatus;

/// The UEFI Shell Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiShellProtocol {
    /// Executes a command line.
    pub execute: EfiShellExecute,
    /// Gets environment variable.
    pub get_env: EfiShellGetEnv,
    /// Sets environment variable.
    pub set_env: EfiShellSetEnv,
}
