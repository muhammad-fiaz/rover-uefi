//! # SPCR
//!
//! Serial Port Console Redirection Table parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_SPCR;

/// Parsed SPCR data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpcrInfo {
    /// The interface type.
    pub interface_type: u8,
    /// The specification revision.
    pub spec_revision: u8,
    /// The interrupt type.
    pub interrupt_type: u8,
    /// The interrupt number.
    pub interrupt_number: u8,
    /// The port type.
    pub port_type: u8,
    /// The port address.
    pub port_address: u64,
    /// The baud rate.
    pub baud_rate: u16,
    /// The parity.
    pub parity: u8,
    /// The stop bits.
    pub stop_bits: u8,
}

/// Parses an SPCR buffer.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
#[must_use]
pub fn parse_spcr(buffer: &[u8]) -> Option<SpcrInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_SPCR {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    if table_length < 58 {
        return None;
    }
    let mut port_raw = [0_u8; 8];
    port_raw.copy_from_slice(buffer.get(46..54)?);
    let mut baud_raw = [0_u8; 2];
    baud_raw.copy_from_slice(buffer.get(54..56)?);
    Some(SpcrInfo {
        interface_type: *buffer.get(36)?,
        spec_revision: *buffer.get(37)?,
        interrupt_type: *buffer.get(39)?,
        interrupt_number: *buffer.get(40)?,
        port_type: *buffer.get(45)?,
        port_address: u64::from_le_bytes(port_raw),
        baud_rate: u16::from_le_bytes(baud_raw),
        parity: *buffer.get(56)?,
        stop_bits: *buffer.get(57)?,
    })
}
