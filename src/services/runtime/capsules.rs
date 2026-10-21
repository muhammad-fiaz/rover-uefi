//! # Capsules
//!
//! Capsule update and capability query.
//!
//! Capsule arrays are arrays of pointers to
//! headers. Header validity, physical memory
//! placement, and scatter gather placement
//! cannot be verified by Rust, so the firmware
//! calls are unsafe. Size helpers use checked
//! arithmetic and do not dereference firmware
//! memory.

use crate::abi::types::UIntN;
use crate::base::{EfiPhysicalAddress, EfiStatus, ResetType};
use crate::services::runtime::RuntimeServices;
use crate::tables::runtime::EfiCapsuleHeader;

/// Returns the checked byte size of a pointer
/// array for a capsule count.
///
/// Returns `None` on multiplication overflow.
#[must_use]
pub fn checked_capsule_array_size(count: UIntN) -> Option<UIntN> {
    count.checked_mul(core::mem::size_of::<*mut EfiCapsuleHeader>())
}

/// Returns whether a capsule count is non-zero.
#[must_use]
pub fn validate_capsule_count(count: UIntN) -> bool {
    count != 0
}

impl RuntimeServices<'_> {
    /// Updates capsules.
    ///
    /// # Safety
    ///
    /// The capsule array must be valid for
    /// `capsule_count` pointers, each header
    /// must be valid, and the scatter gather
    /// address must be valid when non-zero.
    #[must_use]
    pub unsafe fn update_capsule(
        &self,
        capsule_header_array: *mut *mut EfiCapsuleHeader,
        capsule_count: UIntN,
        scatter_gather_list: EfiPhysicalAddress,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees array, header, and scatter gather validity.
        unsafe {
            (self.table.update_capsule)(capsule_header_array, capsule_count, scatter_gather_list)
        }
    }

    /// Queries capsule capabilities.
    ///
    /// # Safety
    ///
    /// The capsule array must be valid for
    /// `capsule_count` pointers, each header
    /// must be valid, and both out pointers
    /// must be valid for writing.
    #[must_use]
    pub unsafe fn query_capsule_capabilities(
        &self,
        capsule_header_array: *mut *mut EfiCapsuleHeader,
        capsule_count: UIntN,
        maximum_capsule_size: *mut u64,
        reset_type: *mut ResetType,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe {
            (self.table.query_capsule_capabilities)(
                capsule_header_array,
                capsule_count,
                maximum_capsule_size,
                reset_type,
            )
        }
    }
}
