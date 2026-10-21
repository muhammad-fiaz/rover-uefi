//! # Load File protocols
//!
//! Implementations and declarations for `EFI_LOAD_FILE_PROTOCOL` and
//! `EFI_LOAD_FILE2_PROTOCOL`.
//!
//! These protocols are used by the boot manager and firmware components to
//! obtain files (such as boot images, kernel images, and initrd binaries)
//! from arbitrary devices or memory sources.

use crate::abi::types::{Bool, EFI_FALSE, EFI_TRUE, UIntN};
use crate::base::EfiStatus;
use crate::protocols::device_path::EfiDevicePath;
use core::ffi::c_void;

pub use crate::protocols::storage::guid::{
    LOAD_FILE_PROTOCOL_GUID as LOAD_FILE_GUID, LOAD_FILE_PROTOCOL_GUID,
    LOAD_FILE2_PROTOCOL_GUID as LOAD_FILE2_GUID, LOAD_FILE2_PROTOCOL_GUID,
};

/// Function pointer type for `EFI_LOAD_FILE.LoadFile`.
pub type EfiLoadFile = unsafe extern "efiapi" fn(
    this: *mut EfiLoadFileProtocol,
    file_path: *const EfiDevicePath,
    boot_policy: Bool,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus;

/// The `EFI_LOAD_FILE_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiLoadFileProtocol {
    /// Loads the file specified by `file_path`.
    pub load_file: EfiLoadFile,
}

/// Function pointer type for `EFI_LOAD_FILE2.LoadFile`.
pub type EfiLoadFile2 = unsafe extern "efiapi" fn(
    this: *mut EfiLoadFile2Protocol,
    file_path: *const EfiDevicePath,
    boot_policy: Bool,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus;

/// The `EFI_LOAD_FILE2_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiLoadFile2Protocol {
    /// Loads the file specified by `file_path`.
    pub load_file: EfiLoadFile2,
}

/// A safe wrapper for `EFI_LOAD_FILE_PROTOCOL`.
#[derive(Debug)]
pub struct LoadFile<'a> {
    protocol: &'a EfiLoadFileProtocol,
}

impl<'a> LoadFile<'a> {
    /// Creates a safe wrapper from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiLoadFileProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying raw protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiLoadFileProtocol {
        self.protocol
    }

    /// Queries the size required to load the specified file.
    ///
    /// # Safety
    ///
    /// `file_path` must point to a valid EFI device path.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware cannot locate or load the file metadata.
    pub unsafe fn query_file_size(
        &self,
        file_path: *const EfiDevicePath,
        boot_policy: bool,
    ) -> Result<usize, EfiStatus> {
        let mut size: UIntN = 0;
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let policy: Bool = if boot_policy { EFI_TRUE } else { EFI_FALSE };
        // SAFETY: Invoking firmware function pointer with NULL buffer queries size.
        let status = unsafe {
            (self.protocol.load_file)(ptr, file_path, policy, &raw mut size, core::ptr::null_mut())
        };
        if status == EfiStatus::BUFFER_TOO_SMALL {
            Ok(size)
        } else if status.is_error() {
            Err(status)
        } else {
            Ok(size)
        }
    }

    /// Loads the specified file into the provided buffer.
    ///
    /// # Safety
    ///
    /// `file_path` must point to a valid EFI device path.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware fails to load the file into the buffer.
    pub unsafe fn load_into_buffer(
        &self,
        file_path: *const EfiDevicePath,
        boot_policy: bool,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size: UIntN = buffer.len();
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let policy: Bool = if boot_policy { EFI_TRUE } else { EFI_FALSE };
        // SAFETY: Invoking firmware function pointer with caller provided buffer.
        let status = unsafe {
            (self.protocol.load_file)(
                ptr,
                file_path,
                policy,
                &raw mut size,
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }
}

/// A safe wrapper for `EFI_LOAD_FILE2_PROTOCOL`.
#[derive(Debug)]
pub struct LoadFile2<'a> {
    protocol: &'a EfiLoadFile2Protocol,
}

impl<'a> LoadFile2<'a> {
    /// Creates a safe wrapper from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiLoadFile2Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying raw protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiLoadFile2Protocol {
        self.protocol
    }

    /// Queries the size required to load the specified file.
    ///
    /// # Safety
    ///
    /// `file_path` must point to a valid EFI device path.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware cannot locate or load the file metadata.
    pub unsafe fn query_file_size(
        &self,
        file_path: *const EfiDevicePath,
    ) -> Result<usize, EfiStatus> {
        let mut size: UIntN = 0;
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        // SAFETY: Invoking firmware function pointer with NULL buffer queries size.
        // For LoadFile2, boot_policy is always FALSE according to UEFI specification.
        let status = unsafe {
            (self.protocol.load_file)(
                ptr,
                file_path,
                EFI_FALSE,
                &raw mut size,
                core::ptr::null_mut(),
            )
        };
        if status == EfiStatus::BUFFER_TOO_SMALL {
            Ok(size)
        } else if status.is_error() {
            Err(status)
        } else {
            Ok(size)
        }
    }

    /// Loads the specified file into the provided buffer.
    ///
    /// # Safety
    ///
    /// `file_path` must point to a valid EFI device path.
    ///
    /// # Errors
    ///
    /// Returns an error status if firmware fails to load the file into the buffer.
    pub unsafe fn load_into_buffer(
        &self,
        file_path: *const EfiDevicePath,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size: UIntN = buffer.len();
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        // SAFETY: Invoking firmware function pointer with caller provided buffer.
        let status = unsafe {
            (self.protocol.load_file)(
                ptr,
                file_path,
                EFI_FALSE,
                &raw mut size,
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }
}
