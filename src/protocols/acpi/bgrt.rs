//! # BGRT
//!
//! Boot Graphics Resource Table parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_BGRT;

/// Parsed BGRT data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BgrtInfo {
    /// The version.
    pub version: u16,
    /// The status.
    pub status: u8,
    /// The image type.
    pub image_type: u8,
    /// The image address.
    pub image_address: u64,
    /// The image offset.
    pub image_offset: u32,
}

/// Parses a BGRT buffer.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn parse_bgrt(buffer: &[u8]) -> Option<BgrtInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_BGRT {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 52 {
        return None;
    }
    let mut version_raw = [0_u8; 2];
    version_raw.copy_from_slice(buffer.get(36..38)?);
    let mut address_raw = [0_u8; 8];
    address_raw.copy_from_slice(buffer.get(40..48)?);
    let mut offset_raw = [0_u8; 4];
    offset_raw.copy_from_slice(buffer.get(48..52)?);
    Some(BgrtInfo {
        version: u16::from_le_bytes(version_raw),
        status: *buffer.get(38)?,
        image_type: *buffer.get(39)?,
        image_address: u64::from_le_bytes(address_raw),
        image_offset: u32::from_le_bytes(offset_raw),
    })
}
