//! # HEST
//!
//! Hardware Error Source Table (HEST) parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_HEST;

/// HEST Error Source Types.
pub mod error_source_type {
    /// IA-32 / AMD64 Machine Check Exception.
    pub const IA32_MCE: u16 = 0;
    /// IA-32 / AMD64 Corrected Machine Check.
    pub const IA32_CMC: u16 = 1;
    /// IA-32 / AMD64 Non-Maskable Interrupt.
    pub const IA32_NMI: u16 = 2;
    /// PCI Express Root Port AER.
    pub const PCIE_ROOT_AER: u16 = 6;
    /// PCI Express Device AER.
    pub const PCIE_DEVICE_AER: u16 = 7;
    /// PCI Express / PCI-X Bridge AER.
    pub const PCIE_BRIDGE_AER: u16 = 8;
    /// Generic Hardware Error Source version 1.
    pub const GENERIC_HARDWARE_ERROR: u16 = 9;
    /// Generic Hardware Error Source version 2.
    pub const GENERIC_HARDWARE_ERROR_V2: u16 = 10;
    /// IA-32 Architecture Deferred Machine Check.
    pub const IA32_DEFERRED_MCE: u16 = 11;
}

/// Parsed HEST table header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HestHeader {
    /// Total number of error sources declared in this table.
    pub error_source_count: u32,
}

/// A generic HEST error source entry view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HestEntry<'a> {
    /// Error source type identifier.
    pub source_type: u16,
    /// Error source identifier.
    pub source_id: u16,
    /// Raw slice containing this error source structure.
    pub raw: &'a [u8],
}

/// Parses the HEST table header.
#[must_use]
pub fn parse_hest_header(buffer: &[u8]) -> Option<HestHeader> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_HEST {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 40 || buffer.len() < 40 {
        return None;
    }

    let mut count_raw = [0_u8; 4];
    count_raw.copy_from_slice(buffer.get(36..40)?);

    Some(HestHeader {
        error_source_count: u32::from_le_bytes(count_raw),
    })
}

/// Helper to determine structure length for a given HEST entry.
fn hest_entry_length(slice: &[u8]) -> Option<usize> {
    if slice.len() < 4 {
        return None;
    }
    let entry_type = u16::from_le_bytes([slice[0], slice[1]]);
    match entry_type {
        error_source_type::IA32_MCE => {
            // Header (4) + Flags(1) + Reserved(1) + NumBankProps(1) + MCBankProps(NumBankProps * 24)
            let bank_count = usize::from(*slice.get(6)?);
            Some(7 + bank_count * 24)
        }
        error_source_type::IA32_CMC => {
            // Header (4) + Flags(1) + Enabled(1) + NumRecords(4) + MaxSections(4) + Notify(28) + NumBanks(1) + MCBankProps
            if slice.len() < 43 {
                return None;
            }
            let bank_count = usize::from(*slice.get(42)?);
            Some(43 + bank_count * 24)
        }
        error_source_type::IA32_NMI => Some(4 + 1 + 3 + 4 + 4 + 4), // 20 bytes
        error_source_type::PCIE_ROOT_AER | error_source_type::PCIE_DEVICE_AER => {
            Some(4 + 1 + 1 + 2 + 4 + 4 + 4 + 4 + 4 + 4 + 4)
        }
        error_source_type::PCIE_BRIDGE_AER => Some(4 + 1 + 1 + 2 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4),
        error_source_type::GENERIC_HARDWARE_ERROR => {
            // Header (4) + RelatedSourceId(2) + Reserved(1) + Enabled(1) + NumRecords(4) + MaxSections(4) + MaxRawLen(4) + ErrorStatusAddress(12) + Notify(28) + ErrorStatusBlockLength(4)
            Some(64)
        }
        error_source_type::GENERIC_HARDWARE_ERROR_V2 => {
            // GHES v1 (64) + ReadAckRegister(12) + ReadAckPreserve(8) + ReadAckWrite(8)
            Some(92)
        }
        error_source_type::IA32_DEFERRED_MCE => {
            if slice.len() < 43 {
                return None;
            }
            let bank_count = usize::from(*slice.get(42)?);
            Some(43 + bank_count * 24)
        }
        _ => None,
    }
}

/// An iterator over error source structures in a HEST table.
pub struct HestIter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    remaining: u32,
}

impl<'a> Iterator for HestIter<'a> {
    type Item = HestEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        let length = hest_entry_length(slice)?;
        if length > slice.len() {
            return None;
        }

        let source_type = u16::from_le_bytes([*slice.first()?, *slice.get(1)?]);
        let source_id = u16::from_le_bytes([*slice.get(2)?, *slice.get(3)?]);

        let entry_raw = &slice[..length];
        self.current_offset = self.current_offset.saturating_add(length);
        self.remaining = self.remaining.saturating_sub(1);

        Some(HestEntry {
            source_type,
            source_id,
            raw: entry_raw,
        })
    }
}

/// Returns an iterator over error source descriptors in a HEST buffer.
#[must_use]
pub fn hest_iter(buffer: &[u8]) -> Option<HestIter<'_>> {
    let header = parse_hest_header(buffer)?;
    Some(HestIter {
        buffer,
        current_offset: 40,
        remaining: header.error_source_count,
    })
}
