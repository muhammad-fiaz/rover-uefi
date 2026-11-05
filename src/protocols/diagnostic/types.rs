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

/// The OEM-specific class bitmask/range start (values 0x80..0xFF in top byte).
pub const CLASS_OEM_MASK: u32 = 0x8000_0000;

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

/// Returns whether a status-code value has a defined class and valid operation.
///
/// Status code classes defined by PI spec:
/// - 0x00: Computing Unit (`CLASS_COMPUTING_UNIT`)
/// - 0x01: Peripheral (`CLASS_PERIPHERAL`)
/// - 0x02: I/O Bus (`CLASS_IO_BUS`)
/// - 0x03: Software (`CLASS_SOFTWARE`)
/// - 0x80..0xFF: Reserved for OEM use
///
/// An all-zero status code (`0x0000_0000`) is invalid / uninitialized.
#[must_use]
pub fn validate_status_code_value(value: u32) -> bool {
    if value == 0 {
        return false;
    }
    let class = (value & 0xFF00_0000) >> 24;
    class <= 0x03 || class >= 0x80
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
