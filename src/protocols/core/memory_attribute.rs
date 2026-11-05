//! # Memory Attribute Protocol
//!
//! UEFI 2.10 Memory Attribute Protocol definition and safe wrapper.
//! Provides services to retrieve or update memory protection attributes
//! (such as read-protect, execute-protect, read-only).

use crate::base::{EfiGuid, EfiStatus};

/// Memory Attribute Protocol GUID: `f4560cf6-40ec-4b4a-a192-bf1d57d0b189`
pub const MEMORY_ATTRIBUTE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xf456_0cf6,
    0x40ec,
    0x4b4a,
    [0xa1, 0x92, 0xbf, 0x1d, 0x57, 0xd0, 0xb1, 0x89],
);

/// Attribute bit indicating read-protect (access fault on read).
pub const EFI_MEMORY_RP: u64 = 0x0000_2000;

/// Attribute bit indicating execute-protect (instruction fetch fault / NX).
pub const EFI_MEMORY_XP: u64 = 0x0000_4000;

/// Attribute bit indicating read-only (access fault on write / WP).
pub const EFI_MEMORY_RO: u64 = 0x0002_0000;

/// Valid mask of settable/clearable memory protection attributes.
pub const EFI_MEMORY_ATTRIBUTE_MASK: u64 = EFI_MEMORY_RP | EFI_MEMORY_XP | EFI_MEMORY_RO;

/// Function pointer type for `GetMemoryAttributes`.
pub type EfiGetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryAttributeProtocol,
    base_address: u64,
    length: u64,
    attributes: *mut u64,
) -> EfiStatus;

/// Function pointer type for `SetMemoryAttributes`.
pub type EfiSetMemoryAttributes = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryAttributeProtocol,
    base_address: u64,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// Function pointer type for `ClearMemoryAttributes`.
pub type EfiClearMemoryAttributes = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryAttributeProtocol,
    base_address: u64,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// Raw UEFI Memory Attribute Protocol interface (`EFI_MEMORY_ATTRIBUTE_PROTOCOL`).
#[repr(C)]
pub struct EfiMemoryAttributeProtocol {
    /// Retrieves current memory protection attributes.
    pub get_memory_attributes: EfiGetMemoryAttributes,
    /// Sets memory protection attributes.
    pub set_memory_attributes: EfiSetMemoryAttributes,
    /// Clears memory protection attributes.
    pub clear_memory_attributes: EfiClearMemoryAttributes,
}

/// Safe wrapper around `EfiMemoryAttributeProtocol`.
pub struct MemoryAttributeProtocol<'a> {
    raw: &'a mut EfiMemoryAttributeProtocol,
}

impl MemoryAttributeProtocol<'_> {
    /// Wraps a raw `EfiMemoryAttributeProtocol` instance.
    ///
    /// # Safety
    /// The caller must ensure that `raw` points to a valid, live protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut EfiMemoryAttributeProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            // SAFETY: Caller guarantees `raw` points to a valid, live protocol instance.
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Retrieves attributes for the specified memory region.
    ///
    /// Returns `(EfiStatus, u64)`. On success, returns `(EfiStatus::SUCCESS, attributes)`.
    pub fn get_attributes(&mut self, base_address: u64, length: u64) -> (EfiStatus, u64) {
        if length == 0 {
            return (EfiStatus::INVALID_PARAMETER, 0);
        }
        let mut attrs: u64 = 0;
        let status = unsafe {
            (self.raw.get_memory_attributes)(
                core::ptr::from_mut(self.raw),
                base_address,
                length,
                &raw mut attrs,
            )
        };
        (status, attrs)
    }

    /// Sets protection attributes for the specified memory range.
    pub fn set_attributes(&mut self, base_address: u64, length: u64, attributes: u64) -> EfiStatus {
        if length == 0 || (attributes & !EFI_MEMORY_ATTRIBUTE_MASK) != 0 {
            return EfiStatus::INVALID_PARAMETER;
        }
        unsafe {
            (self.raw.set_memory_attributes)(
                core::ptr::from_mut(self.raw),
                base_address,
                length,
                attributes,
            )
        }
    }

    /// Clears protection attributes for the specified memory range.
    pub fn clear_attributes(
        &mut self,
        base_address: u64,
        length: u64,
        attributes: u64,
    ) -> EfiStatus {
        if length == 0 || (attributes & !EFI_MEMORY_ATTRIBUTE_MASK) != 0 {
            return EfiStatus::INVALID_PARAMETER;
        }
        unsafe {
            (self.raw.clear_memory_attributes)(
                core::ptr::from_mut(self.raw),
                base_address,
                length,
                attributes,
            )
        }
    }
}
