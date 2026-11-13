//! # Device Security and Security Policy Protocols
//!
//! Provides hardware device authentication, measurement interfaces, platform
//! security policy interrogation, and TPM 2.0 CRB over FF-A services prior to driver binding.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr
)]

use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// Device Security Protocol GUID.
pub const DEVICE_SECURITY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5D6B_38C8,
    0x5510,
    0x4458,
    [0xB4, 0x8D, 0x95, 0x81, 0xCF, 0xA7, 0xB0, 0x0D],
);

/// Device Security Policy Protocol GUID.
pub const DEVICE_SECURITY_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4386_C174,
    0xAE19,
    0x4896,
    [0x95, 0x73, 0x2F, 0x7E, 0x77, 0x98, 0x96, 0x2E],
);

/// TPM 2.0 Service over FF-A GUID (ARM DEN0138).
pub const TPM2_SERVICE_FFA_GUID: EfiGuid = EfiGuid::new(
    0x17B8_62A4,
    0x1806,
    0x4FAF,
    [0x86, 0xB3, 0x08, 0x9A, 0x58, 0x35, 0x38, 0x61],
);

/// Device Identifier PCI type GUID.
pub const DEVICE_IDENTIFIER_TYPE_PCI_GUID: EfiGuid = EfiGuid::new(
    0x2509_B2F1,
    0xA022,
    0x4CCA,
    [0xAF, 0x70, 0xF9, 0xD3, 0x21, 0xFB, 0x66, 0x49],
);

/// Device Identifier USB type GUID.
pub const DEVICE_IDENTIFIER_TYPE_USB_GUID: EfiGuid = EfiGuid::new(
    0x7394_F350,
    0x394D,
    0x488C,
    [0xBB, 0x75, 0x0C, 0xAB, 0x7B, 0x12, 0x0A, 0xC5],
);

/// Protocol revision 1.0.
pub const DEVICE_SECURITY_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Identifier revision 1.0.
pub const DEVICE_IDENTIFIER_REVISION: u32 = 0x0001_0000;

/// Device Security Policy Protocol revision (1.0).
pub const DEVICE_SECURITY_POLICY_PROTOCOL_REVISION: u32 = 0x0001_0000;

/// Device Security Policy revision (1.0).
pub const DEVICE_SECURITY_POLICY_REVISION: u32 = 0x0001_0000;

/// Device Security State revision (1.0).
pub const DEVICE_SECURITY_STATE_REVISION: u32 = 0x0001_0000;

/// Policy requirements.
pub mod policy {
    /// Device measurement is required.
    pub const MEASUREMENT_REQUIRED: u32 = 1 << 0;
    /// Device authentication is required.
    pub const AUTHENTICATION_REQUIRED: u32 = 1 << 0;
}

/// Device Security state values.
pub mod security_state {
    /// Device passed measurement and authentication.
    pub const SUCCESS: u32 = 0;
    /// Generic device error flag.
    pub const ERROR_BIT: u32 = 1 << 31;
    /// UEFI unsupported error.
    pub const ERROR_UEFI_UNSUPPORTED: u32 = ERROR_BIT;
    /// Failed to retrieve policy protocol.
    pub const ERROR_UEFI_GET_POLICY_PROTOCOL: u32 = ERROR_BIT | 0x01;
    /// Out of resources error.
    pub const ERROR_UEFI_OUT_OF_RESOURCE: u32 = ERROR_BIT | 0x02;
    /// Device lacks requested capabilities.
    pub const ERROR_DEVICE_NO_CAPABILITIES: u32 = ERROR_BIT | 0x10;
    /// Device hardware error.
    pub const ERROR_DEVICE_ERROR: u32 = ERROR_BIT | 0x11;
    /// TCG extend TPM PCR error.
    pub const ERROR_TCG_EXTEND_TPM_PCR: u32 = ERROR_BIT | 0x20;
    /// Measurement authentication failed.
    pub const ERROR_MEASUREMENT_AUTH_FAILURE: u32 = ERROR_BIT | 0x21;
    /// Authentication challenge failed.
    pub const ERROR_CHALLENGE_FAILURE: u32 = ERROR_BIT | 0x30;
    /// Certificate validation failed.
    pub const ERROR_CERTIFICATE_FAILURE: u32 = ERROR_BIT | 0x31;
    /// No certificate provisioned on device.
    pub const ERROR_NO_CERT_PROVISION: u32 = ERROR_BIT | 0x32;
}

/// TPM 2.0 CRB over FF-A function identifiers and status constants.
pub mod tpm2_ffa {
    /// Get TPM 2.0 interface version.
    pub const GET_INTERFACE_VERSION: u32 = 0x0F00_0001;
    /// Get feature information.
    pub const GET_FEATURE_INFO: u32 = 0x0F00_0101;
    /// Start CRB command.
    pub const START: u32 = 0x0F00_0201;
    /// Register for command completion notification.
    pub const REGISTER_FOR_NOTIFICATION: u32 = 0x0F00_0301;
    /// Unregister from notification.
    pub const UNREGISTER_FROM_NOTIFICATION: u32 = 0x0F00_0401;
    /// Finish notified command.
    pub const FINISH_NOTIFIED: u32 = 0x0F00_0501;
    /// Get CRB buffer info.
    pub const GET_CRB_INFO: u32 = 0x0F00_0601;
    /// Manage CRB locality.
    pub const MANAGE_LOCALITY: u32 = 0x0F00_0701;

    /// Success code.
    pub const SUCCESS_OK: u32 = 0x0500_0001;
    /// Success with results returned.
    pub const SUCCESS_OK_RESULTS_RETURNED: u32 = 0x0500_0002;

    /// No function error.
    pub const ERROR_NOFUNC: u32 = 0x8E00_0001;
    /// Not supported error.
    pub const ERROR_NOTSUP: u32 = 0x8E00_0002;
    /// Invalid argument error.
    pub const ERROR_INVARG: u32 = 0x8E00_0005;
    /// Invalid CRB control data error.
    pub const ERROR_INV_CRB_CTRL_DATA: u32 = 0x8E00_0006;
    /// Already started or registered.
    pub const ERROR_ALREADY: u32 = 0x8E00_0009;
    /// Access denied error.
    pub const ERROR_DENIED: u32 = 0x8E00_000A;
    /// Out of memory error.
    pub const ERROR_NOMEM: u32 = 0x8E00_000B;
}

/// The device identifier.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiDeviceIdentifier {
    /// Version of this data structure.
    pub version: u32,
    /// Device type GUID.
    pub device_type: EfiGuid,
    /// Device handle.
    pub device_handle: EfiHandle,
}

/// Authenticates a device.
pub type EfiDeviceAuthenticate = unsafe extern "efiapi" fn(
    this: *mut EfiDeviceSecurityProtocol,
    device_id: *mut EfiDeviceIdentifier,
) -> EfiStatus;

/// The Device Security Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiDeviceSecurityProtocol {
    /// Protocol revision.
    pub revision: u64,
    /// Authenticates the device.
    pub device_authenticate: EfiDeviceAuthenticate,
}

/// High-level safe wrapper around [`EfiDeviceSecurityProtocol`].
pub struct DeviceSecurity<'a> {
    proto: &'a mut EfiDeviceSecurityProtocol,
}

impl<'a> DeviceSecurity<'a> {
    /// Wraps a mutable reference to [`EfiDeviceSecurityProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiDeviceSecurityProtocol) -> Self {
        Self { proto }
    }

    /// Authenticates the device specified by its identifier.
    ///
    /// # Errors
    /// Returns [`EfiStatus::SECURITY_VIOLATION`] if measurement or authentication fails,
    /// or other [`EfiStatus`] on transport error.
    pub fn authenticate(&mut self, device_id: &mut EfiDeviceIdentifier) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.device_authenticate)(self.proto, device_id) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}

/// Device Security Policy data structure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct DeviceSecurityPolicyData {
    /// Revision of structure ([`DEVICE_SECURITY_POLICY_REVISION`]).
    pub revision: u32,
    /// Measurement requirements ([`policy::MEASUREMENT_REQUIRED`]).
    pub measurement_policy: u32,
    /// Authentication requirements ([`policy::AUTHENTICATION_REQUIRED`]).
    pub authentication_policy: u32,
}

/// Device Security State data structure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct DeviceSecurityStateData {
    /// Revision of structure ([`DEVICE_SECURITY_STATE_REVISION`]).
    pub revision: u32,
    /// Measurement state outcome ([`security_state`]).
    pub measurement_state: u32,
    /// Authentication state outcome ([`security_state`]).
    pub authentication_state: u32,
}

/// Raw function pointer for querying device policy.
pub type EfiDeviceSecurityGetDevicePolicy = unsafe extern "efiapi" fn(
    this: *mut EfiDeviceSecurityPolicyProtocol,
    device_id: *const EfiDeviceIdentifier,
    policy: *mut DeviceSecurityPolicyData,
) -> EfiStatus;

/// Raw function pointer for notifying device security state.
pub type EfiDeviceSecurityNotifyDeviceState = unsafe extern "efiapi" fn(
    this: *mut EfiDeviceSecurityPolicyProtocol,
    device_id: *const EfiDeviceIdentifier,
    state: *const DeviceSecurityStateData,
) -> EfiStatus;

/// EFI Device Security Policy Protocol interface structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiDeviceSecurityPolicyProtocol {
    /// Protocol revision ([`DEVICE_SECURITY_POLICY_PROTOCOL_REVISION`]).
    pub revision: u32,
    /// Queries the policy for a given device.
    pub get_device_policy: EfiDeviceSecurityGetDevicePolicy,
    /// Notifies the platform of the authentication/measurement outcome.
    pub notify_device_state: EfiDeviceSecurityNotifyDeviceState,
}

/// High-level safe wrapper around [`EfiDeviceSecurityPolicyProtocol`].
pub struct DeviceSecurityPolicy<'a> {
    proto: &'a mut EfiDeviceSecurityPolicyProtocol,
}

impl<'a> DeviceSecurityPolicy<'a> {
    /// Wraps a mutable reference to [`EfiDeviceSecurityPolicyProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiDeviceSecurityPolicyProtocol) -> Self {
        Self { proto }
    }

    /// Retrieves the policy requirements for the specified device.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure or unsupported device.
    pub fn get_policy(
        &mut self,
        device_id: &EfiDeviceIdentifier,
    ) -> Result<DeviceSecurityPolicyData, EfiStatus> {
        let mut policy = DeviceSecurityPolicyData::default();
        let status = unsafe { (self.proto.get_device_policy)(self.proto, device_id, &mut policy) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(policy)
        }
    }

    /// Notifies the platform of the resulting security state for a device.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure.
    pub fn notify_state(
        &mut self,
        device_id: &EfiDeviceIdentifier,
        state: &DeviceSecurityStateData,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.notify_device_state)(self.proto, device_id, state) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}
