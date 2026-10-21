//! # File protocol
//!
//! The `EFI_FILE_PROTOCOL` and wrappers.
//!
//! `FileProtocol` borrows a firmware handle.
//! `OwnedFile` owns the handle and closes it
//! on drop or explicit close. All file data
//! buffers are caller-provided; no allocation
//! occurs.

use core::ffi::c_void;

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::storage::filesystem::info::EfiFileIoToken;
use crate::protocols::storage::filesystem::validation::validate_file_name;

/// Open mode read.
pub const FILE_MODE_READ: u64 = 0x0000_0000_0000_0001;

/// Open mode write.
pub const FILE_MODE_WRITE: u64 = 0x0000_0000_0000_0002;

/// Open mode create.
pub const FILE_MODE_CREATE: u64 = 0x8000_0000_0000_0000;

/// Read-only attribute.
pub const FILE_READ_ONLY: u64 = 0x0000_0000_0000_0001;

/// Hidden attribute.
pub const FILE_HIDDEN: u64 = 0x0000_0000_0000_0002;

/// System attribute.
pub const FILE_SYSTEM: u64 = 0x0000_0000_0000_0004;

/// Directory attribute.
pub const FILE_DIRECTORY: u64 = 0x0000_0000_0000_0010;

/// Archive attribute.
pub const FILE_ARCHIVE: u64 = 0x0000_0000_0000_0020;

/// Opens a file.
pub type EfiFileOpen = unsafe extern "efiapi" fn(
    *mut EfiFileProtocol,
    *mut *mut EfiFileProtocol,
    *mut Char16,
    u64,
    u64,
) -> EfiStatus;

/// Closes a file.
pub type EfiFileClose = unsafe extern "efiapi" fn(*mut EfiFileProtocol) -> EfiStatus;

/// Deletes a file.
pub type EfiFileDelete = unsafe extern "efiapi" fn(*mut EfiFileProtocol) -> EfiStatus;

/// Reads data.
pub type EfiFileRead =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut UIntN, *mut c_void) -> EfiStatus;

/// Writes data.
pub type EfiFileWrite =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut UIntN, *mut c_void) -> EfiStatus;

/// Sets the position.
pub type EfiFileSetPosition = unsafe extern "efiapi" fn(*mut EfiFileProtocol, u64) -> EfiStatus;

/// Gets the position.
pub type EfiFileGetPosition =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut u64) -> EfiStatus;

/// Gets information.
pub type EfiFileGetInfo = unsafe extern "efiapi" fn(
    *mut EfiFileProtocol,
    *mut EfiGuid,
    *mut UIntN,
    *mut c_void,
) -> EfiStatus;

/// Sets information.
pub type EfiFileSetInfo =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut EfiGuid, UIntN, *mut c_void) -> EfiStatus;

/// Flushes a file.
pub type EfiFileFlush = unsafe extern "efiapi" fn(*mut EfiFileProtocol) -> EfiStatus;

/// Opens a file with a token.
pub type EfiFileOpenEx = unsafe extern "efiapi" fn(
    *mut EfiFileProtocol,
    *mut *mut EfiFileProtocol,
    *mut Char16,
    u64,
    u64,
    *mut EfiFileIoToken,
) -> EfiStatus;

/// Reads with a token.
pub type EfiFileReadEx =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut EfiFileIoToken) -> EfiStatus;

/// Writes with a token.
pub type EfiFileWriteEx =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut EfiFileIoToken) -> EfiStatus;

/// Flushes with a token.
pub type EfiFileFlushEx =
    unsafe extern "efiapi" fn(*mut EfiFileProtocol, *mut EfiFileIoToken) -> EfiStatus;

/// The file protocol.
///
/// The struct is `#[repr(C)]`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFileProtocol {
    /// The protocol revision.
    pub revision: u64,
    /// Opens a file.
    pub open: EfiFileOpen,
    /// Closes a file.
    pub close: EfiFileClose,
    /// Deletes a file.
    pub delete: EfiFileDelete,
    /// Reads data.
    pub read: EfiFileRead,
    /// Writes data.
    pub write: EfiFileWrite,
    /// Sets the position.
    pub set_position: EfiFileSetPosition,
    /// Gets the position.
    pub get_position: EfiFileGetPosition,
    /// Gets information.
    pub get_info: EfiFileGetInfo,
    /// Sets information.
    pub set_info: EfiFileSetInfo,
    /// Flushes a file.
    pub flush: EfiFileFlush,
    /// Opens with a token.
    pub open_ex: EfiFileOpenEx,
    /// Reads with a token.
    pub read_ex: EfiFileReadEx,
    /// Writes with a token.
    pub write_ex: EfiFileWriteEx,
    /// Flushes with a token.
    pub flush_ex: EfiFileFlushEx,
}

const _: () = {
    assert!(core::mem::size_of::<EfiFileProtocol>() == 8 + 14 * core::mem::size_of::<usize>());
};

/// A borrowed file protocol view.
pub struct FileProtocol<'a> {
    protocol: &'a EfiFileProtocol,
}

impl<'a> FileProtocol<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiFileProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiFileProtocol {
        self.protocol
    }

    /// Opens a file relative to this directory.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` without
    /// calling firmware when the name is not
    /// null-terminated.
    pub fn open(
        &self,
        file_name: &[Char16],
        open_mode: u64,
        attributes: u64,
    ) -> Result<OwnedFile, EfiStatus> {
        if !validate_file_name(file_name) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let mut handle: *mut EfiFileProtocol = core::ptr::null_mut();
        // SAFETY: The name slice is valid and null-terminated for the call.
        let status = unsafe {
            (self.protocol.open)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut handle,
                file_name.as_ptr().cast_mut(),
                open_mode,
                attributes,
            )
        };
        if status.is_success() {
            Ok(OwnedFile { handle })
        } else {
            Err(status)
        }
    }

    /// Closes an owned file.
    #[must_use]
    pub fn close(&self, file: OwnedFile) -> EfiStatus {
        file.close()
    }
}

/// An owned file handle.
///
/// Closing is explicit through `close` and
/// also occurs on drop when the handle is
/// still owned. Handles must be closed before
/// `ExitBootServices`.
pub struct OwnedFile {
    handle: *mut EfiFileProtocol,
}

impl OwnedFile {
    /// Returns the raw handle pointer.
    #[must_use]
    pub const fn raw(&self) -> *mut EfiFileProtocol {
        self.handle
    }

    /// Closes the file.
    #[must_use]
    pub fn close(self) -> EfiStatus {
        let mut this = self;
        let handle = this.handle;
        this.handle = core::ptr::null_mut();
        core::mem::forget(this);
        // SAFETY: The handle was obtained from firmware and is closed exactly once.
        unsafe { ((*handle).close)(handle) }
    }

    /// Borrows the file protocol.
    ///
    /// # Safety
    ///
    /// The caller must ensure the handle is
    /// still valid.
    #[must_use]
    pub unsafe fn as_protocol(&self) -> &EfiFileProtocol {
        // SAFETY: The caller guarantees the handle is valid.
        unsafe { &*self.handle }
    }

    /// Reads into a buffer.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// read fails.
    pub fn read(&self, buffer: &mut [u8]) -> Result<UIntN, EfiStatus> {
        let mut size = buffer.len();
        // SAFETY: The slice pointer is valid for `size` bytes.
        let status = unsafe {
            ((*self.handle).read)(self.handle, &raw mut size, buffer.as_mut_ptr().cast())
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Writes from a buffer.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// write fails.
    pub fn write(&self, buffer: &[u8]) -> Result<UIntN, EfiStatus> {
        let mut size = buffer.len();
        // SAFETY: The slice pointer is valid for `size` bytes.
        let status = unsafe {
            ((*self.handle).write)(
                self.handle,
                &raw mut size,
                buffer.as_ptr().cast::<c_void>().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Gets the position.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// position cannot be retrieved.
    pub fn get_position(&self) -> Result<u64, EfiStatus> {
        let mut position: u64 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe { ((*self.handle).get_position)(self.handle, &raw mut position) };
        if status.is_success() {
            Ok(position)
        } else {
            Err(status)
        }
    }

    /// Sets the position.
    #[must_use]
    pub fn set_position(&self, position: u64) -> EfiStatus {
        // SAFETY: The handle is valid for the call.
        unsafe { ((*self.handle).set_position)(self.handle, position) }
    }

    /// Flushes the file.
    #[must_use]
    pub fn flush(&self) -> EfiStatus {
        // SAFETY: The handle is valid for the call.
        unsafe { ((*self.handle).flush)(self.handle) }
    }
}

impl Drop for OwnedFile {
    /// Closes the handle when still owned.
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let handle = self.handle;
            self.handle = core::ptr::null_mut();
            // SAFETY: The handle is closed exactly once on drop.
            unsafe {
                let _status = ((*handle).close)(handle);
            }
        }
    }
}

impl core::fmt::Debug for OwnedFile {
    /// Formats without dereferencing firmware
    /// state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("OwnedFile")
            .field("handle", &self.handle)
            .finish()
    }
}
