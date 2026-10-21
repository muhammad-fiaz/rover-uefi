//! # Monotonic counter
//!
//! The high monotonic counter service.
//!
//! The safe method uses a stack local so the
//! caller receives a value instead of a raw
//! pointer.

use crate::base::EfiStatus;
use crate::services::runtime::RuntimeServices;

impl RuntimeServices<'_> {
    /// Returns the next high monotonic count.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// count cannot be retrieved.
    pub fn get_next_high_monotonic_count(&self) -> Result<u32, EfiStatus> {
        let mut high_count: u32 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe { (self.table.get_next_high_monotonic_count)(&raw mut high_count) };
        if status.is_success() {
            Ok(high_count)
        } else {
            Err(status)
        }
    }
}
