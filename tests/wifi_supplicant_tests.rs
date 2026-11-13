//! Integration tests for `WiFi` 2 and Supplicant protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiEvent, EfiGuid, EfiStatus};
use rover_uefi::protocols::network::{
    EFI_MAX_KEY_LEN, EFI_MAX_SSID_LEN, EFI_SUPPLICANT_PROTOCOL_GUID,
    EFI_SUPPLICANT_SERVICE_BINDING_PROTOCOL_GUID, EFI_WIFI2_PROTOCOL_GUID,
    EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID, EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID,
    Efi80211BssType, Efi80211ConnectNetworkData, Efi80211ConnectNetworkResultCode,
    Efi80211ConnectNetworkToken, Efi80211DisconnectNetworkToken, Efi80211GetNetworksData,
    Efi80211GetNetworksResult, Efi80211GetNetworksToken, Efi80211MacAddress, Efi80211Network,
    Efi80211NetworkDescription, Efi80211Ssid, Efi80211SuiteSelector, EfiSupplicantCryptMode,
    EfiSupplicantDataType, EfiSupplicantFragmentData, EfiSupplicantKey, EfiSupplicantKeyDirection,
    EfiSupplicantKeyRefresh, EfiSupplicantKeyType, EfiSupplicantProtocol,
    EfiWirelessMacConnectionIiProtocol, Supplicant, Wifi2,
};

// --- Mock WiFi2 implementation ---

unsafe extern "efiapi" fn mock_wifi_get_networks(
    _this: *const EfiWirelessMacConnectionIiProtocol,
    token: *mut Efi80211GetNetworksToken,
) -> EfiStatus {
    if token.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*token).status = EfiStatus::SUCCESS;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_wifi_connect_network(
    _this: *const EfiWirelessMacConnectionIiProtocol,
    token: *mut Efi80211ConnectNetworkToken,
) -> EfiStatus {
    if token.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*token).status = EfiStatus::SUCCESS;
        (*token).result_code = Efi80211ConnectNetworkResultCode::Success;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_wifi_disconnect_network(
    _this: *const EfiWirelessMacConnectionIiProtocol,
    token: *mut Efi80211DisconnectNetworkToken,
) -> EfiStatus {
    if token.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*token).status = EfiStatus::SUCCESS;
    }
    EfiStatus::SUCCESS
}

// --- Mock Supplicant implementation ---

unsafe extern "efiapi" fn mock_supplicant_build_response(
    _this: *const EfiSupplicantProtocol,
    _req: *const u8,
    _req_size: UIntN,
    buffer: *mut u8,
    buffer_size: *mut UIntN,
) -> EfiStatus {
    if buffer.is_null() || buffer_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if *buffer_size >= 4 {
            *buffer_size = 4;
            let slice = core::slice::from_raw_parts_mut(buffer, 4);
            slice.copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
            EfiStatus::SUCCESS
        } else {
            *buffer_size = 4;
            EfiStatus::BUFFER_TOO_SMALL
        }
    }
}

unsafe extern "efiapi" fn mock_supplicant_process_packet(
    _this: *const EfiSupplicantProtocol,
    _fragment_table: *mut *mut EfiSupplicantFragmentData,
    _fragment_count: *mut u32,
    _crypt_mode: EfiSupplicantCryptMode,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

static mut MOCK_SUPPLICANT_PSK: [u8; 16] = [0u8; 16];

#[allow(clippy::unnecessary_cast)]
unsafe extern "efiapi" fn mock_supplicant_set_data(
    _this: *const EfiSupplicantProtocol,
    data_type: EfiSupplicantDataType,
    data: *const c_void,
    data_size: UIntN,
) -> EfiStatus {
    if data.is_null() || data_size == 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if data_type == EfiSupplicantDataType::PskPassword && data_size <= 16 {
            let src = core::slice::from_raw_parts(data.cast::<u8>(), data_size as usize);
            let dst = core::slice::from_raw_parts_mut(
                core::ptr::addr_of_mut!(MOCK_SUPPLICANT_PSK).cast::<u8>(),
                data_size as usize,
            );
            dst.copy_from_slice(src);
            EfiStatus::SUCCESS
        } else {
            EfiStatus::UNSUPPORTED
        }
    }
}

unsafe extern "efiapi" fn mock_supplicant_get_data(
    _this: *const EfiSupplicantProtocol,
    data_type: EfiSupplicantDataType,
    data: *mut u8,
    data_size: *mut UIntN,
) -> EfiStatus {
    if data.is_null() || data_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if data_type == EfiSupplicantDataType::PskPassword {
            if *data_size >= 16 {
                let dst = core::slice::from_raw_parts_mut(data, 16);
                let src = core::slice::from_raw_parts(
                    core::ptr::addr_of!(MOCK_SUPPLICANT_PSK).cast::<u8>(),
                    16,
                );
                dst.copy_from_slice(src);
                *data_size = 16;
                EfiStatus::SUCCESS
            } else {
                *data_size = 16;
                EfiStatus::BUFFER_TOO_SMALL
            }
        } else {
            EfiStatus::UNSUPPORTED
        }
    }
}

#[test]
fn test_wifi_guids_and_constants() {
    assert_eq!(
        EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID,
        EfiGuid::new(
            0x1B0F_B9BF,
            0x699D,
            0x4FDD,
            [0xA7, 0xC3, 0x25, 0x46, 0x68, 0x1B, 0xF6, 0x3B],
        )
    );
    assert_eq!(
        EFI_WIFI2_PROTOCOL_GUID,
        EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID
    );
    assert_eq!(
        EFI_WIRELESS_MAC_CONNECTION_PROTOCOL_GUID,
        EfiGuid::new(
            0x0DA5_5BC9,
            0x45F8,
            0x4BB4,
            [0x87, 0x19, 0x52, 0x24, 0xF1, 0x8A, 0x4D, 0x45],
        )
    );
    assert_eq!(EFI_MAX_SSID_LEN, 32);

    assert_eq!(Efi80211BssType::Infrastructure as u32, 0);
    assert_eq!(Efi80211BssType::Independent as u32, 1);
    assert_eq!(Efi80211BssType::Mesh as u32, 2);
    assert_eq!(Efi80211BssType::Any as u32, 3);

    assert_eq!(Efi80211ConnectNetworkResultCode::Success as u32, 0);
    assert_eq!(Efi80211ConnectNetworkResultCode::Refused as u32, 1);
    assert_eq!(Efi80211ConnectNetworkResultCode::Failed as u32, 2);
    assert_eq!(Efi80211ConnectNetworkResultCode::Timeout as u32, 3);
    assert_eq!(
        Efi80211ConnectNetworkResultCode::FailedReasonUnspecified as u32,
        4
    );

    let mac = Efi80211MacAddress {
        addr: [1, 2, 3, 4, 5, 6],
    };
    assert_eq!(mac.addr[0], 1);

    let ssid = Efi80211Ssid::default();
    assert_eq!(ssid.ssid_len, 0);

    let suite = Efi80211SuiteSelector {
        oui: [0x00, 0x0F, 0xAC],
        suite_type: 4,
    };
    assert_eq!(suite.suite_type, 4);

    let net = Efi80211Network {
        bss_type: Efi80211BssType::Infrastructure,
        ssid,
        akm_suite: core::ptr::null(),
        cipher_suite: core::ptr::null(),
    };
    assert_eq!(net.bss_type, Efi80211BssType::Infrastructure);

    let net_desc = Efi80211NetworkDescription {
        network: net,
        network_quality: 95,
    };
    assert_eq!(net_desc.network_quality, 95);
}

#[test]
fn test_wifi2_client_wrapper() {
    let mock_proto = EfiWirelessMacConnectionIiProtocol {
        get_networks: mock_wifi_get_networks,
        connect_network: mock_wifi_connect_network,
        disconnect_network: mock_wifi_disconnect_network,
    };

    let wifi = Wifi2::new(&raw const mock_proto);
    assert_eq!(wifi.raw(), &raw const mock_proto);

    let dummy_event = EfiEvent::from_ptr(core::ptr::null_mut());

    // Test get_networks
    let mut get_data = Efi80211GetNetworksData { num_of_ssid: 0 };
    let mut get_res = Efi80211GetNetworksResult {
        num_of_network_desc: 0,
    };
    let mut get_token = Efi80211GetNetworksToken {
        event: dummy_event,
        status: EfiStatus::NOT_READY,
        data: core::ptr::addr_of_mut!(get_data),
        result: core::ptr::addr_of_mut!(get_res),
    };
    assert!(wifi.get_networks(&mut get_token).is_ok());
    assert_eq!(get_token.status, EfiStatus::SUCCESS);

    // Test connect_network
    let mut conn_data = Efi80211ConnectNetworkData {
        network: core::ptr::null(),
        failure_timeout: 10,
    };
    let mut conn_token = Efi80211ConnectNetworkToken {
        event: dummy_event,
        status: EfiStatus::NOT_READY,
        data: core::ptr::addr_of_mut!(conn_data),
        result_code: Efi80211ConnectNetworkResultCode::Failed,
    };
    assert!(wifi.connect_network(&mut conn_token).is_ok());
    assert_eq!(conn_token.status, EfiStatus::SUCCESS);
    assert_eq!(
        conn_token.result_code,
        Efi80211ConnectNetworkResultCode::Success
    );

    // Test disconnect_network
    let mut disconn_token = Efi80211DisconnectNetworkToken {
        event: dummy_event,
        status: EfiStatus::NOT_READY,
    };
    assert!(wifi.disconnect_network(&mut disconn_token).is_ok());
    assert_eq!(disconn_token.status, EfiStatus::SUCCESS);
}

#[test]
fn test_supplicant_guids_and_client_wrapper() {
    assert_eq!(
        EFI_SUPPLICANT_SERVICE_BINDING_PROTOCOL_GUID,
        EfiGuid::new(
            0x45BC_D98E,
            0x59AD,
            0x4174,
            [0x95, 0x46, 0x34, 0x4A, 0x07, 0x48, 0x58, 0x98],
        )
    );
    assert_eq!(
        EFI_SUPPLICANT_PROTOCOL_GUID,
        EfiGuid::new(
            0x54FC_C43E,
            0xAA89,
            0x4333,
            [0x9A, 0x85, 0xCD, 0xEA, 0x24, 0x05, 0x1E, 0x9E],
        )
    );
    assert_eq!(EFI_MAX_KEY_LEN, 64);

    let refresh = EfiSupplicantKeyRefresh { gtk_refresh: true };
    assert!(refresh.gtk_refresh);

    let key = EfiSupplicantKey {
        key: [0u8; EFI_MAX_KEY_LEN],
        key_len: 16,
        key_id: 1,
        key_type: EfiSupplicantKeyType::Pairwise,
        addr: Efi80211MacAddress { addr: [0; 6] },
        rsc: [0; 8],
        rsc_len: 6,
        is_authenticator: false,
        cipher_suite: Efi80211SuiteSelector {
            oui: [0; 3],
            suite_type: 2,
        },
        direction: EfiSupplicantKeyDirection::Both,
    };
    assert_eq!(key.key_type, EfiSupplicantKeyType::Pairwise);
    assert_eq!(key.direction, EfiSupplicantKeyDirection::Both);

    let mock_proto = EfiSupplicantProtocol {
        build_response_packet: mock_supplicant_build_response,
        process_packet: mock_supplicant_process_packet,
        set_data: mock_supplicant_set_data,
        get_data: mock_supplicant_get_data,
    };

    let supp = Supplicant::new(&raw const mock_proto);
    assert_eq!(supp.raw(), &raw const mock_proto);

    // Test build_response_packet
    let mut resp_buf = [0u8; 8];
    let written = supp.build_response_packet(None, &mut resp_buf).unwrap_or(0);
    assert_eq!(written, 4);
    assert_eq!(&resp_buf[..4], &[0x01, 0x02, 0x03, 0x04]);

    // Test set_data & get_data
    let password = b"SecretPassword12";
    assert!(
        supp.set_data(EfiSupplicantDataType::PskPassword, password)
            .is_ok()
    );

    let mut read_buf = [0u8; 16];
    let read_len = supp
        .get_data(EfiSupplicantDataType::PskPassword, &mut read_buf)
        .unwrap_or(0);
    assert_eq!(read_len, 16);
    assert_eq!(&read_buf, password);
}
