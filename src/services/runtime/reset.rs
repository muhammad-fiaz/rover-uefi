//! # Reset
//!
//! The system reset service.
//!
//! Firmware may not return normally after a
//! successful reset. The wrapper returns no
//! value because the firmware function returns
//! void. Callers must not assume execution
//! continues after the call.

use crate::base::{EfiStatus, ResetType};
use crate::services::runtime::RuntimeServices;

impl RuntimeServices<'_> {
    /// Resets the system.
    ///
    /// Data may be empty, in which case a null
    /// pointer and zero size are passed.
    pub fn reset_system(&self, reset_type: ResetType, reset_status: EfiStatus, reset_data: &[u8]) {
        let data_ptr = if reset_data.is_empty() {
            core::ptr::null_mut()
        } else {
            reset_data.as_ptr().cast::<core::ffi::c_void>().cast_mut()
        };
        // SAFETY: The data pointer is null with zero size or derived from a valid slice for the call.
        unsafe { (self.table.reset_system)(reset_type, reset_status, reset_data.len(), data_ptr) }
    }
}
