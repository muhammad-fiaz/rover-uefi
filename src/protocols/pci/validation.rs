//! # PCI validation
//!
//! Pure validation helpers.

/// Returns whether a BAR index is valid.
#[must_use]
pub fn validate_bar_index(index: u8) -> bool {
    index < 6
}

/// Returns whether a configuration offset is
/// in bounds.
#[must_use]
pub fn validate_config_offset(offset: u32, length: u32, config_size: u32) -> bool {
    config_size >= 64
        && offset
            .checked_add(length)
            .is_some_and(|end| end <= config_size)
}

/// Returns whether a location is valid.
#[must_use]
pub fn validate_location(_segment: u8, _bus: u8, device: u8, function: u8) -> bool {
    device < 32 && function < 8
}
