//! # USB host controllers
//!
//! USB 2.0 and USB 3.0 host controller
//! protocols.

pub mod root_hub;
pub mod usb2;
pub mod usb3;

pub use usb2::{EfiUsb2HcProtocol, Usb2Hc, UsbPortStatus};
pub use usb3::{EfiUsb3HcProtocol, Usb3Hc};
