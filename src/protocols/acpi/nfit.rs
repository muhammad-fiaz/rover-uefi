//! # NFIT
//!
//! NVDIMM Firmware Interface Table (NFIT) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_NFIT;

/// NFIT Sub-structure Types.
pub mod nfit_type {
    /// System Physical Address (SPA) Range Structure.
    pub const SPA_RANGE: u16 = 0;
    /// NVDIMM Region Mapping Structure.
    pub const REGION_MAPPING: u16 = 1;
    /// Interleave Structure.
    pub const INTERLEAVE: u16 = 2;
    /// SMBIOS Management Information Structure.
    pub const SMBIOS_MGMT_INFO: u16 = 3;
    /// NVDIMM Control Region Structure.
    pub const CONTROL_REGION: u16 = 4;
    /// NVDIMM Block Data Window Region Structure.
    pub const BLOCK_DATA_WINDOW: u16 = 5;
    /// Flush Hint Address Structure.
    pub const FLUSH_HINT: u16 = 6;
    /// Platform Capabilities Structure.
    pub const PLATFORM_CAPABILITIES: u16 = 7;
}

/// Parsed NFIT table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NfitHeader {
    /// Length of the NFIT table.
    pub length: u32,
}

/// A parsed generic NFIT Sub-structure entry view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NfitEntry<'a> {
    /// Sub-structure type.
    pub structure_type: u16,
    /// Length of this sub-structure in bytes.
    pub length: u16,
    /// Raw byte slice containing this sub-structure.
    pub raw: &'a [u8],
}

/// Parsed System Physical Address (SPA) Range Structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpaRangeStructure {
    /// Range index.
    pub range_index: u16,
    /// Flags.
    pub flags: u16,
    /// Proximity domain.
    pub proximity_domain: u32,
    /// Address range type GUID.
    pub address_type_guid: [u8; 16],
    /// SPA range base physical address.
    pub spa_base: u64,
    /// SPA range length in bytes.
    pub spa_length: u64,
    /// Memory mapping attributes.
    pub address_attr: u64,
}

/// Parses the NFIT table header.
#[must_use]
pub fn parse_nfit_header(buffer: &[u8]) -> Option<NfitHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_NFIT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 40 || buffer.len() < 40 {
        return None;
    }

    Some(NfitHeader {
        length: header.length,
    })
}

/// An iterator over sub-structures in an NFIT table.
pub struct NfitIter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    table_len: usize,
}

impl<'a> Iterator for NfitIter<'a> {
    type Item = NfitEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset >= self.table_len || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        if slice.len() < 4 {
            return None;
        }

        let structure_type = u16::from_le_bytes([*slice.first()?, *slice.get(1)?]);
        let length = u16::from_le_bytes([*slice.get(2)?, *slice.get(3)?]);

        if usize::from(length) < 4 || usize::from(length) > slice.len() {
            return None;
        }

        let entry_raw = &slice[..usize::from(length)];
        self.current_offset = self.current_offset.saturating_add(usize::from(length));

        Some(NfitEntry {
            structure_type,
            length,
            raw: entry_raw,
        })
    }
}

/// Returns an iterator over sub-structures in an NFIT buffer.
#[must_use]
pub fn nfit_iter(buffer: &[u8]) -> Option<NfitIter<'_>> {
    let header = parse_nfit_header(buffer)?;
    let table_len = usize::try_from(header.length).ok()?;
    let bounded_len = table_len.min(buffer.len());

    Some(NfitIter {
        buffer,
        current_offset: 40,
        table_len: bounded_len,
    })
}

/// Parses a System Physical Address (SPA) Range Structure from raw entry bytes.
#[must_use]
pub fn parse_spa_range(raw: &[u8]) -> Option<SpaRangeStructure> {
    if raw.len() < 56 {
        return None;
    }
    let structure_type = u16::from_le_bytes([*raw.first()?, *raw.get(1)?]);
    if structure_type != nfit_type::SPA_RANGE {
        return None;
    }

    let range_index = u16::from_le_bytes([*raw.get(4)?, *raw.get(5)?]);
    let flags = u16::from_le_bytes([*raw.get(6)?, *raw.get(7)?]);
    let proximity_domain =
        u32::from_le_bytes([*raw.get(12)?, *raw.get(13)?, *raw.get(14)?, *raw.get(15)?]);

    let mut guid = [0_u8; 16];
    guid.copy_from_slice(raw.get(16..32)?);

    let mut spa_base = [0_u8; 8];
    spa_base.copy_from_slice(raw.get(32..40)?);
    let mut spa_len = [0_u8; 8];
    spa_len.copy_from_slice(raw.get(40..48)?);
    let mut attr = [0_u8; 8];
    attr.copy_from_slice(raw.get(48..56)?);

    Some(SpaRangeStructure {
        range_index,
        flags,
        proximity_domain,
        address_type_guid: guid,
        spa_base: u64::from_le_bytes(spa_base),
        spa_length: u64::from_le_bytes(spa_len),
        address_attr: u64::from_le_bytes(attr),
    })
}
