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

/// Shell Interface Protocol `GUID` (`SHELL_INTERFACE_PROTOCOL_GUID`):
/// `{47c7b223-c42a-11d2-8e57-00a0c969723b}`.
pub const SHELL_INTERFACE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x47C7_B223,
    0xC42A,
    0x11D2,
    [0x8E, 0x57, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// Shell Environment Protocol `GUID` (`SHELL_ENVIRONMENT_PROTOCOL_GUID`):
/// `{47c7b221-c42a-11d2-8e57-00a0c969723b}`.
pub const SHELL_ENVIRONMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x47C7_B221,
    0xC42A,
    0x11D2,
    [0x8E, 0x57, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// Shell Environment2 Extension Signature `GUID`:
/// `{d2c18636-40e5-4eb5-a31b-36695fd42c87}`.
pub const SHELL_ENVIRONMENT2_EXTENSION_GUID: EfiGuid = EfiGuid::new(
    0xD2C1_8636,
    0x40E5,
    0x4EB5,
    [0xA3, 0x1B, 0x36, 0x69, 0x5F, 0xD4, 0x2C, 0x87],
);

/// Shell Alias Variable `GUID`:
/// `{0053d9d6-2659-4599-a266-7936f8e7e376}`.
pub const SHELL_ALIAS_VARIABLE_GUID: EfiGuid = EfiGuid::new(
    0x0053_D9D6,
    0x2659,
    0x4599,
    [0xA2, 0x66, 0x79, 0x36, 0xF8, 0xE7, 0xE3, 0x76],
);

/// Shell Variable `GUID`:
/// `{158def5a-f656-419c-b027-7a3192c079d2}`.
pub const SHELL_VARIABLE_GUID: EfiGuid = EfiGuid::new(
    0x158D_EF5A,
    0xF656,
    0x419C,
    [0xB0, 0x27, 0x7A, 0x31, 0x92, 0xC0, 0x79, 0xD2],
);

/// Shell Map `GUID`:
/// `{51271e13-7de3-43af-a024-600f533080ce}`.
pub const SHELL_MAP_GUID: EfiGuid = EfiGuid::new(
    0x5127_1E13,
    0x7DE3,
    0x43AF,
    [0xA0, 0x24, 0x60, 0x0F, 0x53, 0x30, 0x80, 0xCE],
);

/// Shell argument attribute: No special attributes.
pub const ARG_NO_ATTRIB: u32 = 0x0;
/// Shell argument attribute: Entire argument was quoted.
pub const ARG_IS_QUOTED: u32 = 1 << 0;
/// Shell argument attribute: Argument was partially quoted.
pub const ARG_PARTIALLY_QUOTED: u32 = 1 << 1;
/// Shell argument attribute: First half of argument was quoted.
pub const ARG_FIRST_HALF_QUOTED: u32 = 1 << 2;
/// Shell argument attribute: First character was escaped.
pub const ARG_FIRST_CHAR_IS_ESC: u32 = 1 << 3;

/// Argument attribute metadata for shell arguments.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiShellArgInfo {
    /// Bitmask of `ARG_*` flags.
    pub attributes: u32,
}

/// The legacy/compatibility Shell Interface protocol structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiShellInterface {
    /// Handle back to the original image handle.
    pub image_handle: EfiHandle,
    /// Pointer to loaded image protocol information.
    pub info: *mut c_void,
    /// Array of UTF-16 argument string pointers.
    pub argv: *mut *mut u16,
    /// Argument count.
    pub argc: usize,
    /// Storage for file redirection arguments.
    pub redir_argv: *mut *mut u16,
    /// Redirection argument count.
    pub redir_argc: usize,
    /// File-style handle for standard input.
    pub std_in: ShellFileHandle,
    /// File-style handle for standard output.
    pub std_out: ShellFileHandle,
    /// File-style handle for standard error.
    pub std_err: ShellFileHandle,
    /// Array of attribute information per argument.
    pub arg_info: *mut EfiShellArgInfo,
    /// Indicates whether command echo is active.
    pub echo_on: bool,
}

/// Safe client wrapper for [`EfiShellInterface`].
#[derive(Debug)]
pub struct ShellInterface<'a> {
    interface: &'a EfiShellInterface,
}

impl<'a> ShellInterface<'a> {
    /// Creates a new safe wrapper around [`EfiShellInterface`].
    #[must_use]
    pub fn new(interface: &'a EfiShellInterface) -> Self {
        Self { interface }
    }

    /// Returns the argument count (`argc`).
    #[must_use]
    pub const fn argc(&self) -> usize {
        self.interface.argc
    }

    /// Retrieves an argument string pointer by 0-based index.
    #[must_use]
    pub fn arg(&self, index: usize) -> Option<*const u16> {
        if index < self.interface.argc && !self.interface.argv.is_null() {
            unsafe {
                let ptr = *self.interface.argv.add(index);
                if ptr.is_null() { None } else { Some(ptr) }
            }
        } else {
            None
        }
    }

    /// Returns the redirection argument count (`redir_argc`).
    #[must_use]
    pub const fn redir_argc(&self) -> usize {
        self.interface.redir_argc
    }

    /// Retrieves a redirection argument string pointer by 0-based index.
    #[must_use]
    pub fn redir_arg(&self, index: usize) -> Option<*const u16> {
        if index < self.interface.redir_argc && !self.interface.redir_argv.is_null() {
            unsafe {
                let ptr = *self.interface.redir_argv.add(index);
                if ptr.is_null() { None } else { Some(ptr) }
            }
        } else {
            None
        }
    }

    /// Retrieves argument attributes by index if available.
    #[must_use]
    pub fn arg_attributes(&self, index: usize) -> Option<u32> {
        if index < self.interface.argc && !self.interface.arg_info.is_null() {
            unsafe {
                let info = *self.interface.arg_info.add(index);
                Some(info.attributes)
            }
        } else {
            None
        }
    }

    /// Returns whether echo is enabled.
    #[must_use]
    pub const fn is_echo_on(&self) -> bool {
        self.interface.echo_on
    }
}

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
