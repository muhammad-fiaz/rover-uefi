//! # Device path validation
//!
//! Validation for device path buffers.
//!
//! A valid path is a non-empty sequence of
//! nodes whose lengths are at least 4, whose
//! total does not exceed the buffer, and whose
//! last node is an end-entire node.

use crate::protocols::device_path::iterator::device_path_iter;

/// Returns whether the buffer holds a valid
/// device path.
///
/// The entire buffer must be consumed by
/// nodes ending with an end-entire node.
#[must_use]
pub fn validate_device_path(bytes: &[u8]) -> bool {
    if bytes.len() < 4 || bytes.len() > u16::MAX as usize {
        return false;
    }
    let mut saw_entire = false;
    let mut count = 0_usize;
    for item in device_path_iter(bytes) {
        let Ok(node) = item else {
            return false;
        };
        if node.is_end_entire() {
            saw_entire = true;
        }
        count += 1;
        if count > 255 {
            return false;
        }
    }
    saw_entire
}

/// Returns the buffer length when valid.
///
/// Returns `None` when invalid.
#[must_use]
pub fn validated_length(bytes: &[u8]) -> Option<usize> {
    if validate_device_path(bytes) {
        Some(bytes.len())
    } else {
        None
    }
}

/// Returns the number of nodes.
///
/// Returns `None` when invalid.
#[must_use]
pub fn node_count(bytes: &[u8]) -> Option<usize> {
    if !validate_device_path(bytes) {
        return None;
    }
    let mut count = 0_usize;
    for item in device_path_iter(bytes) {
        if item.is_ok() {
            count += 1;
        }
    }
    Some(count)
}
