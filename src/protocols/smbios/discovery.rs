//! # SMBIOS discovery
//!
//! Configuration table lookup.
//!
//! Returns the entry point pointer for the
//! requested `GUID`.

use crate::base::EfiGuid;
use crate::tables::system::EfiSystemTable;

/// Finds a configuration entry.
///
/// # Safety
///
/// The system table must be valid and the
/// configuration array must be valid.
#[must_use]
pub unsafe fn find_config_entry(
    system_table: &EfiSystemTable,
    guid: &EfiGuid,
) -> *mut core::ffi::c_void {
    // SAFETY: The caller guarantees the system table and array are valid.
    unsafe { crate::protocols::acpi::discovery::find_config_entry(system_table, guid) }
}
