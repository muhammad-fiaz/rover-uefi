//! # System table
//!
//! The `EFI System Table` passed to every image
//! entry point.
//!
//! The table begins with the shared `EFI_TABLE_HEADER`,
//! followed by firmware information, console handles
//! and protocol pointers, pointers to the boot and
//! runtime services tables, and the configuration
//! table array. The signature in the header identifies
//! the table, and the header size field gives the
//! total table size.
//!
//! The table is owned by firmware. Images must not
//! modify its lifetime; the boot services pointer
//! becomes invalid after a successful `ExitBootServices`
//! call, while runtime services remain available
//! under the post-transition rules. Console protocol
//! types are defined in the console protocol module.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiHandle, EfiStatus};
use crate::tables::boot::EfiBootServices;
use crate::tables::configuration::EfiConfigurationTable;
use crate::tables::header::EfiTableHeader;
use crate::tables::runtime::EfiRuntimeServices;

pub use crate::protocols::console::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol};

/// The `EFI System Table`.
///
/// The struct is `#[repr(C)]` so field order and
/// offsets match the specification exactly. The
/// size is 120 bytes on 64-bit targets and 72
/// bytes on 32-bit targets.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSystemTable {
    /// The table header.
    pub hdr: EfiTableHeader,
    /// The firmware vendor string.
    pub firmware_vendor: *mut Char16,
    /// The firmware revision.
    pub firmware_revision: u32,
    /// The console input handle.
    pub console_in_handle: EfiHandle,
    /// The console input protocol pointer.
    pub con_in: *mut EfiSimpleTextInputProtocol,
    /// The console output handle.
    pub console_out_handle: EfiHandle,
    /// The console output protocol pointer.
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    /// The standard error handle.
    pub standard_error_handle: EfiHandle,
    /// The standard error protocol pointer.
    pub std_err: *mut EfiSimpleTextOutputProtocol,
    /// The runtime services table pointer.
    pub runtime_services: *mut EfiRuntimeServices,
    /// The boot services table pointer.
    pub boot_services: *mut EfiBootServices,
    /// The number of configuration table entries.
    pub number_of_table_entries: UIntN,
    /// The configuration table array pointer.
    pub configuration_table: *mut EfiConfigurationTable,
}

/// An image entry point.
///
/// The function receives the image handle allocated
/// by firmware and a pointer to the system table,
/// and returns a status code. Firmware calls the
/// entry point to start an application, loader, or
/// driver. The first argument is always valid when
/// called by firmware; the second points to a live
/// system table for the duration of the call.
pub type EfiImageEntryPoint =
    unsafe extern "efiapi" fn(EfiHandle, *mut EfiSystemTable) -> EfiStatus;

const _: () = {
    assert!(
        core::mem::size_of::<EfiSystemTable>()
            == if core::mem::size_of::<usize>() == 8 {
                120
            } else {
                72
            }
    );
    assert!(core::mem::align_of::<EfiSystemTable>() == 8);
    assert!(core::mem::offset_of!(EfiSystemTable, hdr) == 0);
    assert!(core::mem::offset_of!(EfiSystemTable, firmware_vendor) == 24);
    assert!(
        core::mem::offset_of!(EfiSystemTable, firmware_revision)
            == 24 + core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, console_in_handle)
            == if core::mem::size_of::<usize>() == 8 {
                40
            } else {
                32
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, con_in)
            == if core::mem::size_of::<usize>() == 8 {
                48
            } else {
                36
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, console_out_handle)
            == if core::mem::size_of::<usize>() == 8 {
                56
            } else {
                40
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, con_out)
            == if core::mem::size_of::<usize>() == 8 {
                64
            } else {
                44
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, standard_error_handle)
            == if core::mem::size_of::<usize>() == 8 {
                72
            } else {
                48
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, std_err)
            == if core::mem::size_of::<usize>() == 8 {
                80
            } else {
                52
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, runtime_services)
            == if core::mem::size_of::<usize>() == 8 {
                88
            } else {
                56
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, boot_services)
            == if core::mem::size_of::<usize>() == 8 {
                96
            } else {
                60
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, number_of_table_entries)
            == if core::mem::size_of::<usize>() == 8 {
                104
            } else {
                64
            }
    );
    assert!(
        core::mem::offset_of!(EfiSystemTable, configuration_table)
            == if core::mem::size_of::<usize>() == 8 {
                112
            } else {
                68
            }
    );
};
