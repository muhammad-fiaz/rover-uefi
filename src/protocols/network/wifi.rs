//! # Wireless MAC Connection (`WiFi`) Protocol
//!
//! Defined in UEFI Specification 2.5 Section 28 and IEEE 802.11 specifications.
//!
//! Provides management service interfaces for the EFI wireless network stack
//! to establish wireless connectivity with an Access Point (AP). Includes
//! scanning, association, disassociation, authentication, and deauthentication.

use crate::abi::Bool;
use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::network::wifi2::{Efi80211BssType, Efi80211MacAddress};
use core::ffi::c_void;

/// EFI Wireless MAC Connection Protocol GUID (UEFI 2.5).
pub const EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0DA5_5BC9,
    0x45F8,
    0x4BB4,
    [0x87, 0x19, 0x52, 0x24, 0xF1, 0x8A, 0x4D, 0x45],
);

/// Alias for `EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID`.
pub const EFI_WIFI_PROTOCOL_GUID: EfiGuid = EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID;

/// IEEE 802.11 Access Network Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211AccNetType {
    /// Private network.
    Private = 0,
    /// Private network with guest access.
    PrivateWithGuest = 1,
    /// Chargeable public network.
    ChargeablePublic = 2,
    /// Free public network.
    FreePublic = 3,
    /// Personal device network.
    Personal = 4,
    /// Emergency services only network.
    EmergencyServicesOnly = 5,
    /// Test or experimental network.
    TestOrExp = 14,
    /// Wildcard matching all access network types.
    Wildcard = 15,
}

/// Result code for an association attempt.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211AssociateResultCode {
    /// Association succeeded.
    Success = 0,
    /// Association refused for an unspecified reason.
    RefusedReasonUnspecified = 1,
    /// Association refused due to capability mismatch.
    RefusedCapsMismatch = 2,
    /// Association refused due to external reason.
    RefusedExtReason = 3,
    /// Association refused because AP is out of memory.
    RefusedApOutOfMemory = 4,
    /// Association refused due to basic data rate mismatch.
    RefusedBasicRatesMismatch = 5,
    /// Association rejected because emergency services are not supported.
    RejectedEmergencyServicesNotSupported = 6,
    /// Association refused temporarily.
    RefusedTemporarily = 7,
}

/// Result code for a wireless network scan operation.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211ScanResultCode {
    /// Scan completed successfully.
    Success = 0,
    /// Scan operation is not supported by the implementation.
    NotSupported = 1,
}

/// Result code for a disassociation request.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211DisassociateResultCode {
    /// Disassociation completed successfully.
    Success = 0,
    /// Disassociation failed due to invalid parameters.
    InvalidParameters = 1,
}

/// IEEE 802.11 Deauthentication and Disassociation Reason Codes.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211ReasonCode {
    /// Unspecified reason.
    UnspecifiedReason = 1,
    /// Previous authentication is no longer valid.
    PreviousAuthInvalid = 2,
    /// Deauthenticated because sending station is leaving BSS.
    DeauthLeaving = 3,
    /// Disassociated due to inactivity.
    DisassociatedDueToInactivity = 4,
    /// Disassociated because AP is unable to handle additional stations.
    DisassociatedApUnable = 5,
    /// Class 2 frame received from nonauthenticated station.
    Class2FromNonAuth = 6,
    /// Class 3 frame received from nonassociated station.
    Class3FromNonAssoc = 7,
    /// Disassociated because sending station is leaving BSS.
    DisassocLeaving = 8,
    /// Station requesting association is not authenticated with responding station.
    StationNotAuth = 9,
    /// Disassociated because power capability element is unacceptable.
    DisassocBadPower = 10,
    /// Disassociated because supported channels element is unacceptable.
    DisassocBadChannels = 11,
    /// Invalid information element.
    InvalidElement = 13,
    /// Message integrity check (MIC) failure.
    MicFailure = 14,
    /// 4-Way Handshake timeout.
    FourWayHandshakeTimeout = 15,
    /// Group Key Handshake timeout.
    GroupKeyHandshakeTimeout = 16,
    /// Information element in 4-Way Handshake differs from (Re)Association Request.
    HandshakeElementMismatch = 17,
    /// Invalid group cipher.
    InvalidGroupCipher = 18,
    /// Invalid pairwise cipher.
    InvalidPairwiseCipher = 19,
    /// Invalid AKMP.
    InvalidAkmp = 20,
    /// Unsupported RSNE version.
    UnsupportedRsneVersion = 21,
    /// Invalid RSNE capabilities.
    InvalidRsneCapabilities = 22,
    /// IEEE 802.1X authentication failed.
    AuthFailed8021X = 23,
    /// Cipher suite rejected because of the security policy.
    CipherRejectedByPolicy = 24,
}

/// IEEE 802.11 Authentication Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211AuthenticationType {
    /// Open System authentication.
    OpenSystem = 0,
    /// Shared Key authentication (WEP).
    SharedKey = 1,
    /// Fast BSS Transition (FT).
    FastBssTransition = 2,
    /// Simultaneous Authentication of Equals (SAE / WPA3-Personal).
    Sae = 3,
}

/// Common header for an IEEE 802.11 Information Element.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Efi80211ElementHeader {
    /// Element ID.
    pub id: u8,
    /// Length of the element body in bytes.
    pub length: u8,
}

/// IEEE 802.11 SSID Information Element.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Efi80211ElementSsid {
    /// Element header.
    pub header: Efi80211ElementHeader,
    /// Raw SSID octets (up to 32 bytes).
    pub ssid: [u8; 32],
}

impl Default for Efi80211ElementSsid {
    fn default() -> Self {
        Self {
            header: Efi80211ElementHeader { id: 0, length: 0 },
            ssid: [0u8; 32],
        }
    }
}

/// Parameters for initiating a wireless BSS scan.
#[repr(C)]
pub struct Efi80211ScanData {
    /// Type of BSS to scan for (infrastructure, independent, mesh, any).
    pub bss_type: Efi80211BssType,
    /// Target BSSID address (or all 1s for wildcard).
    pub bssid: Efi80211MacAddress,
    /// Length of target SSID in bytes.
    pub ssid_len: u8,
    /// Pointer to target SSID octets (or NULL for wildcard).
    pub ssid: *const u8,
    /// Whether to perform passive scanning.
    pub passive_mode: Bool,
    /// Probe delay in microseconds for active scanning.
    pub probe_delay: u32,
    /// Pointer to array of channels to examine (NULL scans all channels).
    pub channel_list: *const u32,
    /// Minimum time in Time Units (TU) to spend on each channel.
    pub min_channel_time: u32,
    /// Maximum time in Time Units (TU) to spend on each channel.
    pub max_channel_time: u32,
    /// Optional pointer to requested information elements.
    pub request_information: *const c_void,
    /// Optional list of SSID elements.
    pub ssid_list: *const Efi80211ElementSsid,
    /// Target access network type.
    pub access_network_type: Efi80211AccNetType,
    /// Optional vendor-specific information elements.
    pub vendor_specific_info: *const u8,
}

/// Asynchronous completion token for wireless scan.
#[repr(C)]
pub struct Efi80211ScanDataToken {
    /// Event triggered upon scan completion.
    pub event: EfiEvent,
    /// Result status of the scan operation.
    pub status: EfiStatus,
    /// Pointer to input scan parameters.
    pub data: *mut Efi80211ScanData,
    /// Scan operation result code.
    pub result_code: Efi80211ScanResultCode,
    /// Number of discovered BSS descriptions.
    pub bss_description_count: UIntN,
    /// Pointer to array of discovered BSS description pointers.
    pub bss_descriptions: *mut *mut c_void,
}

/// Parameters for associating with an Access Point.
#[repr(C)]
pub struct Efi80211AssociateData {
    /// MAC address of the target AP.
    pub bssid: Efi80211MacAddress,
    /// Capability information bitmask.
    pub capabilities: u16,
    /// Listen interval in beacon intervals.
    pub listen_interval: u16,
    /// Pointer to optional vendor-specific elements.
    pub vendor_specific_info: *const u8,
}

/// Asynchronous completion token for association.
#[repr(C)]
pub struct Efi80211AssociateDataToken {
    /// Event triggered upon association completion.
    pub event: EfiEvent,
    /// Result status of the association operation.
    pub status: EfiStatus,
    /// Pointer to association parameters.
    pub data: *mut Efi80211AssociateData,
    /// Association attempt result code.
    pub result_code: Efi80211AssociateResultCode,
}

/// Parameters for terminating an association.
#[repr(C)]
pub struct Efi80211DisassociateData {
    /// MAC address of the AP to disassociate from.
    pub bssid: Efi80211MacAddress,
    /// Reason code for disassociation.
    pub reason_code: u16,
    /// Optional vendor-specific elements.
    pub vendor_specific_info: *const u8,
}

/// Asynchronous completion token for disassociation.
#[repr(C)]
pub struct Efi80211DisassociateDataToken {
    /// Event triggered upon disassociation completion.
    pub event: EfiEvent,
    /// Result status of the disassociation.
    pub status: EfiStatus,
    /// Pointer to disassociation parameters.
    pub data: *mut Efi80211DisassociateData,
}

/// Parameters for authenticating with an Access Point.
#[repr(C)]
pub struct Efi80211AuthenticateData {
    /// MAC address of the target peer.
    pub bssid: Efi80211MacAddress,
    /// Authentication algorithm type.
    pub auth_type: Efi80211AuthenticationType,
    /// Optional challenge/response credentials or SAE commit data.
    pub auth_data: *const c_void,
    /// Length of authentication data in bytes.
    pub auth_data_len: UIntN,
}

/// Asynchronous completion token for authentication.
#[repr(C)]
pub struct Efi80211AuthenticateDataToken {
    /// Event triggered upon authentication completion.
    pub event: EfiEvent,
    /// Result status of the authentication.
    pub status: EfiStatus,
    /// Pointer to authentication parameters.
    pub data: *mut Efi80211AuthenticateData,
}

/// Parameters for deauthenticating from a peer.
#[repr(C)]
pub struct Efi80211DeauthenticateData {
    /// MAC address of the target peer.
    pub bssid: Efi80211MacAddress,
    /// Reason code for deauthentication.
    pub reason_code: u16,
    /// Optional vendor-specific elements.
    pub vendor_specific_info: *const u8,
}

/// Asynchronous completion token for deauthentication.
#[repr(C)]
pub struct Efi80211DeauthenticateDataToken {
    /// Event triggered upon deauthentication completion.
    pub event: EfiEvent,
    /// Result status of the deauthentication.
    pub status: EfiStatus,
    /// Pointer to deauthentication parameters.
    pub data: *mut Efi80211DeauthenticateData,
}

/// Function pointer for `EFI_WIRELESS_MAC_CONNECTION_SCAN`.
pub type EfiWirelessMacConnectionScan = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionProtocol,
    data: *mut Efi80211ScanDataToken,
) -> EfiStatus;

/// Function pointer for `EFI_WIRELESS_MAC_CONNECTION_ASSOCIATE`.
pub type EfiWirelessMacConnectionAssociate = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionProtocol,
    data: *mut Efi80211AssociateDataToken,
) -> EfiStatus;

/// Function pointer for `EFI_WIRELESS_MAC_CONNECTION_DISASSOCIATE`.
pub type EfiWirelessMacConnectionDisassociate = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionProtocol,
    data: *mut Efi80211DisassociateDataToken,
) -> EfiStatus;

/// Function pointer for `EFI_WIRELESS_MAC_CONNECTION_AUTHENTICATE`.
pub type EfiWirelessMacConnectionAuthenticate = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionProtocol,
    data: *mut Efi80211AuthenticateDataToken,
) -> EfiStatus;

/// Function pointer for `EFI_WIRELESS_MAC_CONNECTION_DEAUTHENTICATE`.
pub type EfiWirelessMacConnectionDeauthenticate = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionProtocol,
    data: *mut Efi80211DeauthenticateDataToken,
) -> EfiStatus;

/// Raw ABI structure for `EFI_WIRELESS_MAC_CONNECTION_PROTOCOL`.
#[repr(C)]
pub struct EfiWirelessMacConnectionProtocol {
    /// Requests a survey of available BSSs.
    pub scan: EfiWirelessMacConnectionScan,
    /// Requests association with an AP.
    pub associate: EfiWirelessMacConnectionAssociate,
    /// Requests termination of an existing association.
    pub disassociate: EfiWirelessMacConnectionDisassociate,
    /// Requests authentication with a peer MAC entity.
    pub authenticate: EfiWirelessMacConnectionAuthenticate,
    /// Invalidates authentication relationship with a peer MAC entity.
    pub deauthenticate: EfiWirelessMacConnectionDeauthenticate,
}

/// Safe client wrapper around [`EfiWirelessMacConnectionProtocol`].
pub struct WirelessMacConnection<'a> {
    proto: &'a mut EfiWirelessMacConnectionProtocol,
}

impl<'a> WirelessMacConnection<'a> {
    /// Creates a new `WirelessMacConnection` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiWirelessMacConnectionProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiWirelessMacConnectionProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Initiates an asynchronous scan for available wireless networks.
    ///
    /// # Safety
    ///
    /// `token` must reference a valid, properly initialized `Efi80211ScanDataToken`.
    ///
    /// # Errors
    ///
    /// Returns an error status if initiating the scan fails.
    pub unsafe fn scan(&mut self, token: *mut Efi80211ScanDataToken) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.scan)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initiates an association with an Access Point.
    ///
    /// # Safety
    ///
    /// `token` must reference a valid, properly initialized `Efi80211AssociateDataToken`.
    ///
    /// # Errors
    ///
    /// Returns an error status if initiating the association fails.
    pub unsafe fn associate(
        &mut self,
        token: *mut Efi80211AssociateDataToken,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.associate)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initiates disassociation from an Access Point.
    ///
    /// # Safety
    ///
    /// `token` must reference a valid, properly initialized `Efi80211DisassociateDataToken`.
    ///
    /// # Errors
    ///
    /// Returns an error status if initiating disassociation fails.
    pub unsafe fn disassociate(
        &mut self,
        token: *mut Efi80211DisassociateDataToken,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.disassociate)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initiates authentication with a peer MAC entity.
    ///
    /// # Safety
    ///
    /// `token` must reference a valid, properly initialized `Efi80211AuthenticateDataToken`.
    ///
    /// # Errors
    ///
    /// Returns an error status if initiating authentication fails.
    pub unsafe fn authenticate(
        &mut self,
        token: *mut Efi80211AuthenticateDataToken,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.authenticate)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Requests deauthentication from a peer MAC entity.
    ///
    /// # Safety
    ///
    /// `token` must reference a valid, properly initialized `Efi80211DeauthenticateDataToken`.
    ///
    /// # Errors
    ///
    /// Returns an error status if initiating deauthentication fails.
    pub unsafe fn deauthenticate(
        &mut self,
        token: *mut Efi80211DeauthenticateDataToken,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.deauthenticate)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
