//! # MCFG
//!
//! PCI Express configuration table.
//!
//! The table header is 36 bytes followed by
//! 8-byte allocation entries.

/// A MCFG allocation entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct McfgEntry {
    /// The base address.
    pub base_address: u64,
    /// The segment group.
    pub segment_group: u16,
    /// The start bus.
    pub start_bus: u8,
    /// The end bus.
    pub end_bus: u8,
}

/// Iterates MCFG entries.
pub struct McfgIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl Iterator for McfgIter<'_> {
    type Item = McfgEntry;

    /// Returns the next entry.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 16 > self.bytes.len() {
            return None;
        }
        let mut base = [0_u8; 8];
        base.copy_from_slice(self.bytes.get(self.offset..self.offset + 8)?);
        let mut segment = [0_u8; 2];
        segment.copy_from_slice(self.bytes.get(self.offset + 8..self.offset + 10)?);
        let start_bus = *self.bytes.get(self.offset + 10)?;
        let end_bus = *self.bytes.get(self.offset + 11)?;
        self.offset += 16;
        Some(McfgEntry {
            base_address: u64::from_le_bytes(base),
            segment_group: u16::from_le_bytes(segment),
            start_bus,
            end_bus,
        })
    }
}

/// Creates an MCFG iterator.
#[must_use]
pub fn mcfg_iter(buffer: &[u8]) -> Option<McfgIter<'_>> {
    if buffer.len() < 36 {
        None
    } else {
        Some(McfgIter {
            bytes: buffer,
            offset: 36,
        })
    }
}
