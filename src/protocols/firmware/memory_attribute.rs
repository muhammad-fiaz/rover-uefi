//! # Memory Attribute Protocol and Client API
//!
//! Provides retrieval and modification services for page table attributes (e.g.
//! Read-Protect, Execute-Protect, Read-Only, Write-Back caching) in the UEFI environment
//! as introduced in the UEFI Specification 2.10.

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// EFI Memory Attribute Protocol GUID.
pub const EFI_MEMORY_ATTRIBUTE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xF456_0CF6,
    0x40EC,
    0x4B4A,
    [0xA1, 0x92, 0xBF, 0x1D, 0x57, 0xD0, 0xB1, 0x89],
);

/// Memory Attribute Bitmask Flags.
#[allow(clippy::module_inception)]
pub mod memory_attribute {
    /// Uncached memory.
    pub const UC: u64 = 0x0000_0000_0000_0001;
    /// Write-coalescing / write-combining memory.
    pub const WC: u64 = 0x0000_0000_0000_0002;
    /// Write-through cacheable memory.
    pub const WT: u64 = 0x0000_0000_0000_0004;
    /// Write-back cacheable memory.
    pub const WB: u64 = 0x0000_0000_0000_0008;
    /// Uncached, exported / uncacheable, fetch-and-add memory.
    pub const UCE: u64 = 0x0000_0000_0000_0010;
    /// Write-protected memory.
    pub const WP: u64 = 0x0000_0000_0000_1000;
    /// Read-protected memory.
    pub const RP: u64 = 0x0000_0000_0000_2000;
    /// Execute-protected / execute-never (`XP` / `NX`) memory.
    pub const XP: u64 = 0x0000_0000_0000_4000;
    /// Read-only memory.
    pub const RO: u64 = 0x0000_0000_0002_0000;
    /// Non-volatile memory (`NVDIMM`).
    pub const NV: u64 = 0x0000_0000_0000_8000;
    /// Memory requires cryptographic acceptance before access.
    pub const MORE_RELIABLE: u64 = 0x0000_0000_0001_0000;
}

/// Retrieve the attributes of the designated memory region.
pub type EfiGetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *const EfiMemoryAttributeProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: *mut u64,
) -> EfiStatus;

/// Set the given attributes for the designated memory region.
pub type EfiSetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *const EfiMemoryAttributeProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// Clear the given attributes for the designated memory region.
pub type EfiClearMemoryAttributes = unsafe extern "efiapi" fn(
    this: *const EfiMemoryAttributeProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// EFI Memory Attribute Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiMemoryAttributeProtocol {
    /// Retrieve current memory attributes.
    pub get_memory_attributes: Option<EfiGetMemoryAttributes>,
    /// Set memory attributes (e.g. `RO`, `XP`, `RP`).
    pub set_memory_attributes: Option<EfiSetMemoryAttributes>,
    /// Clear memory attributes.
    pub clear_memory_attributes: Option<EfiClearMemoryAttributes>,
}

/// Safe client wrapper for the Memory Attribute protocol.
#[derive(Debug)]
pub struct MemoryAttribute<'a> {
    protocol: &'a EfiMemoryAttributeProtocol,
}

impl<'a> MemoryAttribute<'a> {
    /// Create a new safe view of the Memory Attribute protocol.
    #[must_use]
    pub fn new(protocol: &'a EfiMemoryAttributeProtocol) -> Self {
        Self { protocol }
    }

    /// Retrieve the memory attributes for a designated region.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn get_attributes(&self, base: EfiPhysicalAddress, length: u64) -> Result<u64, EfiStatus> {
        if let Some(get_fn) = self.protocol.get_memory_attributes {
            let mut attrs = 0u64;
            let status = unsafe { get_fn(self.protocol, base, length, &raw mut attrs) };
            if status == EfiStatus::SUCCESS {
                Ok(attrs)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Set memory attributes for a designated region.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn set_attributes(
        &self,
        base: EfiPhysicalAddress,
        length: u64,
        attrs: u64,
    ) -> Result<(), EfiStatus> {
        if let Some(set_fn) = self.protocol.set_memory_attributes {
            let status = unsafe { set_fn(self.protocol, base, length, attrs) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Clear memory attributes for a designated region.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn clear_attributes(
        &self,
        base: EfiPhysicalAddress,
        length: u64,
        attrs: u64,
    ) -> Result<(), EfiStatus> {
        if let Some(clear_fn) = self.protocol.clear_memory_attributes {
            let status = unsafe { clear_fn(self.protocol, base, length, attrs) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}

/// SMM Memory Attribute Protocol GUID.
///
/// `{69b792ea-39ce-402d-a2a6-f721de351dfe}`
pub const SMM_MEMORY_ATTRIBUTE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x69B7_92EA,
    0x39CE,
    0x402D,
    [0xA2, 0xA6, 0xF7, 0x21, 0xDE, 0x35, 0x1D, 0xFE],
);

/// Function pointer prototype to retrieve memory attributes in SMM.
pub type SmmGetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *const SmmMemoryAttributeProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: *mut u64,
) -> EfiStatus;

/// Function pointer prototype to set memory attributes in SMM.
pub type SmmSetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *const SmmMemoryAttributeProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// Function pointer prototype to clear memory attributes in SMM.
pub type SmmClearMemoryAttributes = unsafe extern "efiapi" fn(
    this: *const SmmMemoryAttributeProtocol,
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// SMM Memory Attribute Protocol.
///
/// Provides retrieval and modification services for page table attributes in the EFI SMM environment.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SmmMemoryAttributeProtocol {
    /// Retrieve attributes for a given memory region in SMM.
    pub get_memory_attributes: Option<SmmGetMemoryAttributes>,
    /// Set attributes for a given memory region in SMM.
    pub set_memory_attributes: Option<SmmSetMemoryAttributes>,
    /// Clear attributes for a given memory region in SMM.
    pub clear_memory_attributes: Option<SmmClearMemoryAttributes>,
}

/// Safe client wrapper for the SMM Memory Attribute protocol.
#[derive(Debug)]
pub struct SmmMemoryAttribute<'a> {
    protocol: &'a SmmMemoryAttributeProtocol,
}

impl<'a> SmmMemoryAttribute<'a> {
    /// Create a new safe view of the SMM Memory Attribute protocol.
    #[must_use]
    pub fn new(protocol: &'a SmmMemoryAttributeProtocol) -> Self {
        Self { protocol }
    }

    /// Retrieve the memory attributes for a designated region in SMM.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn get_attributes(&self, base: EfiPhysicalAddress, length: u64) -> Result<u64, EfiStatus> {
        let mut attrs = 0u64;
        if let Some(get_fn) = self.protocol.get_memory_attributes {
            let status = unsafe { get_fn(self.protocol, base, length, &raw mut attrs) };
            if status == EfiStatus::SUCCESS {
                Ok(attrs)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Set memory attributes for a designated region in SMM.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn set_attributes(
        &self,
        base: EfiPhysicalAddress,
        length: u64,
        attrs: u64,
    ) -> Result<(), EfiStatus> {
        if let Some(set_fn) = self.protocol.set_memory_attributes {
            let status = unsafe { set_fn(self.protocol, base, length, attrs) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Clear memory attributes for a designated region in SMM.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn clear_attributes(
        &self,
        base: EfiPhysicalAddress,
        length: u64,
        attrs: u64,
    ) -> Result<(), EfiStatus> {
        if let Some(clear_fn) = self.protocol.clear_memory_attributes {
            let status = unsafe { clear_fn(self.protocol, base, length, attrs) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
