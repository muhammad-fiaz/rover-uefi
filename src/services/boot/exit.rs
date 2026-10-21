//! # Exit boot services
//!
//! Termination of boot services with the
//! memory map key.
//!
//! The caller must retrieve the current map
//! with `get_memory_map` immediately before
//! the exit call. No allocation, event, or
//! other map-changing firmware call may occur
//! between the map retrieval and the exit
//! call. On success firmware does not return:
//! boot services are terminated and control
//! passes to the operating system. On failure
//! the firmware status is returned and boot
//! services remain available.
//!
//! The helper that takes a slice performs the
//! map retrieval and exit in sequence without
//! allocation. The caller must not perform any
//! firmware operation between the two calls
//! besides passing the key.

use crate::abi::types::UIntN;
use crate::base::{EfiHandle, EfiStatus};
use crate::services::boot::BootServices;

impl BootServices<'_> {
    /// Exits boot services with a map key.
    ///
    /// The map key must be the most recent key
    /// from `get_memory_map` with no
    /// intervening map-changing call.
    #[must_use]
    pub fn exit_boot_services(&self, image_handle: EfiHandle, map_key: UIntN) -> EfiStatus {
        // SAFETY: Firmware returns `INVALID_PARAMETER` for a stale key instead of undefined behavior.
        unsafe { (self.table.exit_boot_services)(image_handle, map_key) }
    }

    /// Retrieves the map and exits boot services.
    ///
    /// The buffer must be large enough for the
    /// map. No allocation or other firmware
    /// call occurs between the map retrieval
    /// and the exit call.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the map
    /// retrieval fails.
    pub fn exit_boot_services_with_map(
        &self,
        image_handle: EfiHandle,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        let (map_key, _, _) = self.get_memory_map(buffer)?;
        // SAFETY: The helper performs no allocation between the map retrieval and this call.
        let status = unsafe { (self.table.exit_boot_services)(image_handle, map_key) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
