//! # IOMMU Protocol and Client API
//!
//! Provides hardware IOMMU DMA access control, address mapping, device-level
//! isolation, and common DMA buffer allocation for `PCIe` and `SoC` peripherals.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};

/// IOMMU Protocol GUID: `{4e939de9-d948-4b0f-88ed-e6e1ce517c1e}`.
pub const IOMMU_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4E93_9DE9,
    0xD948,
    0x4B0F,
    [0x88, 0xED, 0xE6, 0xE1, 0xCE, 0x51, 0x7C, 0x1E],
);

/// Current revision of the IOMMU protocol interface.
pub const IOMMU_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Read access permission for DMA transfers.
pub const IOMMU_ACCESS_READ: u64 = 0x01;

/// Write access permission for DMA transfers.
pub const IOMMU_ACCESS_WRITE: u64 = 0x02;

/// Memory attribute for write-combining DMA buffer.
pub const IOMMU_ATTRIBUTE_MEMORY_WRITE_COMBINE: u64 = 0x0080;

/// Memory attribute for cached DMA buffer.
pub const IOMMU_ATTRIBUTE_MEMORY_CACHED: u64 = 0x0800;

/// Memory attribute for 64-bit Dual Address Cycle (`DAC`) capable buffers.
pub const IOMMU_ATTRIBUTE_DUAL_ADDRESS_CYCLE: u64 = 0x8000;

/// Bus master DMA operation type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoMmuOperation {
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
pub type IoMmuSetAttribute = unsafe extern "efiapi" fn(
    this: *mut IoMmuProtocol,
    device_handle: EfiHandle,
    mapping: *mut (),
    iommu_access: u64,
) -> EfiStatus;

/// Maps a system host address to a device-accessible DMA address.
pub type IoMmuMap = unsafe extern "efiapi" fn(
    this: *mut IoMmuProtocol,
    operation: IoMmuOperation,
    host_address: *mut (),
    number_of_bytes: *mut UIntN,
    device_address: *mut EfiPhysicalAddress,
    mapping: *mut *mut (),
) -> EfiStatus;

/// Unmaps a DMA range and flushes any pending IOMMU translations.
pub type IoMmuUnmap =
    unsafe extern "efiapi" fn(this: *mut IoMmuProtocol, mapping: *mut ()) -> EfiStatus;

/// Allocates memory pages suitable for bus master common buffer DMA.
pub type IoMmuAllocateBuffer = unsafe extern "efiapi" fn(
    this: *mut IoMmuProtocol,
    alloc_type: u32,
    memory_type: u32,
    pages: UIntN,
    host_address: *mut *mut (),
    attributes: u64,
) -> EfiStatus;

/// Frees pages previously allocated with `AllocateBuffer`.
pub type IoMmuFreeBuffer = unsafe extern "efiapi" fn(
    this: *mut IoMmuProtocol,
    pages: UIntN,
    host_address: *mut (),
) -> EfiStatus;

/// Raw IOMMU Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IoMmuProtocol {
    /// Interface revision.
    pub revision: u64,
    /// Sets IOMMU DMA access attributes.
    pub set_attribute: IoMmuSetAttribute,
    /// Maps host buffer to DMA address.
    pub map: IoMmuMap,
    /// Unmaps DMA mapping.
    pub unmap: IoMmuUnmap,
    /// Allocates DMA common buffer.
    pub allocate_buffer: IoMmuAllocateBuffer,
    /// Frees DMA common buffer.
    pub free_buffer: IoMmuFreeBuffer,
}

/// Safe native client-side wrapper for the IOMMU protocol.
#[derive(Debug)]
pub struct IoMmu<'a> {
    protocol: &'a mut IoMmuProtocol,
}

impl<'a> IoMmu<'a> {
    /// Create a new safe IOMMU client wrapper.
    #[must_use]
    pub fn new(protocol: &'a mut IoMmuProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Set IOMMU attributes for a device and mapping.
    ///
    /// # Safety
    /// Caller must ensure `mapping` points to valid IOMMU tracking context.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware IOMMU protocol.
    pub unsafe fn set_attribute(
        &mut self,
        device_handle: EfiHandle,
        mapping: *mut (),
        access: u64,
    ) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.protocol.set_attribute)(self.protocol, device_handle, mapping, access) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Map a host buffer for bus master DMA.
    ///
    /// # Safety
    /// Caller must ensure `host_address` is a valid memory buffer of at least `bytes` length.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware IOMMU protocol.
    pub unsafe fn map(
        &mut self,
        operation: IoMmuOperation,
        host_address: *mut (),
        bytes: usize,
    ) -> Result<(EfiPhysicalAddress, *mut (), usize), EfiStatus> {
        let mut num_bytes = bytes;
        let mut dev_addr = EfiPhysicalAddress::NULL;
        let mut mapping: *mut () = core::ptr::null_mut();

        let status = unsafe {
            (self.protocol.map)(
                self.protocol,
                operation,
                host_address,
                &raw mut num_bytes,
                &raw mut dev_addr,
                &raw mut mapping,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((dev_addr, mapping, num_bytes))
        } else {
            Err(status)
        }
    }

    /// Unmap a previously mapped DMA range.
    ///
    /// # Safety
    /// Caller must pass a valid `mapping` handle returned by `map`.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware IOMMU protocol.
    pub unsafe fn unmap(&mut self, mapping: *mut ()) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unmap)(self.protocol, mapping) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Allocate pages for common buffer DMA.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware IOMMU protocol.
    pub fn allocate_buffer(
        &mut self,
        alloc_type: u32,
        memory_type: u32,
        pages: usize,
        attributes: u64,
    ) -> Result<*mut (), EfiStatus> {
        let mut host_address: *mut () = core::ptr::null_mut();
        let status = unsafe {
            (self.protocol.allocate_buffer)(
                self.protocol,
                alloc_type,
                memory_type,
                pages,
                &raw mut host_address,
                attributes,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(host_address)
        } else {
            Err(status)
        }
    }

    /// Free pages previously allocated for common buffer DMA.
    ///
    /// # Safety
    /// Caller must ensure `host_address` points to a buffer allocated via `allocate_buffer`.
    ///
    /// # Errors
    /// Returns the underlying status reported by the firmware IOMMU protocol.
    pub unsafe fn free_buffer(
        &mut self,
        pages: usize,
        host_address: *mut (),
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.free_buffer)(self.protocol, pages, host_address) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
