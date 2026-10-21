//! # `VirtIO` Device Protocol
//!
//! Provides the `VirtIO` device protocol abstraction and standard subsystem IDs
//! defined in the OASIS `VirtIO` Specification.

use crate::base::{EfiGuid, EfiStatus};

/// `VirtIO` Device Protocol `GUID`.
pub const VIRTIO_DEVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xFA92_0010,
    0x6785,
    0x4941,
    [0xB6, 0xEC, 0x49, 0x8C, 0x57, 0x9F, 0x16, 0x0A],
);

/// `VirtIO` Subsystem Device IDs.
pub mod device_id {
    /// Network device.
    pub const NETWORK: u16 = 1;
    /// Block device.
    pub const BLOCK: u16 = 2;
    /// Console device.
    pub const CONSOLE: u16 = 3;
    /// Entropy (RNG) device.
    pub const ENTROPY: u16 = 4;
    /// Memory balloon device.
    pub const BALLOON: u16 = 5;
    /// SCSI host bus adapter.
    pub const SCSI: u16 = 8;
    /// 9P transport.
    pub const NINE_P: u16 = 9;
    /// GPU device.
    pub const GPU: u16 = 16;
    /// Input device.
    pub const INPUT: u16 = 18;
    /// Socket device (vsock).
    pub const SOCKET: u16 = 19;
    /// Crypto device.
    pub const CRYPTO: u16 = 20;
    /// `VirtIO` file system (virtio-fs).
    pub const FS: u16 = 26;
}

pub use device_id as virtio_device_id;
pub use status as virtio_status;

/// `VirtIO` Device Status Bits.
pub mod status {
    /// Guest OS has noticed the device.
    pub const ACKNOWLEDGE: u8 = 1;
    /// Guest OS knows how to drive the device.
    pub const DRIVER: u8 = 2;
    /// Driver is set up and ready to drive the device.
    pub const DRIVER_OK: u8 = 4;
    /// Feature negotiation complete.
    pub const FEATURES_OK: u8 = 8;
    /// Device has experienced an error from which it cannot recover.
    pub const DEVICE_NEEDS_RESET: u8 = 64;
    /// Driver gave up on device.
    pub const FAILED: u8 = 128;
}

/// Gets device features.
pub type VirtioGetDeviceFeatures =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, features: *mut u64) -> EfiStatus;

/// Sets guest features.
pub type VirtioSetGuestFeatures =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, features: u64) -> EfiStatus;

/// Gets device status.
pub type VirtioGetDeviceStatus = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    device_status: *mut u8,
) -> EfiStatus;

/// Sets device status.
pub type VirtioSetDeviceStatus =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, device_status: u8) -> EfiStatus;

/// The `VirtIO` Device Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiVirtioDeviceProtocol {
    /// Specification revision.
    pub revision: u32,
    /// Subsystem device ID.
    pub subsystem_device_id: i32,
    /// Gets 64-bit device features.
    pub get_device_features: VirtioGetDeviceFeatures,
    /// Sets 64-bit guest features.
    pub set_guest_features: VirtioSetGuestFeatures,
    /// Gets device status byte.
    pub get_device_status: VirtioGetDeviceStatus,
    /// Sets device status byte.
    pub set_device_status: VirtioSetDeviceStatus,
}
