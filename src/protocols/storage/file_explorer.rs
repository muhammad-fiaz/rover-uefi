//! # File Explorer Protocol and Client API
//!
//! Provides a standardized interface for browsing directories and selecting
//! files across available firmware file systems.

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;

/// File Explorer Protocol GUID: `{2c03c536-4594-4515-9e7a-d3d204fe1363}`.
pub const FILE_EXPLORER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2C03_C536,
    0x4594,
    0x4515,
    [0x9E, 0x7A, 0xD3, 0xD2, 0x04, 0xFE, 0x13, 0x63],
);

/// Callback handler executed after a file is chosen by the user.
///
/// Returns `1` (`true`) if the file explorer should exit immediately,
/// or `0` (`false`) to remain in the file browser.
pub type EfiFileExplorerChooseHandler =
    unsafe extern "efiapi" fn(file_path: *const EfiDevicePath) -> Bool;

/// Function pointer prototype to launch the file explorer dialog.
pub type EfiFileExplorerChooseFile = unsafe extern "efiapi" fn(
    root_directory: *const EfiDevicePath,
    file_type: *const u16,
    choose_handler: Option<EfiFileExplorerChooseHandler>,
    file: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Low-level File Explorer Protocol ABI table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiFileExplorerProtocol {
    /// Interactive file selection service.
    pub choose_file: EfiFileExplorerChooseFile,
}

/// Safe client wrapper around `EfiFileExplorerProtocol`.
#[derive(Debug, Clone, Copy)]
pub struct FileExplorer {
    proto: *const EfiFileExplorerProtocol,
}

impl FileExplorer {
    /// Wraps a raw pointer to `EfiFileExplorerProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiFileExplorerProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiFileExplorerProtocol) -> Self {
        Self { proto }
    }

    /// Invokes the file explorer dialog to choose a file.
    ///
    /// If `root_directory` is `None`, browsing begins at the root of available media.
    /// If `file_type` is specified, files are filtered by matching extension.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if no file was selected or media was removed.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    /// Returns an error status on unexpected hardware or dialog failure.
    pub fn choose_file(
        &self,
        root_directory: Option<&EfiDevicePath>,
        file_type: Option<&[u16]>,
        choose_handler: Option<EfiFileExplorerChooseHandler>,
    ) -> Result<*mut EfiDevicePath, EfiStatus> {
        let root_ptr = root_directory.map_or(core::ptr::null(), core::ptr::from_ref);
        let type_ptr = file_type.map_or(core::ptr::null(), <[u16]>::as_ptr);
        let mut chosen_file = core::ptr::null_mut();

        let status = unsafe {
            ((*self.proto).choose_file)(root_ptr, type_ptr, choose_handler, &raw mut chosen_file)
        };

        if status == EfiStatus::SUCCESS {
            Ok(chosen_file)
        } else {
            Err(status)
        }
    }
}
