//! # Disk I/O protocol
//!
//! The `EFI_DISK_IO_PROTOCOL`.
//!
//! Offsets are in bytes, not blocks.

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiStatus};

/// Reads from disk.
pub type EfiDiskRead = unsafe extern "efiapi" fn(
    *mut EfiDiskIoProtocol,
    u32,
    u64,
    UIntN,
    *mut core::ffi::c_void,
) -> EfiStatus;

/// Writes to disk.
pub type EfiDiskWrite = unsafe extern "efiapi" fn(
    *mut EfiDiskIoProtocol,
    u32,
    u64,
    UIntN,
    *mut core::ffi::c_void,
) -> EfiStatus;

/// The disk I/O protocol.
///
/// The struct is `#[repr(C)]`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDiskIoProtocol {
    /// The protocol revision.
    pub revision: u64,
    /// Reads from disk.
    pub read_disk: EfiDiskRead,
    /// Writes to disk.
    pub write_disk: EfiDiskWrite,
}

/// A typed disk I/O view.
pub struct DiskIo<'a> {
    protocol: &'a EfiDiskIoProtocol,
}

impl<'a> DiskIo<'a> {
    /// Creates a view from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiDiskIoProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDiskIoProtocol {
        self.protocol
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Reads bytes from disk.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// read fails.
    pub fn read_disk(
        &self,
        media_id: u32,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for `buffer.len()` bytes.
        let status = unsafe {
            (self.protocol.read_disk)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                media_id,
                offset,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes bytes to disk.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// write fails.
    pub fn write_disk(&self, media_id: u32, offset: u64, buffer: &[u8]) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for `buffer.len()` bytes.
        let status = unsafe {
            (self.protocol.write_disk)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                media_id,
                offset,
                buffer.len(),
                buffer.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

impl core::fmt::Debug for DiskIo<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("DiskIo")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// Token structure for non-blocking operations on `EFI_DISK_IO2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDiskIo2Token {
    /// Event signaled when the requested operation completes.
    pub event: EfiEvent,
    /// Transaction completion status.
    pub transaction_status: EfiStatus,
}

/// Cancels asynchronous I/O requests (`EFI_DISK_CANCEL_EX`).
pub type EfiDiskCancelEx = unsafe extern "efiapi" fn(*mut EfiDiskIo2Protocol) -> EfiStatus;

/// Reads from disk extended (`EFI_DISK_READ_EX`).
pub type EfiDiskReadEx = unsafe extern "efiapi" fn(
    *mut EfiDiskIo2Protocol,
    u32,
    u64,
    *mut EfiDiskIo2Token,
    UIntN,
    *mut core::ffi::c_void,
) -> EfiStatus;

/// Writes to disk extended (`EFI_DISK_WRITE_EX`).
pub type EfiDiskWriteEx = unsafe extern "efiapi" fn(
    *mut EfiDiskIo2Protocol,
    u32,
    u64,
    *mut EfiDiskIo2Token,
    UIntN,
    *const core::ffi::c_void,
) -> EfiStatus;

/// Flushes the disk extended (`EFI_DISK_FLUSH_EX`).
pub type EfiDiskFlushEx =
    unsafe extern "efiapi" fn(*mut EfiDiskIo2Protocol, *mut EfiDiskIo2Token) -> EfiStatus;

/// The `EFI_DISK_IO2_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDiskIo2Protocol {
    /// Protocol revision.
    pub revision: u64,
    /// Cancels outstanding asynchronous I/O requests.
    pub cancel: EfiDiskCancelEx,
    /// Reads bytes from disk.
    pub read_disk_ex: EfiDiskReadEx,
    /// Writes bytes to disk.
    pub write_disk_ex: EfiDiskWriteEx,
    /// Flushes modified data to the physical device.
    pub flush_disk_ex: EfiDiskFlushEx,
}

/// A safe wrapper for `EFI_DISK_IO2_PROTOCOL`.
#[derive(Debug)]
pub struct DiskIo2<'a> {
    protocol: &'a EfiDiskIo2Protocol,
}

impl<'a> DiskIo2<'a> {
    /// Creates a view from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiDiskIo2Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiDiskIo2Protocol {
        self.protocol
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Cancels outstanding I/O requests.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when cancellation fails.
    pub fn cancel(&self) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.protocol.cancel)(core::ptr::from_ref(self.protocol).cast_mut()) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
