//! # IOMMU Protocol
//!
//! Defined in EDK II `MdeModulePkg`.
//!
//! Provides hardware IOMMU DMA access control, address mapping, device-level
//! isolation, and common DMA buffer allocation for `PCIe` and `SoC` peripherals.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};

/// EDK II IOMMU Protocol GUID: `{4e939de9-d948-4b0f-88ed-e6e1ce517c1e}`.
pub const EDKII_IOMMU_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4E93_9DE9,
    0xD948,
    0x4B0F,
    [0x88, 0xED, 0xE6, 0xE1, 0xCE, 0x51, 0x7C, 0x1E],
);

/// Current revision of the IOMMU protocol interface.
pub const EDKII_IOMMU_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Read access permission for DMA transfers.
pub const EDKII_IOMMU_ACCESS_READ: u64 = 0x01;
/// Write access permission for DMA transfers.
pub const EDKII_IOMMU_ACCESS_WRITE: u64 = 0x02;

/// Memory attribute for write-combining DMA buffer.
pub const EDKII_IOMMU_ATTRIBUTE_MEMORY_WRITE_COMBINE: u64 = 0x0080;
/// Memory attribute for cached DMA buffer.
pub const EDKII_IOMMU_ATTRIBUTE_MEMORY_CACHED: u64 = 0x0800;
/// Memory attribute for 64-bit Dual Address Cycle (DAC) capable buffers.
pub const EDKII_IOMMU_ATTRIBUTE_DUAL_ADDRESS_CYCLE: u64 = 0x8000;

/// Bus master DMA operation type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdkiiIoMmuOperation {
    /// Read from system memory by a 32-bit bus master.
    BusMasterRead = 0,
    /// Write to system memory by a 32-bit bus master.
    BusMasterWrite = 1,
    /// Common buffer bidirectional access by a 32-bit bus master.
    BusMasterCommonBuffer = 2,
    /// Read from system memory by a 64-bit bus master.
    BusMasterRead64 = 3,
    /// Write to system memory by a 64-bit bus master.
    BusMasterWrite64 = 4,
    /// Common buffer bidirectional access by a 64-bit bus master.
    BusMasterCommonBuffer64 = 5,
}

/// Sets IOMMU access permissions for a mapped DMA buffer.
pub type EdkiiIoMmuSetAttribute = unsafe extern "efiapi" fn(
    this: *mut EdkiiIoMmuProtocol,
    device_handle: EfiHandle,
    mapping: *mut (),
    iommu_access: u64,
) -> EfiStatus;

/// Maps a system host address to a device-accessible DMA address.
pub type EdkiiIoMmuMap = unsafe extern "efiapi" fn(
    this: *mut EdkiiIoMmuProtocol,
    operation: EdkiiIoMmuOperation,
    host_address: *mut (),
    number_of_bytes: *mut UIntN,
    device_address: *mut EfiPhysicalAddress,
    mapping: *mut *mut (),
) -> EfiStatus;

/// Unmaps a DMA range and flushes any pending IOMMU translations.
pub type EdkiiIoMmuUnmap =
    unsafe extern "efiapi" fn(this: *mut EdkiiIoMmuProtocol, mapping: *mut ()) -> EfiStatus;

/// Allocates memory pages suitable for bus master common buffer DMA.
pub type EdkiiIoMmuAllocateBuffer = unsafe extern "efiapi" fn(
    this: *mut EdkiiIoMmuProtocol,
    alloc_type: u32,
    memory_type: u32,
    pages: UIntN,
    host_address: *mut *mut (),
    attributes: u64,
) -> EfiStatus;

/// Frees pages previously allocated with `AllocateBuffer`.
pub type EdkiiIoMmuFreeBuffer = unsafe extern "efiapi" fn(
    this: *mut EdkiiIoMmuProtocol,
    pages: UIntN,
    host_address: *mut (),
) -> EfiStatus;

/// EDK II IOMMU Protocol interface (`EDKII_IOMMU_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EdkiiIoMmuProtocol {
    /// Interface revision.
    pub revision: u64,
    /// Sets IOMMU DMA access attributes.
    pub set_attribute: EdkiiIoMmuSetAttribute,
    /// Maps host buffer to DMA address.
    pub map: EdkiiIoMmuMap,
    /// Unmaps DMA mapping.
    pub unmap: EdkiiIoMmuUnmap,
    /// Allocates DMA common buffer.
    pub allocate_buffer: EdkiiIoMmuAllocateBuffer,
    /// Frees DMA common buffer.
    pub free_buffer: EdkiiIoMmuFreeBuffer,
}
