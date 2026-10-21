//! # Devices
//!
//! Device location and identification.

use crate::base::EfiHandle;

/// A PCI device location.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PciLocation {
    /// The segment number.
    pub segment: u8,
    /// The bus number.
    pub bus: u8,
    /// The device number.
    pub device: u8,
    /// The function number.
    pub function: u8,
}

/// A PCI device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PciDevice {
    /// The firmware handle.
    pub handle: EfiHandle,
    /// The location.
    pub location: PciLocation,
}

/// Returns whether a vendor/device pair
/// matches.
#[must_use]
pub fn matches_id(vendor_id: u16, device_id: u16, want_vendor: u16, want_device: u16) -> bool {
    vendor_id == want_vendor && device_id == want_device
}
