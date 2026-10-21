//! # Capabilities
//!
//! Bounded capability list traversal.
//!
//! Traversal stops on cycles, invalid
//! pointers, or the fixed limit.

/// The maximum capabilities per device.
pub const MAX_CAPABILITIES: usize = 32;

/// A capability entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Capability {
    /// The capability ID.
    pub id: u8,
    /// The offset in configuration space.
    pub offset: u8,
}

/// Returns the capability ID at an offset.
#[must_use]
pub fn capability_at(buffer: &[u8], offset: u8) -> Option<Capability> {
    let index = usize::from(offset);
    let id = *buffer.get(index)?;
    let next = *buffer.get(index.checked_add(1)?)?;
    let _ = next;
    Some(Capability { id, offset })
}

/// Iterates capabilities from the start.
pub struct CapabilityIter<'a> {
    bytes: &'a [u8],
    offset: u8,
    remaining: usize,
}

impl Iterator for CapabilityIter<'_> {
    type Item = Capability;

    /// Returns the next capability.
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.offset == 0 {
            return None;
        }
        let current = capability_at(self.bytes, self.offset)?;
        self.remaining -= 1;
        let next_offset = *self.bytes.get(usize::from(self.offset).checked_add(1)?)?;
        self.offset = next_offset;
        Some(current)
    }
}

/// Creates a capability iterator.
///
/// Returns `None` when the start offset is
/// zero.
#[must_use]
pub fn capability_iter(bytes: &[u8], start: u8) -> Option<CapabilityIter<'_>> {
    if start == 0 {
        None
    } else {
        Some(CapabilityIter {
            bytes,
            offset: start,
            remaining: MAX_CAPABILITIES,
        })
    }
}
