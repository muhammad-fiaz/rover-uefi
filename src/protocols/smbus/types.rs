//! # `SMBus` Types and Data Structures
//!
//! Defined in System Management Bus (`SMBus`) Specification 2.0 and UEFI PI Specification.

use crate::abi::types::UIntN;

/// `SMBus` device slave address (7-bit address).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiSmbusDeviceAddress {
    /// 7-bit hardware slave address.
    pub smbus_device_address: UIntN,
}

impl EfiSmbusDeviceAddress {
    /// Creates a new device address from a 7-bit integer.
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        Self {
            smbus_device_address: (addr & 0x7F) as UIntN,
        }
    }

    /// Returns the 7-bit address.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn address(&self) -> u8 {
        (self.smbus_device_address & 0x7F) as u8
    }
}

/// `SMBus` command code or register offset.
pub type EfiSmbusDeviceCommand = UIntN;

/// Unique Device Identifier (UDID) of an `SMBus` device.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiSmbusUdid {
    /// Vendor-specific device ID.
    pub vendor_specific_id: u32,
    /// Subsystem device ID.
    pub subsystem_device_id: u16,
    /// Subsystem vendor ID.
    pub subsystem_vendor_id: u16,
    /// Interface type.
    pub interface: u16,
    /// Device ID.
    pub device_id: u16,
    /// Vendor ID.
    pub vendor_id: u16,
    /// Vendor revision number.
    pub vendor_revision: u8,
    /// Device capabilities bitfield.
    pub device_capabilities: u8,
}

/// `SMBus` Device Map entry mapping slave address to UDID.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiSmbusDeviceMap {
    /// Preassigned or allocated slave address.
    pub smbus_device_address: EfiSmbusDeviceAddress,
    /// Unique device identifier.
    pub smbus_device_udid: EfiSmbusUdid,
}

/// Standard `SMBus` hardware operations.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiSmbusOperation {
    /// Quick read (1 bit).
    QuickRead = 0,
    /// Quick write (1 bit).
    QuickWrite = 1,
    /// Receive byte (no command byte).
    ReceiveByte = 2,
    /// Send byte (no command byte).
    SendByte = 3,
    /// Read byte with command offset.
    ReadByte = 4,
    /// Write byte with command offset.
    WriteByte = 5,
    /// Read word (16 bits) with command offset.
    ReadWord = 6,
    /// Write word (16 bits) with command offset.
    WriteWord = 7,
    /// Read block of data up to 32 bytes.
    ReadBlock = 8,
    /// Write block of data up to 32 bytes.
    WriteBlock = 9,
    /// Process call (send word, receive word).
    ProcessCall = 10,
    /// Block write - block read process call.
    BWBRProcessCall = 11,
}
