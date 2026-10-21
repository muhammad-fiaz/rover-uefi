//! # SRAT
//!
//! System Resource Affinity Table parsing.
//!
//! The simplified layout uses a 40-byte header
//! with a reserved field at offset 36 followed
//! by affinity entries.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_SRAT;

/// The CPU affinity entry type.
pub const SRAT_TYPE_CPU_AFFINITY: u8 = 0;

/// The memory affinity entry type.
pub const SRAT_TYPE_MEMORY_AFFINITY: u8 = 1;

/// The I/O APIC affinity entry type.
pub const SRAT_TYPE_IO_APIC_AFFINITY: u8 = 2;

/// The x2APIC affinity entry type.
pub const SRAT_TYPE_X2APIC_AFFINITY: u8 = 3;

/// SRAT header data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SratHeader {
    /// The reserved field.
    pub reserved: u32,
}

/// A SRAT entry view.
pub struct SratEntry<'a> {
    entry_type: u8,
    bytes: &'a [u8],
}

impl SratEntry<'_> {
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

/// Iterates SRAT entries.
pub struct SratIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    end: usize,
}

impl<'a> Iterator for SratIter<'a> {
    type Item = SratEntry<'a>;

    /// Returns the next entry.
    fn next(&mut self) -> Option<Self::Item> {
        let entry_type = *self.bytes.get(self.offset)?;
        let length_offset = self.offset.checked_add(1)?;
        let length = usize::from(*self.bytes.get(length_offset)?);
        if length < 2 {
            return None;
        }
        let end = self.offset.checked_add(length)?;
        if end > self.end {
            return None;
        }
        let slice = self.bytes.get(self.offset..end)?;
        self.offset = end;
        Some(SratEntry {
            entry_type,
            bytes: slice,
        })
    }
}

/// Parses the SRAT header.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn parse_srat_header(buffer: &[u8]) -> Option<SratHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_SRAT {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 40 {
        return None;
    }
    let mut reserved_raw = [0_u8; 4];
    reserved_raw.copy_from_slice(buffer.get(36..40)?);
    Some(SratHeader {
        reserved: u32::from_le_bytes(reserved_raw),
    })
}

/// Creates a SRAT entry iterator.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn srat_iter(buffer: &[u8]) -> Option<SratIter<'_>> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_SRAT {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 40 {
        return None;
    }
    Some(SratIter {
        bytes: buffer,
        offset: 40,
        end: table_length,
    })
}
