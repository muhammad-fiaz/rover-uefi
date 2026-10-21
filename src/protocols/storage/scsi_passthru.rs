//! # Extended SCSI Pass Thru Protocol
//!
//! Declarations for `EFI_EXT_SCSI_PASS_THRU_PROTOCOL` according to the
//! UEFI Specification.
//!
//! Provides direct low-level SCSI command submission to targets and LUNs.

use crate::base::{EfiEvent, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;
use core::ffi::c_void;

pub use crate::protocols::storage::guid::{
    EXT_SCSI_PASS_THRU_PROTOCOL_GUID as SCSI_PASS_THRU_GUID, EXT_SCSI_PASS_THRU_PROTOCOL_GUID,
};

/// Maximum bytes for target port identifier array.
pub const TARGET_MAX_BYTES: usize = 16;

/// Interface is for physical devices on the SCSI channel.
pub const EXT_SCSI_PASS_THRU_ATTRIBUTES_PHYSICAL: u32 = 0x0001;
/// Interface is for logical devices on the SCSI channel.
pub const EXT_SCSI_PASS_THRU_ATTRIBUTES_LOGICAL: u32 = 0x0002;
/// Interface supports non-blocking I/O.
pub const EXT_SCSI_PASS_THRU_ATTRIBUTES_NONBLOCKIO: u32 = 0x0004;

/// Data direction: Inbound read transfer.
pub const EXT_SCSI_DATA_DIRECTION_READ: u8 = 0;
/// Data direction: Outbound write transfer.
pub const EXT_SCSI_DATA_DIRECTION_WRITE: u8 = 1;
/// Data direction: Bidirectional transfer.
pub const EXT_SCSI_DATA_DIRECTION_BIDIRECTIONAL: u8 = 2;

/// Mode structure describing extended SCSI pass-thru channel capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiExtScsiPassThruMode {
    /// Adapter Target ID on the SCSI channel.
    pub adapter_id: u32,
    /// Channel attributes bitmask.
    pub attributes: u32,
    /// Alignment requirement for data transfer buffers.
    pub io_align: u32,
}

/// Packet describing an extended SCSI Request Packet.
#[derive(Debug)]
#[repr(C)]
pub struct EfiExtScsiPassThruScsiRequestPacket {
    /// Timeout in 100ns units (0 means wait indefinitely).
    pub timeout: u64,
    /// Inbound data buffer pointer.
    pub in_data_buffer: *mut c_void,
    /// Outbound data buffer pointer.
    pub out_data_buffer: *const c_void,
    /// Sense data buffer pointer.
    pub sense_data: *mut c_void,
    /// Command Data Block pointer.
    pub cdb: *const c_void,
    /// Inbound transfer length in bytes.
    pub in_transfer_length: u32,
    /// Outbound transfer length in bytes.
    pub out_transfer_length: u32,
    /// Length of the CDB buffer in bytes.
    pub cdb_length: u8,
    /// Data direction (`EXT_SCSI_DATA_DIRECTION_*`).
    pub data_direction: u8,
    /// Host adapter status code.
    pub host_adapter_status: u8,
    /// Target status code.
    pub target_status: u8,
    /// Sense data buffer length in bytes.
    pub sense_data_length: u8,
}

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_PASSTHRU`.
pub type EfiExtScsiPassThruPassThru = unsafe extern "efiapi" fn(
    this: *mut EfiExtScsiPassThruProtocol,
    target: *const u8,
    lun: u64,
    packet: *mut EfiExtScsiPassThruScsiRequestPacket,
    event: EfiEvent,
) -> EfiStatus;

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_GET_NEXT_TARGET_LUN`.
pub type EfiExtScsiPassThruGetNextTargetLun = unsafe extern "efiapi" fn(
    this: *mut EfiExtScsiPassThruProtocol,
    target: *mut *mut u8,
    lun: *mut u64,
) -> EfiStatus;

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_BUILD_DEVICE_PATH`.
pub type EfiExtScsiPassThruBuildDevicePath = unsafe extern "efiapi" fn(
    this: *mut EfiExtScsiPassThruProtocol,
    target: *const u8,
    lun: u64,
    device_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_GET_TARGET_LUN`.
pub type EfiExtScsiPassThruGetTargetLun = unsafe extern "efiapi" fn(
    this: *mut EfiExtScsiPassThruProtocol,
    device_path: *const EfiDevicePath,
    target: *mut *mut u8,
    lun: *mut u64,
) -> EfiStatus;

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_RESET_CHANNEL`.
pub type EfiExtScsiPassThruResetChannel =
    unsafe extern "efiapi" fn(this: *mut EfiExtScsiPassThruProtocol) -> EfiStatus;

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_RESET_TARGET_LUN`.
pub type EfiExtScsiPassThruResetTargetLun = unsafe extern "efiapi" fn(
    this: *mut EfiExtScsiPassThruProtocol,
    target: *const u8,
    lun: u64,
) -> EfiStatus;

/// Function pointer for `EFI_EXT_SCSI_PASS_THRU_GET_NEXT_TARGET`.
pub type EfiExtScsiPassThruGetNextTarget = unsafe extern "efiapi" fn(
    this: *mut EfiExtScsiPassThruProtocol,
    target: *mut *mut u8,
) -> EfiStatus;

/// The `EFI_EXT_SCSI_PASS_THRU_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiExtScsiPassThruProtocol {
    /// Pointer to SCSI channel mode configuration.
    pub mode: *const EfiExtScsiPassThruMode,
    /// Sends a SCSI Request Packet to a target and LUN.
    pub pass_thru: EfiExtScsiPassThruPassThru,
    /// Enumerates target and LUN pairs.
    pub get_next_target_lun: EfiExtScsiPassThruGetNextTargetLun,
    /// Builds a device path for a target and LUN.
    pub build_device_path: EfiExtScsiPassThruBuildDevicePath,
    /// Resolves target and LUN from a device path.
    pub get_target_lun: EfiExtScsiPassThruGetTargetLun,
    /// Resets an entire SCSI channel.
    pub reset_channel: EfiExtScsiPassThruResetChannel,
    /// Resets a specific logical unit.
    pub reset_target_lun: EfiExtScsiPassThruResetTargetLun,
    /// Enumerates target IDs.
    pub get_next_target: EfiExtScsiPassThruGetNextTarget,
}
