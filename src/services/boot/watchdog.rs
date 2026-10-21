//! # Watchdog
//!
//! The watchdog timer boot service.
//!
//! A zero timeout disables the timer. The data
//! pointer is optional and must be valid when
//! non-null.

use crate::abi::types::{Char16, UIntN};
use crate::base::EfiStatus;
use crate::services::boot::BootServices;

impl BootServices<'_> {
    /// Sets the watchdog timer.
    ///
    /// Data size and data must be consistent:
    /// a non-null data pointer requires a size
    /// that covers the null-terminated string.
    ///
    /// # Safety
    ///
    /// The data pointer must be valid when
    /// non-null. The watchdog code is passed
    /// through to firmware logging.
    #[must_use]
    pub unsafe fn set_watchdog_timer(
        &self,
        timeout: UIntN,
        watchdog_code: u64,
        data_size: UIntN,
        watchdog_data: *mut Char16,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe { (self.table.set_watchdog_timer)(timeout, watchdog_code, data_size, watchdog_data) }
    }

    /// Disables the watchdog timer.
    #[must_use]
    pub fn disable_watchdog(&self) -> EfiStatus {
        // SAFETY: No data pointer is supplied and the timeout disables the timer.
        unsafe { (self.table.set_watchdog_timer)(0, 0, 0, core::ptr::null_mut()) }
    }
}
