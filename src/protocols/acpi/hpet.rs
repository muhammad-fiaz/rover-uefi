//! # HPET
//!
//! High Precision Event Timer Table parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_HPET;

/// Parsed HPET data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HpetInfo {
    /// The base address.
    pub base_address: u64,
    /// The event timer block identifier.
    pub event_timer_block_id: u16,
    /// The main counter minimum.
    pub main_counter_min: u16,
    /// The sequence number.
    pub sequence: u16,
}

/// Parses an HPET buffer.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn parse_hpet(buffer: &[u8]) -> Option<HpetInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_HPET {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 54 {
        return None;
    }
    let mut base_raw = [0_u8; 8];
    base_raw.copy_from_slice(buffer.get(40..48)?);
    let mut block_raw = [0_u8; 2];
    block_raw.copy_from_slice(buffer.get(48..50)?);
    let event_timer_block_id = u16::from_le_bytes(block_raw);
    block_raw.copy_from_slice(buffer.get(50..52)?);
    let main_counter_min = u16::from_le_bytes(block_raw);
    block_raw.copy_from_slice(buffer.get(52..54)?);
    let sequence = u16::from_le_bytes(block_raw);
    Some(HpetInfo {
        base_address: u64::from_le_bytes(base_raw),
        event_timer_block_id,
        main_counter_min,
        sequence,
    })
}
