//! # Memory allocation
//!
//! Pool and page allocation through boot services.
//!
//! The safe allocation methods hide the
//! firmware out pointer with a stack local so
//! callers receive a value instead of a raw
//! pointer. The free pool method remains unsafe
//! because correctness depends on the pointer
//! originating from a prior allocation and on
//! the pointer being freed exactly once.
//!
//! Allocated memory is not initialized unless
//! the specification guarantees it. Callers must
//! validate firmware-provided addresses before
//! dereferencing them.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{AllocationType, EfiPhysicalAddress, EfiStatus, MemoryType};
use crate::services::boot::BootServices;

impl BootServices<'_> {
    /// Allocates pages and returns the address.
    ///
    /// The pages are not initialized. The caller
    /// must validate the address and must free
    /// the pages with `free_pages`.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// allocation fails.
    pub fn allocate_pages(
        &self,
        allocation_type: AllocationType,
        memory_type: MemoryType,
        pages: UIntN,
    ) -> Result<EfiPhysicalAddress, EfiStatus> {
        let mut memory = EfiPhysicalAddress::NULL;
        // SAFETY: The stack local lives for the full call and its address is valid for the out parameter.
        let status = unsafe {
            (self.table.allocate_pages)(allocation_type, memory_type, pages, &raw mut memory)
        };
        if status.is_success() {
            Ok(memory)
        } else {
            Err(status)
        }
    }

    /// Frees pages.
    ///
    /// Pages must have been allocated by
    /// `allocate_pages` with the same count.
    #[must_use]
    pub fn free_pages(&self, memory: EfiPhysicalAddress, pages: UIntN) -> EfiStatus {
        // SAFETY: Firmware returns an error for invalid addresses and counts instead of undefined behavior.
        unsafe { (self.table.free_pages)(memory, pages) }
    }

    /// Allocates pool memory and returns the pointer.
    ///
    /// The buffer is not initialized. The caller
    /// owns the allocation and must free it with
    /// the unsafe free method. The pointer must
    /// be validated before dereferencing.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// allocation fails.
    pub fn allocate_pool(
        &self,
        pool_type: MemoryType,
        size: UIntN,
    ) -> Result<*mut c_void, EfiStatus> {
        let mut buffer: *mut c_void = core::ptr::null_mut();
        // SAFETY: The stack local lives for the full call and its address is valid for the out parameter.
        let status = unsafe { (self.table.allocate_pool)(pool_type, size, &raw mut buffer) };
        if status.is_success() {
            Ok(buffer)
        } else {
            Err(status)
        }
    }

    /// Frees pool memory.
    ///
    /// The buffer must have been returned by a
    /// prior `allocate_pool` call and must not
    /// have been freed already.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the buffer
    /// was allocated by firmware pool allocation
    /// and has not been freed. The buffer must
    /// not be accessed after the call.
    #[must_use]
    pub unsafe fn free_pool(&self, buffer: *mut c_void) -> EfiStatus {
        // SAFETY: The caller guarantees the buffer originates from pool allocation and is freed exactly once.
        unsafe { (self.table.free_pool)(buffer) }
    }
}
