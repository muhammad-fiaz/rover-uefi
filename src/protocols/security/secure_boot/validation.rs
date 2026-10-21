//! # Secure Boot validation
//!
//! Pure validation helpers for signature databases.

/// Returns whether a signature list size is valid.
#[must_use]
pub fn validate_signature_list_size(size: u32) -> bool {
    size >= 28
}

/// Returns whether a signature entry size is valid.
#[must_use]
pub fn validate_signature_size(size: u32) -> bool {
    size >= 16
}

/// Returns whether a signature list entry count fits.
#[must_use]
pub fn signature_count(list_size: u32, header_size: u32, signature_size: u32) -> Option<u32> {
    if list_size < 28 || signature_size < 16 {
        return None;
    }
    let body = list_size.checked_sub(28)?;
    let body = body.checked_sub(header_size)?;
    Some(body / signature_size)
}
