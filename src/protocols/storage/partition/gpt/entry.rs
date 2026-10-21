//! # GPT entries
//!
//! Parsing of GPT partition entries.

use crate::abi::types::Char16;
use crate::base::EfiGuid;
use crate::protocols::storage::partition::types::PartitionTableError;

/// A parsed GPT entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GptEntry {
    /// The partition type `GUID`.
    pub type_guid: EfiGuid,
    /// The unique partition `GUID`.
    pub unique_guid: EfiGuid,
    /// The starting LBA.
    pub starting_lba: u64,
    /// The ending LBA.
    pub ending_lba: u64,
    /// The attributes.
    pub attributes: u64,
    /// The UTF-16 name.
    pub name: [Char16; 36],
}

fn read_u64(bytes: &[u8]) -> u64 {
    crate::base::bytes::read_u64_le(bytes, 0).unwrap_or_default()
}

fn read_u16(bytes: &[u8]) -> u16 {
    crate::base::bytes::read_u16_le(bytes, 0).unwrap_or_default()
}

/// Parses one GPT entry.
///
/// # Errors
///
/// Returns `Truncated` when the buffer is
/// shorter than one entry.
pub fn parse_gpt_entry(buffer: &[u8]) -> Result<GptEntry, PartitionTableError> {
    if buffer.len() < 128 {
        return Err(PartitionTableError::Truncated);
    }
    let mut name = [0 as Char16; 36];
    for (index, slot) in name.iter_mut().enumerate() {
        let offset = 56 + index * 2;
        *slot = read_u16(&buffer[offset..offset + 2]);
    }
    Ok(GptEntry {
        type_guid: EfiGuid::from_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14],
            buffer[15],
        ]),
        unique_guid: EfiGuid::from_bytes([
            buffer[16], buffer[17], buffer[18], buffer[19], buffer[20], buffer[21], buffer[22],
            buffer[23], buffer[24], buffer[25], buffer[26], buffer[27], buffer[28], buffer[29],
            buffer[30], buffer[31],
        ]),
        starting_lba: read_u64(&buffer[32..40]),
        ending_lba: read_u64(&buffer[40..48]),
        attributes: read_u64(&buffer[48..56]),
        name,
    })
}

/// Returns whether an entry is unused.
#[must_use]
pub fn is_unused(entry: &GptEntry) -> bool {
    entry.type_guid.is_zero()
}
