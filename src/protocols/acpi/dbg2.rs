//! # DBG2
//!
//! Microsoft / UEFI Debug Port Table 2 parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_DBG2;

/// Port types defined in DBG2.
pub mod port_type {
    /// Serial port interface.
    pub const SERIAL: u16 = 0x8000;
    /// IEEE 1394 (`FireWire`) port interface.
    pub const FIREWIRE: u16 = 0x8001;
    /// USB port interface.
    pub const USB: u16 = 0x8002;
    /// Network port interface.
    pub const NET: u16 = 0x8003;
}

/// Port subtypes defined in DBG2.
pub mod port_subtype {
    /// Fully 16550-compatible serial port.
    pub const SERIAL_16550: u16 = 0x0000;
    /// 16550-compatible serial port with sub-properties.
    pub const SERIAL_16550_SUBSET: u16 = 0x0001;
    /// ARM PL011 UART.
    pub const SERIAL_ARM_PL011: u16 = 0x0002;
    /// ARM SBSA (Server Base System Architecture) generic 32-bit UART.
    pub const SERIAL_ARM_SBSA_32BIT: u16 = 0x0003;
    /// ARM SBSA generic UART (any register width).
    pub const SERIAL_ARM_SBSA_GENERIC: u16 = 0x0004;
    /// ARM DCC (Debug Communications Channel).
    pub const SERIAL_ARM_DCC: u16 = 0x0005;
    /// Broadcom 2835 Mini UART.
    pub const SERIAL_BCM2835: u16 = 0x0006;
    /// USB xHCI debug capability.
    pub const USB_XHCI: u16 = 0x0000;
    /// USB EHCI debug capability.
    pub const USB_EHCI: u16 = 0x0001;
}

/// Parsed DBG2 header info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Dbg2Header {
    /// Offset from table start to device info array.
    pub devices_offset: u32,
    /// Number of debug device info structures.
    pub devices_count: u32,
}

/// A parsed Debug Device Information structure view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dbg2Device<'a> {
    /// Revision of the structure.
    pub revision: u8,
    /// Full length of this structure in bytes.
    pub length: u16,
    /// Number of generic address registers.
    pub register_count: u8,
    /// Port type (e.g. Serial, USB).
    pub port_type: u16,
    /// Port subtype (e.g. 16550, PL011, xHCI).
    pub port_subtype: u16,
    /// Namespace string if present.
    pub namespace_string: Option<&'a str>,
    /// Raw byte slice containing this device structure.
    pub raw: &'a [u8],
}

/// Parses the DBG2 table header.
///
/// Returns `None` if the buffer is shorter than the standard
/// ACPI table header plus DBG2 fields, or if signature is not DBG2.
#[must_use]
pub fn parse_dbg2_header(buffer: &[u8]) -> Option<Dbg2Header> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_DBG2 {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 44 || buffer.len() < 44 {
        return None;
    }

    let mut dev_offset = [0_u8; 4];
    dev_offset.copy_from_slice(buffer.get(36..40)?);
    let mut dev_count = [0_u8; 4];
    dev_count.copy_from_slice(buffer.get(40..44)?);

    Some(Dbg2Header {
        devices_offset: u32::from_le_bytes(dev_offset),
        devices_count: u32::from_le_bytes(dev_count),
    })
}

/// An iterator over debug devices in a DBG2 table.
pub struct Dbg2Iter<'a> {
    buffer: &'a [u8],
    current_offset: usize,
    remaining: u32,
}

impl<'a> Iterator for Dbg2Iter<'a> {
    type Item = Dbg2Device<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.current_offset >= self.buffer.len() {
            return None;
        }

        let slice = self.buffer.get(self.current_offset..)?;
        if slice.len() < 22 {
            return None;
        }

        let revision = *slice.first()?;
        let length = u16::from_le_bytes([*slice.get(1)?, *slice.get(2)?]);
        if usize::from(length) < 22 || usize::from(length) > slice.len() {
            return None;
        }

        let reg_count = *slice.get(3)?;
        let name_len = u16::from_le_bytes([*slice.get(4)?, *slice.get(5)?]);
        let name_offset = u16::from_le_bytes([*slice.get(6)?, *slice.get(7)?]);
        let port_type = u16::from_le_bytes([*slice.get(12)?, *slice.get(13)?]);
        let port_subtype = u16::from_le_bytes([*slice.get(14)?, *slice.get(15)?]);

        let device_raw = &slice[..usize::from(length)];

        let namespace_str = if name_len > 0 && usize::from(name_offset) < usize::from(length) {
            let start = usize::from(name_offset);
            let end = (start + usize::from(name_len)).min(usize::from(length));
            let str_bytes = device_raw.get(start..end)?;
            let trimmed = if let Some(&0) = str_bytes.last() {
                &str_bytes[..str_bytes.len().saturating_sub(1)]
            } else {
                str_bytes
            };
            core::str::from_utf8(trimmed).ok()
        } else {
            None
        };

        self.current_offset = self.current_offset.saturating_add(usize::from(length));
        self.remaining = self.remaining.saturating_sub(1);

        Some(Dbg2Device {
            revision,
            length,
            register_count: reg_count,
            port_type,
            port_subtype,
            namespace_string: namespace_str,
            raw: device_raw,
        })
    }
}

/// Returns an iterator over the debug devices in a DBG2 buffer.
#[must_use]
pub fn dbg2_iter(buffer: &[u8]) -> Option<Dbg2Iter<'_>> {
    let header = parse_dbg2_header(buffer)?;
    let start_offset = usize::try_from(header.devices_offset).ok()?;
    if start_offset > buffer.len() {
        return None;
    }

    Some(Dbg2Iter {
        buffer,
        current_offset: start_offset,
        remaining: header.devices_count,
    })
}
