//! Integration tests for EAP, EAP Configuration, and EAP Management Protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::network::eap::{
    EFI_EAP_CONFIGURATION_PROTOCOL_GUID, EFI_EAP_MANAGEMENT_PROTOCOL_GUID,
    EFI_EAP_MANAGEMENT2_PROTOCOL_GUID, EFI_EAP_PROTOCOL_GUID, Eap, EapConfigDataType,
    EapConfiguration, EapManagement2, EapolSupplicantPaeState, EfiEapConfigurationProtocol,
    EfiEapManagement2Protocol, EfiEapProtocol, eap_type, pae_capabilities, supplicant_field_mask,
};

#[test]
fn test_eap_guids_and_constants() {
    assert_eq!(
        EFI_EAP_PROTOCOL_GUID,
        EfiGuid::new(
            0x5D9F_96DB,
            0xE731,
            0x4CAA,
            [0xA0, 0x0D, 0x72, 0xE1, 0x87, 0xCD, 0x77, 0x62],
        )
    );
    assert_eq!(
        EFI_EAP_CONFIGURATION_PROTOCOL_GUID,
        EfiGuid::new(
            0xE5B5_8DBB,
            0x7688,
            0x44B4,
            [0x97, 0xBF, 0x5F, 0x1D, 0x4B, 0x7C, 0xC8, 0xDB],
        )
    );
    assert_eq!(
        EFI_EAP_MANAGEMENT_PROTOCOL_GUID,
        EfiGuid::new(
            0xBB62_E663,
            0x625D,
            0x40B2,
            [0xA0, 0x88, 0xBB, 0xE8, 0x36, 0x23, 0xA2, 0x45],
        )
    );
    assert_eq!(
        EFI_EAP_MANAGEMENT2_PROTOCOL_GUID,
        EfiGuid::new(
            0x5E93_C847,
            0x456D,
            0x40B3,
            [0xA6, 0xB4, 0x78, 0xB0, 0xC9, 0xCF, 0x7F, 0x20],
        )
    );

    assert_eq!(eap_type::IDENTITY, 1);
    assert_eq!(eap_type::TLS, 13);
    assert_eq!(eap_type::PEAP, 25);
    assert_eq!(eap_type::MSCHAPV2, 26);

    assert_eq!(pae_capabilities::AUTHENTICATOR, 0x01);
    assert_eq!(pae_capabilities::SUPPLICANT, 0x02);

    assert_eq!(supplicant_field_mask::AUTH_PERIOD, 0x01);
    assert_eq!(supplicant_field_mask::HELD_PERIOD, 0x02);
    assert_eq!(supplicant_field_mask::START_PERIOD, 0x04);
    assert_eq!(supplicant_field_mask::MAX_START, 0x08);

    assert_eq!(EapolSupplicantPaeState::Logoff as u32, 0);
    assert_eq!(EapolSupplicantPaeState::Connecting as u32, 2);
    assert_eq!(EapolSupplicantPaeState::Authenticated as u32, 6);

    assert_eq!(EapConfigDataType::EapAuthMethod as u32, 0);
    assert_eq!(EapConfigDataType::IdentityString as u32, 2);
    assert_eq!(EapConfigDataType::EapTlsCaCert as u32, 3);
}

use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};

static MOCK_DESIRED_AUTH_TYPE: AtomicU8 = AtomicU8::new(0);

unsafe extern "efiapi" fn mock_set_desired_auth_method(
    _this: *mut EfiEapProtocol,
    eap_auth_type: u8,
) -> EfiStatus {
    MOCK_DESIRED_AUTH_TYPE.store(eap_auth_type, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_auth_method(
    _this: *mut EfiEapProtocol,
    _eap_auth_type: u8,
    _handler: rover_uefi::protocols::network::eap::EfiEapBuildResponsePacket,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_packet_builder(
    _port_number: *mut c_void,
    _request_buffer: *const u8,
    _request_size: UIntN,
    _buffer: *mut u8,
    _buffer_size: *mut UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_eap_client_wrapper() {
    let mut raw_proto = EfiEapProtocol {
        set_desired_auth_method: mock_set_desired_auth_method,
        register_auth_method: mock_register_auth_method,
    };

    let mut client = Eap::new(&mut raw_proto);
    assert!(!client.raw().is_null());

    let res = client.set_desired_auth_method(eap_type::TLS);
    assert!(res.is_ok());
    assert_eq!(MOCK_DESIRED_AUTH_TYPE.load(Ordering::SeqCst), eap_type::TLS);

    let reg_res = client.register_auth_method(eap_type::TLS, dummy_packet_builder);
    assert!(reg_res.is_ok());
}

static MOCK_CONFIG_DATA: Mutex<[u8; 16]> = Mutex::new([0x55; 16]);

unsafe extern "efiapi" fn mock_set_data(
    _this: *mut EfiEapConfigurationProtocol,
    _eap_type: u8,
    _data_type: EapConfigDataType,
    data: *const c_void,
    data_size: UIntN,
) -> EfiStatus {
    if data.is_null() || data_size == 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    let src = data.cast::<u8>();
    let copy_len = if data_size > 16 { 16 } else { data_size };
    if let Ok(mut guard) = MOCK_CONFIG_DATA.lock() {
        for (i, item) in guard.iter_mut().enumerate().take(copy_len) {
            *item = unsafe { *src.add(i) };
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_data(
    _this: *mut EfiEapConfigurationProtocol,
    _eap_type: u8,
    _data_type: EapConfigDataType,
    data: *mut c_void,
    data_size: *mut UIntN,
) -> EfiStatus {
    if data.is_null() || data_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let current_size = unsafe { *data_size };
    if current_size < 16 {
        unsafe {
            *data_size = 16;
        }
        return EfiStatus::BUFFER_TOO_SMALL;
    }
    unsafe {
        *data_size = 16;
    }
    let dest = data.cast::<u8>();
    if let Ok(guard) = MOCK_CONFIG_DATA.lock() {
        for (i, item) in guard.iter().enumerate() {
            unsafe {
                *dest.add(i) = *item;
            }
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_eap_configuration_client_wrapper() {
    let mut raw_proto = EfiEapConfigurationProtocol {
        set_data: mock_set_data,
        get_data: mock_get_data,
    };

    let mut client = EapConfiguration::new(&mut raw_proto);
    assert!(!client.raw().is_null());

    let payload = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let set_res = client.set_data(eap_type::TLS, EapConfigDataType::IdentityString, &payload);
    assert!(set_res.is_ok());

    let mut small_buf = [0u8; 8];
    let small_res = client.get_data(
        eap_type::TLS,
        EapConfigDataType::IdentityString,
        &mut small_buf,
    );
    assert_eq!(small_res, Err(EfiStatus::BUFFER_TOO_SMALL));

    let mut full_buf = [0u8; 32];
    let get_res = client.get_data(
        eap_type::TLS,
        EapConfigDataType::IdentityString,
        &mut full_buf,
    );
    assert!(get_res.is_ok());
    if let Ok(written) = get_res {
        assert_eq!(written, 16);
        assert_eq!(full_buf[0], 1);
        assert_eq!(full_buf[7], 8);
    }
}

unsafe extern "efiapi" fn mock_noop_management(
    _this: *mut EfiEapManagement2Protocol,
    _param: *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_noop_management_const(
    _this: *mut EfiEapManagement2Protocol,
    _param: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_key(
    _this: *mut EfiEapManagement2Protocol,
    msk: *mut u8,
    msk_size: *mut UIntN,
    emsk: *mut u8,
    emsk_size: *mut UIntN,
) -> EfiStatus {
    if msk.is_null() || msk_size.is_null() || emsk.is_null() || emsk_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *msk_size = 64;
        *emsk_size = 64;
        let msk_ptr = msk;
        let emsk_ptr = emsk;
        for i in 0..64 {
            *msk_ptr.add(i) = 0x11;
            *emsk_ptr.add(i) = 0x22;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_eap_management2_client_wrapper() {
    let mut raw_proto = EfiEapManagement2Protocol {
        get_system_configuration: mock_noop_management,
        set_system_configuration: mock_noop_management_const,
        initialize_port: mock_noop_management,
        user_logon: mock_noop_management,
        user_logoff: mock_noop_management,
        get_supplicant_status: mock_noop_management,
        set_supplicant_configuration: mock_noop_management_const,
        get_supplicant_statistics: mock_noop_management,
        get_key: mock_get_key,
    };

    let mut client = EapManagement2::new(&mut raw_proto);
    let mut msk_buf = [0u8; 64];
    let mut emsk_buf = [0u8; 64];

    let key_res = client.get_key(&mut msk_buf, &mut emsk_buf);
    assert!(key_res.is_ok());
    if let Ok((msk_len, emsk_len)) = key_res {
        assert_eq!(msk_len, 64);
        assert_eq!(emsk_len, 64);
        assert_eq!(msk_buf[0], 0x11);
        assert_eq!(emsk_buf[0], 0x22);
    }
}
