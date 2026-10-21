//! # MBR entries
//!
//! Parsing of MBR partition records.

use crate::protocols::storage::partition::types::PartitionTableError;

/// A parsed MBR entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MbrEntry {
    /// The boot indicator.
    pub boot_indicator: u8,
    /// The partition type.
    pub partition_type: u8,
    /// The starting LBA.
    pub starting_lba: u32,
    /// The sector count.
    pub sector_count: u32,
}

fn read_u32(bytes: &[u8]) -> u32 {
    crate::base::bytes::read_u32_le(bytes, 0).unwrap_or_default()
}

/// Parses one MBR entry from 16 bytes.
///
/// # Errors
///
/// Returns `Truncated` when the buffer is
/// shorter than one entry.
pub fn parse_mbr_entry(buffer: &[u8]) -> Result<MbrEntry, PartitionTableError> {
    if buffer.len() < 16 {
        return Err(PartitionTableError::Truncated);
    }
    Ok(MbrEntry {
        boot_indicator: buffer[0],
        partition_type: buffer[4],
        starting_lba: read_u32(&buffer[8..12]),
        sector_count: read_u32(&buffer[12..16]),
    })
}

/// Returns whether the entry is empty.
#[must_use]
pub fn is_empty(entry: &MbrEntry) -> bool {
    entry.partition_type == 0
}
