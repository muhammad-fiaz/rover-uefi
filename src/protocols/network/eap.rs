//! # Extensible Authentication Protocol (EAP)
//!
//! Provides services to configure, manage, and monitor 802.1X EAP authentication
//! and supplicant state machines (WPA2/WPA3-Enterprise, 802.1X wired network access).
//! Defined in UEFI Specification 2.2 through 2.5+.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// EAP Protocol GUID: `{5d9f96db-e731-4caa-a00d-72e187cd7762}`.
pub const EFI_EAP_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5D9F_96DB,
    0xE731,
    0x4CAA,
    [0xA0, 0x0D, 0x72, 0xE1, 0x87, 0xCD, 0x77, 0x62],
);

/// EAP Configuration Protocol GUID: `{e5b58dbb-7688-44b4-97bf-5f1d4b7cc8db}`.
pub const EFI_EAP_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE5B5_8DBB,
    0x7688,
    0x44B4,
    [0x97, 0xBF, 0x5F, 0x1D, 0x4B, 0x7C, 0xC8, 0xDB],
);

/// EAP Management Protocol GUID: `{bb62e663-625d-40b2-a088-bbe83623a245}`.
pub const EFI_EAP_MANAGEMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBB62_E663,
    0x625D,
    0x40B2,
    [0xA0, 0x88, 0xBB, 0xE8, 0x36, 0x23, 0xA2, 0x45],
);

/// EAP Management 2 Protocol GUID: `{5e93c847-456d-40b3-a6b4-78b0c9cf7f20}`.
pub const EFI_EAP_MANAGEMENT2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5E93_C847,
    0x456D,
    0x40B3,
    [0xA6, 0xB4, 0x78, 0xB0, 0xC9, 0xCF, 0x7F, 0x20],
);

/// Standard EAP method types (RFC 3748 / RFC 5216 / RFC 2716).
pub mod eap_type {
    /// EAP attribute method.
    pub const ATTRIBUTE: u8 = 0;
    /// Identity method.
    pub const IDENTITY: u8 = 1;
    /// Notification method.
    pub const NOTIFICATION: u8 = 2;
    /// Nak method.
    pub const NAK: u8 = 3;
    /// MD5-Challenge method.
    pub const MD5_CHALLENGE: u8 = 4;
    /// One-Time Password (OTP) method.
    pub const OTP: u8 = 5;
    /// Generic Token Card (GTC) method.
    pub const GTC: u8 = 6;
    /// EAP-TLS method (RFC 5216).
    pub const TLS: u8 = 13;
    /// EAP-SIM method.
    pub const SIM: u8 = 18;
    /// EAP-TTLS method.
    pub const TTLS: u8 = 21;
    /// PEAP method.
    pub const PEAP: u8 = 25;
    /// MS-CHAPv2 method.
    pub const MSCHAPV2: u8 = 26;
    /// EAP extension method.
    pub const EXTENSION: u8 = 33;
}

/// EAP configuration data types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EapConfigDataType {
    /// Authentication method.
    EapAuthMethod = 0,
    /// Supported authentication methods.
    EapSupportedAuthMethod = 1,
    /// Identity string.
    IdentityString = 2,
    /// EAP-TLS / PEAP CA certificate.
    EapTlsCaCert = 3,
    /// EAP-TLS client certificate.
    EapTlsClientCert = 4,
    /// EAP-TLS client private key file.
    EapTlsClientPrivateKeyFile = 5,
    /// EAP-TLS client private key password.
    EapTlsClientPrivateKeyFilePassword = 6,
    /// EAP-TLS cipher suite.
    EapTlsCipherSuite = 7,
    /// EAP-TLS supported cipher suites.
    EapTlsSupportedCipherSuite = 8,
    /// MS-CHAPv2 password.
    EapMsChapV2Password = 9,
    /// PEAP secondary authentication method.
    EapSecondAuthMethod = 10,
}

/// Port Access Entity (PAE) capability bitmasks.
pub mod pae_capabilities {
    /// PAE supports Authenticator role.
    pub const AUTHENTICATOR: u8 = 0x01;
    /// PAE supports Supplicant role.
    pub const SUPPLICANT: u8 = 0x02;
}

/// 802.1X Supplicant PAE state machine states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EapolSupplicantPaeState {
    /// Supplicant has logged off.
    Logoff = 0,
    /// Link disconnected.
    Disconnected = 1,
    /// Connecting and sending EAPOL-Start.
    Connecting = 2,
    /// Response acquired from Authenticator.
    Acquired = 3,
    /// Authenticating credentials.
    Authenticating = 4,
    /// Held after authentication failure.
    Held = 5,
    /// Successfully authenticated.
    Authenticated = 6,
}

/// Supplicant PAE configuration field valid masks.
pub mod supplicant_field_mask {
    /// `AuthPeriod` is valid.
    pub const AUTH_PERIOD: u8 = 0x01;
    /// `HeldPeriod` is valid.
    pub const HELD_PERIOD: u8 = 0x02;
    /// `StartPeriod` is valid.
    pub const START_PERIOD: u8 = 0x04;
    /// `MaxStart` is valid.
    pub const MAX_START: u8 = 0x08;
}

/// Information describing an EAPOL port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiEapolPortInfo {
    /// Port handle.
    pub port_number: *mut c_void,
    /// EAPOL protocol version.
    pub protocol_version: u8,
    /// PAE capability mask.
    pub pae_capabilities: u8,
}

/// Supplicant PAE timer and retry configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiEapolSupplicantPaeConfiguration {
    /// Valid field bitmask.
    pub valid_field_mask: u8,
    /// Initial value for authWhile timer in seconds (default 30s).
    pub auth_period: UIntN,
    /// Initial value for heldWhile timer in seconds (default 60s).
    pub held_period: UIntN,
    /// Initial value for startWhen timer in seconds (default 30s).
    pub start_period: UIntN,
    /// Maximum number of EAPOL-Start retries (default 3).
    pub max_start: UIntN,
}

/// Function pointer for building an EAP response packet.
pub type EfiEapBuildResponsePacket = unsafe extern "efiapi" fn(
    port_number: *mut c_void,
    request_buffer: *const u8,
    request_size: UIntN,
    buffer: *mut u8,
    buffer_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_EAP_SET_DESIRED_AUTHENTICATION_METHOD`.
pub type EfiEapSetDesiredAuthMethod =
    unsafe extern "efiapi" fn(this: *mut EfiEapProtocol, eap_auth_type: u8) -> EfiStatus;

/// Function pointer for `EFI_EAP_REGISTER_AUTHENTICATION_METHOD`.
pub type EfiEapRegisterAuthMethod = unsafe extern "efiapi" fn(
    this: *mut EfiEapProtocol,
    eap_auth_type: u8,
    handler: EfiEapBuildResponsePacket,
) -> EfiStatus;

/// Raw ABI structure for `EFI_EAP_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiEapProtocol {
    /// Sets desired authentication method for the port.
    pub set_desired_auth_method: EfiEapSetDesiredAuthMethod,
    /// Registers a custom EAP authentication method handler.
    pub register_auth_method: EfiEapRegisterAuthMethod,
}

/// Safe client wrapper around [`EfiEapProtocol`].
pub struct Eap<'a> {
    proto: &'a mut EfiEapProtocol,
}

impl<'a> Eap<'a> {
    /// Creates a new `Eap` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiEapProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiEapProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiEapProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiEapProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiEapProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Sets the desired EAP authentication method (e.g. `eap_type::TLS`).
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if type is invalid, or `UNSUPPORTED`.
    pub fn set_desired_auth_method(&mut self, auth_type: u8) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_desired_auth_method)(self.proto, auth_type) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers a custom EAP authentication packet builder.
    ///
    /// # Errors
    ///
    /// Returns an error status if registration fails.
    pub fn register_auth_method(
        &mut self,
        auth_type: u8,
        handler: EfiEapBuildResponsePacket,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.register_auth_method)(self.proto, auth_type, handler) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Function pointer for `EFI_EAP_CONFIGURATION_SET_DATA`.
pub type EfiEapConfigurationSetData = unsafe extern "efiapi" fn(
    this: *mut EfiEapConfigurationProtocol,
    eap_type: u8,
    data_type: EapConfigDataType,
    data: *const c_void,
    data_size: UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_EAP_CONFIGURATION_GET_DATA`.
pub type EfiEapConfigurationGetData = unsafe extern "efiapi" fn(
    this: *mut EfiEapConfigurationProtocol,
    eap_type: u8,
    data_type: EapConfigDataType,
    data: *mut c_void,
    data_size: *mut UIntN,
) -> EfiStatus;

/// Raw ABI structure for `EFI_EAP_CONFIGURATION_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiEapConfigurationProtocol {
    /// Sets EAP configuration data.
    pub set_data: EfiEapConfigurationSetData,
    /// Retrieves EAP configuration data.
    pub get_data: EfiEapConfigurationGetData,
}

/// Safe client wrapper around [`EfiEapConfigurationProtocol`].
pub struct EapConfiguration<'a> {
    proto: &'a mut EfiEapConfigurationProtocol,
}

impl<'a> EapConfiguration<'a> {
    /// Creates a new `EapConfiguration` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiEapConfigurationProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiEapConfigurationProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiEapConfigurationProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiEapConfigurationProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiEapConfigurationProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Sets configuration data for an EAP type.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting fails or unsupported.
    pub fn set_data(
        &mut self,
        eap_type: u8,
        data_type: EapConfigDataType,
        data: &[u8],
    ) -> Result<(), EfiStatus> {
        if data.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let status = unsafe {
            (self.proto.set_data)(
                self.proto,
                eap_type,
                data_type,
                data.as_ptr().cast::<c_void>(),
                data.len() as UIntN,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves configuration data for an EAP type into `buffer`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` is too small, or an error status.
    pub fn get_data(
        &mut self,
        eap_type: u8,
        data_type: EapConfigDataType,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            (self.proto.get_data)(
                self.proto,
                eap_type,
                data_type,
                buffer.as_mut_ptr().cast::<c_void>(),
                core::ptr::from_mut(&mut size),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(size as usize)
        } else {
            Err(status)
        }
    }
}

/// Function pointer for `EFI_EAP_GET_KEY`.
pub type EfiEapGetKey = unsafe extern "efiapi" fn(
    this: *mut EfiEapManagement2Protocol,
    msk: *mut u8,
    msk_size: *mut UIntN,
    emsk: *mut u8,
    emsk_size: *mut UIntN,
) -> EfiStatus;

/// Raw ABI structure for `EFI_EAP_MANAGEMENT2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiEapManagement2Protocol {
    /// Gets system configuration.
    pub get_system_configuration:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *mut c_void) -> EfiStatus,
    /// Sets system configuration.
    pub set_system_configuration:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *const c_void) -> EfiStatus,
    /// Initializes a port.
    pub initialize_port:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *mut c_void) -> EfiStatus,
    /// Authenticates user logon.
    pub user_logon:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *mut c_void) -> EfiStatus,
    /// Performs user logoff.
    pub user_logoff:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *mut c_void) -> EfiStatus,
    /// Queries current supplicant status.
    pub get_supplicant_status:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *mut c_void) -> EfiStatus,
    /// Sets supplicant configuration.
    pub set_supplicant_configuration:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *const c_void) -> EfiStatus,
    /// Queries supplicant statistics.
    pub get_supplicant_statistics:
        unsafe extern "efiapi" fn(*mut EfiEapManagement2Protocol, *mut c_void) -> EfiStatus,
    /// Returns keys (MSK and EMSK) generated during EAP session.
    pub get_key: EfiEapGetKey,
}

/// Safe client wrapper around [`EfiEapManagement2Protocol`].
pub struct EapManagement2<'a> {
    proto: &'a mut EfiEapManagement2Protocol,
}

impl<'a> EapManagement2<'a> {
    /// Creates a new `EapManagement2` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiEapManagement2Protocol) -> Self {
        Self { proto }
    }

    /// Returns keys (Master Session Key and Extended Master Session Key).
    ///
    /// # Errors
    ///
    /// Returns `NOT_READY` if keys are not yet derived, or an error status.
    pub fn get_key(
        &mut self,
        msk: &mut [u8],
        emsk: &mut [u8],
    ) -> Result<(usize, usize), EfiStatus> {
        let mut msk_len = msk.len() as UIntN;
        let mut emsk_len = emsk.len() as UIntN;
        let status = unsafe {
            (self.proto.get_key)(
                self.proto,
                msk.as_mut_ptr(),
                core::ptr::from_mut(&mut msk_len),
                emsk.as_mut_ptr(),
                core::ptr::from_mut(&mut emsk_len),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((msk_len as usize, emsk_len as usize))
        } else {
            Err(status)
        }
    }
}
