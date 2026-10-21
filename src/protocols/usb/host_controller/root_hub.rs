//! # Root hub
//!
//! Root hub port helpers.

use crate::protocols::usb::host_controller::usb2::UsbPortStatus;

/// Returns whether the port is connected.
#[must_use]
pub fn is_connected(status: &UsbPortStatus) -> bool {
    status.port_status & 0x0001 != 0
}

/// Returns whether the port is enabled.
#[must_use]
pub fn is_enabled(status: &UsbPortStatus) -> bool {
    status.port_status & 0x0002 != 0
}
