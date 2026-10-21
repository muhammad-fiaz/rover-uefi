//! # SLIT
//!
//! System Locality Information Table parsing.
//!
//! The table header is 36 bytes followed by a
//! locality count and a distances matrix.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_SLIT;

/// Parsed SLIT data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlitInfo {
    /// The number of localities.
    pub num_localities: u64,
}

/// Iterates SLIT distance values.
pub struct SlitIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    end: usize,
}

impl Iterator for SlitIter<'_> {
    type Item = u8;

    /// Returns the next distance value.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.end {
            return None;
        }
        let value = *self.bytes.get(self.offset)?;
        self.offset = self.offset.checked_add(1)?;
        Some(value)
    }
}

/// Parses a SLIT buffer.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, the checksum fails, or
/// the distances matrix does not fit.
#[must_use]
pub fn parse_slit(buffer: &[u8]) -> Option<SlitInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_SLIT {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 44 {
        return None;
    }
    let mut count_raw = [0_u8; 8];
    count_raw.copy_from_slice(buffer.get(36..44)?);
    let num_localities = u64::from_le_bytes(count_raw);
    let count_usize = usize::try_from(num_localities).ok()?;
    let cell_count = count_usize.checked_mul(count_usize)?;
    let required = 44_usize.checked_add(cell_count)?;
    if required > table_length {
        return None;
    }
    Some(SlitInfo { num_localities })
}

/// Creates a SLIT distance iterator.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, the checksum fails, or
/// the distances matrix does not fit.
#[must_use]
pub fn slit_iter(buffer: &[u8]) -> Option<SlitIter<'_>> {
    let info = parse_slit(buffer)?;
    let header = parse_header(buffer)?;
    let _table_length = usize::try_from(header.length).ok()?;
    let count_usize = usize::try_from(info.num_localities).ok()?;
    let cell_count = count_usize.checked_mul(count_usize)?;
    let end = 44_usize.checked_add(cell_count)?;
    Some(SlitIter {
        bytes: buffer,
        offset: 44,
        end,
    })
}
