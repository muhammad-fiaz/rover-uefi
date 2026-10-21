//! # Diagnostic types
//!
//! Status-code encoding and reporting structures.

use crate::base::EfiGuid;

/// The progress code type.
pub const PROGRESS_CODE: u32 = 0x0000_0001;

/// The error code type.
pub const ERROR_CODE: u32 = 0x0000_0002;

/// The debug code type.
pub const DEBUG_CODE: u32 = 0x0000_0003;

/// The minor-error severity.
pub const ERROR_MINOR: u32 = 0x4000_0000;

/// The major-error severity.
pub const ERROR_MAJOR: u32 = 0x8000_0000;

/// The unrecovered-error severity.
pub const ERROR_UNRECOVERED: u32 = 0x9000_0000;

/// The uncontained-error severity.
pub const ERROR_UNCONTAINED: u32 = 0xA000_0000;

/// The computing-unit class.
pub const CLASS_COMPUTING_UNIT: u32 = 0x0000_0000;

/// The peripheral class.
pub const CLASS_PERIPHERAL: u32 = 0x0100_0000;

/// The I/O-bus class.
pub const CLASS_IO_BUS: u32 = 0x0200_0000;

/// The software class.
pub const CLASS_SOFTWARE: u32 = 0x0300_0000;

/// The status-code data header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiStatusCodeData {
    /// The header size.
    pub header_size: u16,
    /// The data size, excluding the header.
    pub size: u16,
    /// The data-type `GUID`.
    pub data_type: EfiGuid,
}

/// Returns whether a status-code value has a defined class.
#[must_use]
pub fn validate_status_code_value(value: u32) -> bool {
    value != 0 && (value & 0xFF00_0000) != 0
}

/// Returns the class portion of a status-code value.
#[must_use]
pub fn status_code_class(value: u32) -> u32 {
    value & 0xFF00_0000
}

/// Returns the subclass portion of a status-code value.
#[must_use]
pub fn status_code_subclass(value: u32) -> u32 {
    (value & 0x00FF_0000) >> 16
}

/// Returns whether a status-code data header is valid.
#[must_use]
pub fn validate_status_code_data(header_size: u16, size: u16, buffer_len: usize) -> bool {
    let total = usize::from(header_size).checked_add(usize::from(size));
    total.is_some_and(|total| total <= buffer_len && header_size >= 20)
}
