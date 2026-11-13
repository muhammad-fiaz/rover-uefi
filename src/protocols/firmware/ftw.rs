//! # Fault Tolerant Write Protocol
//!
//! Provides transactional, recoverable write operations for firmware block and flash devices.
//! Guarantees consistency across unexpected power interruptions.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// Fault Tolerant Write Protocol GUID: `{3ebd9e82-2c78-4de6-9786-8d4bfcbfb7c8}`.
pub const FAULT_TOLERANT_WRITE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3EBD_9E82,
    0x2C78,
    0x4DE6,
    [0x97, 0x86, 0x8D, 0x4B, 0xFC, 0xB7, 0xC8, 0x81],
);

/// SMM Fault Tolerant Write Protocol GUID: `{a6a57d97-1528-4e89-8d77-62a22533ca9b}`.
pub const FAULT_TOLERANT_WRITE_SMM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA6A5_7D97,
    0x1528,
    0x4E89,
    [0x8D, 0x77, 0x62, 0xA2, 0x25, 0x33, 0xCA, 0x9B],
);

/// Function pointer for `GetMaxBlockSize`.
pub type EfiFaultTolerantWriteGetMaxBlockSize = unsafe extern "efiapi" fn(
    this: *const EfiFaultTolerantWriteProtocol,
    block_size: *mut usize,
) -> EfiStatus;

/// Function pointer for `Allocate`.
pub type EfiFaultTolerantWriteAllocate = unsafe extern "efiapi" fn(
    this: *const EfiFaultTolerantWriteProtocol,
    caller_id: *const EfiGuid,
    private_data_size: usize,
    number_of_writes: usize,
) -> EfiStatus;

/// Function pointer for `Write`.
pub type EfiFaultTolerantWriteWrite = unsafe extern "efiapi" fn(
    this: *const EfiFaultTolerantWriteProtocol,
    lba: u64,
    offset: usize,
    length: usize,
    private_data: *mut c_void,
    fvb_handle: EfiHandle,
    buffer: *mut c_void,
) -> EfiStatus;

/// Function pointer for `Restart`.
pub type EfiFaultTolerantWriteRestart = unsafe extern "efiapi" fn(
    this: *const EfiFaultTolerantWriteProtocol,
    fvb_handle: EfiHandle,
) -> EfiStatus;

/// Function pointer for `Abort`.
pub type EfiFaultTolerantWriteAbort =
    unsafe extern "efiapi" fn(this: *const EfiFaultTolerantWriteProtocol) -> EfiStatus;

/// Function pointer for `GetLastWrite`.
pub type EfiFaultTolerantWriteGetLastWrite = unsafe extern "efiapi" fn(
    this: *const EfiFaultTolerantWriteProtocol,
    caller_id: *mut EfiGuid,
    lba: *mut u64,
    offset: *mut usize,
    length: *mut usize,
    private_data_size: *mut usize,
    private_data: *mut c_void,
    complete: *mut bool,
) -> EfiStatus;

/// Low-level Fault Tolerant Write Protocol ABI table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFaultTolerantWriteProtocol {
    /// Gets maximum block size.
    pub get_max_block_size: EfiFaultTolerantWriteGetMaxBlockSize,
    /// Allocates resources for upcoming writes.
    pub allocate: EfiFaultTolerantWriteAllocate,
    /// Executes a fault-tolerant block update.
    pub write: EfiFaultTolerantWriteWrite,
    /// Restarts an interrupted write.
    pub restart: EfiFaultTolerantWriteRestart,
    /// Aborts all allocated writes.
    pub abort: EfiFaultTolerantWriteAbort,
    /// Queries the last write record.
    pub get_last_write: EfiFaultTolerantWriteGetLastWrite,
}

/// Safe client wrapper around the Fault Tolerant Write Protocol.
#[derive(Debug, Clone, Copy)]
pub struct FaultTolerantWrite {
    proto: *const EfiFaultTolerantWriteProtocol,
}

impl FaultTolerantWrite {
    /// Wraps a Fault Tolerant Write protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiFaultTolerantWriteProtocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiFaultTolerantWriteProtocol) -> Self {
        Self { proto }
    }

    /// Returns the maximum block size in bytes that can be updated fault-tolerantly.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::ABORTED` if operation fails.
    pub fn get_max_block_size(&self) -> Result<usize, EfiStatus> {
        let mut block_size = 0_usize;
        let status = unsafe {
            let func = (*self.proto).get_max_block_size;
            func(self.proto, &raw mut block_size)
        };
        if status == EfiStatus::SUCCESS {
            Ok(block_size)
        } else {
            Err(status)
        }
    }

    /// Allocates internal storage to track upcoming writes.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::ACCESS_DENIED` if prior writes are pending.
    pub fn allocate(
        &self,
        caller_id: &EfiGuid,
        private_data_size: usize,
        number_of_writes: usize,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).allocate;
            func(
                self.proto,
                core::ptr::from_ref(caller_id),
                private_data_size,
                number_of_writes,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Executes a fault-tolerant write to a target logical block.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BAD_BUFFER_SIZE` if write spans a block boundary.
    /// Returns `EfiStatus::ACCESS_DENIED` if no writes were allocated.
    /// Returns `EfiStatus::NOT_READY` if a previous write must be restarted first.
    pub fn write(
        &self,
        lba: u64,
        offset: usize,
        private_data: &mut [u8],
        fvb_handle: EfiHandle,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).write;
            func(
                self.proto,
                lba,
                offset,
                buffer.len(),
                private_data.as_mut_ptr().cast::<c_void>(),
                fvb_handle,
                buffer.as_mut_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Restarts an interrupted write.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::ACCESS_DENIED` if no pending writes exist.
    pub fn restart(&self, fvb_handle: EfiHandle) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).restart;
            func(self.proto, fvb_handle)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Aborts all allocated writes.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if no allocated writes exist.
    pub fn abort(&self) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).abort;
            func(self.proto)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
