//! Integration tests for Wi-Fi Profile Sync Protocol.
#![allow(clippy::expect_used)]

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::network::wifi_profile_sync::{
    WIFI_MAX_KEY_LEN, WIFI_MAX_SSID_LEN, WIFI_PROFILE_SYNC_PROTOCOL_GUID,
    WIFI_PROFILE_SYNC_PROTOCOL_REVISION, WifiNetworkProfile, WifiProfileSync,
    WifiProfileSyncProtocol, WifiSecuritySuite,
};

static mut SAVED_STATE: u32 = 0;

unsafe extern "efiapi" fn mock_set_connect_state(state: u32) {
    unsafe {
        SAVED_STATE = state;
    }
}

unsafe extern "efiapi" fn mock_get_connect_state() -> EfiStatus {
    if unsafe { SAVED_STATE } == 1 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_READY
    }
}

unsafe extern "efiapi" fn mock_get_profile(
    profile: *mut WifiNetworkProfile,
    mac_address: *mut [u8; 6],
) -> EfiStatus {
    if profile.is_null() || mac_address.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }

    unsafe {
        let p = &mut *profile;
        p.ssid_len = 8;
        p.ssid[..8].copy_from_slice(b"RoverNet");
        p.security_suite = WifiSecuritySuite::Wpa2Psk;
        p.key_len = 10;
        p.key[..10].copy_from_slice(b"SecurePass");

        *mac_address = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
    }

    EfiStatus::SUCCESS
}

#[test]
fn test_wifi_profile_sync_guid_and_constants() {
    assert_eq!(
        WIFI_PROFILE_SYNC_PROTOCOL_GUID.to_string(),
        "399a2b8a-c267-44aa-9ab4-30588cd22dcc"
    );
    assert_eq!(WIFI_PROFILE_SYNC_PROTOCOL_REVISION, 1);
    assert_eq!(WIFI_MAX_SSID_LEN, 32);
    assert_eq!(WIFI_MAX_KEY_LEN, 64);
}

#[test]
fn test_wifi_profile_sync_wrapper() {
    let mut raw_proto = WifiProfileSyncProtocol {
        revision: WIFI_PROFILE_SYNC_PROTOCOL_REVISION,
        set_connect_state: mock_set_connect_state,
        get_connect_state: mock_get_connect_state,
        get_profile: mock_get_profile,
    };

    let mut client =
        unsafe { WifiProfileSync::from_raw(&raw mut raw_proto) }.expect("valid wifi profile sync");

    assert_eq!(client.revision(), 1);

    let (profile, mac) = client.get_profile().expect("retrieved profile");
    assert_eq!(profile.ssid_len, 8);
    assert_eq!(&profile.ssid[..8], b"RoverNet");
    assert_eq!(profile.security_suite, WifiSecuritySuite::Wpa2Psk);
    assert_eq!(mac, [0x02, 0x11, 0x22, 0x33, 0x44, 0x55]);

    client.set_connect_state(1);
    assert_eq!(client.get_connect_state(), EfiStatus::SUCCESS);

    client.set_connect_state(0);
    assert_eq!(client.get_connect_state(), EfiStatus::NOT_READY);
}
