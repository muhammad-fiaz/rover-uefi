//! # BAR parsing
//!
//! Base Address Register handling.
//!
//! 64-bit memory BARs occupy two indices. The
//! upper register is not a separate BAR.

/// A parsed BAR.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PciBar {
    /// The BAR index.
    pub index: usize,
    /// The raw register value.
    pub raw: u32,
    /// The 64-bit address when applicable.
    pub address: u64,
    /// Whether the BAR is I/O.
    pub is_io: bool,
    /// Whether memory is prefetchable.
    pub is_prefetchable: bool,
}

/// Returns a little-endian `u32` from four bytes.
fn read_u32(bytes: &[u8]) -> u32 {
    crate::base::bytes::read_u32_le(bytes, 0).unwrap_or_default()
}

/// Parses a 32-bit BAR at an index.
#[must_use]
pub fn parse_bar32(buffer: &[u8], index: usize) -> Option<PciBar> {
    if index >= 6 {
        return None;
    }
    let offset = 0x10 + index * 4;
    let raw = read_u32(buffer.get(offset..offset.checked_add(4)?)?);
    Some(classify_bar(index, raw, 0))
}

/// Parses a 64-bit memory BAR.
#[must_use]
pub fn parse_bar64(buffer: &[u8], low_index: usize) -> Option<PciBar> {
    if low_index >= 5 {
        return None;
    }
    let low = parse_bar32(buffer, low_index)?;
    if low.is_io {
        return None;
    }
    let offset = 0x10 + (low_index + 1) * 4;
    let high = read_u32(buffer.get(offset..offset.checked_add(4)?)?);
    let address = (u64::from(high) << 32) | u64::from(low.raw & 0xFFFF_FFF0);
    Some(PciBar {
        index: low_index,
        raw: low.raw,
        address,
        is_io: false,
        is_prefetchable: low.raw & 8 != 0,
    })
}

/// Classifies a BAR value.
fn classify_bar(index: usize, raw: u32, high: u32) -> PciBar {
    let is_io = raw & 1 != 0;
    let address = if is_io {
        u64::from(raw & 0xFFFF_FFFC)
    } else if raw & 6 == 2 {
        (u64::from(high) << 32) | u64::from(raw & 0xFFFF_FFF0)
    } else {
        u64::from(raw & 0xFFFF_FFF0)
    };
    let is_prefetchable = !is_io && raw & 8 != 0;
    PciBar {
        index,
        raw,
        address,
        is_io,
        is_prefetchable,
    }
}
