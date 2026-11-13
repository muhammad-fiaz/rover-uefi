//! # EFI HTTP Boot Callback Protocol
//!
//! Defined in UEFI Specification 2.7+ Section 29.7.
//!
//! Provides notification hooks for HTTP boot events (DHCP, DNS, TLS, HTTP request/response).

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiStatus};

/// EFI HTTP Boot Callback Protocol `GUID`.
pub const EFI_HTTP_BOOT_CALLBACK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBA23_B311,
    0x343D,
    0x11E6,
    [0x91, 0x85, 0x58, 0x20, 0xB1, 0xD6, 0x52, 0x99],
);

/// HTTP Boot Callback Data Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiHttpBootCallbackDataType {
    /// DHCP4 packet about to be transmitted or received.
    HttpBootDhcp4 = 0,
    /// DHCP6 packet about to be transmitted or received.
    HttpBootDhcp6 = 1,
    /// HTTP request message about to be transmitted.
    HttpBootHttpRequest = 2,
    /// HTTP response message received.
    HttpBootHttpResponse = 3,
    /// Partial entity body received.
    HttpBootHttpEntityBody = 4,
}

/// HTTP Boot Callback function type.
pub type EfiHttpBootCallback = unsafe extern "efiapi" fn(
    this: *mut EfiHttpBootCallbackProtocol,
    data_type: EfiHttpBootCallbackDataType,
    received: bool,
    data_length: u32,
    data: *const c_void,
) -> EfiStatus;

/// EFI HTTP Boot Callback Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiHttpBootCallbackProtocol {
    /// Callback function invoked during HTTP boot stages.
    pub callback: EfiHttpBootCallback,
}
