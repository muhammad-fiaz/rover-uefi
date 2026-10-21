//! # Runtime services table
//!
//! The `EFI Runtime Services Table` and all
//! function pointer types that it contains.
//!
//! Runtime services remain available after
//! `ExitBootServices`, but use is restricted to
//! the virtual memory map established by the
//! operating system. Each entry is a function
//! pointer using the `EFIAPI` calling convention
//! with the direct alias type so the field size
//! equals the target pointer size.
//!
//! This module defines signatures only. Wrappers
//! that invoke the services are defined in later
//! phases.

use core::ffi::c_void;

use crate::abi::types::{Bool, Char16, UIntN};
use crate::base::{
    EfiGuid, EfiMemoryDescriptor, EfiPhysicalAddress, EfiStatus, EfiTime, EfiTimeCapabilities,
    ResetType,
};
use crate::tables::header::EfiTableHeader;

/// Mark a capsule as persistent across reset.
pub const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: u32 = 0x0001_0000;

/// Mark a capsule to populate the system table.
pub const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: u32 = 0x0002_0000;

/// Mark a capsule to initiate reset.
pub const CAPSULE_FLAGS_INITIATE_RESET: u32 = 0x0004_0000;

/// An EFI capsule header.
///
/// The struct is `#[repr(C)]` so field order and
/// offsets match the specification. The size is
/// 28 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiCapsuleHeader {
    /// The `GUID` that defines the capsule contents.
    pub capsule_guid: EfiGuid,
    /// The capsule header size.
    pub header_size: u32,
    /// The capsule attribute flags.
    pub flags: u32,
    /// The capsule image size in bytes.
    pub capsule_image_size: u32,
}

/// Retrieve the current time.
///
/// The function writes the current time and, when
/// the pointer is not null, the time capabilities.
pub type EfiGetTime =
    unsafe extern "efiapi" fn(*mut EfiTime, *mut EfiTimeCapabilities) -> EfiStatus;

/// Set the current time.
///
/// The function sets the local time from the
/// supplied time value.
pub type EfiSetTime = unsafe extern "efiapi" fn(*mut EfiTime) -> EfiStatus;

/// Retrieve the wakeup alarm setting.
///
/// The function writes the enabled flag, the
/// pending flag, and the alarm time.
pub type EfiGetWakeupTime =
    unsafe extern "efiapi" fn(*mut Bool, *mut Bool, *mut EfiTime) -> EfiStatus;

/// Set the wakeup alarm.
///
/// The function sets the alarm to the supplied
/// time, or disables the alarm when the enable
/// flag is false.
pub type EfiSetWakeupTime = unsafe extern "efiapi" fn(Bool, *mut EfiTime) -> EfiStatus;

/// Set the virtual address map.
///
/// The function changes the runtime addressing
/// mode from physical to virtual using the
/// supplied map.
pub type EfiSetVirtualAddressMap =
    unsafe extern "efiapi" fn(UIntN, UIntN, u32, *mut EfiMemoryDescriptor) -> EfiStatus;

/// Convert a pointer to the virtual map.
///
/// The function updates the pointer to the new
/// virtual address.
pub type EfiConvertPointer = unsafe extern "efiapi" fn(UIntN, *mut *mut c_void) -> EfiStatus;

/// Retrieve a variable value.
///
/// The function returns the variable contents
/// into the supplied buffer and updates the
/// data size.
pub type EfiGetVariable = unsafe extern "efiapi" fn(
    *mut Char16,
    *mut EfiGuid,
    *mut u32,
    *mut UIntN,
    *mut c_void,
) -> EfiStatus;

/// Enumerate variable names.
///
/// The function returns the next variable name
/// and vendor `GUID` in the enumeration.
pub type EfiGetNextVariableName =
    unsafe extern "efiapi" fn(*mut UIntN, *mut Char16, *mut EfiGuid) -> EfiStatus;

/// Store a variable value.
///
/// The function stores the variable with the
/// given attributes. A zero data size deletes
/// the variable unless append or authentication
/// attributes are set.
pub type EfiSetVariable =
    unsafe extern "efiapi" fn(*mut Char16, *mut EfiGuid, u32, UIntN, *mut c_void) -> EfiStatus;

/// Retrieve the next high monotonic count.
///
/// The function writes the high 32 bits of the
/// monotonic counter.
pub type EfiGetNextHighMonotonicCount = unsafe extern "efiapi" fn(*mut u32) -> EfiStatus;

/// Reset the system.
///
/// The function performs the requested reset
/// with the given status and data.
pub type EfiResetSystem = unsafe extern "efiapi" fn(ResetType, EfiStatus, UIntN, *mut c_void);

/// Update capsules.
///
/// The function passes capsules to firmware with
/// a scatter gather list describing physical
/// memory.
pub type EfiUpdateCapsule =
    unsafe extern "efiapi" fn(*mut *mut EfiCapsuleHeader, UIntN, EfiPhysicalAddress) -> EfiStatus;

/// Query capsule capabilities.
///
/// The function returns the maximum capsule size
/// and the reset type for the capsules.
pub type EfiQueryCapsuleCapabilities = unsafe extern "efiapi" fn(
    *mut *mut EfiCapsuleHeader,
    UIntN,
    *mut u64,
    *mut ResetType,
) -> EfiStatus;

/// Query variable information.
///
/// The function returns storage information for
/// the variable attributes.
pub type EfiQueryVariableInfo =
    unsafe extern "efiapi" fn(u32, *mut u64, *mut u64, *mut u64) -> EfiStatus;

/// The `EFI Runtime Services Table`.
///
/// The struct is `#[repr(C)]` so field order and
/// offsets match the specification exactly. The
/// size is 136 bytes on 64-bit targets and 80
/// bytes on 32-bit targets.
#[derive(Debug)]
#[repr(C)]
pub struct EfiRuntimeServices {
    /// The table header.
    pub hdr: EfiTableHeader,
    /// Retrieve the current time.
    pub get_time: EfiGetTime,
    /// Set the current time.
    pub set_time: EfiSetTime,
    /// Retrieve the wakeup alarm setting.
    pub get_wakeup_time: EfiGetWakeupTime,
    /// Set the wakeup alarm.
    pub set_wakeup_time: EfiSetWakeupTime,
    /// Set the virtual address map.
    pub set_virtual_address_map: EfiSetVirtualAddressMap,
    /// Convert a pointer to the virtual map.
    pub convert_pointer: EfiConvertPointer,
    /// Retrieve a variable value.
    pub get_variable: EfiGetVariable,
    /// Enumerate variable names.
    pub get_next_variable_name: EfiGetNextVariableName,
    /// Store a variable value.
    pub set_variable: EfiSetVariable,
    /// Retrieve the next high monotonic count.
    pub get_next_high_monotonic_count: EfiGetNextHighMonotonicCount,
    /// Reset the system.
    pub reset_system: EfiResetSystem,
    /// Update capsules.
    pub update_capsule: EfiUpdateCapsule,
    /// Query capsule capabilities.
    pub query_capsule_capabilities: EfiQueryCapsuleCapabilities,
    /// Query variable information.
    pub query_variable_info: EfiQueryVariableInfo,
}

const _: () = {
    assert!(core::mem::size_of::<EfiRuntimeServices>() == 24 + 14 * core::mem::size_of::<usize>());
    assert!(core::mem::align_of::<EfiRuntimeServices>() == 8);
    assert!(core::mem::offset_of!(EfiRuntimeServices, hdr) == 0);
    assert!(core::mem::offset_of!(EfiRuntimeServices, get_time) == 24);
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, set_time) == 24 + core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, get_wakeup_time)
            == 24 + 2 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, set_wakeup_time)
            == 24 + 3 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, set_virtual_address_map)
            == 24 + 4 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, convert_pointer)
            == 24 + 5 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, get_variable)
            == 24 + 6 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, get_next_variable_name)
            == 24 + 7 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, set_variable)
            == 24 + 8 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, get_next_high_monotonic_count)
            == 24 + 9 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, reset_system)
            == 24 + 10 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, update_capsule)
            == 24 + 11 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, query_capsule_capabilities)
            == 24 + 12 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiRuntimeServices, query_variable_info)
            == 24 + 13 * core::mem::size_of::<usize>()
    );
};
