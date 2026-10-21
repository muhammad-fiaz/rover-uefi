//! # Simple file system protocol
//!
//! The `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL`.

use crate::base::EfiStatus;
use crate::protocols::storage::filesystem::file::{EfiFileProtocol, FileProtocol};

/// Opens the root volume.
pub type EfiSimpleFileSystemOpenVolume = unsafe extern "efiapi" fn(
    *mut EfiSimpleFileSystemProtocol,
    *mut *mut EfiFileProtocol,
) -> EfiStatus;

/// The simple file system protocol.
///
/// The struct is `#[repr(C)]`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    /// The protocol revision.
    pub revision: u64,
    /// Opens the root volume.
    pub open_volume: EfiSimpleFileSystemOpenVolume,
}

/// A typed simple file system view.
pub struct SimpleFileSystem<'a> {
    protocol: &'a EfiSimpleFileSystemProtocol,
}

impl<'a> SimpleFileSystem<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiSimpleFileSystemProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiSimpleFileSystemProtocol {
        self.protocol
    }

    /// Opens the root directory.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// volume cannot be opened.
    pub fn open_volume(&self) -> Result<FileProtocol<'_>, EfiStatus> {
        let mut root: *mut EfiFileProtocol = core::ptr::null_mut();
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe {
            (self.protocol.open_volume)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut root,
            )
        };
        if status.is_success() && !root.is_null() {
            // SAFETY: Firmware returns a valid handle after success.
            Ok(FileProtocol::new(unsafe { &*root }))
        } else if status.is_success() {
            Err(EfiStatus::NOT_FOUND)
        } else {
            Err(status)
        }
    }
}

impl core::fmt::Debug for SimpleFileSystem<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SimpleFileSystem")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
