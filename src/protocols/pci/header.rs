//! # Configuration header
//!
//! Standard PCI header parsing.
//!
//! Parsing uses little-endian reads and
//! checked offsets. No unaligned dereferences
//! occur.

/// A parsed configuration header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PciHeader {
    /// The vendor ID.
    pub vendor_id: u16,
    /// The device ID.
    pub device_id: u16,
    /// The command register.
    pub command: u16,
    /// The status register.
    pub status: u16,
    /// The revision ID.
    pub revision_id: u8,
    /// The programming interface.
    pub programming_interface: u8,
    /// The subclass.
    pub subclass: u8,
    /// The base class.
    pub base_class: u8,
    /// The header type.
    pub header_type: u8,
    /// The capability pointer.
    pub capability_pointer: u8,
    /// The subsystem vendor ID.
    pub subsystem_vendor_id: u16,
    /// The subsystem device ID.
    pub subsystem_device_id: u16,
    /// The interrupt line.
    pub interrupt_line: u8,
    /// The interrupt pin.
    pub interrupt_pin: u8,
}

fn read_u16(bytes: &[u8]) -> u16 {
    crate::base::bytes::read_u16_le(bytes, 0).unwrap_or_default()
}

/// Parses a standard header.
///
/// The buffer must hold at least 64 bytes
/// for type 0 and 128 bytes for bridges.
#[must_use]
pub fn parse_header(buffer: &[u8]) -> Option<PciHeader> {
    if buffer.len() < 64 {
        return None;
    }
    let header_type = *buffer.get(0x0E)?;
    let base_type = header_type & 0x7F;
    if base_type > 2 {
        return None;
    }
    if base_type != 0 && buffer.len() < 128 {
        return None;
    }
    Some(PciHeader {
        vendor_id: read_u16(&buffer[0x00..0x02]),
        device_id: read_u16(&buffer[0x02..0x04]),
        command: read_u16(&buffer[0x04..0x06]),
        status: read_u16(&buffer[0x06..0x08]),
        revision_id: *buffer.get(0x08)?,
        programming_interface: *buffer.get(0x09)?,
        subclass: *buffer.get(0x0A)?,
        base_class: *buffer.get(0x0B)?,
        header_type,
        capability_pointer: *buffer.get(0x34).unwrap_or(&0),
        subsystem_vendor_id: read_u16(&buffer[0x2C..0x2E]),
        subsystem_device_id: read_u16(&buffer[0x2E..0x30]),
        interrupt_line: *buffer.get(0x3C)?,
        interrupt_pin: *buffer.get(0x3D)?,
    })
}

/// Returns whether the header is multifunction.
#[must_use]
pub fn is_multifunction(header: &PciHeader) -> bool {
    header.header_type & 0x80 != 0
}
