//! # ATA Pass Thru Protocol
//!
//! Declarations for `EFI_ATA_PASS_THRU_PROTOCOL` according to the
//! UEFI Specification.
//!
//! Provides direct low-level ATA command submission to ports and port multipliers.

use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;
use core::ffi::c_void;

pub use crate::protocols::storage::guid::{
    ATA_PASS_THRU_PROTOCOL_GUID as ATA_PASS_THRU_GUID, ATA_PASS_THRU_PROTOCOL_GUID,
};

/// Interface is for physical devices on the ATA controller.
pub const ATA_PASS_THRU_ATTRIBUTES_PHYSICAL: u32 = 0x0001;
/// Interface is for logical devices on the ATA controller.
pub const ATA_PASS_THRU_ATTRIBUTES_LOGICAL: u32 = 0x0002;
/// Interface supports non-blocking I/O.
pub const ATA_PASS_THRU_ATTRIBUTES_NONBLOCKIO: u32 = 0x0004;

/// Mode structure describing ATA pass-thru controller capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiAtaPassThruMode {
    /// Controller attributes bitmask.
    pub attributes: u32,
    /// Minimum required alignment for data buffers.
    pub io_align: u32,
}

/// ATA Task File Command Block register image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiAtaCommandBlock {
    /// Reserved bytes.
    pub reserved1: [u8; 2],
    /// ATA Command register.
    pub ata_command: u8,
    /// ATA Features register.
    pub ata_features: u8,
    /// ATA Sector Number register.
    pub ata_sector_number: u8,
    /// ATA Cylinder Low register.
    pub ata_cylinder_low: u8,
    /// ATA Cylinder High register.
    pub ata_cylinder_high: u8,
    /// ATA Device/Head register.
    pub ata_device_head: u8,
    /// ATA Sector Number register (expanded 48-bit).
    pub ata_sector_number_exp: u8,
    /// ATA Cylinder Low register (expanded 48-bit).
    pub ata_cylinder_low_exp: u8,
    /// ATA Cylinder High register (expanded 48-bit).
    pub ata_cylinder_high_exp: u8,
    /// ATA Features register (expanded 48-bit).
    pub ata_features_exp: u8,
    /// ATA Sector Count register.
    pub ata_sector_count: u8,
    /// ATA Sector Count register (expanded 48-bit).
    pub ata_sector_count_exp: u8,
    /// Reserved trailing bytes.
    pub reserved2: [u8; 6],
}

/// ATA Task File Status Block register image returned by device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiAtaStatusBlock {
    /// Reserved bytes.
    pub reserved1: [u8; 2],
    /// ATA Status register.
    pub ata_status: u8,
    /// ATA Error register.
    pub ata_error: u8,
    /// ATA Sector Number register.
    pub ata_sector_number: u8,
    /// ATA Cylinder Low register.
    pub ata_cylinder_low: u8,
    /// ATA Cylinder High register.
    pub ata_cylinder_high: u8,
    /// ATA Device/Head register.
    pub ata_device_head: u8,
    /// ATA Sector Number register (expanded 48-bit).
    pub ata_sector_number_exp: u8,
    /// ATA Cylinder Low register (expanded 48-bit).
    pub ata_cylinder_low_exp: u8,
    /// ATA Cylinder High register (expanded 48-bit).
    pub ata_cylinder_high_exp: u8,
    /// Reserved byte.
    pub reserved2: u8,
    /// ATA Sector Count register.
    pub ata_sector_count: u8,
    /// ATA Sector Count register (expanded 48-bit).
    pub ata_sector_count_exp: u8,
    /// Reserved trailing bytes.
    pub reserved3: [u8; 6],
}

/// Packet describing an ATA pass-thru transaction.
#[derive(Debug)]
#[repr(C)]
pub struct EfiAtaPassThruCommandPacket {
    /// Pointer to sense/status block buffer.
    pub asb: *mut EfiAtaStatusBlock,
    /// Pointer to task file command block buffer.
    pub acb: *mut EfiAtaCommandBlock,
    /// Timeout in 100ns units (0 means wait indefinitely).
    pub timeout: u64,
    /// Data buffer for inbound transfers.
    pub in_data_buffer: *mut c_void,
    /// Data buffer for outbound transfers.
    pub out_data_buffer: *const c_void,
    /// Inbound transfer length in bytes.
    pub in_transfer_length: u32,
    /// Outbound transfer length in bytes.
    pub out_transfer_length: u32,
    /// Protocol used when the ATA device executes command.
    pub protocol: u8,
    /// Way in which the command length is encoded.
    pub length: u8,
}

/// Function pointer for `EFI_ATA_PASS_THRU_PASSTHRU`.
pub type EfiAtaPassThruPassThru = unsafe extern "efiapi" fn(
    this: *mut EfiAtaPassThruProtocol,
    port: u16,
    port_multiplier_port: u16,
    packet: *mut EfiAtaPassThruCommandPacket,
    event: EfiEvent,
) -> EfiStatus;

/// Function pointer for `EFI_ATA_PASS_THRU_GET_NEXT_PORT`.
pub type EfiAtaPassThruGetNextPort =
    unsafe extern "efiapi" fn(this: *mut EfiAtaPassThruProtocol, port: *mut u16) -> EfiStatus;

/// Function pointer for `EFI_ATA_PASS_THRU_GET_NEXT_DEVICE`.
pub type EfiAtaPassThruGetNextDevice = unsafe extern "efiapi" fn(
    this: *mut EfiAtaPassThruProtocol,
    port: u16,
    port_multiplier_port: *mut u16,
) -> EfiStatus;

/// Function pointer for `EFI_ATA_PASS_THRU_BUILD_DEVICE_PATH`.
pub type EfiAtaPassThruBuildDevicePath = unsafe extern "efiapi" fn(
    this: *mut EfiAtaPassThruProtocol,
    port: u16,
    port_multiplier_port: u16,
    device_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function pointer for `EFI_ATA_PASS_THRU_GET_DEVICE`.
pub type EfiAtaPassThruGetDevice = unsafe extern "efiapi" fn(
    this: *mut EfiAtaPassThruProtocol,
    device_path: *const EfiDevicePath,
    port: *mut u16,
    port_multiplier_port: *mut u16,
) -> EfiStatus;

/// Function pointer for `EFI_ATA_PASS_THRU_RESET_PORT`.
pub type EfiAtaPassThruResetPort =
    unsafe extern "efiapi" fn(this: *mut EfiAtaPassThruProtocol, port: u16) -> EfiStatus;

/// Function pointer for `EFI_ATA_PASS_THRU_RESET_DEVICE`.
pub type EfiAtaPassThruResetDevice = unsafe extern "efiapi" fn(
    this: *mut EfiAtaPassThruProtocol,
    port: u16,
    port_multiplier_port: u16,
) -> EfiStatus;

/// The `EFI_ATA_PASS_THRU_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiAtaPassThruProtocol {
    /// Controller mode configuration pointer.
    pub mode: *const EfiAtaPassThruMode,
    /// Sends an ATA command packet to an attached device.
    pub pass_thru: EfiAtaPassThruPassThru,
    /// Retrieves next legal port number.
    pub get_next_port: EfiAtaPassThruGetNextPort,
    /// Retrieves next legal port multiplier port number.
    pub get_next_device: EfiAtaPassThruGetNextDevice,
    /// Builds a device path for an ATA device.
    pub build_device_path: EfiAtaPassThruBuildDevicePath,
    /// Resolves an ATA port and multiplier port from a device path.
    pub get_device: EfiAtaPassThruGetDevice,
    /// Resets an ATA port and all connected devices.
    pub reset_port: EfiAtaPassThruResetPort,
    /// Resets an individual ATA device.
    pub reset_device: EfiAtaPassThruResetDevice,
}

/// ATA ATAPI Policy Protocol GUID.
///
/// `{e59cd769-5083-4f26-9094-6c919f916c4e}`
pub const ATA_ATAPI_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE59C_D769,
    0x5083,
    0x4F26,
    [0x90, 0x94, 0x6C, 0x91, 0x9F, 0x91, 0x6C, 0x4E],
);

/// ATA ATAPI Policy Protocol revision (1.0).
pub const ATA_ATAPI_POLICY_VERSION: u32 = 0x0001_0000;

/// Power-Up In Standby (PUIS) settings.
pub mod puis {
    /// Disable PUIS feature.
    pub const DISABLE: u8 = 0;
    /// Enable PUIS feature.
    pub const ENABLE: u8 = 1;
    /// Retain current non-volatile hardware PUIS setting.
    pub const KEEP_HARDWARE: u8 = 2;
}

/// Device Sleep (`DevSleep`) settings.
pub mod devsleep {
    /// Disable Device Sleep.
    pub const DISABLE: u8 = 0;
    /// Enable Device Sleep.
    pub const ENABLE: u8 = 1;
}

/// EFI ATA ATAPI Policy Protocol structure.
///
/// Produced by platform firmware and consumed by ATA/ATAPI pass-thru driver
/// to configure Power-Up In Standby and aggressive Device Sleep modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiAtaAtapiPolicyProtocol {
    /// Protocol version ([`ATA_ATAPI_POLICY_VERSION`]).
    pub version: u32,
    /// 0: Disable PUIS, 1: Enable PUIS, other: Keep hardware setting.
    pub puis_enable: u8,
    /// 0: Disable Device Sleep, 1: Enable Device Sleep.
    pub device_sleep_enable: u8,
    /// 0: Disable Aggressive Device Sleep, 1: Enable Aggressive Device Sleep.
    pub aggressive_device_sleep_enable: u8,
    /// Reserved byte for structure alignment.
    pub reserved: u8,
}

/// Safe client wrapper around [`EfiAtaAtapiPolicyProtocol`].
pub struct AtaAtapiPolicy<'a> {
    proto: &'a EfiAtaAtapiPolicyProtocol,
}

impl<'a> AtaAtapiPolicy<'a> {
    /// Wraps a reference to [`EfiAtaAtapiPolicyProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiAtaAtapiPolicyProtocol) -> Self {
        Self { proto }
    }

    /// Returns protocol specification version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.proto.version
    }

    /// Returns whether Power-Up In Standby is explicitly enabled.
    #[must_use]
    pub const fn is_puis_enabled(&self) -> bool {
        self.proto.puis_enable == puis::ENABLE
    }

    /// Returns whether Device Sleep is enabled.
    #[must_use]
    pub const fn is_device_sleep_enabled(&self) -> bool {
        self.proto.device_sleep_enable == devsleep::ENABLE
    }

    /// Returns whether Aggressive Device Sleep is enabled.
    #[must_use]
    pub const fn is_aggressive_device_sleep_enabled(&self) -> bool {
        self.proto.aggressive_device_sleep_enable == devsleep::ENABLE
    }
}
