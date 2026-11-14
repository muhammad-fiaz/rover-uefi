//! # Console Control Protocol
//!
//! Provides services to switch between text and graphical console display modes,
//! query current console status, and lock/unlock console input.

use crate::base::{EfiGuid, EfiStatus};

/// Console Control Protocol GUID: `{f42f7782-012e-4c70-99a6-4bb77dddbabf}`.
pub const EFI_CONSOLE_CONTROL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xf42f_7782,
    0x012e,
    0x4c70,
    [0x99, 0xa6, 0x4b, 0xb7, 0x7d, 0xdd, 0xba, 0xbf],
);

/// Display modes for the console device.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EfiConsoleControlScreenMode {
    /// Console is displaying text output (Simple Text Output).
    Text = 0,
    /// Console is displaying graphics output (Graphics Output Protocol).
    Graphics = 1,
    /// Upper bound delimiter for valid screen modes.
    MaxValue = 2,
}

/// Retrieves the current console display mode and status.
pub type EfiConsoleControlGetMode = unsafe extern "efiapi" fn(
    this: *const EfiConsoleControlProtocol,
    mode: *mut EfiConsoleControlScreenMode,
    gop_uva: *mut bool,
    std_in_locked: *mut bool,
) -> EfiStatus;

/// Switches the console between text and graphics modes.
pub type EfiConsoleControlSetMode = unsafe extern "efiapi" fn(
    this: *const EfiConsoleControlProtocol,
    mode: EfiConsoleControlScreenMode,
) -> EfiStatus;

/// Locks standard console input with an optional password.
pub type EfiConsoleControlLockStdIn = unsafe extern "efiapi" fn(
    this: *const EfiConsoleControlProtocol,
    password: *const u16,
) -> EfiStatus;

/// Raw interface table for the Console Control Protocol.
#[repr(C)]
pub struct EfiConsoleControlProtocol {
    /// Retrieves current console screen mode.
    pub get_mode: EfiConsoleControlGetMode,
    /// Sets console screen mode.
    pub set_mode: EfiConsoleControlSetMode,
    /// Locks standard input.
    pub lock_std_in: EfiConsoleControlLockStdIn,
}

/// Information returned by `ConsoleControl::get_mode`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ConsoleControlModeInfo {
    /// Active screen display mode (Text or Graphics).
    pub mode: EfiConsoleControlScreenMode,
    /// Indicates whether GOP or UGA is currently active.
    pub gop_uva: bool,
    /// Indicates whether standard input is currently locked.
    pub std_in_locked: bool,
}

/// Safe client wrapper around [`EfiConsoleControlProtocol`].
pub struct ConsoleControl<'a> {
    proto: &'a EfiConsoleControlProtocol,
}

impl ConsoleControl<'_> {
    /// Protocol GUID.
    pub const GUID: EfiGuid = EFI_CONSOLE_CONTROL_PROTOCOL_GUID;

    /// Wraps a raw pointer to an `EfiConsoleControlProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `proto` points to a valid, live `EfiConsoleControlProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiConsoleControlProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Queries the current console screen mode and state flags.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if query failed.
    pub fn get_mode(&self) -> Result<ConsoleControlModeInfo, EfiStatus> {
        let mut mode = EfiConsoleControlScreenMode::Text;
        let mut gop_uva = false;
        let mut std_in_locked = false;

        let status = unsafe {
            (self.proto.get_mode)(
                self.proto,
                &raw mut mode,
                &raw mut gop_uva,
                &raw mut std_in_locked,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(ConsoleControlModeInfo {
                mode,
                gop_uva,
                std_in_locked,
            })
        } else {
            Err(status)
        }
    }

    /// Switches the console display mode.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if mode change failed or requested mode is unsupported.
    pub fn set_mode(&self, mode: EfiConsoleControlScreenMode) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_mode)(self.proto, mode) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Locks standard console input with an optional password.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if locking operation failed.
    pub fn lock_std_in(&self, password: Option<&[u16]>) -> Result<(), EfiStatus> {
        let pass_ptr = password.map_or(core::ptr::null(), <[u16]>::as_ptr);
        let status = unsafe { (self.proto.lock_std_in)(self.proto, pass_ptr) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
