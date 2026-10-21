//! # SMBIOS structures
//!
//! The common structure header.

/// A structure header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SmbiosHeader {
    /// The structure type.
    pub structure_type: u8,
    /// The formatted length.
    pub length: u8,
    /// The handle.
    pub handle: u16,
}

/// Parses a structure header.
#[must_use]
pub fn parse_header(buffer: &[u8]) -> Option<SmbiosHeader> {
    if buffer.len() < 4 {
        return None;
    }
    let mut handle = [0_u8; 2];
    handle.copy_from_slice(buffer.get(2..4)?);
    Some(SmbiosHeader {
        structure_type: *buffer.first()?,
        length: *buffer.get(1)?,
        handle: u16::from_le_bytes(handle),
    })
}
