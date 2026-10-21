//! # Utilities
//!
//! Task priority, CRC, and memory copy services.
//!
//! Slice methods validate lengths and derive
//! pointers from valid slices so they can be
//! safe. The underlying firmware call is
//! preserved; no service is replaced by a Rust
//! implementation.

use crate::base::{EfiStatus, EfiTpl};
use crate::services::boot::BootServices;

impl BootServices<'_> {
    /// Raises the task priority level.
    #[must_use]
    pub fn raise_tpl(&self, new_tpl: EfiTpl) -> EfiTpl {
        // SAFETY: Firmware returns the previous level for the requested level.
        unsafe { (self.table.raise_tpl)(new_tpl) }
    }

    /// Restores the task priority level.
    pub fn restore_tpl(&self, old_tpl: EfiTpl) {
        // SAFETY: Firmware restores the previous level.
        unsafe { (self.table.restore_tpl)(old_tpl) }
    }

    /// Computes a 32-bit CRC.
    ///
    /// # Errors
    ///
    /// Returns the firmware status without calling
    /// firmware when the slice is empty, matching
    /// the specification rule that a zero size
    /// is invalid.
    pub fn calculate_crc32(&self, data: &[u8]) -> Result<u32, EfiStatus> {
        if data.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let mut crc: u32 = 0;
        // SAFETY: The slice pointer is valid for `data.len()` bytes and the stack local is valid for the out parameter.
        let status = unsafe {
            (self.table.calculate_crc32)(
                data.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
                data.len(),
                &raw mut crc,
            )
        };
        if status.is_success() {
            Ok(crc)
        } else {
            Err(status)
        }
    }

    /// Copies memory through firmware.
    ///
    /// The slices must have equal length.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` without
    /// calling firmware when the lengths differ.
    pub fn copy_mem(&self, destination: &mut [u8], source: &[u8]) -> Result<(), EfiStatus> {
        if destination.len() != source.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        // SAFETY: Both slice pointers are valid for the shared length.
        unsafe {
            (self.table.copy_mem)(
                destination.as_mut_ptr().cast(),
                source.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
                destination.len(),
            );
        }
        Ok(())
    }

    /// Fills memory through firmware.
    pub fn set_mem(&self, buffer: &mut [u8], value: u8) {
        // SAFETY: The slice pointer is valid for `buffer.len()` bytes.
        unsafe { (self.table.set_mem)(buffer.as_mut_ptr().cast(), buffer.len(), value) }
    }
}
