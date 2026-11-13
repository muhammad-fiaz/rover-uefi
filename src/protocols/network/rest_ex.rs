//! # REST EX Protocol
//!
//! Provides an interface to interact with REST services (such as Redfish or `OData`)
//! using HTTP message payloads.
//!
//! Defined in the UEFI Specification, Section 30.3.

use core::ffi::c_void;

use crate::base::{EfiEvent, EfiGuid, EfiStatus, EfiTime};

pub use super::guid::{
    REST_EX_PROTOCOL_GUID, REST_EX_SERVICE_BINDING_PROTOCOL_GUID, REST_PROTOCOL_GUID,
};

/// Version information for the REST EX service.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EfiRestExServiceInfoVer {
    /// Major version.
    pub major: u8,
    /// Minor version.
    pub minor: u8,
}

/// Service information header.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EfiRestExServiceInfoHeader {
    /// Total length of the service info structure.
    pub length: u32,
    /// Service info version.
    pub rest_service_info_ver: EfiRestExServiceInfoVer,
}

/// Service type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiRestExServiceType {
    /// Unspecific REST service.
    Unspecific = 1,
    /// Redfish REST service.
    Redfish = 2,
    /// `OData` REST service.
    OData = 3,
    /// Vendor-specific REST service.
    VendorSpecific = 0xFF,
}

/// Service access mode.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiRestExServiceAccessMode {
    /// In-band access.
    InBandAccess = 1,
    /// Out-of-band access.
    OutOfBandAccess = 2,
}

/// Config type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiRestExConfigType {
    /// HTTP configuration.
    Http = 0,
    /// Unspecific configuration.
    Unspecific = 1,
}

/// Service information structure (version 1.0).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRestExServiceInfoV10 {
    /// Service information header.
    pub header: EfiRestExServiceInfoHeader,
    /// Service type.
    pub rest_service_type: u32,
    /// Service access mode.
    pub rest_service_access_mode: u32,
    /// Vendor REST service name GUID.
    pub vendor_rest_service_name: EfiGuid,
    /// Vendor specific data length in bytes.
    pub vendor_specific_data_length: u32,
    /// Pointer to vendor specific data.
    pub vendor_specific_data: *mut u8,
    /// REST EX configuration type.
    pub rest_ex_config_type: u32,
    /// REST EX configuration data length.
    pub rest_ex_config_data_length: u8,
}

/// HTTP message structure used with REST EX.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiHttpMessage {
    /// Request data pointer (if request).
    pub request: *mut c_void,
    /// Response data pointer (if response).
    pub response: *mut c_void,
    /// Number of headers.
    pub header_count: usize,
    /// Headers array pointer.
    pub headers: *mut c_void,
    /// Body length in bytes.
    pub body_length: usize,
    /// Body buffer pointer.
    pub body: *mut c_void,
}

/// Token for asynchronous REST EX operations.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRestExToken {
    /// Event signaled upon completion.
    pub event: EfiEvent,
    /// Status code resulting from the transaction.
    pub status: EfiStatus,
    /// Pointer to the response message.
    pub response_message: *mut EfiHttpMessage,
}

/// Sends an HTTP request and retrieves the response.
pub type EfiRestSendReceive = unsafe extern "efiapi" fn(
    this: *mut EfiRestExProtocol,
    request_message: *mut EfiHttpMessage,
    response_message: *mut EfiHttpMessage,
) -> EfiStatus;

/// Obtains the current time from the REST service instance.
pub type EfiRestGetTime =
    unsafe extern "efiapi" fn(this: *mut EfiRestExProtocol, time: *mut EfiTime) -> EfiStatus;

/// Returns information about the REST service provided by this instance.
pub type EfiRestExGetService = unsafe extern "efiapi" fn(
    this: *mut EfiRestExProtocol,
    rest_ex_service_info: *mut *mut EfiRestExServiceInfoV10,
) -> EfiStatus;

/// Returns operational configuration of the REST EX instance.
pub type EfiRestExGetModeData = unsafe extern "efiapi" fn(
    this: *mut EfiRestExProtocol,
    rest_ex_config_data: *mut *mut c_void,
) -> EfiStatus;

/// Configures the REST EX instance.
pub type EfiRestExConfigure = unsafe extern "efiapi" fn(
    this: *mut EfiRestExProtocol,
    rest_ex_config_data: *mut c_void,
) -> EfiStatus;

/// Asynchronously sends a request and signals token upon completion.
pub type EfiRestExAsyncSendReceive = unsafe extern "efiapi" fn(
    this: *mut EfiRestExProtocol,
    request_message: *mut EfiHttpMessage,
    rest_ex_token: *mut EfiRestExToken,
    timeout_in_milli_seconds: *mut usize,
) -> EfiStatus;

/// Sends a request to a REST Event service.
pub type EfiRestExEventService = unsafe extern "efiapi" fn(
    this: *mut EfiRestExProtocol,
    request_message: *mut EfiHttpMessage,
    rest_ex_token: *mut EfiRestExToken,
) -> EfiStatus;

/// The EFI REST EX Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRestExProtocol {
    /// Sends an HTTP request and waits for response.
    pub send_receive: EfiRestSendReceive,
    /// Gets service time.
    pub get_service_time: EfiRestGetTime,
    /// Gets service info.
    pub get_service: EfiRestExGetService,
    /// Gets mode data.
    pub get_mode_data: EfiRestExGetModeData,
    /// Configures the instance.
    pub configure: EfiRestExConfigure,
    /// Asynchronously sends and receives.
    pub async_send_receive: EfiRestExAsyncSendReceive,
    /// Subscribes to event service.
    pub event_service: EfiRestExEventService,
}

/// The EFI REST Protocol (UEFI 2.5+).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRestProtocol {
    /// Sends an HTTP request and waits for response.
    pub send_receive: unsafe extern "efiapi" fn(
        this: *mut EfiRestProtocol,
        request_message: *mut EfiHttpMessage,
        response_message: *mut EfiHttpMessage,
    ) -> EfiStatus,
    /// Gets service time.
    pub get_service_time:
        unsafe extern "efiapi" fn(this: *mut EfiRestProtocol, time: *mut EfiTime) -> EfiStatus,
}

/// Safe wrapper for `EfiRestProtocol`.
pub struct Rest<'a> {
    protocol: &'a mut EfiRestProtocol,
}

impl<'a> Rest<'a> {
    /// Wraps a mutable reference to `EfiRestProtocol`.
    pub fn new(protocol: &'a mut EfiRestProtocol) -> Self {
        Self { protocol }
    }

    /// Sends an HTTP request and returns the response synchronously.
    ///
    /// # Errors
    /// Returns `EfiStatus` on error.
    pub fn send_receive(
        &mut self,
        request: &mut EfiHttpMessage,
        response: &mut EfiHttpMessage,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.send_receive)(
                core::ptr::from_mut(self.protocol),
                core::ptr::from_mut(request),
                core::ptr::from_mut(response),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Obtains current time from the REST service.
    ///
    /// # Errors
    /// Returns `EfiStatus` on error.
    pub fn service_time(&mut self) -> Result<EfiTime, EfiStatus> {
        let mut time = EfiTime::default();
        let status = unsafe {
            (self.protocol.get_service_time)(
                core::ptr::from_mut(self.protocol),
                core::ptr::from_mut(&mut time),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(time)
        }
    }
}

/// Safe wrapper for `EfiRestExProtocol`.
pub struct RestEx<'a> {
    protocol: &'a mut EfiRestExProtocol,
}

impl<'a> RestEx<'a> {
    /// Wraps a mutable reference to `EfiRestExProtocol`.
    pub fn new(protocol: &'a mut EfiRestExProtocol) -> Self {
        Self { protocol }
    }

    /// Sends an HTTP request and receives the response synchronously.
    ///
    /// # Errors
    /// Returns `EfiStatus` on error.
    pub fn send_receive(
        &mut self,
        request: &mut EfiHttpMessage,
        response: &mut EfiHttpMessage,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.send_receive)(
                core::ptr::from_mut(self.protocol),
                core::ptr::from_mut(request),
                core::ptr::from_mut(response),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Obtains the current time from the REST service.
    ///
    /// # Errors
    /// Returns `EfiStatus` on error.
    pub fn service_time(&mut self) -> Result<EfiTime, EfiStatus> {
        let mut time = EfiTime::default();
        let status = unsafe {
            (self.protocol.get_service_time)(
                core::ptr::from_mut(self.protocol),
                core::ptr::from_mut(&mut time),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(time)
        }
    }

    /// Retrieves service information for the REST EX service.
    ///
    /// # Errors
    /// Returns `EfiStatus` on error.
    pub fn service_info(&mut self) -> Result<*mut EfiRestExServiceInfoV10, EfiStatus> {
        let mut info = core::ptr::null_mut();
        let status = unsafe {
            (self.protocol.get_service)(
                core::ptr::from_mut(self.protocol),
                core::ptr::from_mut(&mut info),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(info)
        }
    }

    /// Configures the REST EX instance.
    ///
    /// # Errors
    /// Returns `EfiStatus` on error.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn configure(&mut self, config: *mut c_void) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.protocol.configure)(core::ptr::from_mut(self.protocol), config) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
