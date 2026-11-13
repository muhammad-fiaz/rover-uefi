//! UEFI Authentication Info Protocol definition and safe wrapper.
//!
//! Defined in UEFI Specification 2.0+ Section 31.
//! This protocol associates authentication data (such as CHAP, RADIUS, or local
//! credentials) with controller handles representing physical or logical devices.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// UEFI Authentication Info Protocol GUID: `{7671d9d0-53db-4173-aa69-2327f21f0bc7}`.
pub const EFI_AUTHENTICATION_INFO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x7671_D9D0,
    0x53DB,
    0x4173,
    [0xAA, 0x69, 0x23, 0x27, 0xF2, 0x1F, 0x0B, 0xC7],
);

/// CHAP with RADIUS Authentication Type GUID: `{d6062b50-15ca-11da-9219-001083ffca4d}`.
pub const EFI_AUTHENTICATION_CHAP_RADIUS_GUID: EfiGuid = EfiGuid::new(
    0xD606_2B50,
    0x15CA,
    0x11DA,
    [0x92, 0x19, 0x00, 0x10, 0x83, 0xFF, 0xCA, 0x4D],
);

/// Local CHAP Authentication Type GUID: `{c280c73e-15ca-11da-b0ca-001083ffca4d}`.
pub const EFI_AUTHENTICATION_CHAP_LOCAL_GUID: EfiGuid = EfiGuid::new(
    0xC280_C73E,
    0x15CA,
    0x11DA,
    [0xB0, 0xCA, 0x00, 0x10, 0x83, 0xFF, 0xCA, 0x4D],
);

/// Common header for authentication node records.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthNodeHeader {
    /// Authentication type GUID.
    pub guid: EfiGuid,
    /// Total byte length of this authentication node structure.
    pub length: u16,
}

/// Function prototype for retrieving authentication information.
pub type EfiAuthenticationInfoProtocolGet = unsafe extern "efiapi" fn(
    this: *const EfiAuthenticationInfoProtocol,
    controller_handle: EfiHandle,
    buffer: *mut *mut c_void,
) -> EfiStatus;

/// Function prototype for configuring authentication information.
pub type EfiAuthenticationInfoProtocolSet = unsafe extern "efiapi" fn(
    this: *const EfiAuthenticationInfoProtocol,
    controller_handle: EfiHandle,
    buffer: *const c_void,
) -> EfiStatus;

/// UEFI Authentication Info Protocol structure (`EFI_AUTHENTICATION_INFO_PROTOCOL`).
#[repr(C)]
pub struct EfiAuthenticationInfoProtocol {
    /// Retrieves authentication information for `ControllerHandle`.
    pub get: EfiAuthenticationInfoProtocolGet,
    /// Sets authentication information for `ControllerHandle`.
    pub set: EfiAuthenticationInfoProtocolSet,
}

/// Safe idiomatic wrapper around `EfiAuthenticationInfoProtocol`.
pub struct AuthenticationInfo {
    proto: *const EfiAuthenticationInfoProtocol,
}

impl AuthenticationInfo {
    /// Creates a new `AuthenticationInfo` wrapper.
    #[must_use]
    pub const fn new(proto: *const EfiAuthenticationInfoProtocol) -> Self {
        Self { proto }
    }

    /// Returns the raw protocol pointer.
    #[must_use]
    pub const fn raw(&self) -> *const EfiAuthenticationInfoProtocol {
        self.proto
    }

    /// Retrieves an allocated authentication info buffer pointer for `controller_handle`.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if no matching info found, or an error status on hardware error.
    pub fn get(&self, controller_handle: EfiHandle) -> Result<*mut c_void, EfiStatus> {
        let mut buf_ptr = core::ptr::null_mut();
        let status = unsafe {
            ((*self.proto).get)(
                self.proto,
                controller_handle,
                core::ptr::from_mut(&mut buf_ptr),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(buf_ptr)
        } else {
            Err(status)
        }
    }

    /// Configures authentication info for `controller_handle`.
    ///
    /// # Errors
    ///
    /// Returns `UNSUPPORTED` if platform policy prohibits setting info, or an error status.
    pub fn set(&self, controller_handle: EfiHandle, buffer: &[u8]) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).set)(
                self.proto,
                controller_handle,
                buffer.as_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
