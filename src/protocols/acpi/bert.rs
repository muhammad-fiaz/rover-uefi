//! # BERT
//!
//! Boot Error Record Table (BERT) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_BERT;

/// Error severity levels in Generic Error Status Block.
pub mod error_severity {
    /// Recoverable error.
    pub const RECOVERABLE: u32 = 0;
    /// Fatal error.
    pub const FATAL: u32 = 1;
    /// Corrected error.
    pub const CORRECTED: u32 = 2;
    /// Information / None.
    pub const NONE: u32 = 3;
}

/// Parsed BERT table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BertInfo {
    /// Boot Error Region length in bytes.
    pub region_length: u32,
    /// Boot Error Region physical address.
    pub region_address: u64,
}

/// Generic Error Status Block (GESB) header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenericErrorStatusBlock {
    /// Status flags (uncorrectable/correctable valid, entry count).
    pub block_status: u32,
    /// Offset to raw unformatted error data.
    pub raw_data_offset: u32,
    /// Length of raw error data in bytes.
    pub raw_data_length: u32,
    /// Length of generic error data entries in bytes.
    pub data_length: u32,
    /// Error severity (Recoverable, Fatal, Corrected, None).
    pub error_severity: u32,
}

/// Parses the BERT table.
#[must_use]
pub fn parse_bert(buffer: &[u8]) -> Option<BertInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_BERT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 48 || buffer.len() < 48 {
        return None;
    }

    let mut reg_len = [0_u8; 4];
    reg_len.copy_from_slice(buffer.get(36..40)?);
    let mut reg_addr = [0_u8; 8];
    reg_addr.copy_from_slice(buffer.get(40..48)?);

    Some(BertInfo {
        region_length: u32::from_le_bytes(reg_len),
        region_address: u64::from_le_bytes(reg_addr),
    })
}

/// Parses a Generic Error Status Block (GESB) from a memory buffer.
#[must_use]
pub fn parse_generic_error_status(buffer: &[u8]) -> Option<GenericErrorStatusBlock> {
    if buffer.len() < 20 {
        return None;
    }

    let mut block_status = [0_u8; 4];
    block_status.copy_from_slice(buffer.get(0..4)?);
    let mut raw_data_offset = [0_u8; 4];
    raw_data_offset.copy_from_slice(buffer.get(4..8)?);
    let mut raw_data_length = [0_u8; 4];
    raw_data_length.copy_from_slice(buffer.get(8..12)?);
    let mut data_length = [0_u8; 4];
    data_length.copy_from_slice(buffer.get(12..16)?);
    let mut err_severity = [0_u8; 4];
    err_severity.copy_from_slice(buffer.get(16..20)?);

    Some(GenericErrorStatusBlock {
        block_status: u32::from_le_bytes(block_status),
        raw_data_offset: u32::from_le_bytes(raw_data_offset),
        raw_data_length: u32::from_le_bytes(raw_data_length),
        data_length: u32::from_le_bytes(data_length),
        error_severity: u32::from_le_bytes(err_severity),
    })
}
