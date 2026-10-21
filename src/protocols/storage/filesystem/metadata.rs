//! # File metadata
//!
//! Information structures for files and volumes.
//!
//! Each structure is `#[repr(C)]` and ends in a
//! flexible array member: the trailing character
//! array is declared with one element, but the
//! structure is always followed by a variable-length
//! UTF-16 string that extends past the declared
//! field. The `SIZE_OF` constants give the fixed
//! header size excluding that string.

use crate::abi::types::{Bool, Char16};
use crate::base::EfiTime;

/// The size of `EFI_FILE_INFO` without the name.
pub const SIZE_OF_EFI_FILE_INFO: usize = 80;

/// The size of `EFI_FILE_SYSTEM_INFO` without the label.
pub const SIZE_OF_EFI_FILE_SYSTEM_INFO: usize = 36;

/// The size of the volume label without the label.
pub const SIZE_OF_EFI_FILE_SYSTEM_VOLUME_LABEL: usize = 0;

/// File information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiFileInfo {
    /// The total size including the name.
    pub size: u64,
    /// The file size in bytes.
    pub file_size: u64,
    /// The physical size in bytes.
    pub physical_size: u64,
    /// The create time.
    pub create_time: EfiTime,
    /// The last access time.
    pub last_access_time: EfiTime,
    /// The modification time.
    pub modification_time: EfiTime,
    /// The file attributes.
    pub attribute: u64,
    /// The first code unit of the flexible file name array.
    pub file_name: [Char16; 1],
}

/// File system information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiFileSystemInfo {
    /// The total size including the label.
    pub size: u64,
    /// Whether the volume is read-only.
    pub read_only: Bool,
    /// The volume size in bytes.
    pub volume_size: u64,
    /// The free space in bytes.
    pub free_space: u64,
    /// The nominal block size.
    pub block_size: u32,
    /// The first code unit of the flexible volume label array.
    pub volume_label: [Char16; 1],
}

/// The volume label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiFileSystemVolumeLabel {
    /// The first code unit of the flexible volume label array.
    pub volume_label: [Char16; 1],
}

const _: () = {
    assert!(core::mem::offset_of!(EfiFileInfo, file_name) == SIZE_OF_EFI_FILE_INFO);
    assert!(core::mem::offset_of!(EfiFileSystemInfo, volume_label) == SIZE_OF_EFI_FILE_SYSTEM_INFO);
    assert!(core::mem::offset_of!(EfiFileSystemVolumeLabel, volume_label) == 0);
};

/// Returns the file name bytes as UTF-16.
///
/// Returns `None` when the buffer is too
/// small.
#[must_use]
pub fn file_info_file_name(buffer: &[u8]) -> Option<&[u8]> {
    buffer.get(SIZE_OF_EFI_FILE_INFO..)
}

/// Returns whether a file info buffer is
/// well-formed.
///
/// The check verifies the fixed size, the
/// reported size, and name termination in
/// bounds without dereferencing unaligned
/// data.
#[must_use]
pub fn validate_file_info(buffer: &[u8]) -> bool {
    if buffer.len() < SIZE_OF_EFI_FILE_INFO {
        return false;
    }
    let Some(size_bytes) = buffer.get(0..8) else {
        return false;
    };
    let mut size_raw = [0_u8; 8];
    size_raw.copy_from_slice(size_bytes);
    let reported = u64::from_le_bytes(size_raw);
    let reported_usize = usize::try_from(reported).unwrap_or(usize::MAX);
    if reported_usize != buffer.len() {
        return false;
    }
    buffer.get(SIZE_OF_EFI_FILE_INFO..).is_some_and(|tail| {
        tail.len() >= 2 && tail[tail.len() - 2] == 0 && tail[tail.len() - 1] == 0
    })
}

/// Returns whether a file system info buffer
/// is well-formed.
#[must_use]
pub fn validate_file_system_info(buffer: &[u8]) -> bool {
    if buffer.len() < SIZE_OF_EFI_FILE_SYSTEM_INFO {
        return false;
    }
    let Some(size_bytes) = buffer.get(0..8) else {
        return false;
    };
    let mut size_raw = [0_u8; 8];
    size_raw.copy_from_slice(size_bytes);
    let reported = u64::from_le_bytes(size_raw);
    usize::try_from(reported).unwrap_or(usize::MAX) == buffer.len()
}
