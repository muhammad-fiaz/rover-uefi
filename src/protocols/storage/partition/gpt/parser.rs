//! # GPT parser
//!
//! Iteration over GPT entries.
//!
//! The caller provides the entry array buffer.
//! The parser validates the array CRC and
//! yields non-empty entries.

use crate::protocols::storage::partition::crc32::crc32;
use crate::protocols::storage::partition::gpt::entry::{GptEntry, is_unused, parse_gpt_entry};
use crate::protocols::storage::partition::gpt::header::GptHeader;
use crate::protocols::storage::partition::types::PartitionTableError;

/// Iterates GPT entries.
pub struct GptIter<'a> {
    bytes: &'a [u8],
    entry_size: usize,
    index: usize,
    count: usize,
}

impl Iterator for GptIter<'_> {
    type Item = Result<GptEntry, PartitionTableError>;

    /// Returns the next entry.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let offset = self.index.checked_mul(self.entry_size)?;
        self.index += 1;
        let Some(entry_bytes) = self.bytes.get(offset..offset + self.entry_size) else {
            return Some(Err(PartitionTableError::Truncated));
        };
        match parse_gpt_entry(entry_bytes) {
            Ok(entry) => Some(Ok(entry)),
            Err(error) => Some(Err(error)),
        }
    }
}

/// Creates a GPT iterator after CRC check.
///
/// # Errors
///
/// Returns `InvalidCrc` when the array CRC
/// does not match.
pub fn gpt_iter<'a>(
    header: &GptHeader,
    array: &'a [u8],
) -> Result<GptIter<'a>, PartitionTableError> {
    let total = (header.number_of_entries as usize)
        .checked_mul(header.size_of_entry as usize)
        .ok_or(PartitionTableError::Overflow)?;
    if array.len() < total {
        return Err(PartitionTableError::Truncated);
    }
    let computed = crc32(&array[..total]);
    if computed != header.partition_entry_array_crc32 {
        return Err(PartitionTableError::InvalidCrc);
    }
    Ok(GptIter {
        bytes: array,
        entry_size: header.size_of_entry as usize,
        index: 0,
        count: header.number_of_entries as usize,
    })
}

/// Filters unused entries from an iterator.
pub fn non_empty_entries(
    iter: GptIter<'_>,
) -> impl Iterator<Item = Result<GptEntry, PartitionTableError>> {
    iter.filter(|item| match item {
        Ok(entry) => !is_unused(entry),
        Err(_) => true,
    })
}
