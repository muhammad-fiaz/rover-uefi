//! # CEDT
//!
//! Compute Express Link (CXL) Early Discovery Table (CEDT) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_CEDT;

/// CEDT structure types.
pub mod cedt_type {
    /// CXL Host Bridge Structure (CHBS).
    pub const CHBS: u8 = 0;
    /// CXL Fixed Memory Window Structure (CFMWS).
    pub const CFMWS: u8 = 1;
    /// CXL XOR Interleave Math Structure (CXIMS).
    pub const CXIMS: u8 = 2;
    /// CXL Root Port Component Register Layout Structure (RDPAS).
    pub const RDPAS: u8 = 3;
}

/// Parsed CEDT table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CedtHeader {
    /// Total length of CEDT table.
    pub length: u32,
}

/// A parsed generic CEDT structure entry view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CedtEntry<'a> {
    /// CEDT structure type (CHBS, CFMWS, etc.).
    pub structure_type: u8,
    /// Structure length in bytes.
    pub length: u16,
    /// Raw slice containing this structure.
    pub raw: &'a [u8],
}

/// Parsed CXL Host Bridge Structure (CHBS).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CxlHostBridgeStructure {
    /// ACPI UID of the host bridge.
    pub uid: u32,
    /// CXL version supported.
    pub cxl_version: u32,
    /// Component register base physical address.
    pub component_reg_base: u64,
}

/// Parsed CXL Fixed Memory Window Structure (CFMWS).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CxlFixedMemoryWindowStructure {
    /// Base Host Physical Address.
    pub base_hpa: u64,
    /// Window size in bytes.
    pub window_size: u64,
    /// Interleave ways count.
    pub interleave_ways: u8,
    /// Interleave arithmetic algorithm.
    pub interleave_arithmetic: u8,
    /// Granularity in bytes.
    pub granularity: u32,
    /// Window restrictions / memory types (e.g. volatile, persistent).
    pub restrictions: u16,
}

/// Parses the CEDT table header.
#[must_use]
pub fn parse_cedt_header(buffer: &[u8]) -> Option<CedtHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_CEDT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 36 || buffer.len() < 36 {
        return None;
    }

    Some(CedtHeader {
        length: header.length,
    })
}

/// An iterator over structures in a CEDT table.
pub struct CedtIter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    table_len: usize,
}

impl<'a> Iterator for CedtIter<'a> {
    type Item = CedtEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset >= self.table_len || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        if slice.len() < 4 {
            return None;
        }

        let structure_type = *slice.first()?;
        let length = u16::from_le_bytes([*slice.get(2)?, *slice.get(3)?]);

        if usize::from(length) < 4 || usize::from(length) > slice.len() {
            return None;
        }

        let entry_raw = &slice[..usize::from(length)];
        self.current_offset = self.current_offset.saturating_add(usize::from(length));

        Some(CedtEntry {
            structure_type,
            length,
            raw: entry_raw,
        })
    }
}

/// Returns an iterator over CEDT structures in a CEDT buffer.
#[must_use]
pub fn cedt_iter(buffer: &[u8]) -> Option<CedtIter<'_>> {
    let header = parse_cedt_header(buffer)?;
    let table_len = usize::try_from(header.length).ok()?;
    let bounded_len = table_len.min(buffer.len());

    Some(CedtIter {
        buffer,
        current_offset: 36,
        table_len: bounded_len,
    })
}

/// Parses a CXL Host Bridge Structure (CHBS) from raw entry bytes.
#[must_use]
pub fn parse_chbs(raw: &[u8]) -> Option<CxlHostBridgeStructure> {
    if raw.len() < 24 || *raw.first()? != cedt_type::CHBS {
        return None;
    }

    let mut uid = [0_u8; 4];
    uid.copy_from_slice(raw.get(4..8)?);
    let mut cxl_ver = [0_u8; 4];
    cxl_ver.copy_from_slice(raw.get(8..12)?);
    let mut base_reg = [0_u8; 8];
    base_reg.copy_from_slice(raw.get(16..24)?);

    Some(CxlHostBridgeStructure {
        uid: u32::from_le_bytes(uid),
        cxl_version: u32::from_le_bytes(cxl_ver),
        component_reg_base: u64::from_le_bytes(base_reg),
    })
}

/// Parses a CXL Fixed Memory Window Structure (CFMWS) from raw entry bytes.
#[must_use]
pub fn parse_cfmws(raw: &[u8]) -> Option<CxlFixedMemoryWindowStructure> {
    if raw.len() < 36 || *raw.first()? != cedt_type::CFMWS {
        return None;
    }

    let mut base_hpa = [0_u8; 8];
    base_hpa.copy_from_slice(raw.get(8..16)?);
    let mut win_size = [0_u8; 8];
    win_size.copy_from_slice(raw.get(16..24)?);
    let interleave_ways = *raw.get(24)?;
    let interleave_arithmetic = *raw.get(25)?;
    let mut gran = [0_u8; 4];
    gran.copy_from_slice(raw.get(28..32)?);
    let restr = u16::from_le_bytes([*raw.get(32)?, *raw.get(33)?]);

    Some(CxlFixedMemoryWindowStructure {
        base_hpa: u64::from_le_bytes(base_hpa),
        window_size: u64::from_le_bytes(win_size),
        interleave_ways,
        interleave_arithmetic,
        granularity: u32::from_le_bytes(gran),
        restrictions: restr,
    })
}
