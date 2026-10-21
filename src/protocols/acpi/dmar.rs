//! # DMAR
//!
//! DMA Remapping Table parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_DMAR;

/// DMAR header data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmarHeader {
    /// The address width.
    pub width: u8,
    /// The flags.
    pub flags: u8,
}

/// A DMAR entry view.
pub struct DmarEntry<'a> {
    entry_type: u8,
    bytes: &'a [u8],
}

impl DmarEntry<'_> {
    /// Returns the entry type.
    #[must_use]
    pub const fn entry_type(&self) -> u8 {
        self.entry_type
    }

    /// Returns the entry bytes.
    #[must_use]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

/// Iterates DMAR entries.
pub struct DmarIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    end: usize,
}

impl<'a> Iterator for DmarIter<'a> {
    type Item = DmarEntry<'a>;

    /// Returns the next entry.
    fn next(&mut self) -> Option<Self::Item> {
        let entry_type = *self.bytes.get(self.offset)?;
        let length_start = self.offset.checked_add(1)?;
        let length_end = length_start.checked_add(2)?;
        let mut length_raw = [0_u8; 2];
        length_raw.copy_from_slice(self.bytes.get(length_start..length_end)?);
        let length = usize::from(u16::from_le_bytes(length_raw));
        if length < 3 {
            return None;
        }
        let end = self.offset.checked_add(length)?;
        if end > self.end {
            return None;
        }
        let slice = self.bytes.get(self.offset..end)?;
        self.offset = end;
        Some(DmarEntry {
            entry_type,
            bytes: slice,
        })
    }
}

/// Parses the DMAR header.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn parse_dmar_header(buffer: &[u8]) -> Option<DmarHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_DMAR {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 40 {
        return None;
    }
    Some(DmarHeader {
        width: *buffer.get(36)?,
        flags: *buffer.get(37)?,
    })
}

/// Creates a DMAR entry iterator.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn dmar_iter(buffer: &[u8]) -> Option<DmarIter<'_>> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_DMAR {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 40 {
        return None;
    }
    Some(DmarIter {
        bytes: buffer,
        offset: 40,
        end: table_length,
    })
}
