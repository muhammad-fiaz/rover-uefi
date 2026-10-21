//! # SMBIOS entry points
//!
//! Parsing of 2.x and 3.x entry points.

/// The 2.x anchor.
pub const ANCHOR_2X: [u8; 4] = *b"_SM_";

/// The 3.x anchor.
pub const ANCHOR_3X: [u8; 5] = *b"_SM3_";

/// Parsed 2.x entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Smbios2EntryPoint {
    /// The entry point length.
    pub entry_point_length: u8,
    /// The major version.
    pub major_version: u8,
    /// The minor version.
    pub minor_version: u8,
    /// The table length.
    pub table_length: u16,
    /// The table address.
    pub table_address: u32,
    /// The structure count.
    pub number_of_structures: u16,
}

/// Parsed 3.x entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Smbios3EntryPoint {
    /// The entry point length.
    pub entry_point_length: u8,
    /// The major version.
    pub major_version: u8,
    /// The minor version.
    pub minor_version: u8,
    /// The maximum structure size.
    pub maximum_structure_size: u32,
    /// The maximum table size.
    pub maximum_table_size: u32,
    /// The table address.
    pub table_address: u64,
}

/// Returns whether the buffer checksum is valid.
#[must_use]
pub fn checksum_ok(data: &[u8]) -> bool {
    data.iter().fold(0_u8, |acc, byte| acc.wrapping_add(*byte)) == 0
}

/// Parses a 2.x entry point.
#[must_use]
pub fn parse_2x(buffer: &[u8]) -> Option<Smbios2EntryPoint> {
    if buffer.len() < 31 {
        return None;
    }
    if buffer[0..4] != ANCHOR_2X {
        return None;
    }
    if !checksum_ok(&buffer[..31]) {
        return None;
    }
    let mut addr = [0_u8; 4];
    addr.copy_from_slice(buffer.get(24..28)?);
    let mut length = [0_u8; 2];
    length.copy_from_slice(buffer.get(22..24)?);
    let mut count = [0_u8; 2];
    count.copy_from_slice(buffer.get(28..30)?);
    Some(Smbios2EntryPoint {
        entry_point_length: *buffer.get(6)?,
        major_version: *buffer.get(7)?,
        minor_version: *buffer.get(8)?,
        table_length: u16::from_le_bytes(length),
        table_address: u32::from_le_bytes(addr),
        number_of_structures: u16::from_le_bytes(count),
    })
}

/// Parses a 3.x entry point.
#[must_use]
pub fn parse_3x(buffer: &[u8]) -> Option<Smbios3EntryPoint> {
    if buffer.len() < 24 {
        return None;
    }
    if buffer[0..5] != ANCHOR_3X {
        return None;
    }
    if !checksum_ok(&buffer[..24]) {
        return None;
    }
    let mut max_size = [0_u8; 4];
    max_size.copy_from_slice(buffer.get(12..16)?);
    let mut table_size = [0_u8; 4];
    table_size.copy_from_slice(buffer.get(16..20)?);
    // Table address is 8 bytes at offset 20 for 3.x layouts,
    // after anchor, checksum, length, versions, maximum
    // structure size and maximum table size.
    let mut addr8 = [0_u8; 8];
    addr8.copy_from_slice(buffer.get(20..28)?);
    Some(Smbios3EntryPoint {
        entry_point_length: *buffer.get(6)?,
        major_version: *buffer.get(7)?,
        minor_version: *buffer.get(8)?,
        maximum_structure_size: u32::from_le_bytes(max_size),
        maximum_table_size: u32::from_le_bytes(table_size),
        table_address: u64::from_le_bytes(addr8),
    })
}
