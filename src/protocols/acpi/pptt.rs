//! # PPTT
//!
//! Processor Properties Topology Table (PPTT) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_PPTT;

/// PPTT structure types.
pub mod pptt_type {
    /// Processor Hierarchy Node.
    pub const PROCESSOR: u8 = 0;
    /// Cache Type Structure.
    pub const CACHE: u8 = 1;
    /// ID Structure.
    pub const ID: u8 = 2;
}

/// Processor Hierarchy flags.
pub mod processor_flags {
    /// Node is a physical socket/package.
    pub const PHYSICAL_PACKAGE: u32 = 1 << 0;
    /// ACPI Processor ID is valid.
    pub const ACPI_PROCESSOR_ID_VALID: u32 = 1 << 1;
    /// Processor represents a thread (SMT / hyperthread).
    pub const PROCESSOR_IS_THREAD: u32 = 1 << 2;
    /// Node represents a core or node level.
    pub const NODE_IS_LEAF: u32 = 1 << 3;
    /// Sub-nodes share identical micro-architecture.
    pub const IDENTICAL_IMPLEMENTATION: u32 = 1 << 4;
}

/// Parsed PPTT table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PpttHeader {
    /// Total length of PPTT table.
    pub length: u32,
}

/// A parsed generic PPTT structure entry view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PpttEntry<'a> {
    /// Structure type (Processor, Cache, ID).
    pub structure_type: u8,
    /// Length of this structure in bytes.
    pub length: u8,
    /// Raw byte slice containing this structure.
    pub raw: &'a [u8],
}

/// Parsed Processor Hierarchy Node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProcessorHierarchyNode {
    /// Topology flags (physical package, thread, processor ID valid).
    pub flags: u32,
    /// Offset to parent node structure, or 0 if root.
    pub parent: u32,
    /// ACPI Processor ID (valid if bit 1 of flags is set).
    pub acpi_processor_id: u32,
    /// Number of private resource offsets (e.g. caches).
    pub private_resource_count: u32,
}

/// Parsed Cache Type Structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CacheTypeStructure {
    /// Cache validity flags.
    pub flags: u32,
    /// Offset to next level of cache structure, or 0 if last level.
    pub next_level_of_cache: u32,
    /// Size of cache in bytes.
    pub size: u32,
    /// Number of cache sets.
    pub number_of_sets: u32,
    /// Associativity (e.g. 8 for 8-way associative).
    pub associativity: u8,
    /// Cache attributes (allocation, write policy, cache type).
    pub attributes: u8,
    /// Cache line size in bytes.
    pub line_size: u16,
}

/// Parses the PPTT table header.
#[must_use]
pub fn parse_pptt_header(buffer: &[u8]) -> Option<PpttHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_PPTT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 36 || buffer.len() < 36 {
        return None;
    }

    Some(PpttHeader {
        length: header.length,
    })
}

/// An iterator over PPTT structures in a PPTT table.
pub struct PpttIter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    table_len: usize,
}

impl<'a> Iterator for PpttIter<'a> {
    type Item = PpttEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset >= self.table_len || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        if slice.len() < 2 {
            return None;
        }

        let structure_type = *slice.first()?;
        let length = *slice.get(1)?;

        if usize::from(length) < 2 || usize::from(length) > slice.len() {
            return None;
        }

        let entry_raw = &slice[..usize::from(length)];
        self.current_offset = self.current_offset.saturating_add(usize::from(length));

        Some(PpttEntry {
            structure_type,
            length,
            raw: entry_raw,
        })
    }
}

/// Returns an iterator over PPTT structures in a PPTT buffer.
#[must_use]
pub fn pptt_iter(buffer: &[u8]) -> Option<PpttIter<'_>> {
    let header = parse_pptt_header(buffer)?;
    let table_len = usize::try_from(header.length).ok()?;
    let bounded_len = table_len.min(buffer.len());

    Some(PpttIter {
        buffer,
        current_offset: 36,
        table_len: bounded_len,
    })
}

/// Parses a Processor Hierarchy Node from raw structure bytes.
#[must_use]
pub fn parse_processor_node(raw: &[u8]) -> Option<ProcessorHierarchyNode> {
    if raw.len() < 20 || *raw.first()? != pptt_type::PROCESSOR {
        return None;
    }

    let mut flags = [0_u8; 4];
    flags.copy_from_slice(raw.get(4..8)?);
    let mut parent = [0_u8; 4];
    parent.copy_from_slice(raw.get(8..12)?);
    let mut proc_id = [0_u8; 4];
    proc_id.copy_from_slice(raw.get(12..16)?);
    let mut res_cnt = [0_u8; 4];
    res_cnt.copy_from_slice(raw.get(16..20)?);

    Some(ProcessorHierarchyNode {
        flags: u32::from_le_bytes(flags),
        parent: u32::from_le_bytes(parent),
        acpi_processor_id: u32::from_le_bytes(proc_id),
        private_resource_count: u32::from_le_bytes(res_cnt),
    })
}

/// Parses a Cache Type Structure from raw structure bytes.
#[must_use]
pub fn parse_cache_node(raw: &[u8]) -> Option<CacheTypeStructure> {
    if raw.len() < 24 || *raw.first()? != pptt_type::CACHE {
        return None;
    }

    let mut flags = [0_u8; 4];
    flags.copy_from_slice(raw.get(4..8)?);
    let mut next_level = [0_u8; 4];
    next_level.copy_from_slice(raw.get(8..12)?);
    let mut size = [0_u8; 4];
    size.copy_from_slice(raw.get(12..16)?);
    let mut num_sets = [0_u8; 4];
    num_sets.copy_from_slice(raw.get(16..20)?);
    let associativity = *raw.get(20)?;
    let attributes = *raw.get(21)?;
    let line_size = u16::from_le_bytes([*raw.get(22)?, *raw.get(23)?]);

    Some(CacheTypeStructure {
        flags: u32::from_le_bytes(flags),
        next_level_of_cache: u32::from_le_bytes(next_level),
        size: u32::from_le_bytes(size),
        number_of_sets: u32::from_le_bytes(num_sets),
        associativity,
        attributes,
        line_size,
    })
}
