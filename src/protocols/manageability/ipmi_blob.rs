//! # IPMI OEM Blob Transfer Protocol
//!
//! Provides the IPMI OEM Blob Transfer protocol interface for host-to-BMC binary
//! blob manipulation, firmware updates, and out-of-band configuration transport.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr
)]

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_char;

/// IPMI Blob Transfer Protocol GUID.
///
/// `{05837c75-1d65-468b-b1c2-81af9a315b2c}`
pub const IPMI_BLOB_TRANSFER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0583_7c75,
    0x1d65,
    0x468b,
    [0xb1, 0xc2, 0x81, 0xaf, 0x9a, 0x31, 0x5b, 0x2c],
);

/// IPMI OEM Blob Transfer command code.
pub const IPMI_OEM_BLOB_TRANSFER_CMD: u8 = 0x80;

/// Maximum payload data per transfer packet (64 bytes).
pub const BLOB_MAX_DATA_PER_PACKET: usize = 64;

/// Blob Transfer status bitflags.
pub mod blob_flags {
    /// Open blob for reading.
    pub const OPEN_R: u16 = 1 << 0;
    /// Open blob for writing.
    pub const OPEN_W: u16 = 1 << 1;
    /// Blob is committing.
    pub const COMMITTING: u16 = 1 << 2;
    /// Blob has been committed.
    pub const COMMITTED: u16 = 1 << 3;
    /// Error during blob commit.
    pub const COMMIT_ERROR: u16 = 1 << 4;
}

/// Raw function pointer for getting the count of active blobs.
pub type IpmiBlobGetCount = unsafe extern "efiapi" fn(count: *mut u32) -> EfiStatus;

/// Raw function pointer for enumerating blobs by index.
pub type IpmiBlobEnumerate =
    unsafe extern "efiapi" fn(blob_index: u32, blob_id: *mut c_char) -> EfiStatus;

/// Raw function pointer for opening a blob session.
pub type IpmiBlobOpen = unsafe extern "efiapi" fn(
    blob_id: *const c_char,
    flags: u16,
    session_id: *mut u16,
) -> EfiStatus;

/// Raw function pointer for reading from a blob session.
pub type IpmiBlobRead = unsafe extern "efiapi" fn(
    session_id: u16,
    offset: u32,
    requested_size: u32,
    data: *mut u8,
) -> EfiStatus;

/// Raw function pointer for writing to a blob session.
pub type IpmiBlobWrite = unsafe extern "efiapi" fn(
    session_id: u16,
    offset: u32,
    data: *const u8,
    write_length: u32,
) -> EfiStatus;

/// Raw function pointer for committing a blob session.
pub type IpmiBlobCommit = unsafe extern "efiapi" fn(
    session_id: u16,
    commit_data_length: u8,
    commit_data: *const u8,
) -> EfiStatus;

/// Raw function pointer for closing a blob session.
pub type IpmiBlobClose = unsafe extern "efiapi" fn(session_id: u16) -> EfiStatus;

/// Raw function pointer for deleting a blob.
pub type IpmiBlobDelete = unsafe extern "efiapi" fn(blob_id: *const c_char) -> EfiStatus;

/// Raw function pointer for querying blob status.
pub type IpmiBlobStat = unsafe extern "efiapi" fn(
    blob_id: *const c_char,
    blob_state: *mut u16,
    size: *mut u32,
    metadata_length: *mut u8,
    metadata: *mut u8,
) -> EfiStatus;

/// Raw function pointer for querying session status.
pub type IpmiBlobSessionStat = unsafe extern "efiapi" fn(
    session_id: u16,
    blob_state: *mut u16,
    size: *mut u32,
    metadata_length: *mut u8,
    metadata: *mut u8,
) -> EfiStatus;

/// Raw function pointer for writing metadata to a blob session.
pub type IpmiBlobWriteMeta = unsafe extern "efiapi" fn(
    session_id: u16,
    offset: u32,
    data: *const u8,
    write_length: u32,
) -> EfiStatus;

/// Raw EFI IPMI Blob Transfer Protocol structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IpmiBlobTransferProtocol {
    /// Retrieve the number of active blobs.
    pub blob_get_count: IpmiBlobGetCount,
    /// Enumerate blob ID by index.
    pub blob_enumerate: IpmiBlobEnumerate,
    /// Open a session for a blob.
    pub blob_open: IpmiBlobOpen,
    /// Read data from a blob session.
    pub blob_read: IpmiBlobRead,
    /// Write data to a blob session.
    pub blob_write: IpmiBlobWrite,
    /// Commit a blob session.
    pub blob_commit: IpmiBlobCommit,
    /// Close a blob session.
    pub blob_close: IpmiBlobClose,
    /// Delete a blob.
    pub blob_delete: IpmiBlobDelete,
    /// Retrieve status of a blob by ID.
    pub blob_stat: IpmiBlobStat,
    /// Retrieve status of an active session.
    pub blob_session_stat: IpmiBlobSessionStat,
    /// Write metadata to a blob session.
    pub blob_write_meta: IpmiBlobWriteMeta,
}

/// Status and metadata information returned by blob stat queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlobStatResult {
    /// Current state flags of the blob.
    pub state: u16,
    /// Size in bytes of the blob.
    pub size: u32,
    /// Length of metadata written into the caller's buffer.
    pub metadata_len: usize,
}

/// High-level safe wrapper around [`IpmiBlobTransferProtocol`].
pub struct IpmiBlobTransfer<'a> {
    proto: &'a IpmiBlobTransferProtocol,
}

impl<'a> IpmiBlobTransfer<'a> {
    /// Maximum length supported for stack-buffered null-terminated blob IDs.
    pub const MAX_BLOB_ID_LEN: usize = 128;

    /// Wraps a raw [`IpmiBlobTransferProtocol`] reference.
    #[must_use]
    pub const fn new(proto: &'a IpmiBlobTransferProtocol) -> Self {
        Self { proto }
    }

    /// Retrieves the count of active blobs available through IPMI.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on transport or firmware error.
    pub fn get_count(&self) -> Result<u32, EfiStatus> {
        let mut count: u32 = 0;
        let status = unsafe { (self.proto.blob_get_count)(&mut count) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(count)
        }
    }

    /// Enumerates a blob ID by its 0-based index.
    ///
    /// Copies the null-terminated blob ID string into `id_buf`.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if index is invalid or buffer size is exceeded.
    pub fn enumerate<'b>(
        &self,
        blob_index: u32,
        id_buf: &'b mut [u8],
    ) -> Result<&'b str, EfiStatus> {
        if id_buf.is_empty() {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }
        let status = unsafe { (self.proto.blob_enumerate)(blob_index, id_buf.as_mut_ptr().cast()) };
        if status.is_error() {
            return Err(status);
        }
        let nul_pos = id_buf.iter().position(|&b| b == 0).unwrap_or(id_buf.len());
        core::str::from_utf8(&id_buf[..nul_pos]).map_err(|_| EfiStatus::DEVICE_ERROR)
    }

    /// Opens a session for a specific blob identified by its UTF-8 string ID.
    ///
    /// # Errors
    /// Returns [`EfiStatus::INVALID_PARAMETER`] if `blob_id` is too long, or the protocol error.
    pub fn open(&self, blob_id: &str, flags: u16) -> Result<u16, EfiStatus> {
        let mut id_buf = [0u8; Self::MAX_BLOB_ID_LEN];
        let bytes = blob_id.as_bytes();
        if bytes.len() >= id_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        id_buf[..bytes.len()].copy_from_slice(bytes);
        id_buf[bytes.len()] = 0;

        let mut session_id: u16 = 0;
        let status =
            unsafe { (self.proto.blob_open)(id_buf.as_ptr().cast(), flags, &mut session_id) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(session_id)
        }
    }

    /// Reads data from a blob session into `buffer`.
    ///
    /// Reads at most [`BLOB_MAX_DATA_PER_PACKET`] per transaction.
    ///
    /// # Errors
    /// Returns [`EfiStatus::BAD_BUFFER_SIZE`] if buffer exceeds maximum packet size, or transport error.
    pub fn read(
        &self,
        session_id: u16,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        if buffer.len() > BLOB_MAX_DATA_PER_PACKET {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }
        let requested_size = buffer.len() as u32;
        let status = unsafe {
            (self.proto.blob_read)(session_id, offset, requested_size, buffer.as_mut_ptr())
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(buffer.len())
        }
    }

    /// Writes data to a blob session.
    ///
    /// # Errors
    /// Returns [`EfiStatus::BAD_BUFFER_SIZE`] if data exceeds [`BLOB_MAX_DATA_PER_PACKET`], or protocol error.
    pub fn write(&self, session_id: u16, offset: u32, data: &[u8]) -> Result<(), EfiStatus> {
        if data.len() > BLOB_MAX_DATA_PER_PACKET {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }
        let write_len = data.len() as u32;
        let status =
            unsafe { (self.proto.blob_write)(session_id, offset, data.as_ptr(), write_len) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Commits a blob session, optionally providing commit verification data.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on transport or commit failure.
    pub fn commit(&self, session_id: u16, commit_data: Option<&[u8]>) -> Result<(), EfiStatus> {
        let (data_ptr, data_len) = match commit_data {
            Some(slice) => {
                if slice.len() > u8::MAX as usize {
                    return Err(EfiStatus::BAD_BUFFER_SIZE);
                }
                (slice.as_ptr(), slice.len() as u8)
            }
            None => (core::ptr::null(), 0),
        };

        let status = unsafe { (self.proto.blob_commit)(session_id, data_len, data_ptr) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Closes an active blob session.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if closing fails or session is invalid.
    pub fn close(&self, session_id: u16) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.blob_close)(session_id) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Deletes a specific blob identified by its UTF-8 string ID.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on deletion or parameter error.
    pub fn delete(&self, blob_id: &str) -> Result<(), EfiStatus> {
        let mut id_buf = [0u8; Self::MAX_BLOB_ID_LEN];
        let bytes = blob_id.as_bytes();
        if bytes.len() >= id_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        id_buf[..bytes.len()].copy_from_slice(bytes);
        id_buf[bytes.len()] = 0;

        let status = unsafe { (self.proto.blob_delete)(id_buf.as_ptr().cast()) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Retrieves statistics and state of a blob by ID.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on protocol error.
    pub fn stat(
        &self,
        blob_id: &str,
        mut metadata_buf: Option<&mut [u8]>,
    ) -> Result<BlobStatResult, EfiStatus> {
        let mut id_buf = [0u8; Self::MAX_BLOB_ID_LEN];
        let bytes = blob_id.as_bytes();
        if bytes.len() >= id_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        id_buf[..bytes.len()].copy_from_slice(bytes);
        id_buf[bytes.len()] = 0;

        let mut blob_state: u16 = 0;
        let mut size: u32 = 0;
        let mut meta_len: u8 = metadata_buf.as_ref().map_or(0, |b| b.len().min(255) as u8);
        let meta_ptr = match metadata_buf.as_mut() {
            Some(b) => b.as_mut_ptr(),
            None => core::ptr::null_mut(),
        };

        let status = unsafe {
            (self.proto.blob_stat)(
                id_buf.as_ptr().cast(),
                &mut blob_state,
                &mut size,
                &mut meta_len,
                meta_ptr,
            )
        };

        if status.is_error() {
            Err(status)
        } else {
            Ok(BlobStatResult {
                state: blob_state,
                size,
                metadata_len: meta_len as usize,
            })
        }
    }

    /// Queries the statistics and state of an active blob session.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on protocol error.
    pub fn session_stat(
        &self,
        session_id: u16,
        mut metadata_buf: Option<&mut [u8]>,
    ) -> Result<BlobStatResult, EfiStatus> {
        let mut blob_state: u16 = 0;
        let mut size: u32 = 0;
        let mut meta_len: u8 = metadata_buf.as_ref().map_or(0, |b| b.len().min(255) as u8);
        let meta_ptr = match metadata_buf.as_mut() {
            Some(b) => b.as_mut_ptr(),
            None => core::ptr::null_mut(),
        };

        let status = unsafe {
            (self.proto.blob_session_stat)(
                session_id,
                &mut blob_state,
                &mut size,
                &mut meta_len,
                meta_ptr,
            )
        };

        if status.is_error() {
            Err(status)
        } else {
            Ok(BlobStatResult {
                state: blob_state,
                size,
                metadata_len: meta_len as usize,
            })
        }
    }

    /// Writes metadata associated with a blob session.
    ///
    /// # Errors
    /// Returns [`EfiStatus::BAD_BUFFER_SIZE`] if data exceeds maximum packet size, or protocol error.
    pub fn write_meta(&self, session_id: u16, offset: u32, data: &[u8]) -> Result<(), EfiStatus> {
        if data.len() > BLOB_MAX_DATA_PER_PACKET {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }
        let write_len = data.len() as u32;
        let status =
            unsafe { (self.proto.blob_write_meta)(session_id, offset, data.as_ptr(), write_len) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
