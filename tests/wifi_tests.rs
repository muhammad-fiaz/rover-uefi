//! Integration tests for Wireless MAC Connection (`WiFi` 1) protocol subsystem.

use rover_uefi::protocols::network::wifi::{
    EFI_WIFI_PROTOCOL_GUID, EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID, Efi80211AccNetType,
    Efi80211AssociateResultCode, Efi80211AuthenticationType, Efi80211DisassociateResultCode,
    Efi80211ElementHeader, Efi80211ElementSsid, Efi80211ReasonCode, Efi80211ScanResultCode,
};

#[test]
fn test_wifi_guids() {
    assert_eq!(
        EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID,
        EFI_WIFI_PROTOCOL_GUID
    );
    let bytes = EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID.to_bytes();
    assert_eq!(bytes[0], 0xC9);
    assert_eq!(bytes[1], 0x5B);
    assert_eq!(bytes[2], 0xA5);
    assert_eq!(bytes[3], 0x0D);
}

#[test]
fn test_80211_constants_and_enums() {
    assert_eq!(Efi80211AccNetType::Private as u32, 0);
    assert_eq!(Efi80211AccNetType::Wildcard as u32, 15);

    assert_eq!(Efi80211AssociateResultCode::Success as u32, 0);
    assert_eq!(
        Efi80211AssociateResultCode::RefusedReasonUnspecified as u32,
        1
    );

    assert_eq!(Efi80211ScanResultCode::Success as u32, 0);
    assert_eq!(Efi80211ScanResultCode::NotSupported as u32, 1);

    assert_eq!(Efi80211DisassociateResultCode::Success as u32, 0);
    assert_eq!(Efi80211DisassociateResultCode::InvalidParameters as u32, 1);

    assert_eq!(Efi80211AuthenticationType::OpenSystem as u32, 0);
    assert_eq!(Efi80211AuthenticationType::SharedKey as u32, 1);
    assert_eq!(Efi80211AuthenticationType::FastBssTransition as u32, 2);
    assert_eq!(Efi80211AuthenticationType::Sae as u32, 3);
}

#[test]
fn test_80211_reason_codes() {
    assert_eq!(Efi80211ReasonCode::UnspecifiedReason as u16, 1);
    assert_eq!(Efi80211ReasonCode::DeauthLeaving as u16, 3);
    assert_eq!(Efi80211ReasonCode::MicFailure as u16, 14);
    assert_eq!(Efi80211ReasonCode::FourWayHandshakeTimeout as u16, 15);
    assert_eq!(Efi80211ReasonCode::AuthFailed8021X as u16, 23);
}

#[test]
fn test_80211_element_header_and_ssid() {
    let mut elem_ssid = Efi80211ElementSsid::default();
    assert_eq!(elem_ssid.header.id, 0);
    assert_eq!(elem_ssid.header.length, 0);

    elem_ssid.header = Efi80211ElementHeader { id: 0, length: 11 };
    elem_ssid.ssid[..11].copy_from_slice(b"RoverAP_5G_");
    assert_eq!(&elem_ssid.ssid[..11], b"RoverAP_5G_");
}
