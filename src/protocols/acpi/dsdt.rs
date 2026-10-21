//! # DSDT
//!
//! Differentiated System Description Table parsing.
//!
//! The view exposes raw `AML` bytes after the
//! 36-byte header. Raw `AML` is not interpreted
//! by these parsers. Callers needing `AML`
//! evaluation must use a dedicated interpreter.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_DSDT;

/// A view over raw `AML` bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AmlView<'a> {
    /// The table revision.
    pub revision: u8,
    /// The raw `AML` bytes after the 36-byte header.
    pub aml_bytes: &'a [u8],
}

/// Parses a DSDT buffer.
///
/// Returns `None` when the buffer is shorter
/// than the 36-byte header, the signature or
/// length is invalid, or the checksum fails.
/// The raw `AML` is not interpreted.
#[must_use]
pub fn parse_dsdt(buffer: &[u8]) -> Option<AmlView<'_>> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_DSDT {
        return None;
    }
    let table_length = usize::try_from(header.length).ok()?;
    let aml_bytes = buffer.get(36..table_length)?;
    Some(AmlView {
        revision: header.revision,
        aml_bytes,
    })
}
