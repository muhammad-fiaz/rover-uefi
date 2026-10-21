//! # ACPI tables
//!
//! Bounded table views.
//!
//! The view borrows caller-provided bytes.

use crate::protocols::acpi::header::{AcpiHeader, parse_header};

/// A bounded ACPI table view.
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

/// Parses a table view.
#[must_use]
pub fn parse_table_view(buffer: &[u8]) -> Option<AcpiTable<'_>> {
    let header = parse_header(buffer)?;
    Some(AcpiTable::new(buffer, header))
}
