//! # `IPsec` Protocol Tests
//!
//! Unit tests for `IPsec`, `IPsec2`, and `IPsec` Configuration protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiEvent, EfiStatus};
use rover_uefi::protocols::network::ipsec::{
    EFI_IPSEC_CONFIG_PROTOCOL_GUID, EFI_IPSEC_PROTOCOL_GUID, EFI_IPSEC2_PROTOCOL_GUID,
    EfiIpAddressInfo, EfiIpSec2Protocol, EfiIpSecAction, EfiIpSecConfigDataType,
    EfiIpSecConfigProtocol, EfiIpSecFragmentData, EfiIpSecProtocol, EfiIpSecSaLifetime,
    EfiIpSecSpdSelector, EfiIpSecTrafficDir, IpSec, IpSec2, IpSecConfig,
};

#[test]
fn test_ipsec_guids() {
    assert_ne!(EFI_IPSEC_PROTOCOL_GUID, EFI_IPSEC2_PROTOCOL_GUID);
    assert_ne!(EFI_IPSEC_PROTOCOL_GUID, EFI_IPSEC_CONFIG_PROTOCOL_GUID);
    assert_ne!(EFI_IPSEC2_PROTOCOL_GUID, EFI_IPSEC_CONFIG_PROTOCOL_GUID);
}

#[test]
fn test_ipsec_enums_and_structs() {
    assert_eq!(EfiIpSecTrafficDir::Inbound as u32, 0);
    assert_eq!(EfiIpSecTrafficDir::Outbound as u32, 1);

    assert_eq!(EfiIpSecAction::Discard as u32, 0);
    assert_eq!(EfiIpSecAction::Bypass as u32, 1);
    assert_eq!(EfiIpSecAction::Protect as u32, 2);

    assert_eq!(EfiIpSecConfigDataType::Spd as u32, 0);
    assert_eq!(EfiIpSecConfigDataType::Sad as u32, 1);
    assert_eq!(EfiIpSecConfigDataType::Pad as u32, 2);
    assert_eq!(EfiIpSecConfigDataType::Maximum as u32, 3);

    let frag = EfiIpSecFragmentData {
        fragment_length: 64,
        fragment_buffer: core::ptr::null_mut(),
    };
    assert_eq!(frag.fragment_length, 64);

    let addr_info = EfiIpAddressInfo {
        address: [192, 168, 1, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        prefix_length: 24,
    };
    assert_eq!(addr_info.prefix_length, 24);

    let spd = EfiIpSecSpdSelector {
        local_address_count: 1,
        local_address: core::ptr::null_mut(),
        remote_address_count: 0,
        remote_address: core::ptr::null_mut(),
        next_layer_protocol: 6, // TCP
        local_port: 80,
        local_port_range: 0,
        remote_port: 0,
        remote_port_range: 0,
    };
    assert_eq!(spd.local_port, 80);
    assert_eq!(spd.next_layer_protocol, 6);

    let lifetime = EfiIpSecSaLifetime {
        byte_count: 1024 * 1024,
        soft_lifetime: 3600,
        hard_lifetime: 7200,
    };
    assert_eq!(lifetime.hard_lifetime, 7200);
}

unsafe extern "efiapi" fn mock_ipsec_process(
    _this: *mut EfiIpSecProtocol,
    _nic_handle: rover_uefi::base::EfiHandle,
    _ip_ver: u8,
    _ip_head: *mut c_void,
    _last_head: *mut u8,
    _options_buffer: *mut c_void,
    _options_length: u32,
    _fragment_table: *mut *mut EfiIpSecFragmentData,
    _fragment_count: *mut u32,
    _traffic_direction: EfiIpSecTrafficDir,
    _recycle_signal: *mut EfiEvent,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ipsec2_process_ext(
    _this: *mut EfiIpSec2Protocol,
    _nic_handle: rover_uefi::base::EfiHandle,
    _ip_ver: u8,
    _ip_head: *mut c_void,
    _last_head: *mut u8,
    _options_buffer: *mut *mut c_void,
    _options_length: *mut u32,
    _fragment_table: *mut *mut EfiIpSecFragmentData,
    _fragment_count: *mut u32,
    _traffic_direction: EfiIpSecTrafficDir,
    _recycle_signal: *mut EfiEvent,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_ipsec_and_ipsec2_wrappers() {
    let mut raw_ipsec = EfiIpSecProtocol {
        process: mock_ipsec_process,
        disabled_event: EfiEvent::NULL,
        disabled_flag: false,
    };
    let wrapper = IpSec::new(&mut raw_ipsec);
    assert!(!wrapper.is_disabled());
    assert!(!wrapper.raw().is_null());

    let mut raw_ipsec2 = EfiIpSec2Protocol {
        process_ext: mock_ipsec2_process_ext,
        disabled_event: EfiEvent::NULL,
        disabled_flag: true,
    };
    let wrapper2 = IpSec2::new(&mut raw_ipsec2);
    assert!(wrapper2.is_disabled());
    assert!(!wrapper2.raw().is_null());
}

static NOTIFY_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

unsafe extern "efiapi" fn mock_config_set_data(
    _this: *mut EfiIpSecConfigProtocol,
    _data_type: EfiIpSecConfigDataType,
    _selector: *const c_void,
    _data: *const c_void,
    _selector_for_insert: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_config_get_data(
    _this: *mut EfiIpSecConfigProtocol,
    _data_type: EfiIpSecConfigDataType,
    _selector: *const c_void,
    data_size: *mut UIntN,
    data: *mut c_void,
) -> EfiStatus {
    unsafe {
        if data_size.is_null() {
            return EfiStatus::INVALID_PARAMETER;
        }
        if *data_size < 4 {
            *data_size = 4;
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        *data_size = 4;
        if !data.is_null() {
            let out = data.cast::<u8>();
            *out.add(0) = 0xAA;
            *out.add(1) = 0xBB;
            *out.add(2) = 0xCC;
            *out.add(3) = 0xDD;
        }
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_config_get_next_selector(
    _this: *mut EfiIpSecConfigProtocol,
    _data_type: EfiIpSecConfigDataType,
    _selector_size: *mut UIntN,
    _selector: *mut c_void,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

unsafe extern "efiapi" fn mock_config_register_notify(
    _this: *mut EfiIpSecConfigProtocol,
    _data_type: EfiIpSecConfigDataType,
    _event: EfiEvent,
) -> EfiStatus {
    NOTIFY_REGISTERED.store(true, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_config_unregister_notify(
    _this: *mut EfiIpSecConfigProtocol,
    _data_type: EfiIpSecConfigDataType,
    _event: EfiEvent,
) -> EfiStatus {
    NOTIFY_REGISTERED.store(false, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

#[test]
fn test_ipsec_config_wrapper() {
    let mut raw_config = EfiIpSecConfigProtocol {
        set_data: mock_config_set_data,
        get_data: mock_config_get_data,
        get_next_selector: mock_config_get_next_selector,
        register_data_notify: mock_config_register_notify,
        unregister_data_notify: mock_config_unregister_notify,
    };

    let mut config = IpSecConfig::new(&mut raw_config);
    assert!(!config.raw().is_null());

    let mut buf = [0u8; 8];
    let res =
        unsafe { config.get_data_raw(EfiIpSecConfigDataType::Spd, core::ptr::null(), &mut buf) };
    assert_eq!(res, Ok(4));
    assert_eq!(&buf[0..4], &[0xAA, 0xBB, 0xCC, 0xDD]);

    let dummy_event = EfiEvent::from_ptr(0x1234 as *mut c_void);
    assert!(
        config
            .register_notify(EfiIpSecConfigDataType::Spd, dummy_event)
            .is_ok()
    );
    assert!(NOTIFY_REGISTERED.load(std::sync::atomic::Ordering::SeqCst));

    assert!(
        config
            .unregister_notify(EfiIpSecConfigDataType::Spd, dummy_event)
            .is_ok()
    );
    assert!(!NOTIFY_REGISTERED.load(std::sync::atomic::Ordering::SeqCst));
}
