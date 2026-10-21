//! # SMBIOS iterator
//!
//! Bounded iteration over the table.
//!
//! Each structure ends with a double null.
//! The iterator stops at the buffer end.

use crate::protocols::smbios::structure::SmbiosHeader;

/// A single parsed structure view.
pub struct SmbiosStructure<'a> {
    header: SmbiosHeader,
    formatted: &'a [u8],
    strings: &'a [u8],
}

impl SmbiosStructure<'_> {
    /// Returns the header.
    #[must_use]
    pub const fn header(&self) -> &SmbiosHeader {
        &self.header
    }

    /// Returns the formatted bytes.
    #[must_use]
    pub const fn formatted(&self) -> &[u8] {
        self.formatted
    }

    /// Returns the string bytes.
    #[must_use]
    pub const fn strings(&self) -> &[u8] {
        self.strings
    }

    /// Returns the type.
    #[must_use]
    pub const fn structure_type(&self) -> u8 {
        self.header.structure_type
    }
}

/// Iterates structures.
pub struct SmbiosIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for SmbiosIter<'a> {
    type Item = SmbiosStructure<'a>;

    /// Returns the next structure.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 3 > self.bytes.len() {
            return None;
        }
        let length = *self.bytes.get(self.offset + 1)? as usize;
        if length < 3 {
            return None;
        }
        let mut handle = [0_u8; 2];
        handle.copy_from_slice(self.bytes.get(self.offset + 2..self.offset + 4)?);
        let header = SmbiosHeader {
            structure_type: *self.bytes.get(self.offset)?,
            length: u8::try_from(length).unwrap_or(0),
            handle: u16::from_le_bytes(handle),
        };
        let formatted_end = self.offset.checked_add(length)?;
        if formatted_end >= self.bytes.len() {
            return None;
        }
        // Find double null.
        let mut cursor = formatted_end;
        let mut end = None;
        while cursor + 1 < self.bytes.len() {
            if self.bytes[cursor] == 0 && self.bytes[cursor + 1] == 0 {
                end = Some(cursor + 2);
                break;
            }
            cursor += 1;
        }
        let structure_end = end?;
        let formatted = self.bytes.get(self.offset..formatted_end)?;
        let strings = self.bytes.get(formatted_end..structure_end)?;
        self.offset = structure_end;
        Some(SmbiosStructure {
            header,
            formatted,
            strings,
        })
    }
}

/// Creates an iterator.
#[must_use]
pub fn smbios_iter(bytes: &[u8]) -> SmbiosIter<'_> {
    SmbiosIter { bytes, offset: 0 }
}
