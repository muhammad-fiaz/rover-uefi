//! # ACPI discovery
//!
//! Configuration table lookup.
//!
//! The helper walks the firmware configuration
//! table array and returns the vendor pointer
//! for the requested `GUID`. The returned
//! pointer addresses firmware memory and must
//! be mapped by the caller before parsing.

use crate::base::EfiGuid;
use crate::tables::system::EfiSystemTable;

/// Finds a configuration table entry.
///
/// # Safety
///
/// The system table must be valid, and the
/// configuration table pointer and count must
/// describe a valid array for the call.
#[must_use]
pub unsafe fn find_config_entry(
    system_table: &EfiSystemTable,
    guid: &EfiGuid,
) -> *mut core::ffi::c_void {
    let count = system_table.number_of_table_entries;
    let base = system_table.configuration_table;
    if base.is_null() || count == 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: The caller guarantees the array is valid for `count` entries.
    let slice = unsafe { core::slice::from_raw_parts(base, count) };
    for entry in slice {
        if entry.vendor_guid == *guid {
            return entry.vendor_table;
        }
    }
    core::ptr::null_mut()
}
