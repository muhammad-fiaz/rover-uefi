//! # MADT
//!
//! Multiple APIC Description Table parsing.

/// MADT header data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MadtHeader {
    /// The local APIC address.
    pub local_apic_address: u32,
    /// The MADT flags.
    pub flags: u32,
}

/// A MADT entry view.
pub struct MadtEntry<'a> {
    entry_type: u8,
    bytes: &'a [u8],
}

impl MadtEntry<'_> {
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

/// Iterates MADT entries.
pub struct MadtIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for MadtIter<'a> {
    type Item = MadtEntry<'a>;

    /// Returns the next entry.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 2 > self.bytes.len() {
            return None;
        }
        let entry_type = *self.bytes.get(self.offset)?;
        let length = *self.bytes.get(self.offset + 1)? as usize;
        if length < 2 || self.offset + length > self.bytes.len() {
            return None;
        }
        let slice = self.bytes.get(self.offset..self.offset + length)?;
        self.offset += length;
        Some(MadtEntry {
            entry_type,
            bytes: slice,
        })
    }
}

/// Parses the MADT header.
#[must_use]
pub fn parse_madt_header(buffer: &[u8]) -> Option<MadtHeader> {
    if buffer.len() < 44 {
        return None;
    }
    let mut addr = [0_u8; 4];
    addr.copy_from_slice(buffer.get(36..40)?);
    let mut flags = [0_u8; 4];
    flags.copy_from_slice(buffer.get(40..44)?);
    Some(MadtHeader {
        local_apic_address: u32::from_le_bytes(addr),
        flags: u32::from_le_bytes(flags),
    })
}

/// Creates a MADT entry iterator.
#[must_use]
pub fn madt_iter(buffer: &[u8]) -> Option<MadtIter<'_>> {
    if buffer.len() < 44 {
        None
    } else {
        Some(MadtIter {
            bytes: buffer,
            offset: 44,
        })
    }
}
