//! # Device path utilities
//!
//! Length and instance helpers.
//!
//! All size calculations use checked
//! arithmetic. No allocation occurs.

/// Returns the maximum single path size.
pub const MAX_DEVICE_PATH_SIZE: usize = 65535;

/// Returns the checked end of a node.
///
/// Returns `None` on overflow or when the
/// length is below the header size.
#[must_use]
pub fn checked_node_end(offset: usize, length: usize) -> Option<usize> {
    if length < 4 {
        return None;
    }
    offset.checked_add(length)
}

/// Splits a buffer into instances.
pub struct InstanceIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for InstanceIter<'a> {
    type Item = &'a [u8];

    /// Returns the next instance or `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.bytes.len() {
            return None;
        }
        let start = self.offset;
        let mut cursor = self.offset;
        // Scan nodes until end-instance or end-entire.
        loop {
            if cursor + 4 > self.bytes.len() {
                return None;
            }
            let pair = self.bytes.get(cursor + 2..cursor + 4)?;
            let length = u16::from_le_bytes([pair[0], pair[1]]) as usize;
            if length < 4 {
                return None;
            }
            let next = cursor.checked_add(length)?;
            let device_type = self.bytes.get(cursor).copied().unwrap_or(0);
            let subtype = self.bytes.get(cursor + 1).copied().unwrap_or(0);
            cursor = next;
            let is_end = device_type == 0x7F
                && (subtype == crate::protocols::device_path::types::END_INSTANCE
                    || subtype == crate::protocols::device_path::types::END_ENTIRE);
            if is_end {
                break;
            }
            if cursor >= self.bytes.len() {
                break;
            }
        }
        self.offset = cursor;
        self.bytes.get(start..cursor)
    }
}

/// Creates an instance iterator.
#[must_use]
pub fn device_path_instances(bytes: &[u8]) -> InstanceIter<'_> {
    InstanceIter { bytes, offset: 0 }
}
