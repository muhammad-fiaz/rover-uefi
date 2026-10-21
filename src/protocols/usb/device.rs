//! # USB devices
//!
//! Device category helpers.
//!
//! The helpers identify descriptor values
//! only and do not provide drivers.

/// The HID class.
pub const CLASS_HID: u8 = 0x03;

/// The mass storage class.
pub const CLASS_MASS_STORAGE: u8 = 0x08;

/// The hub class.
pub const CLASS_HUB: u8 = 0x09;

/// Returns whether the class is a hub.
#[must_use]
pub fn is_hub(device_class: u8) -> bool {
    device_class == CLASS_HUB
}

/// Returns whether the class is mass storage.
#[must_use]
pub fn is_mass_storage(device_class: u8) -> bool {
    device_class == CLASS_MASS_STORAGE
}

/// Returns whether the class is HID.
#[must_use]
pub fn is_hid(interface_class: u8) -> bool {
    interface_class == CLASS_HID
}
