//! # Device Security Protocol
//!
//! Provides hardware device authentication and measurement interfaces
//! prior to driver binding.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// Device Security Protocol GUID.
pub const DEVICE_SECURITY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5D6B_38C8,
    0x5510,
    0x4458,
    [0xB4, 0x8D, 0x95, 0x81, 0xCF, 0xA7, 0xB0, 0x0D],
);

/// Device Identifier PCI type GUID.
pub const DEVICE_IDENTIFIER_TYPE_PCI_GUID: EfiGuid = EfiGuid::new(
    0x2509_B2F1,
    0xA022,
    0x4CCA,
    [0xAF, 0x70, 0xF9, 0xD3, 0x21, 0xFB, 0x66, 0x49],
);

/// Device Identifier USB type GUID.
pub const DEVICE_IDENTIFIER_TYPE_USB_GUID: EfiGuid = EfiGuid::new(
    0x7394_F350,
    0x394D,
    0x488C,
    [0xBB, 0x75, 0x0C, 0xAB, 0x7B, 0x12, 0x0A, 0xC5],
);

/// Protocol revision 1.0.
pub const DEVICE_SECURITY_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Identifier revision 1.0.
pub const DEVICE_IDENTIFIER_REVISION: u32 = 0x0001_0000;

/// The device identifier.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiDeviceIdentifier {
    /// Version of this data structure.
    pub version: u32,
    /// Device type GUID.
    pub device_type: EfiGuid,
    /// Device handle.
    pub device_handle: EfiHandle,
}

/// Authenticates a device.
pub type EfiDeviceAuthenticate = unsafe extern "efiapi" fn(
    this: *mut EfiDeviceSecurityProtocol,
    device_id: *mut EfiDeviceIdentifier,
) -> EfiStatus;

/// The Device Security Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiDeviceSecurityProtocol {
    /// Protocol revision.
    pub revision: u64,
    /// Authenticates the device.
    pub device_authenticate: EfiDeviceAuthenticate,
}
