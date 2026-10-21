//! # Configuration table
//!
//! One entry in the configuration table array
//! referenced by the `EFI System Table`.
//!
//! Each entry pairs a vendor `GUID` with a pointer
//! to the table for that vendor. Well-known vendors
//! include `ACPI`, `SMBIOS`, and related firmware
//! tables. The array is owned by firmware and
//! addressed by `number_of_table_entries` together
//! with [`EfiConfigurationTable`] in the system
//! table. Parsers for specific tables are defined
//! in later phases; this module provides only the
//! entry layout.

use core::ffi::c_void;

use crate::base::EfiGuid;

/// One firmware configuration table entry.
///
/// The struct is `#[repr(C)]` so the vendor `GUID`
/// at offset zero and the vendor pointer at offset
/// 16 match the firmware definition. The size is
/// 24 bytes on 64-bit targets and 20 bytes on
/// 32-bit targets.
#[derive(Debug)]
#[repr(C)]
pub struct EfiConfigurationTable {
    /// The `GUID` that identifies the vendor table.
    pub vendor_guid: EfiGuid,
    /// The pointer to the vendor table.
    pub vendor_table: *mut c_void,
}

const _: () = {
    assert!(core::mem::size_of::<EfiConfigurationTable>() == 16 + core::mem::size_of::<usize>());
    assert!(core::mem::offset_of!(EfiConfigurationTable, vendor_guid) == 0);
    assert!(core::mem::offset_of!(EfiConfigurationTable, vendor_table) == 16);
};
