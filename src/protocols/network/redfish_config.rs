//! # Redfish Config Handler Protocol
//!
//! Coordinates configuration initialization and teardown across Redfish feature
//! drivers and REST EX services.

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// Redfish Config Handler Protocol GUID: `{bc0fe6bb-2cc9-463e-9082-fa1176fc67de}`.
pub const REDFISH_CONFIG_HANDLER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBC0F_E6BB,
    0x2CC9,
    0x463E,
    [0x90, 0x82, 0xFA, 0x11, 0x76, 0xFC, 0x67, 0xDE],
);

/// Service descriptor providing metadata for an active Redfish service endpoint.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RedfishConfigServiceInformation {
    /// REST EX protocol handle associated with this Redfish service.
    pub rest_ex_handle: EfiHandle,
    /// Redfish service version number.
    pub service_version: usize,
    /// Service network location string (UTF-16).
    pub location: *const u16,
    /// Service UUID string (UTF-16).
    pub uuid: *const u16,
    /// Host operating system string (UTF-16).
    pub os: *const u16,
    /// Host operating system version string (UTF-16).
    pub os_version: *const u16,
    /// Product name string (UTF-16).
    pub product: *const u16,
    /// Product version string (UTF-16).
    pub product_version: *const u16,
    /// Flag indicating whether the service communication uses HTTPS.
    pub use_https: Bool,
}

/// Function pointer to initialize a Redfish configuration handler.
pub type RedfishConfigHandlerInit = unsafe extern "efiapi" fn(
    this: *const EfiRedfishConfigHandlerProtocol,
    service_info: *const RedfishConfigServiceInformation,
) -> EfiStatus;

/// Function pointer to stop a Redfish configuration handler.
pub type RedfishConfigHandlerStop =
    unsafe extern "efiapi" fn(this: *const EfiRedfishConfigHandlerProtocol) -> EfiStatus;

/// Low-level Redfish Config Handler Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiRedfishConfigHandlerProtocol {
    /// Initializes the feature handler against the Redfish endpoint.
    pub init: RedfishConfigHandlerInit,
    /// Stops the feature handler and releases associated resources.
    pub stop: RedfishConfigHandlerStop,
}

/// Safe client wrapper around `EfiRedfishConfigHandlerProtocol`.
#[derive(Debug, Clone, Copy)]
pub struct RedfishConfigHandler {
    proto: *const EfiRedfishConfigHandlerProtocol,
}

impl RedfishConfigHandler {
    /// Wraps a raw pointer to `EfiRedfishConfigHandlerProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiRedfishConfigHandlerProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiRedfishConfigHandlerProtocol) -> Self {
        Self { proto }
    }

    /// Initializes this Redfish configuration feature driver.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if REST EX communication setup fails.
    /// Returns `EfiStatus::ALREADY_STARTED` if already initialized.
    pub fn init(&self, service_info: &RedfishConfigServiceInformation) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).init)(self.proto, service_info) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Stops this Redfish configuration feature driver.
    ///
    /// # Errors
    ///
    /// Returns an error status if terminating the handler fails.
    pub fn stop(&self) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).stop)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
