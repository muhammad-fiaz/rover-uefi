//! # SMBIOS strings
//!
//! String set access.
//!
//! Indices are 1-based. Zero means no string.

/// Returns the indexed string.
///
/// Returns `None` for zero or out-of-range
/// indices.
#[must_use]
pub fn string_at(strings: &[u8], index: u8) -> Option<&[u8]> {
    if index == 0 {
        return None;
    }
    let mut current = 1_u8;
    let mut cursor = 0_usize;
    while cursor < strings.len() {
        let end_offset = strings[cursor..].iter().position(|byte| *byte == 0)?;
        let length = end_offset;
        if current == index {
            return strings.get(cursor..cursor.checked_add(length)?);
        }
        cursor = cursor.checked_add(length)?.checked_add(1)?;
        current += 1;
    }
    None
}
