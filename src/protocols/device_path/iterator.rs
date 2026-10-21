//! # Device path iterator
//!
//! Iteration over validated device path
//! bytes.
//!
//! The iterator reads the header length, checks
//! bounds, and yields the payload. Iteration
//! stops at the end-entire node. A single
//! instance ends at the end-instance node.
//! The caller provides the full buffer; the
//! iterator never reads beyond it.

use crate::protocols::device_path::types::{END_ENTIRE, END_INSTANCE, EfiDevicePath};

/// A single device path node view.
pub struct DevicePathNode<'a> {
    bytes: &'a [u8],
    offset: usize,
    length: usize,
}

impl DevicePathNode<'_> {
    /// Returns the header.
    #[must_use]
    pub fn header(&self) -> EfiDevicePath {
        let mut length = [0_u8; 2];
        if let Some(pair) = self.bytes.get(self.offset + 2..self.offset + 4) {
            length.copy_from_slice(pair);
        }
        let device_type = self.bytes.get(self.offset).copied().unwrap_or(0);
        let subtype = self.bytes.get(self.offset + 1).copied().unwrap_or(0);
        EfiDevicePath {
            device_type,
            subtype,
            length,
        }
    }

    /// Returns the payload after the header.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        if self.length <= 4 {
            return &[];
        }
        self.bytes
            .get(self.offset + 4..self.offset + self.length)
            .unwrap_or(&[])
    }

    /// Returns whether the node ends an instance.
    #[must_use]
    pub fn is_end_instance(&self) -> bool {
        self.header().is_end_instance()
    }

    /// Returns whether the node ends the path.
    #[must_use]
    pub fn is_end_entire(&self) -> bool {
        self.header().is_end_entire()
    }
}

/// Iterates device path nodes.
pub struct DevicePathIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for DevicePathIter<'a> {
    type Item = Result<DevicePathNode<'a>, ()>;

    /// Returns the next node or `None` at the
    /// end of the buffer.
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.bytes.len() {
            return None;
        }
        if self.offset + 4 > self.bytes.len() {
            return Some(Err(()));
        }
        let Some(pair) = self.bytes.get(self.offset + 2..self.offset + 4) else {
            return Some(Err(()));
        };
        let length = u16::from_le_bytes([pair[0], pair[1]]) as usize;
        if length < 4 || self.offset + length > self.bytes.len() {
            return Some(Err(()));
        }
        let current = self.offset;
        // Checked addition keeps the offset within the buffer.
        self.offset = current.checked_add(length)?;
        let is_entire = self.bytes.get(current).copied().unwrap_or(0) == 0x7F
            && self.bytes.get(current + 1).copied().unwrap_or(0) == END_ENTIRE;
        let node = DevicePathNode {
            bytes: self.bytes,
            offset: current,
            length,
        };
        if is_entire {
            self.offset = self.bytes.len();
        }
        if length == 4 {
            let type_byte = self.bytes.get(current).copied().unwrap_or(0);
            let subtype = self.bytes.get(current + 1).copied().unwrap_or(0);
            if type_byte == 0x7F && subtype == END_INSTANCE {
                self.offset = current.checked_add(length).unwrap_or(0);
            }
        }
        Some(Ok(node))
    }
}

/// Creates an iterator over a validated path.
///
/// The caller must provide a buffer that
/// contains at least one header.
#[must_use]
pub fn device_path_iter(bytes: &[u8]) -> DevicePathIter<'_> {
    DevicePathIter { bytes, offset: 0 }
}

/// Returns whether the node is an end node.
#[must_use]
pub const fn is_end_node(device_type: u8, subtype: u8) -> bool {
    device_type == 0x7F && (subtype == END_INSTANCE || subtype == END_ENTIRE)
}
