//! # GTDT
//!
//! Generic Timer Description Table parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_GTDT;

/// Parsed GTDT data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GtdtInfo {
    /// The counter block count.
    pub counter_block_count: u32,
    /// The compare timer count.
    pub compare_timer_count: u32,
    /// The physical timer base.
    pub phys_base: u64,
    /// The secure timer base.
    pub secure_base: u64,
    /// The PPI.
    pub ppi: u8,
    /// The flags.
    pub flags: u8,
}

/// Parses a GTDT buffer.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn parse_gtdt(buffer: &[u8]) -> Option<GtdtInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_GTDT {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 62 {
        return None;
    }
    let mut counter_raw = [0_u8; 4];
    counter_raw.copy_from_slice(buffer.get(36..40)?);
    let mut compare_raw = [0_u8; 4];
    compare_raw.copy_from_slice(buffer.get(40..44)?);
    let mut phys_raw = [0_u8; 8];
    phys_raw.copy_from_slice(buffer.get(44..52)?);
    let mut secure_raw = [0_u8; 8];
    secure_raw.copy_from_slice(buffer.get(52..60)?);
    Some(GtdtInfo {
        counter_block_count: u32::from_le_bytes(counter_raw),
        compare_timer_count: u32::from_le_bytes(compare_raw),
        phys_base: u64::from_le_bytes(phys_raw),
        secure_base: u64::from_le_bytes(secure_raw),
        ppi: *buffer.get(60)?,
        flags: *buffer.get(61)?,
    })
}
