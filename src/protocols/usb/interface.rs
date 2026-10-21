//! # USB interfaces
//!
//! Interface helpers.

/// Returns whether the interface class is
/// mass storage.
#[must_use]
pub fn is_mass_storage_interface(interface_class: u8) -> bool {
    interface_class == 0x08
}
