//! # Wireless MAC Connection II (`WiFi` 2) Protocol
//!
//! Defined in UEFI Specification 2.6+ Section 29.
//!
//! Provides 802.11 wireless network survey, connection establishment,
//! security configuration (`WPA2`/`WPA3`), and disconnection services.

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};

/// EFI Wireless MAC Connection II Protocol GUID: `{1b0fb9bf-699d-4fdd-a7c3-2546681bf63b}`.
pub const EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1B0F_B9BF,
    0x699D,
    0x4FDD,
    [0xA7, 0xC3, 0x25, 0x46, 0x68, 0x1B, 0xF6, 0x3B],
);

/// Alias for `EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID`.
pub const EFI_WIFI2_PROTOCOL_GUID: EfiGuid = EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID;

/// Maximum length of an 802.11 `SSID` in bytes.
pub const EFI_MAX_SSID_LEN: usize = 32;

/// IEEE 802.11 Basic Service Set (`BSS`) Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211BssType {
    /// Infrastructure `BSS` (access point managed).
    Infrastructure = 0,
    /// Independent `BSS` (ad-hoc peer-to-peer).
    Independent = 1,
    /// Mesh `BSS`.
    Mesh = 2,
    /// Any supported `BSS` type.
    Any = 3,
}

/// Result code for wireless network connection attempt.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efi80211ConnectNetworkResultCode {
    /// Connection succeeded.
    Success = 0,
    /// Connection was refused by the network.
    Refused = 1,
    /// Network was not detected or failed to respond.
    Failed = 2,
    /// Connection attempt timed out.
    Timeout = 3,
    /// Connection failed due to unspecified reason.
    FailedReasonUnspecified = 4,
}

/// IEEE 802.11 48-bit MAC address.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Efi80211MacAddress {
    /// 6-byte hardware address.
    pub addr: [u8; 6],
}

/// IEEE 802.11 Service Set Identifier (`SSID`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Efi80211Ssid {
    /// Length of `SSID` in bytes (0 to 32).
    pub ssid_len: u8,
    /// Raw `SSID` byte octets.
    pub ssid: [u8; EFI_MAX_SSID_LEN],
}

impl Default for Efi80211Ssid {
    fn default() -> Self {
        Self {
            ssid_len: 0,
            ssid: [0u8; EFI_MAX_SSID_LEN],
        }
    }
}

/// IEEE 802.11 Cipher / `AKM` Suite Selector.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Efi80211SuiteSelector {
    /// Organization Unique Identifier (typically 00-0F-AC for IEEE 802.11).
    pub oui: [u8; 3],
    /// Suite type identifier.
    pub suite_type: u8,
}

/// Asynchronous token for discovering wireless networks.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Efi80211GetNetworksToken {
    /// Event signaled upon operation completion.
    pub event: EfiEvent,
    /// Result status of the scan.
    pub status: EfiStatus,
    /// Optional caller data.
    pub data: *mut (),
    /// Number of discovered network descriptions.
    pub network_count: UIntN,
    /// Buffer of discovered network descriptions.
    pub network_desc: *mut (),
}

/// Asynchronous token for connecting to a wireless network.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Efi80211ConnectNetworkToken {
    /// Event signaled upon connection completion.
    pub event: EfiEvent,
    /// Result status.
    pub status: EfiStatus,
    /// Optional caller data.
    pub data: *mut (),
    /// Result code of the connection attempt.
    pub result_code: Efi80211ConnectNetworkResultCode,
}

/// Asynchronous token for disconnecting from a wireless network.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Efi80211DisconnectNetworkToken {
    /// Event signaled upon disconnection.
    pub event: EfiEvent,
    /// Result status.
    pub status: EfiStatus,
}

/// Requests a survey/scan of potential wireless networks.
pub type EfiWirelessMacConnectionIiGetNetworks = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionIiProtocol,
    token: *mut Efi80211GetNetworksToken,
) -> EfiStatus;

/// Connects to a wireless network specified in the connection token.
pub type EfiWirelessMacConnectionIiConnectNetwork = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionIiProtocol,
    token: *mut Efi80211ConnectNetworkToken,
) -> EfiStatus;

/// Disconnects from the current wireless network.
pub type EfiWirelessMacConnectionIiDisconnectNetwork = unsafe extern "efiapi" fn(
    this: *mut EfiWirelessMacConnectionIiProtocol,
    token: *mut Efi80211DisconnectNetworkToken,
) -> EfiStatus;

/// EFI Wireless MAC Connection II Protocol (`EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiWirelessMacConnectionIiProtocol {
    /// Surveys available 802.11 wireless networks.
    pub get_networks: EfiWirelessMacConnectionIiGetNetworks,
    /// Connects to a target wireless network.
    pub connect_network: EfiWirelessMacConnectionIiConnectNetwork,
    /// Disconnects from the connected network.
    pub disconnect_network: EfiWirelessMacConnectionIiDisconnectNetwork,
}
