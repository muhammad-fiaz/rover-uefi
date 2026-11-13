//! # Protocols
//!
//! Typed protocol interfaces and helpers.
//!
//! Each protocol wrapper borrows the firmware
//! interface for an explicit lifetime. GUID
//! constants, console input and output, and
//! device path parsing are provided. Wrappers
//! preserve the firmware status and do not
//! extend firmware lifetimes.

pub mod acpi;
pub mod android;
pub mod audio;
pub mod bluetooth;
pub mod console;
pub mod core;
pub mod debug;
pub mod device_path;
pub mod diagnostic;
pub mod driver;
pub mod firmware;
pub mod graphics;
pub mod i2c;
pub mod manageability;
pub mod network;
pub mod pci;
pub mod security;
pub mod serial;
pub mod smbios;
pub mod smbus;
pub mod spi;
pub mod storage;
pub mod usb;

pub use core::{DEVICE_PATH_GUID, SIMPLE_TEXT_INPUT_GUID, SIMPLE_TEXT_OUTPUT_GUID};
