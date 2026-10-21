//! # RSDP
//!
//! RSDP parsing.
//!
//! The first 20 bytes are validated
//! independently. Extended fields are read
//! only when the revision and length allow.

use crate::protocols::acpi::checksum::acpi_checksum_ok;

/// The RSDP signature.
pub const RSDP_SIGNATURE: [u8; 8] = *b"RSD PTR ";

/// Parsed RSDP information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RsdpInfo {
    /// The OEM identifier.
    pub oem_id: [u8; 6],
    /// The revision.
    pub revision: u8,
    /// The RSDT address.
    pub rsdt_address: u64,
    /// The XSDT address.
    pub xsdt_address: Option<u64>,
    /// The extended length.
    pub length: Option<u32>,
}

/// Parses an RSDP buffer.
#[must_use]
pub fn parse_rsdp(buffer: &[u8]) -> Option<RsdpInfo> {
    if buffer.len() < 20 {
        return None;
    }
    if buffer[0..8] != RSDP_SIGNATURE {
        return None;
    }
    if !acpi_checksum_ok(&buffer[0..20]) {
        return None;
    }
    let mut oem_id = [0_u8; 6];
    oem_id.copy_from_slice(&buffer[9..15]);
    let revision = *buffer.get(15)?;
    let mut rsdt_raw = [0_u8; 4];
    rsdt_raw.copy_from_slice(buffer.get(16..20)?);
    let rsdt_address = u64::from(u32::from_le_bytes(rsdt_raw));
    if revision < 2 {
        return Some(RsdpInfo {
            oem_id,
            revision,
            rsdt_address,
            xsdt_address: None,
            length: None,
        });
    }
    if buffer.len() < 36 {
        return None;
    }
    let mut length_raw = [0_u8; 4];
    length_raw.copy_from_slice(buffer.get(20..24)?);
    let length = u32::from_le_bytes(length_raw);
    if (length as usize) > buffer.len() || length < 36 {
        return None;
    }
    if !acpi_checksum_ok(&buffer[..(length as usize)]) {
        return None;
    }
    let mut xsdt_raw = [0_u8; 8];
    xsdt_raw.copy_from_slice(buffer.get(24..32)?);
    Some(RsdpInfo {
        oem_id,
        revision,
        rsdt_address,
        xsdt_address: Some(u64::from_le_bytes(xsdt_raw)),
        length: Some(length),
    })
}
