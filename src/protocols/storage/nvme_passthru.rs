//! # NVM Express Pass Thru Protocol
//!
//! Declarations for `EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL` according to the
//! UEFI Specification.
//!
//! Provides direct low-level `NVMe` command submission to controllers and
//! namespaces.

use crate::base::{EfiEvent, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;
use core::ffi::c_void;

pub use crate::protocols::storage::guid::{
    NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID as NVME_PASS_THRU_PROTOCOL_GUID,
    NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID,
};

/// Interface is for physical devices/namespaces on the controller.
pub const NVME_PASS_THRU_ATTRIBUTES_PHYSICAL: u32 = 0x0001;
/// Interface is for a logical namespace.
pub const NVME_PASS_THRU_ATTRIBUTES_LOGICAL: u32 = 0x0002;
/// Interface supports non-blocking I/O.
pub const NVME_PASS_THRU_ATTRIBUTES_NONBLOCKIO: u32 = 0x0004;

/// Submission queue type: Admin Submission Queue.
pub const NVME_ADMIN_SUBMISSION_QUEUE: u8 = 0x00;
/// Submission queue type: I/O Submission Queue.
pub const NVME_IO_SUBMISSION_QUEUE: u8 = 0x01;

/// Mode structure describing `NVMe` pass-thru controller capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiNvmExpressPassThruMode {
    /// Controller attributes bitmask.
    pub attributes: u32,
    /// Minimum required alignment for data buffers.
    pub io_align: u32,
    /// Major and minor `NVMe` version supported.
    pub nvme_version: u32,
}

/// `NVMe` command doubleword 0 bitfield.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct NvmeCdw0(pub u32);

impl NvmeCdw0 {
    /// Creates a new CDW0 with the given opcode and fused operation.
    #[must_use]
    pub const fn new(opcode: u8, fused_op: u8) -> Self {
        Self((opcode as u32) | (((fused_op & 0x03) as u32) << 8))
    }

    /// Returns the opcode field (bits 7:0).
    #[must_use]
    pub const fn opcode(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the fused operation field (bits 9:8).
    #[must_use]
    pub const fn fused_op(&self) -> u8 {
        ((self.0 >> 8) & 0x03) as u8
    }
}

/// `NVMe` submission command structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiNvmExpressCommand {
    /// Command `DWord` 0.
    pub cdw0: NvmeCdw0,
    /// Bitmask indicating valid command `DWords` (`CDW2..CDW14`).
    pub flags: u8,
    /// Namespace identifier.
    pub nsid: u32,
    /// Command `DWord` 2.
    pub cdw2: u32,
    /// Command `DWord` 3.
    pub cdw3: u32,
    /// Command `DWord` 10.
    pub cdw10: u32,
    /// Command `DWord` 11.
    pub cdw11: u32,
    /// Command `DWord` 12.
    pub cdw12: u32,
    /// Command `DWord` 13.
    pub cdw13: u32,
    /// Command `DWord` 14.
    pub cdw14: u32,
    /// Command `DWord` 15.
    pub cdw15: u32,
}

/// `NVMe` completion queue entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiNvmExpressCompletion {
    /// Completion `DWord` 0.
    pub dw0: u32,
    /// Completion `DWord` 1.
    pub dw1: u32,
    /// Completion `DWord` 2.
    pub dw2: u32,
    /// Completion `DWord` 3.
    pub dw3: u32,
}

/// Packet describing an `NVMe` pass-thru transaction.
#[derive(Debug)]
#[repr(C)]
pub struct EfiNvmExpressPassThruCommandPacket {
    /// Command timeout in 100ns units.
    pub command_timeout: u64,
    /// Pointer to the host data transfer buffer.
    pub transfer_buffer: *mut c_void,
    /// Length of the transfer buffer in bytes.
    pub transfer_length: u32,
    /// Pointer to optional metadata buffer.
    pub metadata_buffer: *mut c_void,
    /// Length of metadata buffer in bytes.
    pub metadata_length: u32,
    /// Submission queue type (`NVME_ADMIN_SUBMISSION_QUEUE` or `NVME_IO_SUBMISSION_QUEUE`).
    pub queue_type: u8,
    /// Reserved byte padding.
    pub reserved: [u8; 7],
    /// Pointer to the `NVMe` command to execute.
    pub nvme_cmd: *const EfiNvmExpressCommand,
    /// Pointer to the completion buffer.
    pub nvme_completion: *mut EfiNvmExpressCompletion,
}

/// Function pointer for `EFI_NVM_EXPRESS_PASS_THRU_PASSTHRU`.
pub type EfiNvmExpressPassThruPassThru = unsafe extern "efiapi" fn(
    this: *mut EfiNvmExpressPassThruProtocol,
    namespace_id: u32,
    packet: *mut EfiNvmExpressPassThruCommandPacket,
    event: EfiEvent,
) -> EfiStatus;

/// Function pointer for `EFI_NVM_EXPRESS_PASS_THRU_GET_NEXT_NAMESPACE`.
pub type EfiNvmExpressPassThruGetNextNamespace = unsafe extern "efiapi" fn(
    this: *mut EfiNvmExpressPassThruProtocol,
    namespace_id: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_NVM_EXPRESS_PASS_THRU_BUILD_DEVICE_PATH`.
pub type EfiNvmExpressPassThruBuildDevicePath = unsafe extern "efiapi" fn(
    this: *mut EfiNvmExpressPassThruProtocol,
    namespace_id: u32,
    device_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function pointer for `EFI_NVM_EXPRESS_PASS_THRU_GET_NAMESPACE`.
pub type EfiNvmExpressPassThruGetNamespace = unsafe extern "efiapi" fn(
    this: *mut EfiNvmExpressPassThruProtocol,
    device_path: *const EfiDevicePath,
    namespace_id: *mut u32,
) -> EfiStatus;

/// The `EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiNvmExpressPassThruProtocol {
    /// Pointer to controller mode configuration.
    pub mode: *const EfiNvmExpressPassThruMode,
    /// Submits an `NVMe` command packet to the controller.
    pub pass_thru: EfiNvmExpressPassThruPassThru,
    /// Enumerates valid namespace identifiers.
    pub get_next_namespace: EfiNvmExpressPassThruGetNextNamespace,
    /// Builds a device path for a namespace.
    pub build_device_path: EfiNvmExpressPassThruBuildDevicePath,
    /// Resolves a namespace identifier from a device path.
    pub get_namespace: EfiNvmExpressPassThruGetNamespace,
}
