//! # PRMT
//!
//! Platform Runtime Mechanism Table (PRMT) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_PRMT;

/// Parsed PRMT table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrmtHeader {
    /// Platform parameter buffer physical address.
    pub platform_param_address: u64,
    /// Number of runtime modules in this table.
    pub module_count: u32,
    /// Offset to first module structure.
    pub module_offset: u32,
}

/// A parsed PRM Module structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrmModule<'a> {
    /// Structure length in bytes.
    pub length: u16,
    /// Structure revision.
    pub revision: u8,
    /// Unique 128-bit module GUID.
    pub module_guid: [u8; 16],
    /// Major version.
    pub major_version: u16,
    /// Minor version.
    pub minor_version: u16,
    /// Number of PRM handlers in this module.
    pub handler_count: u16,
    /// Offset to handler structures within this module structure.
    pub handler_offset: u16,
    /// MMIO Base address allocated to module.
    pub mmio_base: u64,
    /// MMIO Length allocated to module.
    pub mmio_length: u64,
    /// Raw byte slice containing this module structure.
    pub raw: &'a [u8],
}

/// A parsed PRM Handler structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrmHandler {
    /// Handler structure length.
    pub length: u16,
    /// Handler structure revision.
    pub revision: u8,
    /// Unique 128-bit handler GUID.
    pub handler_guid: [u8; 16],
    /// Physical address of handler entry point.
    pub physical_address: u64,
    /// Static data buffer physical address.
    pub static_data_buffer_address: u64,
    /// ACPI parameter buffer physical address.
    pub acpi_param_buffer_address: u64,
}

/// Parses the PRMT table header.
#[must_use]
pub fn parse_prmt_header(buffer: &[u8]) -> Option<PrmtHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_PRMT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 52 || buffer.len() < 52 {
        return None;
    }

    let mut param_addr = [0_u8; 8];
    param_addr.copy_from_slice(buffer.get(36..44)?);
    let mut mod_count = [0_u8; 4];
    mod_count.copy_from_slice(buffer.get(44..48)?);
    let mut mod_offset = [0_u8; 4];
    mod_offset.copy_from_slice(buffer.get(48..52)?);

    Some(PrmtHeader {
        platform_param_address: u64::from_le_bytes(param_addr),
        module_count: u32::from_le_bytes(mod_count),
        module_offset: u32::from_le_bytes(mod_offset),
    })
}

/// An iterator over PRM modules in a PRMT table.
pub struct PrmModuleIter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    remaining: u32,
}

impl<'a> Iterator for PrmModuleIter<'a> {
    type Item = PrmModule<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        if slice.len() < 44 {
            return None;
        }

        let length = u16::from_le_bytes([*slice.first()?, *slice.get(1)?]);
        if usize::from(length) < 44 || usize::from(length) > slice.len() {
            return None;
        }

        let revision = *slice.get(2)?;
        let mut module_guid = [0_u8; 16];
        module_guid.copy_from_slice(slice.get(4..20)?);

        let major_version = u16::from_le_bytes([*slice.get(20)?, *slice.get(21)?]);
        let minor_version = u16::from_le_bytes([*slice.get(22)?, *slice.get(23)?]);
        let handler_count = u16::from_le_bytes([*slice.get(24)?, *slice.get(25)?]);
        let handler_offset = u16::from_le_bytes([*slice.get(26)?, *slice.get(27)?]);

        let mut mmio_base_raw = [0_u8; 8];
        mmio_base_raw.copy_from_slice(slice.get(28..36)?);
        let mut mmio_len_raw = [0_u8; 8];
        mmio_len_raw.copy_from_slice(slice.get(36..44)?);

        let module_raw = &slice[..usize::from(length)];

        self.current_offset = self.current_offset.saturating_add(usize::from(length));
        self.remaining = self.remaining.saturating_sub(1);

        Some(PrmModule {
            length,
            revision,
            module_guid,
            major_version,
            minor_version,
            handler_count,
            handler_offset,
            mmio_base: u64::from_le_bytes(mmio_base_raw),
            mmio_length: u64::from_le_bytes(mmio_len_raw),
            raw: module_raw,
        })
    }
}

/// Returns an iterator over PRM modules in a PRMT buffer.
#[must_use]
pub fn prm_module_iter(buffer: &[u8]) -> Option<PrmModuleIter<'_>> {
    let header = parse_prmt_header(buffer)?;
    let start_offset = usize::try_from(header.module_offset).ok()?;
    if start_offset > buffer.len() {
        return None;
    }

    Some(PrmModuleIter {
        buffer,
        current_offset: start_offset,
        remaining: header.module_count,
    })
}
