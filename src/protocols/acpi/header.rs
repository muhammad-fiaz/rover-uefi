//! # ACPI header
//!
//! The 36-byte system description header.

use crate::protocols::acpi::checksum::acpi_checksum_ok;

/// A parsed ACPI header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AcpiHeader {
    /// The signature.
    pub signature: [u8; 4],
    /// The table length.
    pub length: u32,
    /// The revision.
    pub revision: u8,
    /// The OEM identifier.
    pub oem_id: [u8; 6],
    /// The OEM table identifier.
    pub oem_table_id: [u8; 8],
}

/// Parses a header.
#[must_use]
pub fn parse_header(buffer: &[u8]) -> Option<AcpiHeader> {
    if buffer.len() < 36 {
        return None;
    }
    let mut length_raw = [0_u8; 4];
    length_raw.copy_from_slice(buffer.get(4..8)?);
    let length = u32::from_le_bytes(length_raw);
    if (length as usize) > buffer.len() || length < 36 {
        return None;
    }
    if !acpi_checksum_ok(&buffer[..(length as usize)]) {
        return None;
    }
    let mut signature = [0_u8; 4];
    signature.copy_from_slice(&buffer[0..4]);
    let mut oem_id = [0_u8; 6];
    oem_id.copy_from_slice(buffer.get(10..16)?);
    let mut oem_table_id = [0_u8; 8];
    oem_table_id.copy_from_slice(buffer.get(16..24)?);
    Some(AcpiHeader {
        signature,
        length,
        revision: *buffer.get(8)?,
        oem_id,
        oem_table_id,
    })
}

/// Returns whether the signature matches.
#[must_use]
pub fn signature_matches(header: &AcpiHeader, expected: [u8; 4]) -> bool {
    header.signature == expected
}
