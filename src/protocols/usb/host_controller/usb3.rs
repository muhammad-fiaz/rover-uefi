//! # USB 3 host controller
//!
//! USB 3.0 Controllers are accessed through
//! the USB 2.0 Host Controller protocol in
//! UEFI 2.11. This module provides the Super
//! Speed capability data without inventing a
//! separate protocol structure.

use crate::protocols::usb::host_controller::usb2::{EfiUsb2HcProtocol, Usb2Hc};

/// The USB 3 host controller view.
///
/// Super Speed devices are discovered and
/// controlled through the USB 2.0 Host
/// Controller protocol instance.
pub type Usb3Hc<'a> = Usb2Hc<'a>;

/// The USB 3 protocol alias.
pub type EfiUsb3HcProtocol = EfiUsb2HcProtocol;
