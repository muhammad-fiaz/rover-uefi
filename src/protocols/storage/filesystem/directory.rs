//! # Directory helpers
//!
//! Directory enumeration through file reads.
//!
//! Repeated reads on a directory handle return
//! one entry per call until the buffer is
//! empty. The caller provides the buffer.

use crate::abi::types::UIntN;
use crate::base::EfiStatus;
use crate::protocols::storage::filesystem::file::OwnedFile;

/// Returns whether a read indicates the end
/// of a directory.
///
/// End of directory is reported as success
/// with zero bytes read.
#[must_use]
pub fn is_end_of_directory(bytes_read: UIntN) -> bool {
    bytes_read == 0
}

/// Reads one directory entry.
///
/// Returns `Ok(0)` at the end of the directory.
/// The buffer must be large enough for one
/// `EFI_FILE_INFO` entry.
///
/// # Errors
///
/// Returns the firmware status when the read
/// fails.
pub fn read_directory_entry(file: &OwnedFile, buffer: &mut [u8]) -> Result<UIntN, EfiStatus> {
    file.read(buffer)
}
