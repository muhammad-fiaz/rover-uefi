//! # SMBIOS records
//!
//! Typed record summaries.
//!
//! Only the header is decoded. Formatted
//! fields are read by callers through the
//! formatted slice.

/// The end-of-table type.
pub const TYPE_END_OF_TABLE: u8 = 127;

/// The BIOS information type.
pub const TYPE_BIOS_INFORMATION: u8 = 0;

/// The system information type.
pub const TYPE_SYSTEM_INFORMATION: u8 = 1;

/// The processor information type.
pub const TYPE_PROCESSOR_INFORMATION: u8 = 4;

/// Returns whether a type is the end marker.
#[must_use]
pub fn is_end_of_table(structure_type: u8) -> bool {
    structure_type == TYPE_END_OF_TABLE
}
