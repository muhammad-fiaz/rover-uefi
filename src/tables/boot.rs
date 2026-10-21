//! # Boot services table
//!
//! The `EFI Boot Services Table` and all function
//! pointer types that it contains.
//!
//! Boot services are available only before a
//! successful `ExitBootServices` call. Each entry
//! is a function pointer using the `EFIAPI` calling
//! convention. The table fields use the direct
//! function pointer alias because the specification
//! defines them as required entries; the size of
//! each field equals the target pointer size. The
//! single optional notification callback parameter
//! of `CreateEvent` uses `Option` because the
//! specification permits a null callback and Rust
//! function pointers cannot be null, while
//! `Option` of a function pointer uses the same
//! pointer size with the null niche.
//!
//! This module defines signatures only. Wrappers
//! that invoke the services are defined in later
//! phases. The reserved field is a data pointer
//! and is always represented as a raw pointer.
//!
//! The device path header is defined in the
//! device path protocol module. Signatures in
//! this module store a pointer to one header.

use core::ffi::c_void;

use crate::abi::types::{Bool, Char16, UIntN};
use crate::base::{
    AllocationType, EfiEvent, EfiGuid, EfiHandle, EfiMemoryDescriptor, EfiPhysicalAddress,
    EfiStatus, EfiTpl, InterfaceType, LocateSearchType, MemoryType, TimerDelay,
};
use crate::tables::header::EfiTableHeader;

pub use crate::protocols::device_path::EfiDevicePath;

/// An open protocol information entry.
///
/// The struct is `#[repr(C)]` so field order and
/// offsets match the specification. The size is
/// 24 bytes on 64-bit targets and 16 bytes on
/// 32-bit targets.
#[derive(Debug)]
#[repr(C)]
pub struct EfiOpenProtocolInformationEntry {
    /// The agent handle.
    pub agent_handle: EfiHandle,
    /// The controller handle.
    pub controller_handle: EfiHandle,
    /// The open attributes.
    pub attributes: u32,
    /// The open count.
    pub open_count: u32,
}

/// Open a protocol by handle protocol query.
pub const OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x01;

/// Open a protocol by retrieving the protocol.
pub const OPEN_PROTOCOL_GET_PROTOCOL: u32 = 0x02;

/// Open a protocol by testing the protocol.
pub const OPEN_PROTOCOL_TEST_PROTOCOL: u32 = 0x04;

/// Open a protocol by child controller.
pub const OPEN_PROTOCOL_BY_CHILD_CONTROLLER: u32 = 0x08;

/// Open a protocol by driver.
pub const OPEN_PROTOCOL_BY_DRIVER: u32 = 0x10;

/// Open a protocol exclusively.
pub const OPEN_PROTOCOL_EXCLUSIVE: u32 = 0x20;

/// An event notification function.
///
/// Firmware calls this function when the event
/// reaches the signalled state. The first argument
/// is the event, and the second is the context
/// supplied at creation. The context pointer is
/// the value passed to the create event function.
pub type EfiEventNotify = unsafe extern "efiapi" fn(EfiEvent, *mut c_void);

/// Raise the task priority level.
///
/// The function raises the current task to the new
/// priority level and returns the previous level.
pub type EfiRaiseTpl = unsafe extern "efiapi" fn(EfiTpl) -> EfiTpl;

/// Restore the task priority level.
///
/// The function restores the current task to the
/// previous priority level.
pub type EfiRestoreTpl = unsafe extern "efiapi" fn(EfiTpl);

/// Allocate memory pages.
///
/// The function allocates the requested number of
/// contiguous pages according to the allocation
/// type. The memory pointer receives the base
/// physical address on success.
pub type EfiAllocatePages = unsafe extern "efiapi" fn(
    AllocationType,
    MemoryType,
    UIntN,
    *mut EfiPhysicalAddress,
) -> EfiStatus;

/// Free memory pages.
///
/// The function frees the pages starting at the
/// physical address.
pub type EfiFreePages = unsafe extern "efiapi" fn(EfiPhysicalAddress, UIntN) -> EfiStatus;

/// Retrieve the memory map.
///
/// The function writes the descriptors into the
/// supplied buffer and returns the map key, entry
/// size, and descriptor version. The buffer size
/// is passed in and updated.
pub type EfiGetMemoryMap = unsafe extern "efiapi" fn(
    *mut UIntN,
    *mut EfiMemoryDescriptor,
    *mut UIntN,
    *mut UIntN,
    *mut u32,
) -> EfiStatus;

/// Allocate pool memory.
///
/// The function allocates a pool buffer of the
/// requested size and returns its address.
pub type EfiAllocatePool =
    unsafe extern "efiapi" fn(MemoryType, UIntN, *mut *mut c_void) -> EfiStatus;

/// Free pool memory.
///
/// The function returns the buffer to the firmware
/// pool.
pub type EfiFreePool = unsafe extern "efiapi" fn(*mut c_void) -> EfiStatus;

/// Create an event.
///
/// The function creates an event of the given
/// type and priority. The notification function
/// and context may be null. The event pointer
/// receives the new event.
pub type EfiCreateEvent = unsafe extern "efiapi" fn(
    u32,
    EfiTpl,
    Option<EfiEventNotify>,
    *mut c_void,
    *mut EfiEvent,
) -> EfiStatus;

/// Set the timer for an event.
///
/// The function sets the timer type and trigger
/// time in 100 nanosecond units.
pub type EfiSetTimer = unsafe extern "efiapi" fn(EfiEvent, TimerDelay, u64) -> EfiStatus;

/// Signal an event.
///
/// The function signals the event.
pub type EfiSignalEvent = unsafe extern "efiapi" fn(EfiEvent) -> EfiStatus;

/// Wait for an event.
///
/// The function waits until one of the events is
/// signalled and returns the index of the
/// signalled event.
pub type EfiWaitForEvent = unsafe extern "efiapi" fn(UIntN, *mut EfiEvent, *mut UIntN) -> EfiStatus;

/// Close an event.
///
/// The function closes the event.
pub type EfiCloseEvent = unsafe extern "efiapi" fn(EfiEvent) -> EfiStatus;

/// Check whether an event is signalled.
///
/// The function returns success when the event is
/// in the signalled state.
pub type EfiCheckEvent = unsafe extern "efiapi" fn(EfiEvent) -> EfiStatus;

/// Install a protocol interface.
///
/// The function installs the protocol on the
/// handle, creating the handle when the input
/// handle pointer holds null.
pub type EfiInstallProtocolInterface = unsafe extern "efiapi" fn(
    *mut EfiHandle,
    *mut EfiGuid,
    InterfaceType,
    *mut c_void,
) -> EfiStatus;

/// Reinstall a protocol interface.
///
/// The function replaces the old interface with
/// the new interface on the handle.
pub type EfiReinstallProtocolInterface =
    unsafe extern "efiapi" fn(EfiHandle, *mut EfiGuid, *mut c_void, *mut c_void) -> EfiStatus;

/// Uninstall a protocol interface.
///
/// The function removes the protocol interface
/// from the handle.
pub type EfiUninstallProtocolInterface =
    unsafe extern "efiapi" fn(EfiHandle, *mut EfiGuid, *mut c_void) -> EfiStatus;

/// Query a handle for a protocol.
///
/// The function returns the protocol interface
/// for the handle.
pub type EfiHandleProtocol =
    unsafe extern "efiapi" fn(EfiHandle, *mut EfiGuid, *mut *mut c_void) -> EfiStatus;

/// Register a protocol notification.
///
/// The function registers the event to be
/// signalled when the protocol is installed and
/// returns a registration token.
pub type EfiRegisterProtocolNotify =
    unsafe extern "efiapi" fn(*mut EfiGuid, EfiEvent, *mut *mut c_void) -> EfiStatus;

/// Locate handles matching a search.
///
/// The function writes the matching handles into
/// the buffer and updates the buffer size.
pub type EfiLocateHandle = unsafe extern "efiapi" fn(
    LocateSearchType,
    *mut EfiGuid,
    *mut c_void,
    *mut UIntN,
    *mut EfiHandle,
) -> EfiStatus;

/// Locate a device by device path.
///
/// The function finds the handle that supports
/// the protocol on the device path.
pub type EfiLocateDevicePath =
    unsafe extern "efiapi" fn(*mut EfiGuid, *mut *mut EfiDevicePath, *mut EfiHandle) -> EfiStatus;

/// Install a configuration table.
///
/// The function adds, updates, or removes a
/// configuration table entry. A null table
/// removes the entry.
pub type EfiInstallConfigurationTable =
    unsafe extern "efiapi" fn(*mut EfiGuid, *mut c_void) -> EfiStatus;

/// Load an image into memory.
///
/// The function loads the image from the device
/// path or from the source buffer and returns
/// the new image handle.
pub type EfiLoadImage = unsafe extern "efiapi" fn(
    Bool,
    EfiHandle,
    *mut EfiDevicePath,
    *mut c_void,
    UIntN,
    *mut EfiHandle,
) -> EfiStatus;

/// Start a loaded image.
///
/// The function transfers control to the image
/// entry point and returns the exit data size
/// and exit data.
pub type EfiStartImage =
    unsafe extern "efiapi" fn(EfiHandle, *mut UIntN, *mut *mut Char16) -> EfiStatus;

/// Exit a loaded image.
///
/// The function terminates the image and returns
/// control to boot services.
pub type EfiExit = unsafe extern "efiapi" fn(EfiHandle, EfiStatus, UIntN, *mut Char16) -> EfiStatus;

/// Unload an image.
///
/// The function unloads the image identified by
/// the handle.
pub type EfiUnloadImage = unsafe extern "efiapi" fn(EfiHandle) -> EfiStatus;

/// Exit boot services.
///
/// The function terminates all boot services for
/// the image when the memory map key is current.
pub type EfiExitBootServices = unsafe extern "efiapi" fn(EfiHandle, UIntN) -> EfiStatus;

/// Retrieve the next monotonic count.
///
/// The function writes the next count to the
/// supplied pointer.
pub type EfiGetNextMonotonicCount = unsafe extern "efiapi" fn(*mut u64) -> EfiStatus;

/// Stall execution.
///
/// The function stalls for the requested number
/// of microseconds.
pub type EfiStall = unsafe extern "efiapi" fn(UIntN) -> EfiStatus;

/// Set the watchdog timer.
///
/// The function sets the watchdog timeout and
/// the code and data to log on timeout.
pub type EfiSetWatchdogTimer =
    unsafe extern "efiapi" fn(UIntN, u64, UIntN, *mut Char16) -> EfiStatus;

/// Connect drivers to a controller.
///
/// The function connects the listed drivers, or
/// all suitable drivers when the driver list is
/// null, to the controller.
pub type EfiConnectController =
    unsafe extern "efiapi" fn(EfiHandle, *mut EfiHandle, *mut EfiDevicePath, Bool) -> EfiStatus;

/// Disconnect drivers from a controller.
///
/// The function disconnects the driver from the
/// controller.
pub type EfiDisconnectController =
    unsafe extern "efiapi" fn(EfiHandle, EfiHandle, EfiHandle) -> EfiStatus;

/// Open a protocol on a handle.
///
/// The function opens the protocol with the
/// given attributes and returns the interface.
pub type EfiOpenProtocol = unsafe extern "efiapi" fn(
    EfiHandle,
    *mut EfiGuid,
    *mut *mut c_void,
    EfiHandle,
    EfiHandle,
    u32,
) -> EfiStatus;

/// Close a protocol on a handle.
///
/// The function closes the protocol that was
/// opened by the agent handle.
pub type EfiCloseProtocol =
    unsafe extern "efiapi" fn(EfiHandle, *mut EfiGuid, EfiHandle, EfiHandle) -> EfiStatus;

/// Retrieve open protocol information.
///
/// The function returns the entries describing
/// which agents have the protocol open.
pub type EfiOpenProtocolInformation = unsafe extern "efiapi" fn(
    EfiHandle,
    *mut EfiGuid,
    *mut *mut EfiOpenProtocolInformationEntry,
    *mut UIntN,
) -> EfiStatus;

/// Retrieve protocols for a handle.
///
/// The function returns the list of protocol
/// `GUID` values installed on the handle.
pub type EfiProtocolsPerHandle =
    unsafe extern "efiapi" fn(EfiHandle, *mut *mut *mut EfiGuid, *mut UIntN) -> EfiStatus;

/// Locate handles into a pool buffer.
///
/// The function returns an array of handles that
/// support the requested protocol.
pub type EfiLocateHandleBuffer = unsafe extern "efiapi" fn(
    LocateSearchType,
    *mut EfiGuid,
    *mut c_void,
    *mut UIntN,
    *mut *mut EfiHandle,
) -> EfiStatus;

/// Locate a protocol.
///
/// The function returns the first instance that
/// matches the protocol.
pub type EfiLocateProtocol =
    unsafe extern "efiapi" fn(*mut EfiGuid, *mut c_void, *mut *mut c_void) -> EfiStatus;

/// Install multiple protocol interfaces.
///
/// The function installs one or more interfaces
/// described by the variadic list of `GUID` and
/// interface pairs.
pub type EfiInstallMultipleProtocolInterfaces =
    unsafe extern "efiapi" fn(*mut EfiHandle, ...) -> EfiStatus;

/// Uninstall multiple protocol interfaces.
///
/// The function removes one or more interfaces
/// described by the variadic list of `GUID` and
/// interface pairs.
pub type EfiUninstallMultipleProtocolInterfaces =
    unsafe extern "efiapi" fn(EfiHandle, ...) -> EfiStatus;

/// Compute a 32-bit CRC.
///
/// The function computes the CRC for the data
/// buffer and returns it.
pub type EfiCalculateCrc32 = unsafe extern "efiapi" fn(*mut c_void, UIntN, *mut u32) -> EfiStatus;

/// Copy memory.
///
/// The function copies the requested number of
/// bytes from the source to the destination.
pub type EfiCopyMem = unsafe extern "efiapi" fn(*mut c_void, *mut c_void, UIntN);

/// Fill memory.
///
/// The function fills the buffer with the value.
pub type EfiSetMem = unsafe extern "efiapi" fn(*mut c_void, UIntN, u8);

/// Create an event in a group.
///
/// The function creates an event that may belong
/// to the given group. A null group behaves like
/// the plain create event function.
pub type EfiCreateEventEx = unsafe extern "efiapi" fn(
    u32,
    EfiTpl,
    Option<EfiEventNotify>,
    *const c_void,
    *const EfiGuid,
    *mut EfiEvent,
) -> EfiStatus;

/// The `EFI Boot Services Table`.
///
/// The struct is `#[repr(C)]` so field order and
/// offsets match the specification exactly. The
/// size is 376 bytes on 64-bit targets and 200
/// bytes on 32-bit targets.
#[repr(C)]
pub struct EfiBootServices {
    /// The table header.
    pub hdr: EfiTableHeader,
    /// Raise the task priority level.
    pub raise_tpl: EfiRaiseTpl,
    /// Restore the task priority level.
    pub restore_tpl: EfiRestoreTpl,
    /// Allocate memory pages.
    pub allocate_pages: EfiAllocatePages,
    /// Free memory pages.
    pub free_pages: EfiFreePages,
    /// Retrieve the memory map.
    pub get_memory_map: EfiGetMemoryMap,
    /// Allocate pool memory.
    pub allocate_pool: EfiAllocatePool,
    /// Free pool memory.
    pub free_pool: EfiFreePool,
    /// Create an event.
    pub create_event: EfiCreateEvent,
    /// Set the timer for an event.
    pub set_timer: EfiSetTimer,
    /// Wait for an event.
    pub wait_for_event: EfiWaitForEvent,
    /// Signal an event.
    pub signal_event: EfiSignalEvent,
    /// Close an event.
    pub close_event: EfiCloseEvent,
    /// Check whether an event is signalled.
    pub check_event: EfiCheckEvent,
    /// Install a protocol interface.
    pub install_protocol_interface: EfiInstallProtocolInterface,
    /// Reinstall a protocol interface.
    pub reinstall_protocol_interface: EfiReinstallProtocolInterface,
    /// Uninstall a protocol interface.
    pub uninstall_protocol_interface: EfiUninstallProtocolInterface,
    /// Query a handle for a protocol.
    pub handle_protocol: EfiHandleProtocol,
    /// Reserved for future use.
    pub reserved: *mut c_void,
    /// Register a protocol notification.
    pub register_protocol_notify: EfiRegisterProtocolNotify,
    /// Locate handles matching a search.
    pub locate_handle: EfiLocateHandle,
    /// Locate a device by device path.
    pub locate_device_path: EfiLocateDevicePath,
    /// Install a configuration table.
    pub install_configuration_table: EfiInstallConfigurationTable,
    /// Load an image into memory.
    pub load_image: EfiLoadImage,
    /// Start a loaded image.
    pub start_image: EfiStartImage,
    /// Exit a loaded image.
    pub exit: EfiExit,
    /// Unload an image.
    pub unload_image: EfiUnloadImage,
    /// Exit boot services.
    pub exit_boot_services: EfiExitBootServices,
    /// Retrieve the next monotonic count.
    pub get_next_monotonic_count: EfiGetNextMonotonicCount,
    /// Stall execution.
    pub stall: EfiStall,
    /// Set the watchdog timer.
    pub set_watchdog_timer: EfiSetWatchdogTimer,
    /// Connect drivers to a controller.
    pub connect_controller: EfiConnectController,
    /// Disconnect drivers from a controller.
    pub disconnect_controller: EfiDisconnectController,
    /// Open a protocol on a handle.
    pub open_protocol: EfiOpenProtocol,
    /// Close a protocol on a handle.
    pub close_protocol: EfiCloseProtocol,
    /// Retrieve open protocol information.
    pub open_protocol_information: EfiOpenProtocolInformation,
    /// Retrieve protocols for a handle.
    pub protocols_per_handle: EfiProtocolsPerHandle,
    /// Locate handles into a pool buffer.
    pub locate_handle_buffer: EfiLocateHandleBuffer,
    /// Locate a protocol.
    pub locate_protocol: EfiLocateProtocol,
    /// Install multiple protocol interfaces.
    pub install_multiple_protocol_interfaces: EfiInstallMultipleProtocolInterfaces,
    /// Uninstall multiple protocol interfaces.
    pub uninstall_multiple_protocol_interfaces: EfiUninstallMultipleProtocolInterfaces,
    /// Compute a 32-bit CRC.
    pub calculate_crc32: EfiCalculateCrc32,
    /// Copy memory.
    pub copy_mem: EfiCopyMem,
    /// Fill memory.
    pub set_mem: EfiSetMem,
    /// Create an event in a group.
    pub create_event_ex: EfiCreateEventEx,
}

const _: () = {
    assert!(core::mem::size_of::<EfiBootServices>() == 24 + 44 * core::mem::size_of::<usize>());
    assert!(core::mem::align_of::<EfiBootServices>() == 8);
    assert!(core::mem::offset_of!(EfiBootServices, hdr) == 0);
    assert!(core::mem::offset_of!(EfiBootServices, raise_tpl) == 24);
    assert!(
        core::mem::offset_of!(EfiBootServices, restore_tpl) == 24 + core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, allocate_pages)
            == 24 + 2 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, free_pages)
            == 24 + 3 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, get_memory_map)
            == 24 + 4 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, allocate_pool)
            == 24 + 5 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, free_pool) == 24 + 6 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, create_event)
            == 24 + 7 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, set_timer) == 24 + 8 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, wait_for_event)
            == 24 + 9 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, signal_event)
            == 24 + 10 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, close_event)
            == 24 + 11 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, check_event)
            == 24 + 12 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, install_protocol_interface)
            == 24 + 13 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, reinstall_protocol_interface)
            == 24 + 14 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, uninstall_protocol_interface)
            == 24 + 15 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, handle_protocol)
            == 24 + 16 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, reserved) == 24 + 17 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, register_protocol_notify)
            == 24 + 18 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, locate_handle)
            == 24 + 19 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, locate_device_path)
            == 24 + 20 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, install_configuration_table)
            == 24 + 21 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, load_image)
            == 24 + 22 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, start_image)
            == 24 + 23 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, exit) == 24 + 24 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, unload_image)
            == 24 + 25 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, exit_boot_services)
            == 24 + 26 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, get_next_monotonic_count)
            == 24 + 27 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, stall) == 24 + 28 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, set_watchdog_timer)
            == 24 + 29 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, connect_controller)
            == 24 + 30 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, disconnect_controller)
            == 24 + 31 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, open_protocol)
            == 24 + 32 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, close_protocol)
            == 24 + 33 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, open_protocol_information)
            == 24 + 34 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, protocols_per_handle)
            == 24 + 35 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, locate_handle_buffer)
            == 24 + 36 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, locate_protocol)
            == 24 + 37 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, install_multiple_protocol_interfaces)
            == 24 + 38 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, uninstall_multiple_protocol_interfaces)
            == 24 + 39 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, calculate_crc32)
            == 24 + 40 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, copy_mem) == 24 + 41 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, set_mem) == 24 + 42 * core::mem::size_of::<usize>()
    );
    assert!(
        core::mem::offset_of!(EfiBootServices, create_event_ex)
            == 24 + 43 * core::mem::size_of::<usize>()
    );
};
