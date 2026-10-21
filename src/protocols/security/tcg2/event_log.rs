//! # Event log parsing
//!
//! Bounded parsing for firmware-provided event logs.
//!
//! Two wire formats are supported. The legacy format is a
//! 32-byte header: PCR index (4), event type (4), SHA-1 digest
//! (20) and event size (4), followed by the event payload. The
//! crypto-agile format is a PCR index (4), event type (4),
//! a digest list (4-byte count followed by algorithm identifier
//! and digest for each entry) and an event size (4), followed
//! by the payload.
//!
//! Parsing operates on caller-provided byte slices only. Unknown
//! algorithm identifiers are preserved in the parsed view, but
//! their digest length must be known to advance safely; an
//! unknown algorithm makes the entry unparsable and iteration
//! stops. Parsing an event log does not verify a measured-boot
//! chain.

use crate::base::bytes::{read_u16_le, read_u32_le};

use super::pcr::digest_size_for_algorithm;

/// A single digest in a crypto-agile digest list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DigestEntry<'a> {
    /// The trusted platform module algorithm identifier.
    pub algorithm: u16,
    /// The digest bytes.
    pub digest: &'a [u8],
}

/// A parsed legacy event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LegacyEvent<'a> {
    /// The PCR index.
    pub pcr_index: u32,
    /// The event type.
    pub event_type: u32,
    /// The SHA-1 digest bytes.
    pub digest: &'a [u8],
    /// The event payload.
    pub event: &'a [u8],
    /// The total entry size in bytes.
    pub total_size: usize,
}

/// A parsed crypto-agile event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AgileEvent<'a> {
    /// The PCR index.
    pub pcr_index: u32,
    /// The event type.
    pub event_type: u32,
    /// The digest list bytes consumed by the header. The count
    /// is included.
    pub digest_count: u32,
    /// The event payload.
    pub event: &'a [u8],
    /// The total entry size in bytes.
    pub total_size: usize,
    /// The offset of the event payload within the input.
    pub event_offset: usize,
}

/// Parses one legacy event at the start of the input.
///
/// Returns `None` for truncated input, an invalid PCR index, or
/// an event size that exceeds the buffer.
#[must_use]
pub fn parse_legacy_event(bytes: &[u8]) -> Option<LegacyEvent<'_>> {
    if bytes.len() < 32 {
        return None;
    }
    let pcr_index = read_u32_le(bytes, 0)?;
    if pcr_index > 23 {
        return None;
    }
    let event_type = read_u32_le(bytes, 4)?;
    let event_size = read_u32_le(bytes, 28)? as usize;
    let total_size = 32_usize.checked_add(event_size)?;
    if total_size > bytes.len() {
        return None;
    }
    let digest = bytes.get(8..28)?;
    let event = bytes.get(32..total_size)?;
    Some(LegacyEvent {
        pcr_index,
        event_type,
        digest,
        event,
        total_size,
    })
}

/// Parses one crypto-agile event at the start of the input.
///
/// Returns `None` for truncated input, an invalid PCR index,
/// an unknown algorithm identifier, a digest length mismatch,
/// or an event size that exceeds the buffer.
#[must_use]
pub fn parse_agile_event(bytes: &[u8]) -> Option<AgileEvent<'_>> {
    if bytes.len() < 12 {
        return None;
    }
    let pcr_index = read_u32_le(bytes, 0)?;
    if pcr_index > 23 {
        return None;
    }
    let event_type = read_u32_le(bytes, 4)?;
    let digest_count = read_u32_le(bytes, 8)?;
    if digest_count == 0 || digest_count > 16 {
        return None;
    }
    let mut cursor = 12_usize;
    for _ in 0..digest_count {
        let algorithm = read_u16_le(bytes, cursor)?;
        let size = digest_size_for_algorithm(algorithm)?;
        cursor = cursor.checked_add(2)?.checked_add(size)?;
        if cursor > bytes.len() {
            return None;
        }
    }
    let event_size = read_u32_le(bytes, cursor)? as usize;
    let payload_start = cursor.checked_add(4)?;
    let total_size = payload_start.checked_add(event_size)?;
    if total_size > bytes.len() {
        return None;
    }
    let event = bytes.get(payload_start..total_size)?;
    Some(AgileEvent {
        pcr_index,
        event_type,
        digest_count,
        event,
        total_size,
        event_offset: payload_start,
    })
}

/// Iterates legacy events.
pub struct LegacyEventIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for LegacyEventIter<'a> {
    type Item = LegacyEvent<'a>;

    /// Returns the next event.
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.bytes.get(self.offset..)?;
        let parsed = parse_legacy_event(remaining)?;
        self.offset = self.offset.checked_add(parsed.total_size)?;
        Some(parsed)
    }
}

/// Creates a legacy event iterator.
#[must_use]
pub fn legacy_event_iter(bytes: &[u8]) -> LegacyEventIter<'_> {
    LegacyEventIter { bytes, offset: 0 }
}

/// Iterates crypto-agile events.
pub struct AgileEventIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for AgileEventIter<'a> {
    type Item = AgileEvent<'a>;

    /// Returns the next event.
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.bytes.get(self.offset..)?;
        let parsed = parse_agile_event(remaining)?;
        self.offset = self.offset.checked_add(parsed.total_size)?;
        Some(parsed)
    }
}

/// Creates a crypto-agile event iterator.
#[must_use]
pub fn agile_event_iter(bytes: &[u8]) -> AgileEventIter<'_> {
    AgileEventIter { bytes, offset: 0 }
}

/// Returns whether a byte slice is a complete legacy log.
#[must_use]
pub fn is_valid_legacy_log(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    let mut offset = 0_usize;
    while offset < bytes.len() {
        let Some(remaining) = bytes.get(offset..) else {
            return false;
        };
        let Some(parsed) = parse_legacy_event(remaining) else {
            return false;
        };
        let Some(next) = offset.checked_add(parsed.total_size) else {
            return false;
        };
        offset = next;
    }
    offset == bytes.len()
}
