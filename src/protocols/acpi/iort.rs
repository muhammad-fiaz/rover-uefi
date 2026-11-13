//! # IORT
//!
//! ARM I/O Remapping Table (IORT) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_IORT;

/// IORT Node Types.
pub mod iort_node_type {
    /// Interrupt Translation Service (ITS) Group.
    pub const ITS_GROUP: u8 = 0;
    /// Named Component (platform device).
    pub const NAMED_COMPONENT: u8 = 1;
    /// PCI Express Root Complex.
    pub const ROOT_COMPLEX: u8 = 2;
    /// System MMU version 1 or 2 (`SMMUv1`/`SMMUv2`).
    pub const SMMU_V1_V2: u8 = 3;
    /// System MMU version 3 (`SMMUv3`).
    pub const SMMU_V3: u8 = 4;
    /// Performance Monitoring Counter Group (PMCG).
    pub const PMCG: u8 = 5;
    /// Memory Range Node.
    pub const MEMORY_RANGE: u8 = 6;
}

/// Parsed IORT table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IortHeader {
    /// Number of IORT nodes in table.
    pub node_count: u32,
    /// Offset from table start to first node.
    pub node_offset: u32,
}

/// A parsed generic IORT Node entry view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IortNode<'a> {
    /// Node type.
    pub node_type: u8,
    /// Node length in bytes.
    pub length: u16,
    /// Node revision.
    pub revision: u8,
    /// Node unique identifier.
    pub identifier: u32,
    /// Number of ID mappings.
    pub id_mapping_count: u32,
    /// Offset to ID mapping array within node.
    pub id_mapping_offset: u32,
    /// Raw slice containing this node.
    pub raw: &'a [u8],
}

/// Parses the IORT table header.
#[must_use]
pub fn parse_iort_header(buffer: &[u8]) -> Option<IortHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_IORT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 48 || buffer.len() < 48 {
        return None;
    }

    let mut count_raw = [0_u8; 4];
    count_raw.copy_from_slice(buffer.get(36..40)?);
    let mut offset_raw = [0_u8; 4];
    offset_raw.copy_from_slice(buffer.get(40..44)?);

    Some(IortHeader {
        node_count: u32::from_le_bytes(count_raw),
        node_offset: u32::from_le_bytes(offset_raw),
    })
}

/// An iterator over nodes in an IORT table.
pub struct IortIter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    remaining: u32,
}

impl<'a> Iterator for IortIter<'a> {
    type Item = IortNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        if slice.len() < 16 {
            return None;
        }

        let node_type = *slice.first()?;
        let length = u16::from_le_bytes([*slice.get(1)?, *slice.get(2)?]);
        if usize::from(length) < 16 || usize::from(length) > slice.len() {
            return None;
        }

        let revision = *slice.get(3)?;
        let mut id_raw = [0_u8; 4];
        id_raw.copy_from_slice(slice.get(4..8)?);
        let mut map_count = [0_u8; 4];
        map_count.copy_from_slice(slice.get(8..12)?);
        let mut map_offset = [0_u8; 4];
        map_offset.copy_from_slice(slice.get(12..16)?);

        let node_raw = &slice[..usize::from(length)];
        self.current_offset = self.current_offset.saturating_add(usize::from(length));
        self.remaining = self.remaining.saturating_sub(1);

        Some(IortNode {
            node_type,
            length,
            revision,
            identifier: u32::from_le_bytes(id_raw),
            id_mapping_count: u32::from_le_bytes(map_count),
            id_mapping_offset: u32::from_le_bytes(map_offset),
            raw: node_raw,
        })
    }
}

/// Returns an iterator over nodes in an IORT buffer.
#[must_use]
pub fn iort_iter(buffer: &[u8]) -> Option<IortIter<'_>> {
    let header = parse_iort_header(buffer)?;
    let start_offset = usize::try_from(header.node_offset).ok()?;
    if start_offset > buffer.len() {
        return None;
    }

    Some(IortIter {
        buffer,
        current_offset: start_offset,
        remaining: header.node_count,
    })
}
