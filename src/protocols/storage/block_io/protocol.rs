//! # Block I/O protocol
//!
//! The `EFI_BLOCK_IO_PROTOCOL`.
//!
//! The wrapper borrows the firmware protocol
//! instance. Safe slice methods validate size,
//! alignment, and LBA range before the firmware
//! call. Media state is read from the media
//! pointer, which firmware owns.

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiEvent, EfiStatus};
use crate::protocols::storage::block_io::media::EfiBlockIoMedia;
use crate::protocols::storage::block_io::validation::{
    block_count, is_buffer_aligned, validate_lba_range,
};

/// Resets the block device.
pub type EfiBlockReset = unsafe extern "efiapi" fn(*mut EfiBlockIoProtocol, Bool) -> EfiStatus;

/// Reads blocks.
pub type EfiBlockRead = unsafe extern "efiapi" fn(
    *mut EfiBlockIoProtocol,
    u32,
    u64,
    UIntN,
    *mut core::ffi::c_void,
) -> EfiStatus;

/// Writes blocks.
pub type EfiBlockWrite = unsafe extern "efiapi" fn(
    *mut EfiBlockIoProtocol,
    u32,
    u64,
    UIntN,
    *mut core::ffi::c_void,
) -> EfiStatus;

/// Flushes the block device.
pub type EfiBlockFlush = unsafe extern "efiapi" fn(*mut EfiBlockIoProtocol) -> EfiStatus;

/// The block I/O protocol.
///
/// The struct is `#[repr(C)]` so field order
/// and offsets match the specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBlockIoProtocol {
    /// The protocol revision.
    pub revision: u64,
    /// The media information pointer.
    pub media: *mut EfiBlockIoMedia,
    /// Resets the block device.
    pub reset: EfiBlockReset,
    /// Reads blocks.
    pub read_blocks: EfiBlockRead,
    /// Writes blocks.
    pub write_blocks: EfiBlockWrite,
    /// Flushes the block device.
    pub flush_blocks: EfiBlockFlush,
}

const _: () = {
    assert!(
        core::mem::size_of::<EfiBlockIoProtocol>()
            == if core::mem::size_of::<usize>() == 8 {
                48
            } else {
                32
            }
    );
    assert!(core::mem::offset_of!(EfiBlockIoProtocol, media) == 8);
};

/// A typed block I/O view.
pub struct BlockIo<'a> {
    protocol: &'a EfiBlockIoProtocol,
}

impl<'a> BlockIo<'a> {
    /// Creates a view from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiBlockIoProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiBlockIoProtocol {
        self.protocol
    }

    /// Returns the media information.
    ///
    /// Returns `None` when the media pointer
    /// is null.
    #[must_use]
    pub fn media(&self) -> Option<&EfiBlockIoMedia> {
        if self.protocol.media.is_null() {
            return None;
        }
        // SAFETY: Firmware guarantees the media pointer is valid when non-null.
        Some(unsafe { &*self.protocol.media })
    }

    /// Returns the block size.
    #[must_use]
    pub fn block_size(&self) -> Option<u32> {
        self.media().map(|media| media.block_size)
    }

    /// Returns the last block address.
    #[must_use]
    pub fn last_block(&self) -> Option<u64> {
        self.media().map(|media| media.last_block)
    }

    /// Returns the block count.
    #[must_use]
    pub fn block_count(&self) -> Option<u64> {
        self.media().map(|media| media.last_block.wrapping_add(1))
    }

    /// Returns the media identifier.
    #[must_use]
    pub fn media_id(&self) -> Option<u32> {
        self.media().map(|media| media.media_id)
    }

    /// Returns whether the media is read-only.
    #[must_use]
    pub fn is_read_only(&self) -> bool {
        self.media().is_some_and(|media| media.read_only != 0)
    }

    /// Returns whether the media is present.
    #[must_use]
    pub fn is_media_present(&self) -> bool {
        self.media().is_some_and(|media| media.media_present != 0)
    }

    /// Resets the block device.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when reset
    /// fails.
    pub fn reset(&self, extended_verification: Bool) -> Result<(), EfiStatus> {
        // SAFETY: The borrowed protocol is valid for the call.
        let status = unsafe {
            (self.protocol.reset)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                extended_verification,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads blocks into a slice.
    ///
    /// # Errors
    ///
    /// Returns `BAD_BUFFER_SIZE` without
    /// calling firmware when the size is not
    /// a multiple of the block size, and
    /// `INVALID_PARAMETER` for alignment or
    /// range failures.
    pub fn read_blocks(&self, media_id: u32, lba: u64, buffer: &mut [u8]) -> Result<(), EfiStatus> {
        let Some(media) = self.media() else {
            return Err(EfiStatus::NO_MEDIA);
        };
        if buffer.is_empty() {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }
        let Some(count) = block_count(buffer.len(), media.block_size) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let address = buffer.as_mut_ptr() as UIntN;
        if !is_buffer_aligned(address, media.io_align) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        if !validate_lba_range(lba, count, media.last_block) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice pointer is valid for `buffer.len()` bytes.
        let status = unsafe {
            (self.protocol.read_blocks)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                media_id,
                lba,
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

    /// Writes blocks from a slice.
    ///
    /// # Errors
    ///
    /// Returns `BAD_BUFFER_SIZE`, `INVALID_PARAMETER`,
    /// or `WRITE_PROTECTED` without calling
    /// firmware when the request is invalid.
    pub fn write_blocks(&self, media_id: u32, lba: u64, buffer: &[u8]) -> Result<(), EfiStatus> {
        let Some(media) = self.media() else {
            return Err(EfiStatus::NO_MEDIA);
        };
        if media.read_only != 0 {
            return Err(EfiStatus::WRITE_PROTECTED);
        }
        if buffer.is_empty() {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }
        let Some(count) = block_count(buffer.len(), media.block_size) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let address = buffer.as_ptr() as UIntN;
        if !is_buffer_aligned(address, media.io_align) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        if !validate_lba_range(lba, count, media.last_block) {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: The slice pointer is valid for `buffer.len()` bytes.
        let status = unsafe {
            (self.protocol.write_blocks)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                media_id,
                lba,
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

    /// Flushes the block device.
    #[must_use]
    pub fn flush_blocks(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.flush_blocks)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for BlockIo<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("BlockIo")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// Token structure for non-blocking operations on `EFI_BLOCK_IO2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBlockIo2Token {
    /// Event signaled when the requested operation completes.
    pub event: EfiEvent,
    /// Transaction completion status.
    pub transaction_status: EfiStatus,
}

/// Resets the block device extended (`EFI_BLOCK_RESET_EX`).
pub type EfiBlockResetEx = unsafe extern "efiapi" fn(*mut EfiBlockIo2Protocol, Bool) -> EfiStatus;

/// Reads blocks extended (`EFI_BLOCK_READ_EX`).
pub type EfiBlockReadEx = unsafe extern "efiapi" fn(
    *mut EfiBlockIo2Protocol,
    u32,
    u64,
    *mut EfiBlockIo2Token,
    UIntN,
    *mut core::ffi::c_void,
) -> EfiStatus;

/// Writes blocks extended (`EFI_BLOCK_WRITE_EX`).
pub type EfiBlockWriteEx = unsafe extern "efiapi" fn(
    *mut EfiBlockIo2Protocol,
    u32,
    u64,
    *mut EfiBlockIo2Token,
    UIntN,
    *const core::ffi::c_void,
) -> EfiStatus;

/// Flushes the block device extended (`EFI_BLOCK_FLUSH_EX`).
pub type EfiBlockFlushEx =
    unsafe extern "efiapi" fn(*mut EfiBlockIo2Protocol, *mut EfiBlockIo2Token) -> EfiStatus;

/// The `EFI_BLOCK_IO2_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBlockIo2Protocol {
    /// Pointer to the media information structure.
    pub media: *mut EfiBlockIoMedia,
    /// Resets the block device.
    pub reset: EfiBlockResetEx,
    /// Reads blocks with optional asynchronous notification.
    pub read_blocks_ex: EfiBlockReadEx,
    /// Writes blocks with optional asynchronous notification.
    pub write_blocks_ex: EfiBlockWriteEx,
    /// Flushes the block device with optional asynchronous notification.
    pub flush_blocks_ex: EfiBlockFlushEx,
}

/// A safe wrapper for `EFI_BLOCK_IO2_PROTOCOL`.
#[derive(Debug)]
pub struct BlockIo2<'a> {
    protocol: &'a EfiBlockIo2Protocol,
}

impl<'a> BlockIo2<'a> {
    /// Creates a view from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiBlockIo2Protocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiBlockIo2Protocol {
        self.protocol
    }

    /// Returns the media information if valid.
    #[must_use]
    pub fn media(&self) -> Option<&EfiBlockIoMedia> {
        if self.protocol.media.is_null() {
            return None;
        }
        // SAFETY: Firmware guarantees valid media pointer when non-null.
        Some(unsafe { &*self.protocol.media })
    }

    /// Resets the block device.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when reset fails.
    pub fn reset(&self, extended_verification: Bool) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.reset)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                extended_verification,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
