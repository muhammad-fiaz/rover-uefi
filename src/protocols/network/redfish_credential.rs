//! # Redfish Credential Protocols
//!
//! Provides services to retrieve BMC authentication credentials (HTTP basic auth,
//! Redfish session credentials, bootstrap accounts) and manage Redfish service
//! lifecycle termination across Secure Boot and boot services transitions.

use crate::base::{EfiGuid, EfiStatus};

/// Redfish Credential Protocol GUID: `{08804377-af7a-4496-8a7b-175900e9ab46}`.
pub const REDFISH_CREDENTIAL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0880_4377,
    0xAF7A,
    0x4496,
    [0x8A, 0x7B, 0x17, 0x59, 0x00, 0xE9, 0xAB, 0x46],
);

/// Redfish Credential 2 Protocol GUID: `{936b81dc-348c-42e3-9e82-02914fd34886}`.
pub const REDFISH_CREDENTIAL2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x936B_81DC,
    0x348C,
    0x42E3,
    [0x9E, 0x82, 0x02, 0x91, 0x4F, 0xD3, 0x48, 0x86],
);

/// Current revision of the Redfish Credential 2 interface.
pub const REDFISH_CREDENTIAL_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Authentication method required by the Redfish service.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RedfishAuthMethod {
    /// No authentication required.
    None = 0,
    /// HTTP Basic authentication (RFC 7235).
    HttpBasic = 1,
    /// Redfish session token login (DSP0266).
    RedfishSession = 2,
    /// Maximum delimiter value.
    Max = 3,
}

/// Reason for requesting termination of Redfish services.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RedfishCredentialStopServiceType {
    /// Service stopped without specific reason.
    None = 0,
    /// Service stopped because Secure Boot is disabled.
    SecureBootDisabled = 1,
    /// Service stopped due to `ExitBootServices` handoff.
    ExitBootService = 2,
    /// Maximum delimiter value.
    Max = 3,
}

/// Function pointer to query Redfish authentication information.
pub type RedfishCredentialGetAuthInfo = unsafe extern "efiapi" fn(
    this: *const EfiRedfishCredentialProtocol,
    auth_method: *mut RedfishAuthMethod,
    user_id: *mut *mut u8,
    password: *mut *mut u8,
) -> EfiStatus;

/// Function pointer to stop Redfish service access.
pub type RedfishCredentialStopService = unsafe extern "efiapi" fn(
    this: *const EfiRedfishCredentialProtocol,
    service_stop_type: RedfishCredentialStopServiceType,
) -> EfiStatus;

/// Function pointer to query Redfish 2 authentication information.
pub type RedfishCredential2GetAuthInfo = unsafe extern "efiapi" fn(
    this: *const EfiRedfishCredential2Protocol,
    auth_method: *mut RedfishAuthMethod,
    user_id: *mut *mut u8,
    password: *mut *mut u8,
) -> EfiStatus;

/// Function pointer to stop Redfish 2 service access.
pub type RedfishCredential2StopService = unsafe extern "efiapi" fn(
    this: *const EfiRedfishCredential2Protocol,
    service_stop_type: RedfishCredentialStopServiceType,
) -> EfiStatus;

/// Function pointer to register an active Redfish service consumer.
pub type RedfishCredential2RegisterService = unsafe extern "efiapi" fn(
    this: *const EfiRedfishCredential2Protocol,
    service: *const core::ffi::c_void,
) -> EfiStatus;

/// Function pointer to unregister an active Redfish service consumer.
pub type RedfishCredential2UnregisterService = unsafe extern "efiapi" fn(
    this: *const EfiRedfishCredential2Protocol,
    service: *const core::ffi::c_void,
) -> EfiStatus;

/// Low-level Redfish Credential Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiRedfishCredentialProtocol {
    /// Retrieves authentication credentials.
    pub get_auth_info: RedfishCredentialGetAuthInfo,
    /// Stops Redfish platform service access.
    pub stop_service: RedfishCredentialStopService,
}

/// Low-level Redfish Credential 2 Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiRedfishCredential2Protocol {
    /// Interface revision (`REDFISH_CREDENTIAL_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Retrieves authentication credentials.
    pub get_auth_info: RedfishCredential2GetAuthInfo,
    /// Stops Redfish platform service access.
    pub stop_service: RedfishCredential2StopService,
    /// Registers an active Redfish service reference.
    pub register_redfish_service: RedfishCredential2RegisterService,
    /// Unregisters an active Redfish service reference.
    pub unregister_redfish_service: RedfishCredential2UnregisterService,
}

/// Safe client wrapper around `EfiRedfishCredentialProtocol`.
#[derive(Debug, Clone, Copy)]
pub struct RedfishCredential {
    proto: *const EfiRedfishCredentialProtocol,
}

impl RedfishCredential {
    /// Wraps a raw pointer to `EfiRedfishCredentialProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiRedfishCredentialProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiRedfishCredentialProtocol) -> Self {
        Self { proto }
    }

    /// Queries platform Redfish authentication information.
    ///
    /// Returns the authentication method and raw pointers to null-terminated ASCII
    /// user ID and password buffers.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::ACCESS_DENIED` if Secure Boot is disabled.
    /// Returns `EfiStatus::UNSUPPORTED` if authentication method is not supported.
    pub fn get_auth_info(&self) -> Result<(RedfishAuthMethod, *mut u8, *mut u8), EfiStatus> {
        let mut method = RedfishAuthMethod::None;
        let mut user_id = core::ptr::null_mut();
        let mut password = core::ptr::null_mut();

        let status = unsafe {
            ((*self.proto).get_auth_info)(
                self.proto,
                &raw mut method,
                &raw mut user_id,
                &raw mut password,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((method, user_id, password))
        } else {
            Err(status)
        }
    }

    /// Stops Redfish service access.
    ///
    /// # Errors
    ///
    /// Returns an error status if terminating service access fails.
    pub fn stop_service(
        &self,
        stop_type: RedfishCredentialStopServiceType,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).stop_service)(self.proto, stop_type) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiRedfishCredential2Protocol`.
#[derive(Debug, Clone, Copy)]
pub struct RedfishCredential2 {
    proto: *const EfiRedfishCredential2Protocol,
}

impl RedfishCredential2 {
    /// Wraps a raw pointer to `EfiRedfishCredential2Protocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiRedfishCredential2Protocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiRedfishCredential2Protocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol interface revision.
    #[must_use]
    pub fn revision(&self) -> u64 {
        unsafe { (*self.proto).revision }
    }

    /// Queries platform Redfish authentication information.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::ACCESS_DENIED` if Secure Boot is disabled.
    /// Returns `EfiStatus::UNSUPPORTED` if authentication method is not supported.
    pub fn get_auth_info(&self) -> Result<(RedfishAuthMethod, *mut u8, *mut u8), EfiStatus> {
        let mut method = RedfishAuthMethod::None;
        let mut user_id = core::ptr::null_mut();
        let mut password = core::ptr::null_mut();

        let status = unsafe {
            ((*self.proto).get_auth_info)(
                self.proto,
                &raw mut method,
                &raw mut user_id,
                &raw mut password,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((method, user_id, password))
        } else {
            Err(status)
        }
    }

    /// Stops Redfish service access.
    ///
    /// # Errors
    ///
    /// Returns an error status if terminating service access fails.
    pub fn stop_service(
        &self,
        stop_type: RedfishCredentialStopServiceType,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).stop_service)(self.proto, stop_type) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers a Redfish service consumer.
    ///
    /// # Safety
    ///
    /// `service` must point to an active, valid Redfish service instance or be null.
    ///
    /// # Errors
    ///
    /// Returns an error status if registration fails.
    pub unsafe fn register_service(&self, service: *const core::ffi::c_void) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).register_redfish_service)(self.proto, service) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters a Redfish service consumer.
    ///
    /// # Safety
    ///
    /// `service` must point to an active, valid Redfish service instance or be null.
    ///
    /// # Errors
    ///
    /// Returns an error status if unregistering fails.
    pub unsafe fn unregister_service(&self, service: *const core::ffi::c_void) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).unregister_redfish_service)(self.proto, service) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
