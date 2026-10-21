//! # USB
//!
//! USB I/O, host controller, and descriptor
//! parsing.

pub mod descriptor;
pub mod device;
#[path = "ehci/mod.rs"]
pub mod ehci;
pub mod endpoint;
pub mod enumeration;
pub mod guid;
#[path = "hid/mod.rs"]
pub mod hid;
pub mod host_controller;
pub mod interface;
#[path = "mass_storage/mod.rs"]
pub mod mass_storage;
pub mod types;
pub mod usb_io;
pub mod validation;
pub mod xhci;

pub use guid::{USB_IO_GUID, USB2_HC_GUID};
pub use host_controller::{EfiUsb2HcProtocol, Usb2Hc, Usb3Hc};
pub use usb_io::protocol::{EfiUsbIoProtocol, UsbIo};
