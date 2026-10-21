//! # MBR sector
//!
//! Parsing of the 512-byte MBR sector.

use crate::protocols::storage::partition::mbr::entry::{MbrEntry, parse_mbr_entry};
use crate::protocols::storage::partition::types::PartitionTableError;

/// A parsed MBR sector.
#[derive(Debug, PartialEq, Eq)]
pub struct MbrSector {
    /// The four partition entries.
    pub entries: [MbrEntry; 4],
    /// The raw signature.
    pub signature: u16,
}

/// Parses an MBR sector buffer.
///
/// The buffer must be at least 512 bytes.
///
/// # Errors
///
/// Returns `Truncated` when the buffer is
/// short, `InvalidSignature` when the signature
/// is wrong, and `InvalidBootIndicator` for
/// invalid boot values.
pub fn parse_mbr_sector(buffer: &[u8]) -> Result<MbrSector, PartitionTableError> {
    if buffer.len() < 512 {
        return Err(PartitionTableError::Truncated);
    }
    let signature = u16::from_le_bytes([buffer[510], buffer[511]]);
    if signature != 0xAA55 {
        return Err(PartitionTableError::InvalidSignature);
    }
    let mut entries = [MbrEntry {
        boot_indicator: 0,
        partition_type: 0,
        starting_lba: 0,
        sector_count: 0,
    }; 4];
    for (index, slot) in entries.iter_mut().enumerate() {
        let offset = 446 + index * 16;
        *slot = parse_mbr_entry(&buffer[offset..offset + 16])?;
        let boot = slot.boot_indicator;
        if boot != 0x00 && boot != 0x80 {
            return Err(PartitionTableError::InvalidBootIndicator);
        }
    }
    Ok(MbrSector { entries, signature })
}

/// Returns whether the sector is a
/// protective MBR.
#[must_use]
pub fn is_protective_mbr(sector: &MbrSector) -> bool {
    sector
        .entries
        .iter()
        .any(|entry| entry.partition_type == 0xEE)
}
