//! # Counters and stalls
//!
//! Monotonic counting and fine-grained stalls.
//!
//! The safe methods hide the firmware out
//! pointer with a stack local. The stall
//! method forwards the microsecond count
//! directly.

use crate::abi::types::UIntN;
use crate::base::EfiStatus;
use crate::services::boot::BootServices;

impl BootServices<'_> {
    /// Returns the next monotonic count.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// count cannot be retrieved.
    pub fn get_next_monotonic_count(&self) -> Result<u64, EfiStatus> {
        let mut count: u64 = 0;
        // SAFETY: The stack local is valid for the out parameter.
        let status = unsafe { (self.table.get_next_monotonic_count)(&raw mut count) };
        if status.is_success() {
            Ok(count)
        } else {
            Err(status)
        }
    }

    /// Stalls execution.
    #[must_use]
    pub fn stall(&self, microseconds: UIntN) -> EfiStatus {
        // SAFETY: The argument is a copied integer and firmware handles the delay.
        unsafe { (self.table.stall)(microseconds) }
    }
}
