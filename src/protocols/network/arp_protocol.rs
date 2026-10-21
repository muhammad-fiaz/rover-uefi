//! # ARP Protocol
//!
//! Declarations for `EFI_ARP_PROTOCOL` and `EFI_ARP_SERVICE_BINDING_PROTOCOL`
//! according to the UEFI Specification.
//!
//! Provides resolution between IP addresses and physical hardware addresses (e.g. Ethernet MAC).

use crate::abi::types::Bool;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use core::ffi::c_void;

/// ARP service binding protocol GUID: `{f44c00ee-1f2c-4a00-aa09-1c9f3e0800a3}`.
pub const ARP_SERVICE_BINDING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xf44c_00ee,
    0x1f2c,
    0x4a00,
    [0xaa, 0x09, 0x1c, 0x9f, 0x3e, 0x08, 0x00, 0xa3],
);

/// ARP protocol GUID: `{f4b427bb-ba21-4f16-bc4e-43e416ab619c}`.
pub const ARP_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xf4b4_27bb,
    0xba21,
    0x4f16,
    [0xbc, 0x4e, 0x43, 0xe4, 0x16, 0xab, 0x61, 0x9c],
);

/// ARP cache entry configuration and state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiArpConfigData {
    /// Swapped target IP address if configured.
    pub sw_address_type: u16,
    /// Length of software address in bytes (e.g. 4 for IPv4).
    pub sw_address_length: u8,
    /// Station software address pointer.
    pub station_address: *const c_void,
    /// Entry timeout in 100ns units (0 means infinite).
    pub entry_time_to_live: u32,
    /// Retry count before failing resolution.
    pub retry_count: u32,
    /// Retry time interval in 100ns units.
    pub retry_time_out: u32,
}

/// Function pointer for `EFI_ARP_CONFIGURE`.
pub type EfiArpConfigure = unsafe extern "efiapi" fn(
    this: *mut EfiArpProtocol,
    config_data: *const EfiArpConfigData,
) -> EfiStatus;

/// Function pointer for `EFI_ARP_ADD`.
pub type EfiArpAdd = unsafe extern "efiapi" fn(
    this: *mut EfiArpProtocol,
    deny_flag: Bool,
    target_sw_address: *const c_void,
    target_hw_address: *const c_void,
    timeout_value: u32,
    overwrite: Bool,
) -> EfiStatus;

/// Function pointer for `EFI_ARP_FIND`.
pub type EfiArpFind = unsafe extern "efiapi" fn(
    this: *mut EfiArpProtocol,
    by_sw_address: Bool,
    address_buffer: *const c_void,
    entry_length: *mut u32,
    entry_count: *mut u32,
    entries: *mut *mut c_void,
    refresh: Bool,
) -> EfiStatus;

/// Function pointer for `EFI_ARP_DELETE`.
pub type EfiArpDelete = unsafe extern "efiapi" fn(
    this: *mut EfiArpProtocol,
    by_sw_address: Bool,
    address_buffer: *const c_void,
) -> EfiStatus;

/// Function pointer for `EFI_ARP_FLUSH`.
pub type EfiArpFlush = unsafe extern "efiapi" fn(this: *mut EfiArpProtocol) -> EfiStatus;

/// Function pointer for `EFI_ARP_REQUEST`.
pub type EfiArpRequest = unsafe extern "efiapi" fn(
    this: *mut EfiArpProtocol,
    target_sw_address: *const c_void,
    resolved_event: EfiEvent,
    target_hw_address: *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_ARP_CANCEL`.
pub type EfiArpCancel = unsafe extern "efiapi" fn(
    this: *mut EfiArpProtocol,
    target_sw_address: *const c_void,
    resolved_event: EfiEvent,
) -> EfiStatus;

/// `EFI_ARP_PROTOCOL` interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiArpProtocol {
    /// Configures the ARP driver instance.
    pub configure: EfiArpConfigure,
    /// Adds an entry to the ARP cache.
    pub add: EfiArpAdd,
    /// Searches for matching ARP cache entries.
    pub find: EfiArpFind,
    /// Deletes matching entries from the ARP cache.
    pub delete: EfiArpDelete,
    /// Flushes non-static entries from the ARP cache.
    pub flush: EfiArpFlush,
    /// Issues an ARP request to resolve an address.
    pub request: EfiArpRequest,
    /// Cancels a pending ARP request.
    pub cancel: EfiArpCancel,
}
