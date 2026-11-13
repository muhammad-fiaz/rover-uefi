//! UEFI Supplicant Protocol definition and safe wrapper.
//!
//! Defined in UEFI Specification 2.6+ Section 29.
//! The Supplicant protocol manages IEEE 802.11 wireless security, PSK/802.1X EAP
//! authentication, key derivation (PTK/GTK), and frame encryption/decryption.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::network::wifi2::{Efi80211MacAddress, Efi80211SuiteSelector};
use core::ffi::c_void;

/// EFI Supplicant Service Binding Protocol GUID.
pub const EFI_SUPPLICANT_SERVICE_BINDING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x45BC_D98E,
    0x59AD,
    0x4174,
    [0x95, 0x46, 0x34, 0x4A, 0x07, 0x48, 0x58, 0x98],
);

/// EFI Supplicant Protocol GUID.
pub const EFI_SUPPLICANT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x54FC_C43E,
    0xAA89,
    0x4333,
    [0x9A, 0x85, 0xCD, 0xEA, 0x24, 0x05, 0x1E, 0x9E],
);

/// Supplicant encryption / decryption operation mode.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiSupplicantCryptMode {
    /// Encrypt data provided in fragment buffers.
    Encrypt = 0,
    /// Decrypt data provided in fragment buffers.
    Decrypt = 1,
}

/// Supplicant configuration and session data types.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiSupplicantDataType {
    /// Current authentication type (`AKM` suite).
    AkmSuite = 0,
    /// Group data encryption cipher suite.
    GroupDataCipherSuite = 1,
    /// Pairwise encryption cipher suite.
    PairwiseCipherSuite = 2,
    /// Pre-Shared Key (`PSK`) password string.
    PskPassword = 3,
    /// Target `SSID` name.
    TargetSsidName = 4,
    /// Station MAC address.
    StationMac = 5,
    /// Target `SSID` MAC address.
    TargetSsidMac = 6,
    /// Pairwise Transient Key (`PTK`).
    Ptk = 7,
    /// Group Temporal Key (`GTK`).
    Gtk = 8,
    /// Port Access Entity (`PAE`) state.
    State = 9,
    /// 802.11 link state.
    LinkState = 10,
    /// Key refresh flag.
    KeyRefresh = 11,
    /// Supported authentication types bitmask.
    SupportedAkmSuites = 12,
    /// Supported software encryption types.
    SupportedSoftwareCipherSuites = 13,
    /// Supported hardware encryption types.
    SupportedHardwareCipherSuites = 14,
    /// Integrity Group Temporal Key (`IGTK`).
    Igtk = 15,
    /// Pairwise Master Key (`PMK`).
    Pmk = 16,
    /// Maximum data type value.
    DataTypeMaximum = 17,
}

/// Supplicant key type classification.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiSupplicantKeyType {
    /// Group key.
    Group = 0,
    /// Pairwise key.
    Pairwise = 1,
    /// Peer key.
    PeerKey = 2,
    /// Integrity Group Temporal Key.
    Igtk = 3,
}

/// Supplicant key installation direction.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiSupplicantKeyDirection {
    /// Receive direction only.
    Receive = 0,
    /// Transmit direction only.
    Transmit = 1,
    /// Both receive and transmit directions.
    Both = 2,
}

/// Maximum supported key length in bytes.
pub const EFI_MAX_KEY_LEN: usize = 64;

/// Key refresh indication data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiSupplicantKeyRefresh {
    /// True if GTK was refreshed after building response packet.
    pub gtk_refresh: bool,
}

/// Supplicant cryptographic key descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiSupplicantKey {
    /// Raw key bytes.
    pub key: [u8; EFI_MAX_KEY_LEN],
    /// Length of the key in bytes.
    pub key_len: u8,
    /// Key identifier index.
    pub key_id: u8,
    /// Key classification type.
    pub key_type: EfiSupplicantKeyType,
    /// Target hardware address.
    pub addr: Efi80211MacAddress,
    /// Receive Sequence Count (`RSC`).
    pub rsc: [u8; 8],
    /// Length in bytes of `RSC`.
    pub rsc_len: u8,
    /// True if configured by authenticator.
    pub is_authenticator: bool,
    /// Cipher suite required for association.
    pub cipher_suite: Efi80211SuiteSelector,
    /// Direction for which key applies.
    pub direction: EfiSupplicantKeyDirection,
}

/// Data fragment buffer descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiSupplicantFragmentData {
    /// Length of data in bytes.
    pub fragment_length: u32,
    /// Pointer to the fragment data buffer.
    pub fragment_buffer: *mut c_void,
}

/// Function prototype for building EAPOL response packets.
pub type EfiSupplicantBuildResponsePacket = unsafe extern "efiapi" fn(
    this: *const EfiSupplicantProtocol,
    request_buffer: *const u8,
    request_buffer_size: UIntN,
    buffer: *mut u8,
    buffer_size: *mut UIntN,
) -> EfiStatus;

/// Function prototype for encrypting/decrypting payload fragments.
pub type EfiSupplicantProcessPacket = unsafe extern "efiapi" fn(
    this: *const EfiSupplicantProtocol,
    fragment_table: *mut *mut EfiSupplicantFragmentData,
    fragment_count: *mut u32,
    crypt_mode: EfiSupplicantCryptMode,
) -> EfiStatus;

/// Function prototype for setting Supplicant configuration data.
pub type EfiSupplicantSetData = unsafe extern "efiapi" fn(
    this: *const EfiSupplicantProtocol,
    data_type: EfiSupplicantDataType,
    data: *const c_void,
    data_size: UIntN,
) -> EfiStatus;

/// Function prototype for querying Supplicant configuration data.
pub type EfiSupplicantGetData = unsafe extern "efiapi" fn(
    this: *const EfiSupplicantProtocol,
    data_type: EfiSupplicantDataType,
    data: *mut u8,
    data_size: *mut UIntN,
) -> EfiStatus;

/// UEFI Supplicant Protocol structure.
#[repr(C)]
pub struct EfiSupplicantProtocol {
    /// Builds an EAPOL response packet.
    pub build_response_packet: EfiSupplicantBuildResponsePacket,
    /// Encrypts or decrypts packet fragments.
    pub process_packet: EfiSupplicantProcessPacket,
    /// Configures supplicant parameters.
    pub set_data: EfiSupplicantSetData,
    /// Queries supplicant parameters.
    pub get_data: EfiSupplicantGetData,
}

/// Safe idiomatic wrapper around `EfiSupplicantProtocol`.
pub struct Supplicant {
    proto: *const EfiSupplicantProtocol,
}

impl Supplicant {
    /// Creates a new `Supplicant` client wrapper.
    #[must_use]
    pub const fn new(proto: *const EfiSupplicantProtocol) -> Self {
        Self { proto }
    }

    /// Returns the raw protocol pointer.
    #[must_use]
    pub const fn raw(&self) -> *const EfiSupplicantProtocol {
        self.proto
    }

    /// Builds an EAPOL response packet into `response_buf`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error status if packet construction fails or buffer is too small.
    pub fn build_response_packet(
        &self,
        request: Option<&[u8]>,
        response_buf: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let (req_ptr, req_len) = match request {
            Some(slice) => (slice.as_ptr(), slice.len() as UIntN),
            None => (core::ptr::null(), 0),
        };
        let mut buf_size = response_buf.len() as UIntN;

        let status = unsafe {
            ((*self.proto).build_response_packet)(
                self.proto,
                req_ptr,
                req_len,
                response_buf.as_mut_ptr(),
                core::ptr::from_mut(&mut buf_size),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(buf_size as usize)
        } else {
            Err(status)
        }
    }

    /// Configures Supplicant session parameters.
    ///
    /// # Errors
    ///
    /// Returns an error status if parameter configuration fails.
    pub fn set_data(&self, data_type: EfiSupplicantDataType, data: &[u8]) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).set_data)(
                self.proto,
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

    /// Queries Supplicant session parameters into `buffer`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error status if query fails or buffer is too small.
    pub fn get_data(
        &self,
        data_type: EfiSupplicantDataType,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            ((*self.proto).get_data)(
                self.proto,
                data_type,
                buffer.as_mut_ptr(),
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
