//! # Root tables
//!
//! RSDT and XSDT entry iteration.

use crate::protocols::acpi::header::{AcpiHeader, parse_header};

/// RSDT signatures.
pub const RSDT_SIGNATURE: [u8; 4] = *b"RSDT";

/// XSDT signatures.
pub const XSDT_SIGNATURE: [u8; 4] = *b"XSDT";

/// Returns whether a buffer is a valid root
/// table payload.
#[must_use]
pub fn validate_root_table(buffer: &[u8], is_xsdt: bool) -> bool {
    let Some(header) = parse_header(buffer) else {
        return false;
    };
    let expected = if is_xsdt {
        XSDT_SIGNATURE
    } else {
        RSDT_SIGNATURE
    };
    if header.signature != expected {
        return false;
    }
    let entry_size = if is_xsdt { 8 } else { 4 };
    let total = header.length as usize;
    if total < 36 {
        return false;
    }
    (total - 36).is_multiple_of(entry_size)
}

/// Iterates RSDT addresses.
pub struct RsdtIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl Iterator for RsdtIter<'_> {
    type Item = u32;

    /// Returns the next address.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 4 > self.bytes.len() {
            return None;
        }
        let mut raw = [0_u8; 4];
        raw.copy_from_slice(self.bytes.get(self.offset..self.offset + 4)?);
        self.offset += 4;
        Some(u32::from_le_bytes(raw))
    }
}

/// Creates an RSDT iterator.
///
/// Returns `None` when the payload is misaligned.
#[must_use]
pub fn rsdt_iter(buffer: &[u8]) -> Option<RsdtIter<'_>> {
    if buffer.len() < 36 || !validate_root_table(buffer, false) {
        return None;
    }
    Some(RsdtIter {
        bytes: buffer,
        offset: 36,
    })
}

/// Iterates XSDT addresses.
pub struct XsdtIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl Iterator for XsdtIter<'_> {
    type Item = u64;

    /// Returns the next address.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 8 > self.bytes.len() {
            return None;
        }
        let mut raw = [0_u8; 8];
        raw.copy_from_slice(self.bytes.get(self.offset..self.offset + 8)?);
        self.offset += 8;
        Some(u64::from_le_bytes(raw))
    }
}

/// Creates an XSDT iterator.
#[must_use]
pub fn xsdt_iter(buffer: &[u8]) -> Option<XsdtIter<'_>> {
    if buffer.len() < 36 || !validate_root_table(buffer, true) {
        return None;
    }
    Some(XsdtIter {
        bytes: buffer,
        offset: 36,
    })
}

/// A bounded table view.
pub struct AcpiTable<'a> {
    bytes: &'a [u8],
    header: AcpiHeader,
}

impl<'a> AcpiTable<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(bytes: &'a [u8], header: AcpiHeader) -> Self {
        Self { bytes, header }
    }

    /// Returns the header.
    #[must_use]
    pub const fn header(&self) -> &AcpiHeader {
        &self.header
    }

    /// Returns the full bytes.
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the body after the header.
    #[must_use]
    pub fn body(&self) -> &[u8] {
        self.bytes
            .get(36..self.header.length as usize)
            .unwrap_or(&[])
    }
}
