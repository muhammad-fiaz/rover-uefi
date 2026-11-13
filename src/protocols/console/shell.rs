//! # UEFI Shell Protocols and Client API
//!
//! Provides the UEFI Shell 2.0+ protocol, shell parameters protocol, and dynamic
//! shell command registration interfaces defined in the UEFI Shell Specification.

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use crate::tables::system::EfiSystemTable;

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

/// The UEFI Shell Dynamic Command protocol `GUID`.
pub const EFI_SHELL_DYNAMIC_COMMAND_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3C72_00E9,
    0x005F,
    0x4EA4,
    [0x87, 0xDE, 0xA3, 0xDF, 0xAC, 0x8A, 0x27, 0xC3],
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

/// Handler callback for dynamic shell command execution.
pub type ShellCommandHandler = unsafe extern "efiapi" fn(
    this: *mut EfiShellDynamicCommandProtocol,
    system_table: *mut EfiSystemTable,
    parameters: *mut EfiShellParametersProtocol,
    shell: *mut EfiShellProtocol,
) -> EfiStatus;

/// Callback returning localized help text for a dynamic shell command.
pub type ShellCommandGetHelp = unsafe extern "efiapi" fn(
    this: *mut EfiShellDynamicCommandProtocol,
    language: *const u8,
) -> *mut u16;

/// The UEFI Shell Dynamic Command Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiShellDynamicCommandProtocol {
    /// Null-terminated UTF-16 command name.
    pub command_name: *const u16,
    /// Callback invoked when the command is called in the shell.
    pub handler: Option<ShellCommandHandler>,
    /// Callback returning help text for the command.
    pub get_help: Option<ShellCommandGetHelp>,
}

/// Safe client wrapper for the UEFI Shell parameters protocol.
#[derive(Debug)]
pub struct ShellParameters<'a> {
    protocol: &'a EfiShellParametersProtocol,
}

impl<'a> ShellParameters<'a> {
    /// Create a new safe view of the Shell parameters protocol.
    #[must_use]
    pub fn new(protocol: &'a EfiShellParametersProtocol) -> Self {
        Self { protocol }
    }

    /// Return the argument count (`argc`).
    #[must_use]
    pub const fn argc(&self) -> usize {
        self.protocol.argc
    }

    /// Retrieve argument at `index` if within bounds.
    #[must_use]
    pub fn arg(&self, index: usize) -> Option<*const u16> {
        if index < self.protocol.argc && !self.protocol.argv.is_null() {
            unsafe {
                let ptr = *self.protocol.argv.add(index);
                if ptr.is_null() { None } else { Some(ptr) }
            }
        } else {
            None
        }
    }
}

/// Safe client wrapper for the UEFI Shell protocol.
#[derive(Debug)]
pub struct Shell<'a> {
    protocol: &'a EfiShellProtocol,
}

impl<'a> Shell<'a> {
    /// Create a new safe view of the Shell protocol.
    #[must_use]
    pub fn new(protocol: &'a EfiShellProtocol) -> Self {
        Self { protocol }
    }

    /// Retrieve an environment variable value pointer.
    ///
    /// # Safety
    /// Caller must ensure `name` is a null-terminated UTF-16 string pointer.
    #[must_use]
    pub unsafe fn get_env(&self, name: *const u16) -> Option<*const u16> {
        let ptr = unsafe { (self.protocol.get_env)(name) };
        if ptr.is_null() { None } else { Some(ptr) }
    }

    /// Set an environment variable value.
    ///
    /// # Safety
    /// Caller must ensure `name` and `value` are valid null-terminated UTF-16 string pointers.
    ///
    /// # Errors
    /// Returns the underlying status reported by the shell protocol.
    pub unsafe fn set_env(
        &self,
        name: *const u16,
        value: *const u16,
        volatile: bool,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.set_env)(name, value, volatile) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
