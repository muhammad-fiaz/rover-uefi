//! # Xen Paravirtualized I/O Protocols
//!
//! Provides the Xen I/O and `XenBus` protocol definitions used by guests running
//! on Xen hypervisors (such as AWS EC2 Xen instances and Qubes OS).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Xen I/O Protocol GUID.
pub const XENIO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6EFA_C84F,
    0x0AB0,
    0x4747,
    [0x81, 0xBE, 0x85, 0x55, 0x62, 0x59, 0x04, 0x49],
);

/// `XenBus` Protocol GUID.
pub const XENBUS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3D3C_A290,
    0xB9A5,
    0x11E3,
    [0xB7, 0x5D, 0xB8, 0xAC, 0x6F, 0x7D, 0x65, 0xE6],
);

/// Xen Paravirtualized Device States.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XenBusState {
    /// Device state unknown.
    Unknown = 0,
    /// Device is initialising.
    Initialising = 1,
    /// Device is waiting for init.
    InitWait = 2,
    /// Device initialization complete.
    Initialised = 3,
    /// Device connected and operational.
    Connected = 4,
    /// Device closing.
    Closing = 5,
    /// Device closed.
    Closed = 6,
    /// Device reconfiguring.
    Reconfiguring = 7,
    /// Device reconfigured.
    Reconfigured = 8,
}

/// `XenStore` Status Codes.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XenStoreStatus {
    /// Success.
    Success = 0,
    /// General failure.
    Fail = 1,
    /// Invalid parameter.
    Einval = 2,
    /// Access denied.
    Eacces = 3,
    /// Node already exists.
    Eexist = 4,
    /// Is a directory.
    Eisdir = 5,
    /// No such entity.
    Enoent = 6,
    /// Out of memory.
    Enomem = 7,
    /// Out of space.
    Enospc = 8,
    /// I/O error.
    Eio = 9,
}

/// Xen I/O Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XenIoProtocol {
    /// Guest physical base address of the Xen grant table.
    pub grant_table_address: u64,
}

/// Function pointer to read a `XenStore` node.
pub type XenStoreRead = unsafe extern "efiapi" fn(
    this: *mut XenBusProtocol,
    node: *const u8,
    value: *mut *mut u8,
) -> EfiStatus;

/// Function pointer to write a `XenStore` node.
pub type XenStoreWrite = unsafe extern "efiapi" fn(
    this: *mut XenBusProtocol,
    node: *const u8,
    value: *const u8,
) -> EfiStatus;

/// `XenBus` Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XenBusProtocol {
    /// Xen PV device type string pointer (e.g. "vif" or "vbd").
    pub r#type: *const u8,
    /// Device ID on the bus.
    pub device_id: u32,
    /// `XenStore` node path string pointer for this device.
    pub node: *const u8,
    /// `XenStore` backend node path string pointer.
    pub backend: *const u8,
    /// Current device state.
    pub state: XenBusState,
    /// Reads value from `XenStore`.
    pub xs_read: Option<XenStoreRead>,
    /// Writes value to `XenStore`.
    pub xs_write: Option<XenStoreWrite>,
    /// Waits for backend state transition.
    pub wait_for_backend_state: Option<
        unsafe extern "efiapi" fn(this: *mut XenBusProtocol, state: XenBusState) -> EfiStatus,
    >,
    /// Allocates an event channel.
    pub event_channel_allocate:
        Option<unsafe extern "efiapi" fn(this: *mut XenBusProtocol, port: *mut u32) -> EfiStatus>,
    /// Frees an event channel.
    pub event_channel_free:
        Option<unsafe extern "efiapi" fn(this: *mut XenBusProtocol, port: u32) -> EfiStatus>,
    /// Grants shared page access to backend domain.
    pub grant_access: Option<
        unsafe extern "efiapi" fn(
            this: *mut XenBusProtocol,
            grant_ref: *mut u32,
            page_frame_number: usize,
            readonly: bool,
        ) -> EfiStatus,
    >,
    /// Revokes granted page access.
    pub grant_end_access:
        Option<unsafe extern "efiapi" fn(this: *mut XenBusProtocol, grant_ref: u32) -> EfiStatus>,
    /// Reserved context pointer.
    pub context: *mut c_void,
}
