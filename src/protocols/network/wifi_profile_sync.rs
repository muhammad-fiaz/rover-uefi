//! # Wi-Fi Profile Sync Protocol
//!
//! Facilitates out-of-band and platform-provisioned Wi-Fi profile synchronization
//! for automated system recovery and remote console management flows over 802.11 wireless networks.

use crate::base::{EfiGuid, EfiStatus};

/// Wi-Fi Profile Sync Protocol GUID: `{399a2b8a-c267-44aa-9ab4-30588cd22dcc}`.
pub const WIFI_PROFILE_SYNC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x399A_2B8A,
    0xC267,
    0x44AA,
    [0x9A, 0xB4, 0x30, 0x58, 0x8C, 0xD2, 0x2D, 0xCC],
);

/// Wi-Fi Profile Sync Protocol revision 1.
pub const WIFI_PROFILE_SYNC_PROTOCOL_REVISION: u32 = 1;

/// Maximum SSID length supported in Wi-Fi profiles.
pub const WIFI_MAX_SSID_LEN: usize = 32;
/// Maximum passphrase/key length supported in Wi-Fi profiles.
pub const WIFI_MAX_KEY_LEN: usize = 64;

/// Wi-Fi network security suite profile.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WifiSecuritySuite {
    /// Open unencrypted network.
    Open = 0,
    /// WPA-Personal with Pre-Shared Key.
    WpaPsk = 1,
    /// WPA2-Personal with Pre-Shared Key.
    Wpa2Psk = 2,
    /// WPA3-Personal SAE (Simultaneous Authentication of Equals).
    Wpa3Sae = 3,
    /// WPA/WPA2-Enterprise with 802.1X EAP.
    WpaEnterprise = 4,
}

/// Wi-Fi network profile holding SSID, credential payload, and security modes.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WifiNetworkProfile {
    /// Length of the SSID string in bytes (0..32).
    pub ssid_len: u32,
    /// Raw SSID octets.
    pub ssid: [u8; WIFI_MAX_SSID_LEN],
    /// Security suite configured for this network.
    pub security_suite: WifiSecuritySuite,
    /// Length of the secret key/passphrase in bytes (0..64).
    pub key_len: u32,
    /// Pre-shared key or passphrase data.
    pub key: [u8; WIFI_MAX_KEY_LEN],
}

impl Default for WifiNetworkProfile {
    fn default() -> Self {
        Self {
            ssid_len: 0,
            ssid: [0u8; WIFI_MAX_SSID_LEN],
            security_suite: WifiSecuritySuite::Open,
            key_len: 0,
            key: [0u8; WIFI_MAX_KEY_LEN],
        }
    }
}

/// Function prototype for retrieving the platform-synchronized Wi-Fi profile.
pub type WifiProfileGetFn = unsafe extern "efiapi" fn(
    profile: *mut WifiNetworkProfile,
    mac_address: *mut [u8; 6],
) -> EfiStatus;

/// Function prototype for saving the Wi-Fi connection result state.
pub type WifiSetConnectStateFn = unsafe extern "efiapi" fn(connection_status: u32);

/// Function prototype for retrieving the last recorded Wi-Fi connection result.
pub type WifiGetConnectStateFn = unsafe extern "efiapi" fn() -> EfiStatus;

/// Raw Wi-Fi Profile Sync Protocol interface table.
#[repr(C)]
pub struct WifiProfileSyncProtocol {
    /// Protocol interface revision number.
    pub revision: u32,
    /// Updates the Wi-Fi connection state.
    pub set_connect_state: WifiSetConnectStateFn,
    /// Queries the last Wi-Fi connection state.
    pub get_connect_state: WifiGetConnectStateFn,
    /// Fetches the platform-provisioned profile and target MAC address.
    pub get_profile: WifiProfileGetFn,
}

/// Safe client wrapper around [`WifiProfileSyncProtocol`].
pub struct WifiProfileSync<'a> {
    raw: &'a mut WifiProfileSyncProtocol,
}

impl WifiProfileSync<'_> {
    /// Wraps a raw [`WifiProfileSyncProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut WifiProfileSyncProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Retrieves the protocol revision.
    #[must_use]
    pub fn revision(&self) -> u32 {
        self.raw.revision
    }

    /// Queries the synchronized Wi-Fi profile and associated adapter MAC address.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if no profile was provisioned, or `UNSUPPORTED` if disabled.
    pub fn get_profile(&mut self) -> Result<(WifiNetworkProfile, [u8; 6]), EfiStatus> {
        let mut profile = WifiNetworkProfile::default();
        let mut mac = [0u8; 6];
        let status = unsafe { (self.raw.get_profile)(&raw mut profile, &raw mut mac) };
        if status.is_success() {
            Ok((profile, mac))
        } else {
            Err(status)
        }
    }

    /// Records the outcome of a Wi-Fi connection attempt.
    pub fn set_connect_state(&mut self, state: u32) {
        unsafe {
            (self.raw.set_connect_state)(state);
        }
    }

    /// Retrieves the status of the last Wi-Fi connection attempt.
    #[must_use]
    pub fn get_connect_state(&self) -> EfiStatus {
        unsafe { (self.raw.get_connect_state)() }
    }
}
