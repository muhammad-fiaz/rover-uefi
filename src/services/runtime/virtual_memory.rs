//! # Virtual memory
//!
//! The physical-to-virtual address transition.
//!
//! `SetVirtualAddressMap` must be called exactly
//! once by the operating system loader after
//! `ExitBootServices`. The map must describe
//! every runtime range with correct physical
//! and virtual addresses. An incorrect map
//! will crash runtime services. For that
//! reason the transition methods are unsafe.
//!
//! Descriptor validation and size calculations
//! are pure helpers with checked arithmetic.

use crate::abi::types::UIntN;
use crate::base::{EfiMemoryDescriptor, EfiStatus};
use crate::services::runtime::RuntimeServices;
use core::ffi::c_void;

/// The optional pointer disposition.
pub const OPTIONAL_PTR: UIntN = 0x0000_0001;

/// Returns whether the descriptor size is valid.
///
/// The size must fit at least one descriptor.
#[must_use]
pub fn validate_descriptor_size(descriptor_size: UIntN) -> bool {
    descriptor_size >= core::mem::size_of::<EfiMemoryDescriptor>() as UIntN && descriptor_size != 0
}

/// Returns the descriptor count for a map size.
///
/// Returns `None` when the size is zero or
/// not a multiple of the stride.
#[must_use]
pub fn descriptor_count(map_size: UIntN, descriptor_size: UIntN) -> Option<UIntN> {
    if descriptor_size == 0 || !map_size.is_multiple_of(descriptor_size) {
        return None;
    }
    Some(map_size / descriptor_size)
}

/// Returns the checked map size for a count.
///
/// Returns `None` on multiplication overflow.
#[must_use]
pub fn checked_map_size(count: UIntN, full_stride: UIntN) -> Option<UIntN> {
    count.checked_mul(full_stride)
}

impl RuntimeServices<'_> {
    /// Sets the virtual address map.
    ///
    /// The map must contain descriptors for all
    /// runtime ranges. This call performs the
    /// critical transition and must not be
    /// treated as an ordinary runtime service.
    ///
    /// # Safety
    ///
    /// The map pointer must be valid for
    /// `map_size` bytes split by `descriptor_size`
    /// stride, the descriptor version must be
    /// supported, and the physical and virtual
    /// addresses must correctly describe runtime
    /// memory. This operation must be called
    /// exactly once before other virtualized
    /// runtime services are used.
    #[must_use]
    pub unsafe fn set_virtual_address_map(
        &self,
        map_size: UIntN,
        descriptor_size: UIntN,
        descriptor_version: u32,
        virtual_map: *mut EfiMemoryDescriptor,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees map buffer validity and transition correctness.
        unsafe {
            (self.table.set_virtual_address_map)(
                map_size,
                descriptor_size,
                descriptor_version,
                virtual_map,
            )
        }
    }

    /// Converts a pointer to the virtual map.
    ///
    /// # Safety
    ///
    /// The disposition must use the defined
    /// bits, the address double pointer must be
    /// valid, and the pointed-to address must
    /// belong to a runtime range in the map.
    #[must_use]
    pub unsafe fn convert_pointer(
        &self,
        debug_disposition: UIntN,
        address: *mut *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees disposition and pointer validity.
        unsafe { (self.table.convert_pointer)(debug_disposition, address) }
    }
}
