//! # Tape I/O Protocol and Client API
//!
//! Provides services to control and access sequential magnetic tape drives
//! and archives according to the UEFI Specification (Section 13.9).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// EFI Tape I/O Protocol GUID.
pub const EFI_TAPE_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1E93_E633,
    0xD65A,
    0x459E,
    [0xAB, 0x84, 0x93, 0xD9, 0xEC, 0x26, 0x6D, 0x18],
);

/// Tape Header structure stored at the beginning of bootable tape media (1024 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTapeHeader {
    /// 64-bit signature identifying tape format.
    pub signature: u64,
    /// Header structure revision.
    pub revision: u32,
    /// Boot description size in bytes.
    pub boot_desc_size: u32,
    /// 32-bit CRC of the boot description.
    pub boot_desc_crc: u32,
    /// Unique tape media GUID.
    pub tape_guid: EfiGuid,
    /// Tape type GUID.
    pub tape_type: EfiGuid,
    /// Tape unique identifier GUID.
    pub tape_unique: EfiGuid,
    /// Boot loader block location.
    pub bl_location: u32,
    /// Boot loader block size in bytes.
    pub bl_blocksize: u32,
    /// Boot loader file size in bytes.
    pub bl_filesize: u32,
    /// Target OS version string.
    pub os_version: [u8; 40],
    /// Application version string.
    pub app_version: [u8; 40],
    /// Creation date (YYYY-MM-DD).
    pub creation_date: [u8; 10],
    /// Creation time (HH:MM:SS).
    pub creation_time: [u8; 10],
    /// System name in UTF-8.
    pub system_name: [u8; 256],
    /// Tape title in UTF-8.
    pub tape_title: [u8; 120],
    /// Padding to 1024 bytes.
    pub pad: [u8; 468],
}

/// Reads data sequentially from tape media.
pub type EfiTapeRead = unsafe extern "efiapi" fn(
    this: *mut EfiTapeIoProtocol,
    buffer_size: *mut usize,
    buffer: *mut c_void,
) -> EfiStatus;

/// Writes data sequentially to tape media.
pub type EfiTapeWrite = unsafe extern "efiapi" fn(
    this: *mut EfiTapeIoProtocol,
    buffer_size: *const usize,
    buffer: *const c_void,
) -> EfiStatus;

/// Rewinds tape media to the beginning of partition.
pub type EfiTapeRewind = unsafe extern "efiapi" fn(this: *mut EfiTapeIoProtocol) -> EfiStatus;

/// Positions tape forward or backward by blocks or filemarks.
pub type EfiTapeSpace = unsafe extern "efiapi" fn(
    this: *mut EfiTapeIoProtocol,
    direction: i64,
    space_type: u64,
) -> EfiStatus;

/// Writes filemarks to tape media.
pub type EfiTapeWriteFm =
    unsafe extern "efiapi" fn(this: *mut EfiTapeIoProtocol, count: usize) -> EfiStatus;

/// Resets the tape drive hardware.
pub type EfiTapeReset = unsafe extern "efiapi" fn(
    this: *mut EfiTapeIoProtocol,
    extended_verification: bool,
) -> EfiStatus;

/// EFI Tape I/O Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTapeIoProtocol {
    /// Reads from the tape media.
    pub tape_read: Option<EfiTapeRead>,
    /// Writes to the tape media.
    pub tape_write: Option<EfiTapeWrite>,
    /// Rewinds the tape.
    pub tape_rewind: Option<EfiTapeRewind>,
    /// Repositions the tape head.
    pub tape_space: Option<EfiTapeSpace>,
    /// Writes filemarks.
    pub tape_write_fm: Option<EfiTapeWriteFm>,
    /// Resets the tape device.
    pub tape_reset: Option<EfiTapeReset>,
}

/// Safe client wrapper for Tape I/O operations.
#[derive(Debug)]
pub struct TapeIo<'a> {
    protocol: &'a mut EfiTapeIoProtocol,
}

impl<'a> TapeIo<'a> {
    /// Create a new safe view of the Tape I/O protocol.
    #[must_use]
    pub fn new(protocol: &'a mut EfiTapeIoProtocol) -> Self {
        Self { protocol }
    }

    /// Rewind tape to the beginning of partition.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn rewind(&mut self) -> Result<(), EfiStatus> {
        if let Some(rewind_fn) = self.protocol.tape_rewind {
            let status = unsafe { rewind_fn(self.protocol) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Reset the tape drive hardware.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn reset(&mut self, extended_verification: bool) -> Result<(), EfiStatus> {
        if let Some(reset_fn) = self.protocol.tape_reset {
            let status = unsafe { reset_fn(self.protocol, extended_verification) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
